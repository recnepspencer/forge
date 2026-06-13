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
        identity_report.active_artifact_digest(),
        identity_report.candidate_artifact_digest(),
        vec![module_id("app/main.wui")],
        identity_report
            .graph()
            .active_nodes()
            .iter()
            .map(|node| node.handle().clone())
            .collect(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        WorthUiAccessibilityInvalidation::unchanged(),
        Vec::new(),
        Vec::new(),
        Some(WorthUiLaneImpactClassification::LaneAffecting {
            reason: "test-lane-change",
        }),
        identity_report.graph().active_node_count(),
        WorthUiImpactLookupCounters::default(),
    )
}

pub(super) fn empty_lane_narrowing_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        identity_report.active_artifact_digest(),
        identity_report.candidate_artifact_digest(),
        vec![module_id("app/main.wui")],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        WorthUiAccessibilityInvalidation::unchanged(),
        Vec::new(),
        Vec::new(),
        Some(WorthUiLaneImpactClassification::LaneAffecting {
            reason: "test-lane-change",
        }),
        identity_report.graph().active_node_count(),
        WorthUiImpactLookupCounters::default(),
    )
}

pub(super) fn narrowing_for(
    identity_report: &WorthUiIdentityMatchReport,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        identity_report.active_artifact_digest(),
        identity_report.candidate_artifact_digest(),
        vec![module_id("app/main.wui")],
        identity_report
            .graph()
            .active_nodes()
            .iter()
            .map(|node| node.handle().clone())
            .collect(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        WorthUiAccessibilityInvalidation::unchanged(),
        Vec::new(),
        Vec::new(),
        None,
        identity_report.graph().active_node_count(),
        WorthUiImpactLookupCounters::default(),
    )
}

pub(super) fn narrowing_for_identity(
    identity_report: &WorthUiIdentityMatchReport,
    identity_basis: &str,
) -> WorthUiRuntimeImpactNarrowing {
    WorthUiRuntimeImpactNarrowing::new(
        identity_report.active_artifact_digest(),
        identity_report.candidate_artifact_digest(),
        vec![module_id("app/main.wui")],
        vec![active_handle_for_identity(identity_report, identity_basis)],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        WorthUiAccessibilityInvalidation::unchanged(),
        Vec::new(),
        Vec::new(),
        None,
        identity_report.graph().active_node_count(),
        WorthUiImpactLookupCounters::default(),
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
        identity_report.active_artifact_digest(),
        identity_report.candidate_artifact_digest(),
        impact,
        crate::runtime::WorthUiCommandImpact::Unchanged,
        WorthUiTokenThemeImpact::Unchanged,
        WorthUiAccessibilityImpact::Unchanged,
        crate::runtime::WorthUiRendererResourceImpact::Unchanged,
        WorthUiReplacementImpactCounters::default(),
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
