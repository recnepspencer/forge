use crate::runtime::{
    WorthUiAccessibilityImpact, WorthUiAccessibilityInvalidation, WorthUiIdentityMatchGraph,
    WorthUiIdentityMatchReport, WorthUiImpactLookupCounters, WorthUiLaneImpactClassification,
    WorthUiReplacementImpact, WorthUiReplacementImpactClassification,
    WorthUiReplacementImpactCounters, WorthUiReplacementScope, WorthUiRuntimeImpactNarrowing,
    WorthUiTokenThemeImpact,
};
use crate::source::WorthUiSourceModuleId;

pub(super) fn no_op_impact_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiReplacementImpactClassification {
    impact_for(identity_report, WorthUiReplacementImpact::NoOp)
}

pub(super) fn lane_affecting_impact_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiReplacementImpactClassification {
    impact_for(
        identity_report,
        WorthUiReplacementImpact::LaneAffecting {
            lane_impact: WorthUiLaneImpactClassification::LaneAffecting {
                reason: "test-lane-change",
            },
            scope: WorthUiReplacementScope::structural(
                identity_report
                    .graph()
                    .active_nodes()
                    .iter()
                    .map(|node| node.handle().clone())
                    .collect(),
                identity_report.graph().active_node_count(),
                identity_report.graph().active_node_count(),
            ),
        },
    )
}

pub(super) fn structural_impact_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiReplacementImpactClassification {
    impact_for(
        identity_report,
        WorthUiReplacementImpact::StructuralReplacement(WorthUiReplacementScope::structural(
            identity_report
                .graph()
                .active_nodes()
                .iter()
                .map(|node| node.handle().clone())
                .collect(),
            identity_report.graph().active_node_count(),
            identity_report.graph().active_node_count(),
        )),
    )
}

pub(super) fn structural_impact_for_identity(
    identity_report: &WorthUiIdentityMatchReport,
    identity_basis: &str,
) -> WorthUiReplacementImpactClassification {
    let affected_handle = active_handle_for_identity(identity_report, identity_basis);
    impact_for(
        identity_report,
        WorthUiReplacementImpact::StructuralReplacement(WorthUiReplacementScope::structural(
            vec![affected_handle],
            identity_report.graph().active_node_count(),
            identity_report.graph().active_node_count(),
        )),
    )
}

pub(super) fn lane_narrowing_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        crate::runtime::replacement::narrowing::WorthUiRuntimeImpactNarrowingInput {
            active_artifact_digest: identity_report.active_artifact_digest(),
            candidate_artifact_digest: identity_report.candidate_artifact_digest(),
            affected_source_modules: vec![module_id("app/main.wui")],
            affected_handles: identity_report
                .graph()
                .active_nodes()
                .iter()
                .map(|node| node.handle().clone())
                .collect(),
            affected_subtree_digests: Vec::new(),
            command_binding_invalidations: Vec::new(),
            token_invalidations: Vec::new(),
            accessibility_invalidation: WorthUiAccessibilityInvalidation::unchanged(),
            renderer_resource_invalidations: Vec::new(),
            query_dependency_invalidations: Vec::new(),
            lane_impact: Some(WorthUiLaneImpactClassification::LaneAffecting {
                reason: "test-lane-change",
            }),
            full_artifact_handle_count: identity_report.graph().active_node_count(),
            counters: WorthUiImpactLookupCounters::default(),
        },
    )
}

pub(super) fn empty_lane_narrowing_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        crate::runtime::replacement::narrowing::WorthUiRuntimeImpactNarrowingInput {
            active_artifact_digest: identity_report.active_artifact_digest(),
            candidate_artifact_digest: identity_report.candidate_artifact_digest(),
            affected_source_modules: vec![module_id("app/main.wui")],
            affected_handles: Vec::new(),
            affected_subtree_digests: Vec::new(),
            command_binding_invalidations: Vec::new(),
            token_invalidations: Vec::new(),
            accessibility_invalidation: WorthUiAccessibilityInvalidation::unchanged(),
            renderer_resource_invalidations: Vec::new(),
            query_dependency_invalidations: Vec::new(),
            lane_impact: Some(WorthUiLaneImpactClassification::LaneAffecting {
                reason: "test-lane-change",
            }),
            full_artifact_handle_count: identity_report.graph().active_node_count(),
            counters: WorthUiImpactLookupCounters::default(),
        },
    )
}

