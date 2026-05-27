use forge_query::facade::{
    ForgeQueryBindingAuthorityWitness, ForgeQueryBindingBasisWitness,
    ForgeQueryBindingFamilyWitness, ForgeQueryBindingTargetWitnessSet,
};

fn main() {
    let _ = ForgeQueryBindingAuthorityWitness {
        handle_identity_digest: String::from("handle"),
        operating_context_identity_digest: String::from("world"),
    };
    let _ = ForgeQueryBindingBasisWitness {
        basis_label: "basis",
    };
    let _ = ForgeQueryBindingTargetWitnessSet {
        binding_digests: vec![String::from("binding")],
    };
    let _ = ForgeQueryBindingFamilyWitness {
        family_key: "family",
    };
}
