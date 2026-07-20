use std::collections::{BTreeMap, BTreeSet};

use crate::runtime::{
    WorthUiAmbiguousReplacementDenial, WorthUiIdentityMatchNode, WorthUiNodeLifecycleTransition,
    WorthUiNodeReplacementClassification, WorthUiNodeReplacementCounters,
    WorthUiNodeReplacementPlan, WorthUiReplacementImpact, WorthUiReplacementImpactClassification,
    WorthUiRuntimeImpactNarrowing,
};
use crate::source::WorthUiArtifactHandle;

pub(crate) struct WorthUiNodeReplacementClassifier;

struct WorthUiNodeReplacementClassificationAccumulator {
    classifications: Vec<WorthUiNodeReplacementClassification>,
    classified_identity_bases: BTreeSet<String>,
    counters: WorthUiNodeReplacementCounters,
}

impl WorthUiNodeReplacementClassifier {
    pub(crate) fn classify(
        impact: &WorthUiReplacementImpactClassification,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        identity_report: &crate::runtime::WorthUiIdentityMatchReport,
    ) -> Result<WorthUiNodeReplacementPlan, WorthUiAmbiguousReplacementDenial> {
        let mut counters = WorthUiNodeReplacementCounters::default();
        reject_impact_digest_mismatch(impact, identity_report, counters)?;
        reject_narrowing_digest_mismatch(narrowing, identity_report, counters)?;
        reject_ambiguous_identity_graph(identity_report, &mut counters)?;
        reject_lane_affecting_impact_without_lane_replacement_scope(impact, narrowing, counters)?;

        let active_nodes = index_identity_nodes(identity_report.graph().active_nodes());
        let candidate_nodes = index_identity_nodes(identity_report.graph().candidate_nodes());
        let matched_identity_bases = matched_identity_bases(identity_report.graph().matches());
        let mut accumulator = WorthUiNodeReplacementClassificationAccumulator::new();
        let affected_handles = narrowing.affected_handles_for_runtime();

        classify_matched_identities(
            &active_nodes,
            &candidate_nodes,
            &matched_identity_bases,
            impact,
            narrowing,
            affected_handles,
            &mut accumulator,
        )?;
        classify_dropped_identities(&active_nodes, &matched_identity_bases, &mut accumulator)?;
        classify_created_identities(&candidate_nodes, &matched_identity_bases, &mut accumulator)?;

        Ok(accumulator.finish(
            identity_report.active_artifact_digest(),
            identity_report.candidate_artifact_digest(),
        ))
    }
}

impl WorthUiNodeReplacementClassificationAccumulator {
    fn new() -> Self {
        Self {
            classifications: Vec::new(),
            classified_identity_bases: BTreeSet::new(),
            counters: WorthUiNodeReplacementCounters::default(),
        }
    }

    fn record_matched_classification(
        &mut self,
        classification: WorthUiNodeReplacementClassification,
    ) -> Result<(), WorthUiAmbiguousReplacementDenial> {
        self.counters.record_active_node_classified();
        self.counters.record_candidate_node_classified();
        self.push_classification(classification)
    }

    fn record_dropped_classification(
        &mut self,
        classification: WorthUiNodeReplacementClassification,
    ) -> Result<(), WorthUiAmbiguousReplacementDenial> {
        self.counters.record_active_node_classified();
        self.push_classification(classification)
    }

    fn record_created_classification(
        &mut self,
        classification: WorthUiNodeReplacementClassification,
    ) -> Result<(), WorthUiAmbiguousReplacementDenial> {
        self.counters.record_candidate_node_classified();
        self.push_classification(classification)
    }

    fn push_classification(
        &mut self,
        classification: WorthUiNodeReplacementClassification,
    ) -> Result<(), WorthUiAmbiguousReplacementDenial> {
        if !self
            .classified_identity_bases
            .insert(classification.identity_basis().to_owned())
        {
            self.counters.record_ambiguous_node();
            return Err(
                WorthUiAmbiguousReplacementDenial::DuplicateReplacementClassification {
                    identity_basis: classification.identity_basis().to_owned(),
                    counters: self.counters,
                },
            );
        }

        self.counters.record_transition(classification.transition());
        self.classifications.push(classification);
        Ok(())
    }

    fn finish(
        self,
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
    ) -> WorthUiNodeReplacementPlan {
        WorthUiNodeReplacementPlan::new(
            active_artifact_digest,
            candidate_artifact_digest,
            self.classifications,
            self.counters,
        )
    }
}

fn reject_impact_digest_mismatch(
    impact: &WorthUiReplacementImpactClassification,
    identity_report: &crate::runtime::WorthUiIdentityMatchReport,
    counters: WorthUiNodeReplacementCounters,
) -> Result<(), WorthUiAmbiguousReplacementDenial> {
    if impact.active_artifact_digest() == identity_report.active_artifact_digest()
        && impact.candidate_artifact_digest() == identity_report.candidate_artifact_digest()
    {
        Ok(())
    } else {
        Err(
            WorthUiAmbiguousReplacementDenial::ImpactClassificationDigestMismatch {
                impact_active_artifact_digest: impact.active_artifact_digest(),
                identity_active_artifact_digest: identity_report.active_artifact_digest(),
                impact_candidate_artifact_digest: impact.candidate_artifact_digest(),
                identity_candidate_artifact_digest: identity_report.candidate_artifact_digest(),
                counters,
            },
        )
    }
}

