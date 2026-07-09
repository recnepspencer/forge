use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::logic::transaction::{
    SignalMergeCompatibilityFactInventory, SignalMergeCompatibilityWitness,
};

#[derive(Serialize, Deserialize)]
struct CompatibilityWitnessRecord {
    schema_version: String,
    fact_inventory: SignalMergeCompatibilityFactInventory,
    compatibility_digest: String,
}

pub fn serialize<S>(
    witness: &SignalMergeCompatibilityWitness,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    CompatibilityWitnessRecord {
        schema_version: witness.schema_version().to_owned(),
        fact_inventory: witness.fact_inventory().clone(),
        compatibility_digest: witness.compatibility_digest().to_owned(),
    }
    .serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<SignalMergeCompatibilityWitness, D::Error>
where
    D: Deserializer<'de>,
{
    let record = CompatibilityWitnessRecord::deserialize(deserializer)?;
    SignalMergeCompatibilityWitness::try_from_replay_record(
        record.schema_version,
        record.fact_inventory,
        record.compatibility_digest,
    )
    .map_err(serde::de::Error::custom)
}
