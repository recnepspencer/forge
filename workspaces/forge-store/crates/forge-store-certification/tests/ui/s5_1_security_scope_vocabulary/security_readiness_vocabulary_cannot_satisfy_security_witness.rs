use forge_store_security::{StoreSecurityReadinessVocabulary, StoreSecurityWitnessVocabulary};

fn require_security_witness(_: StoreSecurityWitnessVocabulary) {}

fn main() {
    let readiness: StoreSecurityReadinessVocabulary = unimplemented!();
    require_security_witness(readiness);
}
