use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentEndpointRef,
    PlanarBooleanSplitEdgeFragmentSchedule, PlanarBooleanSplitEdgeFragmentSet,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanLoopRole, PlanarBooleanSourceIntervalSense,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanAdmittedReconstructedLoopSet,
    PlanarBooleanBornLoopSet, PlanarBooleanLoopContainmentEvidencePosture,
    PlanarBooleanLoopContainmentEvidencePostureKind,
    PlanarBooleanLoopContainmentEvidencePostureSet, PlanarBooleanLoopRoleOutcome,
    PlanarBooleanLoopRoleOutcomeKind, PlanarBooleanLoopRoleOutcomeSet,
    PlanarBooleanLoopSourceCarrierRow, PlanarBooleanLoopSourceCarrierSet,
};

pub(super) fn reconstructed_loops(
    rows: Vec<PlanarBooleanAdmittedReconstructedLoop>,
) -> PlanarBooleanAdmittedReconstructedLoopSet {
    PlanarBooleanAdmittedReconstructedLoopSet::new(
        "reconstructed-set".to_string(),
        "request".to_string(),
        rows,
    )
}

pub(super) fn empty_born_loops() -> PlanarBooleanBornLoopSet {
    PlanarBooleanBornLoopSet::new("born-set".to_string(), "request".to_string(), Vec::new())
}

pub(super) fn empty_source_loop_carriers() -> PlanarBooleanLoopSourceCarrierSet {
    PlanarBooleanLoopSourceCarrierSet::new(
        "carrier-set".to_string(),
        "request".to_string(),
        "split-ledger".to_string(),
        Vec::new(),
    )
}

pub(super) fn empty_split_fragments() -> PlanarBooleanSplitEdgeFragmentSet {
    PlanarBooleanSplitEdgeFragmentSet::new(
        "fragment-set".to_string(),
        "interval-subdivision".to_string(),
        "split-vertex-set".to_string(),
        Vec::new(),
        Default::default(),
    )
}

pub(super) fn role_outcomes(
    rows: Vec<PlanarBooleanLoopRoleOutcome>,
) -> PlanarBooleanLoopRoleOutcomeSet {
    PlanarBooleanLoopRoleOutcomeSet::new("role-set".to_string(), "request".to_string(), rows)
}

pub(super) fn preserved_role(loop_identity: &str) -> PlanarBooleanLoopRoleOutcome {
    PlanarBooleanLoopRoleOutcome::new(
        "role-outcome".to_string(),
        loop_identity.to_string(),
        crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
        Vec::new(),
        vec!["source-loop".to_string()],
        Some(PlanarBooleanLoopRole::OuterBoundary),
        PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole,
    )
}

pub(super) fn preserved_containment(
    loop_identity: &str,
) -> PlanarBooleanLoopContainmentEvidencePosture {
    PlanarBooleanLoopContainmentEvidencePosture::new(
        "containment".to_string(),
        loop_identity.to_string(),
        crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
        Vec::new(),
        vec!["source-loop".to_string()],
        PlanarBooleanLoopContainmentEvidencePostureKind::PreservedSourceContainmentEvidence,
    )
}

pub(super) fn triangle_geometry() -> (
    PlanarBooleanLoopSourceCarrierSet,
    PlanarBooleanSplitEdgeFragmentSet,
) {
    let carriers = vec![
        loop_source_carrier_row(
            "a",
            "edge-a",
            "vertex-a",
            [0.0, 0.0],
            "vertex-b",
            [1.0, 0.0],
        ),
        loop_source_carrier_row(
            "b",
            "edge-b",
            "vertex-b",
            [1.0, 0.0],
            "vertex-c",
            [0.0, 1.0],
        ),
        loop_source_carrier_row(
            "c",
            "edge-c",
            "vertex-c",
            [0.0, 1.0],
            "vertex-a",
            [0.0, 0.0],
        ),
    ];
    let schedules = vec![
        fragment_schedule("a", "edge-a", "carrier-a", "fragment-a"),
        fragment_schedule("b", "edge-b", "carrier-b", "fragment-b"),
        fragment_schedule("c", "edge-c", "carrier-c", "fragment-c"),
    ];
    (
        PlanarBooleanLoopSourceCarrierSet::new(
            "carrier-set".to_string(),
            "request".to_string(),
            "split-ledger".to_string(),
            carriers,
        ),
        PlanarBooleanSplitEdgeFragmentSet::new(
            "fragment-set".to_string(),
            "interval-subdivision".to_string(),
            "split-vertex-set".to_string(),
            schedules,
            Default::default(),
        ),
    )
}

pub(super) fn collinear_geometry() -> (
    PlanarBooleanLoopSourceCarrierSet,
    PlanarBooleanSplitEdgeFragmentSet,
) {
    let (_, split_fragments) = triangle_geometry();
    let carriers = vec![
        loop_source_carrier_row(
            "a",
            "edge-a",
            "vertex-a",
            [0.0, 0.0],
            "vertex-b",
            [1.0, 0.0],
        ),
        loop_source_carrier_row(
            "b",
            "edge-b",
            "vertex-b",
            [1.0, 0.0],
            "vertex-c",
            [2.0, 0.0],
        ),
        loop_source_carrier_row(
            "c",
            "edge-c",
            "vertex-c",
            [2.0, 0.0],
            "vertex-a",
            [0.0, 0.0],
        ),
    ];
    (
        PlanarBooleanLoopSourceCarrierSet::new(
            "carrier-set-collinear".to_string(),
            "request".to_string(),
            "split-ledger".to_string(),
            carriers,
        ),
        split_fragments,
    )
}

fn loop_source_carrier_row(
    suffix: &str,
    source_edge_identity: &str,
    start_source_endpoint_identity: &str,
    start_point: [f64; 2],
    end_source_endpoint_identity: &str,
    end_point: [f64; 2],
) -> PlanarBooleanLoopSourceCarrierRow {
    PlanarBooleanLoopSourceCarrierRow::new(
        format!("carrier-row-{suffix}"),
        format!("recovered-{suffix}"),
        format!("carrier-{suffix}"),
        "source-face".to_string(),
        "source-loop".to_string(),
        source_edge_identity.to_string(),
        start_source_endpoint_identity.to_string(),
        [start_point[0].to_bits(), start_point[1].to_bits()],
        end_source_endpoint_identity.to_string(),
        [end_point[0].to_bits(), end_point[1].to_bits()],
        PlanarBooleanLoopRole::OuterBoundary,
    )
}

fn fragment_schedule(
    suffix: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    fragment_identity: &str,
) -> PlanarBooleanSplitEdgeFragmentSchedule {
    PlanarBooleanSplitEdgeFragmentSchedule::new(
        format!("schedule-{suffix}"),
        format!("interval-{suffix}"),
        format!("split-schedule-{suffix}"),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        vec![PlanarBooleanSplitEdgeFragment::new(
            fragment_identity.to_string(),
            source_edge_identity.to_string(),
            carrier_identity.to_string(),
            PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_start(
                source_edge_identity,
                carrier_identity,
                "local-frame",
                "precision-basis",
            ),
            PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_end(
                source_edge_identity,
                carrier_identity,
                "local-frame",
                "precision-basis",
            ),
            [0.0, 1.0],
            [0.0f64.to_bits(), 1.0f64.to_bits()],
            "local-frame".to_string(),
            "precision-basis".to_string(),
            vec![PlanarBooleanSourceIntervalSense::Forward],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )],
    )
}
