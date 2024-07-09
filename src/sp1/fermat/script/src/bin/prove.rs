//! An end-to-end example of using the SP1 SDK to generate a proof of a Fermat test program that can be verified on-chain.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --package fermat-script --bin prove --release
//! ```

use alloy_sol_types::{sol, SolType};
use clap::Parser;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use sp1_sdk::{HashableKey, ProverClient, SP1PlonkBn254Proof, SP1Stdin, SP1VerifyingKey};
use std::path::PathBuf;

/// The ELF (executable and linkable format) file for the Fermat test zkVM.
///
/// This file is generated by running `cargo prove build` inside the `program` directory.
pub const FERMAT_ELF: &[u8] = include_bytes!("../../../program/elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the prove command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ProveArgs {
    #[clap(long, default_value = "1212345")]
    n: u32,
    #[clap(long, default_value = "150")]
    num_of_rounds: u32,
    #[clap(long, default_value = "123131")]
    seed: u64,
}

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    tuple(uint32, bool)
};

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = ProveArgs::parse();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the program.
    let (pk, vk) = client.setup(FERMAT_ELF);

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    // generate a random seed:
    let seed = args.seed;

    println!("seed: {:?}", seed);
    // convert the seed to [u32 ; 8]

    // to be_bytes() returns a [u8; 8] array of the seed meaning that the seed is 8 bytes long
    let seed_arr = seed.to_be_bytes();
    // write the seed to stdin
    stdin.write(&seed_arr);
    stdin.write(&args.num_of_rounds);
    stdin.write(&args.n);

    println!(
        "Generating proof for n = {}, num_of_rounds = {}, seed = {}",
        args.n, args.num_of_rounds, args.seed
    );

    // Generate the proof.
    let proof = client.prove(&pk, stdin).expect("failed to generate proof");
    let (n, is_prime) =
        PublicValuesTuple::abi_decode(proof.public_values.as_slice(), false).unwrap();
    println!("Successfully generated proof!");
    println!("n: {}, is_prime: {}", n, is_prime);

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
}
