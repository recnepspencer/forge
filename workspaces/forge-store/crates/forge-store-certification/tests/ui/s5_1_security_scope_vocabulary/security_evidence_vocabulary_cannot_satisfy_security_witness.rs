use forge_store_security::{StoreSecurityEvidenceVocabulary, StoreSecurityWitnessVocabulary};

fn require_security_witness(_: StoreSecurityWitnessVocabulary) {}

fn main() {
    require_security_witness(StoreSecurityEvidenceVocabulary::PublishableBoundaryEvidence);
}
