fn require_deserialize<T: serde::de::DeserializeOwned>() {}

fn main() {
    require_deserialize::<worth_store::RetainedHeadSet>();
    require_deserialize::<worth_store::StableBasisSet>();
    require_deserialize::<worth_store::RetentionClosureWitness>();
    require_deserialize::<worth_store::PolicyExpiredAuthorityRange>();
    require_deserialize::<worth_store::CompactionCutoverWitness>();
    require_deserialize::<worth_store::ReclaimEligibilityWitness>();
    require_deserialize::<worth_store::BasisSurvivalVerdict>();
}
