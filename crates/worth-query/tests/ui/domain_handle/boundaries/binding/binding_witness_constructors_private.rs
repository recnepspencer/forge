use worth_query::facade::foundation::{WorthQueryBindingAuthorityWitness, WorthQueryBindingBasisWitness, WorthQueryBindingFamilyWitness, WorthQueryBindingTargetWitnessSet};

fn main() {
    let _ = WorthQueryBindingAuthorityWitness {
        handle_identity_digest: String::from("handle"),
        operating_context_identity_digest: String::from("world"),
    };
    let _ = WorthQueryBindingBasisWitness {
        basis_label: "basis",
    };
    let _ = WorthQueryBindingTargetWitnessSet {
        binding_digests: vec![String::from("binding")],
    };
    let _ = WorthQueryBindingFamilyWitness {
        family_key: "family",
    };
}
