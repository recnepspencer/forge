use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::logic::transaction::{
    CausalityCarryPolicy, RetainedArtifactCarryPolicy, RuntimeArtifactCarryPolicy,
};
use crate::logic::transaction::{
    SignalAspectPolicyInventoryEntry, SignalDeliveryStrategyIdentity,
    SignalInvalidationStrategyIdentity, SignalMergeStrategyIdentity, SignalMergeStrategyWitness,
};
use forge_proof::TransitionOutcome;

#[derive(Serialize, Deserialize)]
struct StrategyWitnessRecord {
    merge_strategy: MergeStrategyIdentityRecord,
    invalidation_strategy: InvalidationStrategyIdentityRecord,
    delivery_strategy: DeliveryStrategyIdentityRecord,
}

#[derive(Serialize, Deserialize)]
struct MergeStrategyIdentityRecord {
    merge_strategy: crate::logic::transaction::BranchMergeStrategy,
    selected_strategy_name: crate::logic::transaction::MergeStrategyName,
    selected_strategy_digest: String,
    selected_strategy_basis: crate::logic::transaction::MergeStrategySelectionBasis,
    merge_base_name: crate::logic::transaction::MergeBaseStrategyName,
    merge_base_digest: String,
    merge_base_basis: crate::logic::transaction::MergeBaseSelectionBasis,
    lowered_strategy_bundle_digest: String,
}

#[derive(Serialize, Deserialize)]
struct InvalidationStrategyIdentityRecord {
    boundary_witness_kind: crate::logic::transaction::MergeBoundaryWitnessKind,
    conflict_isolation_name: crate::logic::transaction::ConflictIsolationPolicyName,
    conflict_isolation_digest: String,
    conflict_isolation_basis: crate::logic::transaction::ConflictIsolationSelectionBasis,
    identity_matcher_name: crate::logic::transaction::IdentityMatcherName,
    identity_matcher_digest: String,
    identity_matcher_basis: crate::logic::transaction::IdentityMatcherSelectionBasis,
}

#[derive(Serialize, Deserialize)]
struct DeliveryStrategyIdentityRecord {
    conflict_policy_name: crate::logic::transaction::ConflictPolicyName,
    conflict_policy_digest: String,
    conflict_policy_basis: crate::logic::transaction::ConflictPolicySelectionBasis,
    source_only_policy_name: crate::logic::transaction::SourceOnlyPolicyName,
    source_only_policy_digest: String,
    source_only_policy_basis: crate::logic::transaction::SourceOnlyPolicySelectionBasis,
    deletion_policy_name: crate::logic::transaction::DeletionPolicyName,
    deletion_policy_digest: String,
    deletion_policy_basis: crate::logic::transaction::DeletionPolicySelectionBasis,
    aspect_policy_inventory: Vec<AspectPolicyInventoryRecord>,
    runtime_artifact_carry_policies: Vec<RuntimeArtifactCarryPolicy>,
    retained_artifact_carry_policies: Vec<RetainedArtifactCarryPolicy>,
    causality_carry_policies: Vec<CausalityCarryPolicy>,
}

#[derive(Serialize, Deserialize)]
struct AspectPolicyInventoryRecord {
    policy_name: crate::logic::transaction::AspectMergePolicyName,
    policy_digest: String,
    policy_basis: crate::logic::transaction::AspectMergePolicySelectionBasis,
}

pub fn serialize<S>(witness: &SignalMergeStrategyWitness, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    StrategyWitnessRecord::from(witness).serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<SignalMergeStrategyWitness, D::Error>
where
    D: Deserializer<'de>,
{
    let record = StrategyWitnessRecord::deserialize(deserializer)?;
    decode_record(record)
}

pub fn serialize_option<S>(
    witness: &Option<SignalMergeStrategyWitness>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    witness
        .as_ref()
        .map(StrategyWitnessRecord::from)
        .serialize(serializer)
}

pub fn deserialize_option<'de, D>(
    deserializer: D,
) -> Result<Option<SignalMergeStrategyWitness>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<StrategyWitnessRecord>::deserialize(deserializer)?
        .map(decode_record)
        .transpose()
}

