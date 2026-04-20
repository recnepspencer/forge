fn require_deserialize<T: serde::de::DeserializeOwned>() {}

fn main() {
    require_deserialize::<forge_store::RetainedHeadSet>();
    require_deserialize::<forge_store::StableBasisSet>();
    require_deserialize::<forge_store::RetentionClosureWitness>();
    require_deserialize::<forge_store::PolicyExpiredAuthorityRange>();
    require_deserialize::<forge_store::CompactionCutoverWitness>();
    require_deserialize::<forge_store::ReclaimEligibilityWitness>();
    require_deserialize::<forge_store::BasisSurvivalVerdict>();
}
