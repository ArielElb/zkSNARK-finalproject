use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget};
use ark_ff::PrimeField;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBytesGadget;
use ark_relations::r1cs::ConstraintSystemRef;

const K: usize = 10;

// hash to bytes:
pub fn hash_to_bytes<ConstraintF: PrimeField>(
    x_plus_j: FpVar<ConstraintF>,
) -> DigestVar<ConstraintF> {
    let mut sha256_var = Sha256Gadget::default();
    // convert x_plus_j to bytes:
    let x_plus_j_bytes = x_plus_j.to_bytes().unwrap();
    // calculate the hash(x+j):
    sha256_var.update(&x_plus_j_bytes).unwrap();

    let result = sha256_var.finalize().unwrap();
    result
}
pub fn generate_bases_a<ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    r: FpVar<ConstraintF>,
) -> Vec<FpVar<ConstraintF>> {
    let mut a_j_s = vec![];
    for j in 0..K {
        let mut sha256_var = Sha256Gadget::default();
        let r = r.to_bytes().unwrap();
        let j_bytes = FpVar::<ConstraintF>::constant(ConstraintF::from(j as u64))
            .to_bytes()
            .unwrap();
        sha256_var.update(&r).unwrap();
        sha256_var.update(&j_bytes).unwrap();
        let result: DigestVar<ConstraintF> = sha256_var.finalize().unwrap(); // a_i = hash(r || j)
        let a_j_fpvar =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "r_j_fpvar"), || {
                Ok(ConstraintF::from_le_bytes_mod_order(
                    &result.to_bytes().unwrap().value().unwrap(),
                ))
            })
            .unwrap();
        a_j_s.push(a_j_fpvar);
    }
    a_j_s
}
