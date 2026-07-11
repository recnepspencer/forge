use forge_proof::prelude::{Recipe, Unresolved};
use forge_store_security::StoreSecurityWitnessVocabulary;

fn require_security_witness(_: StoreSecurityWitnessVocabulary) {}

fn main() {
    let proof_progression = Recipe::<Unresolved, _>::new("proof is not store authority");
    require_security_witness(proof_progression);
}
