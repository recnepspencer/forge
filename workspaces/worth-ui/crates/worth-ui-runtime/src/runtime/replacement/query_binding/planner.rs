use std::collections::BTreeSet;

use super::authority_drift::authority_drifts;
use crate::runtime::active::WorthUiActiveArtifact;
use crate::runtime::replacement::query_binding::comparison::{
    WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonCounters,
    WorthUiQueryBindingComparisonDenial, WorthUiQueryBindingComparisonEntry,
    WorthUiQueryBindingComparisonOutcome,
};
use crate::runtime::replacement::query_binding::evidence::WorthUiQueryBindingEvidenceIndex;
#[cfg(any(test, feature = "certification-support"))]
use crate::runtime::WorthUiNodeReplacementPlan;
use crate::runtime::{WorthUiAdmittedReplacementCandidate, WorthUiRuntimeImpactNarrowing};

pub(crate) struct WorthUiQueryBindingComparisonPlanner;

pub(crate) struct WorthUiQueryBindingReplacementAuthority<'a> {
    plan: &'a worth_ui_query_binding::WorthUiQueryBindingPlan,
    binding: &'a worth_ui_query_binding::WorthUiRuntimeQueryBinding,
}

impl<'a> WorthUiQueryBindingReplacementAuthority<'a> {
    pub(crate) fn new(
        plan: &'a worth_ui_query_binding::WorthUiQueryBindingPlan,
        binding: &'a worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    ) -> Self {
        Self { plan, binding }
    }
}

impl WorthUiQueryBindingComparisonPlanner {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn compare(
        active_artifact: &WorthUiActiveArtifact,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
        active_authority: WorthUiQueryBindingReplacementAuthority<'_>,
        candidate_authority: WorthUiQueryBindingReplacementAuthority<'_>,
    ) -> Result<WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial> {
        reject_ambiguous_node_plan(node_plan)?;
        reject_digest_mismatch(active_artifact, node_plan, admitted)?;
        reject_narrowing_mismatch(node_plan, narrowing)?;

        Self::compare_narrowed(
            active_artifact,
            narrowing,
            admitted,
            active_authority,
            candidate_authority,
        )
    }

    pub(crate) fn compare_narrowed(
        active_artifact: &WorthUiActiveArtifact,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
        active_authority: WorthUiQueryBindingReplacementAuthority<'_>,
        candidate_authority: WorthUiQueryBindingReplacementAuthority<'_>,
    ) -> Result<WorthUiQueryBindingComparison, WorthUiQueryBindingComparisonDenial> {
        reject_narrowing_candidate_mismatch(active_artifact, narrowing, admitted)?;
        let candidate_artifact = admitted.artifact_bundle().artifact();
        let mut affected_binding_ids = active_artifact
            .artifact()
            .query_binding_ids()
            .symmetric_difference(candidate_artifact.query_binding_ids())
            .cloned()
            .collect::<BTreeSet<_>>();
        affected_binding_ids.extend(
            narrowing
                .query_dependency_invalidations()
                .iter()
                .map(|invalidation| invalidation.view_binding_id().to_owned()),
        );

        let active = WorthUiQueryBindingEvidenceIndex::from_active_artifact_for_bindings(
            active_artifact,
            &affected_binding_ids,
            active_authority.plan,
            active_authority.binding,
        );
        let candidate = WorthUiQueryBindingEvidenceIndex::from_artifact_and_graph_for_bindings(
            admitted.artifact_bundle().artifact(),
            admitted
                .artifact_bundle()
                .dependency_metadata()
                .invalidation_basis()
                .dependency_graph(),
            &affected_binding_ids,
            candidate_authority.plan,
            candidate_authority.binding,
        );
        Ok(compare_indexes(
            narrowing.active_artifact_digest(),
            narrowing.candidate_artifact_digest(),
            active,
            candidate,
            active_artifact.dependency_graph(),
            admitted
                .artifact_bundle()
                .dependency_metadata()
                .invalidation_basis()
                .dependency_graph(),
        ))
    }
}