pub(super) fn narrowing_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        crate::runtime::replacement::narrowing::WorthUiRuntimeImpactNarrowingInput {
            active_artifact_digest: identity_report.active_artifact_digest(),
            candidate_artifact_digest: identity_report.candidate_artifact_digest(),
            affected_source_modules: vec![module_id("app/main.wui")],
            affected_handles: identity_report
                .graph()
                .active_nodes()
                .iter()
                .map(|node| node.handle().clone())
                .collect(),
            affected_subtree_digests: Vec::new(),
            command_binding_invalidations: Vec::new(),
            token_invalidations: Vec::new(),
            accessibility_invalidation: WorthUiAccessibilityInvalidation::unchanged(),
            renderer_resource_invalidations: Vec::new(),
            query_dependency_invalidations: Vec::new(),
            lane_impact: None,
            full_artifact_handle_count: identity_report.graph().active_node_count(),
            counters: WorthUiImpactLookupCounters::default(),
        },
    )
}

pub(super) fn narrowing_for_identity(
    identity_report: &WorthUiIdentityMatchReport,
    identity_basis: &str,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        crate::runtime::replacement::narrowing::WorthUiRuntimeImpactNarrowingInput {
            active_artifact_digest: identity_report.active_artifact_digest(),
            candidate_artifact_digest: identity_report.candidate_artifact_digest(),
            affected_source_modules: vec![module_id("app/main.wui")],
            affected_handles: vec![active_handle_for_identity(identity_report, identity_basis)],
            affected_subtree_digests: Vec::new(),
            command_binding_invalidations: Vec::new(),
            token_invalidations: Vec::new(),
            accessibility_invalidation: WorthUiAccessibilityInvalidation::unchanged(),
            renderer_resource_invalidations: Vec::new(),
            query_dependency_invalidations: Vec::new(),
            lane_impact: None,
            full_artifact_handle_count: identity_report.graph().active_node_count(),
            counters: WorthUiImpactLookupCounters::default(),
        },
    )
}

pub(super) fn ambiguous_identity_report_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiIdentityMatchReport {
    let mut counters = identity_report.counters();
    counters.record_duplicate_candidate_identity();
    WorthUiIdentityMatchReport::new(
        identity_report.active_artifact_digest(),
        identity_report.candidate_artifact_digest(),
        WorthUiIdentityMatchGraph::new(
            identity_report.graph().active_nodes().to_vec(),
            identity_report.graph().candidate_nodes().to_vec(),
            identity_report.graph().matches().to_vec(),
            Vec::new(),
            identity_report.graph().moved_node_identities().to_vec(),
            counters,
        ),
    )
}

fn impact_for(
    identity_report: &WorthUiIdentityMatchReport,
    impact: WorthUiReplacementImpact,
) -> WorthUiReplacementImpactClassification {
    WorthUiReplacementImpactClassification::new(
        crate::runtime::replacement::impact::WorthUiReplacementImpactClassificationInput {
            active_artifact_digest: identity_report.active_artifact_digest(),
            candidate_artifact_digest: identity_report.candidate_artifact_digest(),
            impact,
            command_impact: crate::runtime::WorthUiCommandImpact::Unchanged,
            token_theme_impact: WorthUiTokenThemeImpact::Unchanged,
            accessibility_impact: WorthUiAccessibilityImpact::Unchanged,
            renderer_resource_impact: crate::runtime::WorthUiRendererResourceImpact::Unchanged,
            counters: WorthUiReplacementImpactCounters::default(),
        },
    )
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(std::path::Path::new(path)).unwrap()
}

fn active_handle_for_identity(
    identity_report: &WorthUiIdentityMatchReport,
    identity_basis: &str,
) -> crate::source::WorthUiArtifactHandle {
    identity_report
        .graph()
        .active_nodes()
        .iter()
        .find(|node| node.identity_basis() == identity_basis)
        .expect("active identity exists")
        .handle()
        .clone()
}