fn reject_narrowing_digest_mismatch(
    narrowing: &WorthUiRuntimeImpactNarrowing,
    identity_report: &crate::runtime::WorthUiIdentityMatchReport,
    counters: WorthUiNodeReplacementCounters,
) -> Result<(), WorthUiAmbiguousReplacementDenial> {
    if narrowing.active_artifact_digest() == identity_report.active_artifact_digest()
        && narrowing.candidate_artifact_digest() == identity_report.candidate_artifact_digest()
    {
        Ok(())
    } else {
        Err(WorthUiAmbiguousReplacementDenial::NarrowingDigestMismatch {
            narrowing_active_artifact_digest: narrowing.active_artifact_digest(),
            identity_active_artifact_digest: identity_report.active_artifact_digest(),
            narrowing_candidate_artifact_digest: narrowing.candidate_artifact_digest(),
            identity_candidate_artifact_digest: identity_report.candidate_artifact_digest(),
            counters,
        })
    }
}

fn reject_ambiguous_identity_graph(
    identity_report: &crate::runtime::WorthUiIdentityMatchReport,
    counters: &mut WorthUiNodeReplacementCounters,
) -> Result<(), WorthUiAmbiguousReplacementDenial> {
    if identity_report.graph().is_unambiguous() {
        Ok(())
    } else {
        counters.record_ambiguous_node();
        Err(WorthUiAmbiguousReplacementDenial::AmbiguousIdentityGraph {
            counters: *counters,
        })
    }
}

fn reject_lane_affecting_impact_without_lane_replacement_scope(
    impact: &WorthUiReplacementImpactClassification,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    counters: WorthUiNodeReplacementCounters,
) -> Result<(), WorthUiAmbiguousReplacementDenial> {
    if !matches!(
        impact.impact(),
        WorthUiReplacementImpact::LaneAffecting { .. }
    ) {
        return Ok(());
    }
    if !narrowing
        .lane_impact()
        .is_some_and(|lane_impact| lane_impact.requires_lane_parity())
    {
        return Err(
            WorthUiAmbiguousReplacementDenial::LaneAffectingImpactWithoutLaneNarrowing { counters },
        );
    }
    if narrowing.affected_handles_for_runtime().is_empty() {
        return Err(
            WorthUiAmbiguousReplacementDenial::LaneAffectingImpactWithoutAffectedLaneScope {
                counters,
            },
        );
    }
    Ok(())
}

fn index_identity_nodes(
    nodes: &[WorthUiIdentityMatchNode],
) -> BTreeMap<String, &WorthUiIdentityMatchNode> {
    nodes
        .iter()
        .map(|node| (node.identity_basis().to_owned(), node))
        .collect()
}

fn matched_identity_bases(
    matches: &[crate::runtime::WorthUiIdentityMatchEdge],
) -> BTreeSet<String> {
    matches
        .iter()
        .map(|edge| edge.identity_basis().to_owned())
        .collect()
}

fn classify_matched_identities(
    active_nodes: &BTreeMap<String, &WorthUiIdentityMatchNode>,
    candidate_nodes: &BTreeMap<String, &WorthUiIdentityMatchNode>,
    matched_identity_bases: &BTreeSet<String>,
    impact: &WorthUiReplacementImpactClassification,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    affected_handles: &[WorthUiArtifactHandle],
    accumulator: &mut WorthUiNodeReplacementClassificationAccumulator,
) -> Result<(), WorthUiAmbiguousReplacementDenial> {
    for identity_basis in matched_identity_bases {
        let Some(active_node) = active_nodes.get(identity_basis) else {
            continue;
        };
        let Some(candidate_node) = candidate_nodes.get(identity_basis) else {
            continue;
        };
        let transition = classify_matched_transition(
            active_node,
            candidate_node,
            impact,
            narrowing,
            affected_handles,
        );
        accumulator.record_matched_classification(WorthUiNodeReplacementClassification::new(
            super::WorthUiNodeReplacementClassificationInput {
                identity_basis: identity_basis.to_owned(),
                authored_provenance_digest: Some(candidate_node.authored_provenance_digest()),
                transition,
                active_kind: Some(active_node.kind()),
                candidate_kind: Some(candidate_node.kind()),
                active_durable_state_eligible: active_node.durable_state_eligible(),
                candidate_durable_state_eligible: candidate_node.durable_state_eligible(),
                active_resize_contract_id: active_node.resize_contract_id().cloned(),
                candidate_resize_contract_id: candidate_node.resize_contract_id().cloned(),
                active_resize_permission: active_node.resize_permission().cloned(),
                candidate_resize_permission: candidate_node.resize_permission().cloned(),
                active_resize_shape_digest: active_node.resize_shape_digest(),
                candidate_resize_shape_digest: candidate_node.resize_shape_digest(),
            },
        ))?;
    }
    Ok(())
}

