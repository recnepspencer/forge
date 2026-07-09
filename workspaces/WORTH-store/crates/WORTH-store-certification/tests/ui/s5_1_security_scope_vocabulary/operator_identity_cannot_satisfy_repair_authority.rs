use worth_store_security::{StoreOperatorIdentityClaim, StoreSecurityWitnessVocabulary};

fn require_repair_authority(_: StoreSecurityWitnessVocabulary) {}

fn main() {
    require_repair_authority(StoreOperatorIdentityClaim::raw("operator-123"));
}
