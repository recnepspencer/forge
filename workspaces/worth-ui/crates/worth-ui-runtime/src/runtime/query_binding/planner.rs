use std::collections::BTreeSet;

use crate::runtime::query_binding::comparison::{
    WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonCounters,
    WorthUiQueryBindingComparisonDenial, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome,
};
use crate::runtime::query_binding::evidence::WorthUiQueryBindingEvidenceIndex;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiNodeReplacementPlan, WorthUiRuntimeImpactNarrowing,
};
use crate::source::WorthUiArtifact;

pub(crate) struct WorthUiQueryBindingComparisonPlanner;

impl WorthUiQueryBindingComparisonPlanner {
    pub(crate) fn compare(
        active_artifact: &WorthUiArtifact,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial> {
        reject_ambiguous_node_plan(node_plan)?;
        reject_digest_mismatch(active_artifact, node_plan, admitted)?;
        reject_narrowing_mismatch(node_plan, narrowing)?;

        let active = WorthUiQueryBindingEvidenceIndex::from_active_artifact(active_artifact);
        let candidate = WorthUiQueryBindingEvidenceIndex::from_artifact_graph_and_support_receipt(
            admitted.artifact_bundle().artifact(),
            admitted
                .artifact_bundle()
                .dependency_metadata()
                .invalidation_basis()
                .dependency_graph(),
            admitted.report().query_support_receipt(),
        );
        Ok(compare_indexes(
            node_plan.active_artifact_digest(),
            node_plan.candidate_artifact_digest(),
            active,
            candidate,
            narrowing.query_dependency_invalidations().len(),
        ))
    }
}

fn compare_indexes(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    active: WorthUiQueryBindingEvidenceIndex,
    candidate: WorthUiQueryBindingEvidenceIndex,
    affected_query_invalidation_count: usize,
) -> WorthUiQueryBindingComparison {
    let mut counters = WorthUiQueryBindingComparisonCounters::default();
    counters.record_active_bindings_indexed(active.len());
    counters.record_candidate_bindings_indexed(candidate.len());
    counters.record_affected_query_invalidations(affected_query_invalidation_count);

    let mut ids = BTreeSet::new();
    ids.extend(active.binding_ids());
    ids.extend(candidate.binding_ids());

    let mut entries = Vec::new();
    for binding_id in ids {
        let active_evidence = active.get(&binding_id);
        let candidate_evidence = candidate.get(&binding_id);
        let identity = candidate_evidence
            .or(active_evidence)
            .expect("binding id came from an index")
            .identity()
            .clone();
        let (outcome, posture_drifts) = match (active_evidence, candidate_evidence) {
            (Some(active), Some(candidate)) => {
                if active.identity() != candidate.identity() {
                    (
                        WorthUiQueryBindingComparisonOutcome::RebindRequired,
                        Vec::new(),
                    )
                } else {
                    let drifts = active.posture().drift_families_against(candidate.posture());
                    if drifts.is_empty() {
                        (
                            WorthUiQueryBindingComparisonOutcome::PreserveMeaning,
                            drifts,
                        )
                    } else {
                        (WorthUiQueryBindingComparisonOutcome::RebindRequired, drifts)
                    }
                }
            }
            (None, Some(_)) => (
                WorthUiQueryBindingComparisonOutcome::MissingActiveBinding,
                Vec::new(),
            ),
            (Some(_), None) => (
                WorthUiQueryBindingComparisonOutcome::MissingCandidateBinding,
                Vec::new(),
            ),
            (None, None) => unreachable!("binding id came from an index"),
        };
        counters.record_entry(outcome, posture_drifts.len());
        entries.push(WorthUiQueryBindingComparisonEntry::new(
            identity,
            active_evidence.map(|evidence| evidence.posture().clone()),
            candidate_evidence.map(|evidence| evidence.posture().clone()),
            outcome,
            posture_drifts,
        ));
    }

    WorthUiQueryBindingComparison::new(
        active_artifact_digest,
        candidate_artifact_digest,
        entries,
        counters,
    )
}

fn reject_ambiguous_node_plan(
    node_plan: &WorthUiNodeReplacementPlan,
) -> Result<(), WorthUiQueryBindingComparisonDenial> {
    if node_plan.is_unambiguous() {
        Ok(())
    } else {
        Err(WorthUiQueryBindingComparisonDenial::AmbiguousNodeReplacementPlan)
    }
}

fn reject_digest_mismatch(
    active_artifact: &WorthUiArtifact,
    node_plan: &WorthUiNodeReplacementPlan,
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiQueryBindingComparisonDenial> {
    let runtime_active_artifact_digest = crate::source::WorthUiArtifactDigestor::digest(
        active_artifact,
        crate::source::WorthUiArtifactEquivalenceBasis::semantic(),
    )
    .raw();
    let admitted_candidate_artifact_digest = admitted.artifact_bundle().artifact_digest().raw();
    if runtime_active_artifact_digest == node_plan.active_artifact_digest()
        && admitted_candidate_artifact_digest == node_plan.candidate_artifact_digest()
    {
        Ok(())
    } else {
        Err(
            WorthUiQueryBindingComparisonDenial::NodePlanDigestMismatch {
                runtime_active_artifact_digest,
                plan_active_artifact_digest: node_plan.active_artifact_digest(),
                admitted_candidate_artifact_digest,
                plan_candidate_artifact_digest: node_plan.candidate_artifact_digest(),
            },
        )
    }
}

fn reject_narrowing_mismatch(
    node_plan: &WorthUiNodeReplacementPlan,
    narrowing: &WorthUiRuntimeImpactNarrowing,
) -> Result<(), WorthUiQueryBindingComparisonDenial> {
    if node_plan.active_artifact_digest() == narrowing.active_artifact_digest()
        && node_plan.candidate_artifact_digest() == narrowing.candidate_artifact_digest()
    {
        Ok(())
    } else {
        Err(
            WorthUiQueryBindingComparisonDenial::NarrowingDigestMismatch {
                plan_active_artifact_digest: node_plan.active_artifact_digest(),
                narrowing_active_artifact_digest: narrowing.active_artifact_digest(),
                plan_candidate_artifact_digest: node_plan.candidate_artifact_digest(),
                narrowing_candidate_artifact_digest: narrowing.candidate_artifact_digest(),
            },
        )
    }
}