fn compare_indexes(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    active: WorthUiQueryBindingEvidenceIndex,
    candidate: WorthUiQueryBindingEvidenceIndex,
    active_graph: &crate::source::WorthUiArtifactDependencyGraph,
    candidate_graph: &crate::source::WorthUiArtifactDependencyGraph,
) -> WorthUiQueryBindingComparison {
    let mut counters = WorthUiQueryBindingComparisonCounters::default();
    counters.record_active_bindings_indexed(active.len());
    counters.record_candidate_bindings_indexed(candidate.len());

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
        let (outcome, ui_requirement_drifts, authority_drifts) =
            match (active_evidence, candidate_evidence) {
                (Some(active), Some(candidate)) => {
                    if active.identity() != candidate.identity() {
                        (
                            WorthUiQueryBindingComparisonOutcome::RebindRequired,
                            Vec::new(),
                            Vec::new(),
                        )
                    } else {
                        let drifts = active
                            .ui_requirements()
                            .drift_families_against(candidate.ui_requirements());
                        let authority_drifts = authority_drifts(active, candidate);
                        let outcome = if authority_drifts.is_empty() {
                            WorthUiQueryBindingComparisonOutcome::PreserveMeaning
                        } else {
                            WorthUiQueryBindingComparisonOutcome::RebindRequired
                        };
                        (outcome, drifts, authority_drifts)
                    }
                }
                (None, Some(_)) => (
                    WorthUiQueryBindingComparisonOutcome::MissingActiveBinding,
                    Vec::new(),
                    Vec::new(),
                ),
                (Some(_), None) => (
                    WorthUiQueryBindingComparisonOutcome::MissingCandidateBinding,
                    Vec::new(),
                    Vec::new(),
                ),
                (None, None) => unreachable!("binding id came from an index"),
            };
        counters.record_entry(outcome, ui_requirement_drifts.len());
        entries.push(WorthUiQueryBindingComparisonEntry::new(
            identity,
            active_evidence.map(|evidence| evidence.ui_requirements().clone()),
            candidate_evidence.map(|evidence| evidence.ui_requirements().clone()),
            outcome,
            ui_requirement_drifts,
            authority_drifts,
        ));
    }

    let mut exact_invalidations = entries
        .iter()
        .filter(|entry| entry.requires_ui_invalidation())
        .flat_map(|entry| {
            let binding_id = entry.identity().view_binding_id();
            active_graph
                .runtime_hooks_for_query_binding(binding_id)
                .chain(candidate_graph.runtime_hooks_for_query_binding(binding_id))
                .map(crate::runtime::WorthUiQueryDependencyInvalidation::from_runtime_hook)
        })
        .collect::<Vec<_>>();
    exact_invalidations.sort();
    exact_invalidations.dedup();
    counters.record_affected_query_invalidations(exact_invalidations.len());
    WorthUiQueryBindingComparison::new(
        active_artifact_digest,
        candidate_artifact_digest,
        entries,
        counters,
        exact_invalidations,
    )
}

#[cfg(any(test, feature = "certification-support"))]
fn reject_ambiguous_node_plan(
    node_plan: &WorthUiNodeReplacementPlan,
) -> Result<(), WorthUiQueryBindingComparisonDenial> {
    if node_plan.is_unambiguous() {
        Ok(())
    } else {
        Err(WorthUiQueryBindingComparisonDenial::AmbiguousNodeReplacementPlan)
    }
}

#[cfg(any(test, feature = "certification-support"))]
fn reject_digest_mismatch(
    active_artifact: &WorthUiActiveArtifact,
    node_plan: &WorthUiNodeReplacementPlan,
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiQueryBindingComparisonDenial> {
    let runtime_active_artifact_digest = active_artifact.digest().raw();
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

fn reject_narrowing_candidate_mismatch(
    active_artifact: &WorthUiActiveArtifact,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiQueryBindingComparisonDenial> {
    let active_digest = active_artifact.digest().raw();
    let candidate_digest = admitted.artifact_bundle().artifact_digest().raw();
    if narrowing.active_artifact_digest() == active_digest
        && narrowing.candidate_artifact_digest() == candidate_digest
    {
        Ok(())
    } else {
        Err(
            WorthUiQueryBindingComparisonDenial::NarrowingDigestMismatch {
                plan_active_artifact_digest: active_digest,
                narrowing_active_artifact_digest: narrowing.active_artifact_digest(),
                plan_candidate_artifact_digest: candidate_digest,
                narrowing_candidate_artifact_digest: narrowing.candidate_artifact_digest(),
            },
        )
    }
}

#[cfg(any(test, feature = "certification-support"))]
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
