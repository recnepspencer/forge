use crate::data::core_profile::StableHashValue;
use crate::data::graph::signal_graph::SignalGraph;
use crate::data::output::{CanonicalChangedRegions, OutputChange};
use crate::data::trace::{ColdArtifactIntent, COLD_ARTIFACT_INTENT_LABEL_LIMIT};
use crate::diagnostics::policy::ArtifactRetentionPolicy;
use crate::logic::evaluation::{EvaluationEffect, EvaluationVerdict, SuppressionReason};
use smallvec::SmallVec;

pub(super) fn count_changed_partitions(
    changed_regions: &[crate::data::output::ChangedRegion],
) -> u32 {
    let mut partitions: SmallVec<[crate::data::output::PartitionToken; 4]> = SmallVec::new();
    for region in changed_regions {
        if partitions
            .iter()
            .any(|partition| partition == &region.partition)
        {
            continue;
        }
        partitions.push(region.partition.clone());
    }
    partitions.len() as u32
}

pub(super) fn verdict_retains_runtime_artifact(verdict: &EvaluationVerdict) -> bool {
    matches!(
        verdict,
        EvaluationVerdict::Recomputed
            | EvaluationVerdict::Suppressed {
                reason: SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            }
    )
}

pub(super) fn verdict_transitions_clean(verdict: &EvaluationVerdict) -> bool {
    matches!(
        verdict,
        EvaluationVerdict::Recomputed | EvaluationVerdict::Suppressed { .. }
    )
}

pub(super) fn verdict_commits_snapshot(verdict: &EvaluationVerdict) -> bool {
    verdict_retains_runtime_artifact(verdict)
}

pub(super) fn normalize_output_change(
    declared: OutputChange,
    output_identity_unchanged: bool,
    has_output_identity: bool,
) -> OutputChange {
    if has_output_identity && output_identity_unchanged {
        OutputChange::Unchanged
    } else {
        declared
    }
}

pub(super) fn trace_identity_hash(
    identity: &crate::data::output::OutputIdentity,
) -> StableHashValue {
    identity.stable_hash()
}

pub(super) fn trace_output_hash(version: crate::data::aspect::AspectVersion) -> StableHashValue {
    let mut hash = 0xcbf29ce484222325_u128;
    for slot in version.slots() {
        hash ^= *slot as u128;
        hash = hash.wrapping_mul(0x100000001b3_u128);
    }
    hash as StableHashValue
}

pub(super) fn build_cold_artifact_intent(
    effect: &EvaluationEffect,
    retention: &crate::diagnostics::policy::RetentionBudget,
) -> Option<ColdArtifactIntent> {
    if matches!(
        retention.explanation_retention,
        ArtifactRetentionPolicy::Omit
    ) && matches!(
        retention.provenance_retention,
        ArtifactRetentionPolicy::Omit
    ) {
        return None;
    }
    let retain_reuse_boundary_detail = matches!(
        effect.operational.reuse_basis.strategy,
        Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
            | Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
    );
    let labels = if matches!(
        retention.explanation_retention,
        ArtifactRetentionPolicy::Retain
    ) || matches!(
        retention.provenance_retention,
        ArtifactRetentionPolicy::Retain
    ) {
        effect
            .labels()
            .iter()
            .take(COLD_ARTIFACT_INTENT_LABEL_LIMIT)
            .cloned()
            .collect()
    } else {
        SmallVec::new()
    };
    let intent = ColdArtifactIntent {
        changed_regions: CanonicalChangedRegions::from_slice(effect.changed_regions()),
        labels,
        keyed_family: effect.keyed_context().and_then(|keyed| {
            keyed
                .family
                .as_ref()
                .map(|family| family.as_str().to_owned())
        }),
        keyed_key: effect
            .keyed_context()
            .and_then(|keyed| keyed.key.as_ref().map(|key| key.as_str().to_owned())),
        reuse_certification: effect.reuse_certification().cloned(),
        reuse_boundary_context: retain_reuse_boundary_detail
            .then(|| effect.reuse_boundary_detail().cloned())
            .flatten(),
    };
    (!intent.is_empty()).then_some(intent)
}

pub(super) fn runtime_policy_omits_cold_artifacts(graph: &SignalGraph) -> bool {
    let retention = graph.runtime_policy().retention_budget;
    matches!(
        retention.explanation_retention,
        ArtifactRetentionPolicy::Omit
    ) && matches!(
        retention.provenance_retention,
        ArtifactRetentionPolicy::Omit
    )
}

pub(super) fn record_reuse_telemetry(
    telemetry: &mut crate::data::telemetry::RuntimeTelemetry,
    effect: &EvaluationEffect,
) {
    telemetry.evaluation.reuse_eligibility_checks_attempted += 1;
    match effect.operational.reuse_origin {
        crate::data::reuse::ReuseOrigin::FreshCompute => {
            telemetry.evaluation.fresh_compute_count += 1
        }
        crate::data::reuse::ReuseOrigin::OutputSuppressed => {
            telemetry.evaluation.output_suppressed_count += 1
        }
        crate::data::reuse::ReuseOrigin::MemoizedArtifactReuse => {
            telemetry.evaluation.memoized_reuse_count += 1
        }
        crate::data::reuse::ReuseOrigin::SnapshotRestore => {
            telemetry.evaluation.snapshot_restore_reuse_count += 1
        }
        crate::data::reuse::ReuseOrigin::ReconciliationAdoption => {
            telemetry.evaluation.reconciliation_adoption_count += 1
        }
        crate::data::reuse::ReuseOrigin::CrossIdentityPersistentReuse => {
            telemetry.evaluation.cross_identity_reuse_count += 1
        }
        crate::data::reuse::ReuseOrigin::PartialArtifactSplice => {
            telemetry.evaluation.partial_artifact_splice_count += 1
        }
    }
    telemetry.evaluation.reuse_dependency_comparison_breadth +=
        u64::from(effect.operational.meaningful_input_changes);
    if effect.reuse_certification().is_some() {
        telemetry
            .evaluation
            .reuse_cold_certification_materialization_count += 1;
    }
}