fn classify_matched_transition(
    active_node: &WorthUiIdentityMatchNode,
    candidate_node: &WorthUiIdentityMatchNode,
    impact: &WorthUiReplacementImpactClassification,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    affected_handles: &[WorthUiArtifactHandle],
) -> WorthUiNodeLifecycleTransition {
    if node_requires_lane_change(active_node, narrowing) {
        return WorthUiNodeLifecycleTransition::LaneChange;
    }
    if node_requires_rebind(active_node, narrowing) {
        return WorthUiNodeLifecycleTransition::Rebind;
    }
    if !active_node.has_same_semantic_meaning(candidate_node) {
        return WorthUiNodeLifecycleTransition::Replace;
    }
    if active_node.handle().module_id() != candidate_node.handle().module_id() {
        return WorthUiNodeLifecycleTransition::Move;
    }
    if node_requires_replace(active_node, impact, affected_handles) {
        return WorthUiNodeLifecycleTransition::Replace;
    }
    WorthUiNodeLifecycleTransition::Preserve
}

fn node_requires_lane_change(
    active_node: &WorthUiIdentityMatchNode,
    narrowing: &WorthUiRuntimeImpactNarrowing,
) -> bool {
    narrowing
        .lane_impact()
        .is_some_and(|lane_impact| lane_impact.requires_lane_parity())
        && narrowing
            .affected_handles_for_runtime()
            .contains(active_node.handle())
}

fn node_requires_rebind(
    active_node: &WorthUiIdentityMatchNode,
    narrowing: &WorthUiRuntimeImpactNarrowing,
) -> bool {
    !narrowing.query_dependency_invalidations().is_empty()
        && matches!(
            active_node.kind(),
            crate::runtime::WorthUiIdentityMatchNodeKind::Binding
        )
}

fn node_requires_replace(
    active_node: &WorthUiIdentityMatchNode,
    impact: &WorthUiReplacementImpactClassification,
    affected_handles: &[WorthUiArtifactHandle],
) -> bool {
    !matches!(impact.impact(), WorthUiReplacementImpact::NoOp)
        && affected_handles.contains(active_node.handle())
}

fn classify_dropped_identities(
    active_nodes: &BTreeMap<String, &WorthUiIdentityMatchNode>,
    matched_identity_bases: &BTreeSet<String>,
    accumulator: &mut WorthUiNodeReplacementClassificationAccumulator,
) -> Result<(), WorthUiAmbiguousReplacementDenial> {
    for (identity_basis, active_node) in active_nodes {
        if matched_identity_bases.contains(identity_basis) {
            continue;
        }
        accumulator.record_dropped_classification(WorthUiNodeReplacementClassification::new(
            super::WorthUiNodeReplacementClassificationInput {
                identity_basis: identity_basis.to_owned(),
                authored_provenance_digest: Some(active_node.authored_provenance_digest()),
                transition: WorthUiNodeLifecycleTransition::Drop,
                active_kind: Some(active_node.kind()),
                candidate_kind: None,
                active_durable_state_eligible: active_node.durable_state_eligible(),
                candidate_durable_state_eligible: false,
                active_resize_contract_id: active_node.resize_contract_id().cloned(),
                candidate_resize_contract_id: None,
                active_resize_permission: active_node.resize_permission().cloned(),
                candidate_resize_permission: None,
                active_resize_shape_digest: active_node.resize_shape_digest(),
                candidate_resize_shape_digest: None,
            },
        ))?;
    }
    Ok(())
}

fn classify_created_identities(
    candidate_nodes: &BTreeMap<String, &WorthUiIdentityMatchNode>,
    matched_identity_bases: &BTreeSet<String>,
    accumulator: &mut WorthUiNodeReplacementClassificationAccumulator,
) -> Result<(), WorthUiAmbiguousReplacementDenial> {
    for (identity_basis, candidate_node) in candidate_nodes {
        if matched_identity_bases.contains(identity_basis) {
            continue;
        }
        accumulator.record_created_classification(WorthUiNodeReplacementClassification::new(
            super::WorthUiNodeReplacementClassificationInput {
                identity_basis: identity_basis.to_owned(),
                authored_provenance_digest: Some(candidate_node.authored_provenance_digest()),
                transition: WorthUiNodeLifecycleTransition::Create,
                active_kind: None,
                candidate_kind: Some(candidate_node.kind()),
                active_durable_state_eligible: false,
                candidate_durable_state_eligible: candidate_node.durable_state_eligible(),
                active_resize_contract_id: None,
                candidate_resize_contract_id: candidate_node.resize_contract_id().cloned(),
                active_resize_permission: None,
                candidate_resize_permission: candidate_node.resize_permission().cloned(),
                active_resize_shape_digest: None,
                candidate_resize_shape_digest: candidate_node.resize_shape_digest(),
            },
        ))?;
    }
    Ok(())
}
