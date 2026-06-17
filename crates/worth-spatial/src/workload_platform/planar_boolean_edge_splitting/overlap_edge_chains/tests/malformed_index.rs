use super::*;
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentCounters,
    PlanarBooleanSplitEdgeFragmentEndpointRef, PlanarBooleanSplitEdgeFragmentSchedule,
    PlanarBooleanSplitEdgeFragmentSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanNormalizedIntervalSubdivisionRow,
};

#[test]
fn overlap_edge_chain_rejects_fragment_with_mismatched_subdivision_authority() {
    let normalized = one_interval_schedule();
    let subdivision = &normalized.schedules()[0].interval_subdivisions()[0];
    let malformed = fragment_with_mismatched_authority(subdivision);
    let forged_fragments = fragment_set_for_normalized(&normalized, vec![malformed]);

    let denial = normalized
        .build_overlap_edge_chains(&forged_fragments)
        .expect_err("fragment/subdivision authority mismatch must deny");

    assert_eq!(
        denial.denial_kind(),
        PlanarBooleanOverlapEdgeChainDenialKind::MismatchedFragmentAuthority
    );
}

#[test]
fn overlap_edge_chain_rejects_missing_fragment_reference() {
    let normalized = one_interval_schedule();
    let empty_same_lineage_fragments = PlanarBooleanSplitEdgeFragmentSet::new(
        "empty-fragment-set".to_string(),
        normalized.schedule_set_identity().to_string(),
        "split-vertex-set".to_string(),
        Vec::new(),
        PlanarBooleanSplitEdgeFragmentCounters::new(1, 0, 0, 0, 0, 0, 0, 0, 0, 0),
    );

    let denial = normalized
        .build_overlap_edge_chains(&empty_same_lineage_fragments)
        .expect_err("each subdivision must be covered by at least one fragment");

    assert_eq!(
        denial.denial_kind(),
        PlanarBooleanOverlapEdgeChainDenialKind::MissingFragmentReference
    );
}

#[test]
fn overlap_edge_chain_rejects_missing_subdivision_reference() {
    let normalized = one_interval_schedule();
    let subdivision = &normalized.schedules()[0].interval_subdivisions()[0];
    let dangling_fragment = fragment_with_subdivision_identity(subdivision, "missing-subdivision");
    let forged_fragments = fragment_set_for_normalized(&normalized, vec![dangling_fragment]);

    let denial = normalized
        .build_overlap_edge_chains(&forged_fragments)
        .expect_err("dangling subdivision reference must deny");

    assert_eq!(
        denial.denial_kind(),
        PlanarBooleanOverlapEdgeChainDenialKind::MissingSubdivisionReference
    );
}

fn one_interval_schedule() -> PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    interval_normalized(
        interval_set(vec![admitted_interval(
            "partial",
            "event:partial",
            "carrier:a",
            "source edge a",
            [0.25, 0.75],
            PlanarBooleanIntervalEventKind::PartialOverlap,
            PlanarBooleanSourceIntervalSense::Forward,
        )]),
        None,
    )
}

fn fragment_with_mismatched_authority(
    subdivision: &PlanarBooleanNormalizedIntervalSubdivisionRow,
) -> PlanarBooleanSplitEdgeFragment {
    PlanarBooleanSplitEdgeFragment::new(
        "malformed-fragment".to_string(),
        "foreign source edge".to_string(),
        "foreign carrier".to_string(),
        PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_start(
            "foreign source edge",
            "foreign carrier",
            subdivision.local_frame_identity(),
            subdivision.precision_basis_identity(),
        ),
        PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_end(
            "foreign source edge",
            "foreign carrier",
            subdivision.local_frame_identity(),
            subdivision.precision_basis_identity(),
        ),
        subdivision.admitted_parameter_range(),
        [
            subdivision.admitted_parameter_range()[0].to_bits(),
            subdivision.admitted_parameter_range()[1].to_bits(),
        ],
        subdivision.local_frame_identity().to_string(),
        subdivision.precision_basis_identity().to_string(),
        vec![subdivision.source_sense()],
        Vec::new(),
        vec![subdivision.subdivision_identity().to_string()],
        vec![subdivision.normalized_interval_identity().to_string()],
        subdivision.event_group_identities().to_vec(),
        subdivision.provenance_entry_identities().to_vec(),
    )
}

fn fragment_with_subdivision_identity(
    subdivision: &PlanarBooleanNormalizedIntervalSubdivisionRow,
    interval_subdivision_identity: &str,
) -> PlanarBooleanSplitEdgeFragment {
    PlanarBooleanSplitEdgeFragment::new(
        "dangling-fragment".to_string(),
        subdivision.source_edge_identity().to_string(),
        subdivision.carrier_identity().to_string(),
        PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_start(
            subdivision.source_edge_identity(),
            subdivision.carrier_identity(),
            subdivision.local_frame_identity(),
            subdivision.precision_basis_identity(),
        ),
        PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_end(
            subdivision.source_edge_identity(),
            subdivision.carrier_identity(),
            subdivision.local_frame_identity(),
            subdivision.precision_basis_identity(),
        ),
        subdivision.admitted_parameter_range(),
        [
            subdivision.admitted_parameter_range()[0].to_bits(),
            subdivision.admitted_parameter_range()[1].to_bits(),
        ],
        subdivision.local_frame_identity().to_string(),
        subdivision.precision_basis_identity().to_string(),
        vec![subdivision.source_sense()],
        Vec::new(),
        vec![interval_subdivision_identity.to_string()],
        vec![subdivision.normalized_interval_identity().to_string()],
        subdivision.event_group_identities().to_vec(),
        subdivision.provenance_entry_identities().to_vec(),
    )
}

fn fragment_set_for_normalized(
    normalized: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    fragments: Vec<PlanarBooleanSplitEdgeFragment>,
) -> PlanarBooleanSplitEdgeFragmentSet {
    PlanarBooleanSplitEdgeFragmentSet::new(
        "forged-fragment-set".to_string(),
        normalized.schedule_set_identity().to_string(),
        "forged-split-vertex-set".to_string(),
        vec![PlanarBooleanSplitEdgeFragmentSchedule::new(
            "forged-fragment-schedule".to_string(),
            normalized.schedules()[0].schedule_identity().to_string(),
            "forged-split-vertex-schedule".to_string(),
            normalized.schedules()[0].source_edge_identity().to_string(),
            normalized.schedules()[0].carrier_identity().to_string(),
            fragments,
        )],
        PlanarBooleanSplitEdgeFragmentCounters::new(1, 1, 0, 2, 1, 1, 0, 0, 0, 0),
    )
}