fn decode_record<E>(record: StrategyWitnessRecord) -> Result<SignalMergeStrategyWitness, E>
where
    E: serde::de::Error,
{
    let merge_strategy = SignalMergeStrategyIdentity::new(
        record.merge_strategy.merge_strategy,
        record.merge_strategy.selected_strategy_name,
        record.merge_strategy.selected_strategy_digest,
        record.merge_strategy.selected_strategy_basis,
        record.merge_strategy.merge_base_name,
        record.merge_strategy.merge_base_digest,
        record.merge_strategy.merge_base_basis,
        record.merge_strategy.lowered_strategy_bundle_digest,
    )
    .map_err(|denial| E::custom(denial.message()))?;
    let invalidation_strategy = SignalInvalidationStrategyIdentity::new(
        record.invalidation_strategy.boundary_witness_kind,
        record.invalidation_strategy.conflict_isolation_name,
        record.invalidation_strategy.conflict_isolation_digest,
        record.invalidation_strategy.conflict_isolation_basis,
        record.invalidation_strategy.identity_matcher_name,
        record.invalidation_strategy.identity_matcher_digest,
        record.invalidation_strategy.identity_matcher_basis,
    )
    .map_err(|denial| E::custom(denial.message()))?;
    let delivery_strategy = SignalDeliveryStrategyIdentity::new(
        record.delivery_strategy.conflict_policy_name,
        record.delivery_strategy.conflict_policy_digest,
        record.delivery_strategy.conflict_policy_basis,
        record.delivery_strategy.source_only_policy_name,
        record.delivery_strategy.source_only_policy_digest,
        record.delivery_strategy.source_only_policy_basis,
        record.delivery_strategy.deletion_policy_name,
        record.delivery_strategy.deletion_policy_digest,
        record.delivery_strategy.deletion_policy_basis,
        record
            .delivery_strategy
            .aspect_policy_inventory
            .into_iter()
            .map(|entry| {
                SignalAspectPolicyInventoryEntry::new(
                    entry.policy_name,
                    entry.policy_digest,
                    entry.policy_basis,
                )
            })
            .collect(),
        record.delivery_strategy.runtime_artifact_carry_policies,
        record.delivery_strategy.retained_artifact_carry_policies,
        record.delivery_strategy.causality_carry_policies,
    )
    .map_err(|denial| E::custom(denial.message()))?;

    match SignalMergeStrategyWitness::try_from_identities(
        Some(merge_strategy),
        Some(invalidation_strategy),
        Some(delivery_strategy),
    ) {
        TransitionOutcome::Success(witness) => Ok(witness),
        TransitionOutcome::Denied(denial) => Err(E::custom(denial.message())),
        outcome => Err(E::custom(format!(
            "unexpected strategy witness outcome during replay decode: {outcome:?}"
        ))),
    }
}

impl From<&SignalMergeStrategyWitness> for StrategyWitnessRecord {
    fn from(witness: &SignalMergeStrategyWitness) -> Self {
        Self {
            merge_strategy: MergeStrategyIdentityRecord {
                merge_strategy: witness.merge_strategy().merge_strategy(),
                selected_strategy_name: witness.merge_strategy().selected_strategy_name().clone(),
                selected_strategy_digest: witness
                    .merge_strategy()
                    .selected_strategy_digest()
                    .to_owned(),
                selected_strategy_basis: witness.merge_strategy().selected_strategy_basis(),
                merge_base_name: witness.merge_strategy().merge_base_name().clone(),
                merge_base_digest: witness.merge_strategy().merge_base_digest().to_owned(),
                merge_base_basis: witness.merge_strategy().merge_base_basis(),
                lowered_strategy_bundle_digest: witness
                    .merge_strategy()
                    .lowered_strategy_bundle_digest()
                    .to_owned(),
            },
            invalidation_strategy: InvalidationStrategyIdentityRecord {
                boundary_witness_kind: witness.invalidation_strategy().boundary_witness_kind(),
                conflict_isolation_name: witness
                    .invalidation_strategy()
                    .conflict_isolation_name()
                    .clone(),
                conflict_isolation_digest: witness
                    .invalidation_strategy()
                    .conflict_isolation_digest()
                    .to_owned(),
                conflict_isolation_basis: witness
                    .invalidation_strategy()
                    .conflict_isolation_basis(),
                identity_matcher_name: witness
                    .invalidation_strategy()
                    .identity_matcher_name()
                    .clone(),
                identity_matcher_digest: witness
                    .invalidation_strategy()
                    .identity_matcher_digest()
                    .to_owned(),
                identity_matcher_basis: witness.invalidation_strategy().identity_matcher_basis(),
            },
            delivery_strategy: DeliveryStrategyIdentityRecord {
                conflict_policy_name: witness.delivery_strategy().conflict_policy_name().clone(),
                conflict_policy_digest: witness
                    .delivery_strategy()
                    .conflict_policy_digest()
                    .to_owned(),
                conflict_policy_basis: witness.delivery_strategy().conflict_policy_basis(),
                source_only_policy_name: witness
                    .delivery_strategy()
                    .source_only_policy_name()
                    .clone(),
                source_only_policy_digest: witness
                    .delivery_strategy()
                    .source_only_policy_digest()
                    .to_owned(),
                source_only_policy_basis: witness.delivery_strategy().source_only_policy_basis(),
                deletion_policy_name: witness.delivery_strategy().deletion_policy_name().clone(),
                deletion_policy_digest: witness
                    .delivery_strategy()
                    .deletion_policy_digest()
                    .to_owned(),
                deletion_policy_basis: witness.delivery_strategy().deletion_policy_basis(),
                aspect_policy_inventory: witness
                    .delivery_strategy()
                    .aspect_policy_inventory()
                    .iter()
                    .map(|entry| AspectPolicyInventoryRecord {
                        policy_name: entry.policy_name().clone(),
                        policy_digest: entry.policy_digest().to_owned(),
                        policy_basis: entry.policy_basis(),
                    })
                    .collect(),
                runtime_artifact_carry_policies: witness
                    .delivery_strategy()
                    .runtime_artifact_carry_policies()
                    .to_vec(),
                retained_artifact_carry_policies: witness
                    .delivery_strategy()
                    .retained_artifact_carry_policies()
                    .to_vec(),
                causality_carry_policies: witness
                    .delivery_strategy()
                    .causality_carry_policies()
                    .to_vec(),
            },
        }
    }
}
