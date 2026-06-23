use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanSplitEdgeFragment,
    PlanarBooleanSplitEdgeFragmentSchedule, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitVertexIdentityCounters, PlanarBooleanSplitVertexIdentityRow,
    PlanarBooleanSplitVertexIdentitySchedule, PlanarBooleanSplitVertexIdentitySet,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanFragmentContinuationIndex, PlanarBooleanFragmentContinuationIndexInput,
    PlanarBooleanFragmentMembershipMap, PlanarBooleanLoopOverlapChainLineageMap,
    PlanarBooleanLoopReconstructionRequest, PlanarBooleanLoopSourceCarrierRow,
    PlanarBooleanLoopSourceCarrierSet, PlanarBooleanLoopSourceProvenanceBundle,
    PlanarBooleanLoopSourceProvenanceRecoveryInput,
};

use super::runtime_subject::{
    prepared_loop_reconstruction_subject, LoopFixtureEntryOrder, PreparedLoopReconstructionSubject,
};

pub(crate) struct PreparedLoopContinuationIndexSubject {
    pub(crate) subject: PreparedLoopReconstructionSubject,
    pub(crate) request: PlanarBooleanLoopReconstructionRequest,
    pub(crate) source_provenance: PlanarBooleanLoopSourceProvenanceBundle,
    pub(crate) continuation_index: PlanarBooleanFragmentContinuationIndex,
}

pub(crate) fn prepared_loop_continuation_subject(
    order: LoopFixtureEntryOrder,
) -> PreparedLoopContinuationIndexSubject {
    let subject = prepared_loop_reconstruction_subject(order);
    let request = subject.admit_loop_request();
    let source_provenance = PlanarBooleanLoopSourceProvenanceBundle::recover(
        PlanarBooleanLoopSourceProvenanceRecoveryInput::from_request_and_split_support(
            &request,
            subject.split_ledger_result.ledger(),
            subject.split_ledger_result.receipt(),
            &subject.recovered_source_carriers,
            &subject.fragments,
            &subject.overlap_chains,
        ),
    )
    .expect("source provenance should recover for continuation tests");
    let continuation_index = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &request,
            &source_provenance,
            &subject.vertices,
            &subject.fragments,
            &subject.overlap_chains,
        ),
    )
    .expect("continuation index should admit for continuation tests");
    PreparedLoopContinuationIndexSubject {
        subject,
        request,
        source_provenance,
        continuation_index,
    }
}

pub(crate) fn split_vertices_without_first_vertex(
    vertices: &PlanarBooleanSplitVertexIdentitySet,
) -> PlanarBooleanSplitVertexIdentitySet {
    let schedules = vertices
        .schedules()
        .iter()
        .filter_map(|schedule| {
            let remaining = schedule
                .vertices()
                .iter()
                .skip(1)
                .cloned()
                .collect::<Vec<_>>();
            if remaining.is_empty() {
                None
            } else {
                Some(PlanarBooleanSplitVertexIdentitySchedule::new(
                    schedule.schedule_identity().to_string(),
                    schedule
                        .interval_subdivision_schedule_identity()
                        .to_string(),
                    schedule.source_edge_identity().to_string(),
                    schedule.carrier_identity().to_string(),
                    remaining,
                    schedule.coalescence_decisions().to_vec(),
                ))
            }
        })
        .collect::<Vec<_>>();
    PlanarBooleanSplitVertexIdentitySet::new(
        vertices.split_vertex_identity_set_identity().to_string(),
        vertices
            .interval_subdivision_schedule_set_identity()
            .to_string(),
        schedules,
        vertices.counters(),
    )
}

pub(crate) fn duplicate_first_fragment_for_continuation_slot(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let schedules = fragments
        .schedules()
        .iter()
        .enumerate()
        .map(|(index, schedule)| {
            let mut rows = schedule.fragments().to_vec();
            if index == 0 {
                let duplicate = rows
                    .first()
                    .cloned()
                    .expect("test support requires at least one fragment to duplicate");
                rows.push(duplicate);
            }
            PlanarBooleanSplitEdgeFragmentSchedule::new(
                schedule.schedule_identity().to_string(),
                schedule
                    .interval_subdivision_schedule_identity()
                    .to_string(),
                schedule
                    .split_vertex_identity_schedule_identity()
                    .to_string(),
                schedule.source_edge_identity().to_string(),
                schedule.carrier_identity().to_string(),
                rows,
            )
        })
        .collect::<Vec<_>>();
    PlanarBooleanSplitEdgeFragmentSet::new(
        fragments.fragment_set_identity().to_string(),
        fragments
            .interval_subdivision_schedule_set_identity()
            .to_string(),
        fragments.split_vertex_identity_set_identity().to_string(),
        schedules,
        fragments.counters(),
    )
}

pub(crate) fn source_provenance_with_missing_fragment_membership(
    source_provenance: &PlanarBooleanLoopSourceProvenanceBundle,
) -> PlanarBooleanLoopSourceProvenanceBundle {
    let rows = source_provenance
        .fragment_membership_map()
        .rows()
        .iter()
        .skip(1)
        .cloned()
        .collect::<Vec<_>>();
    let fragment_membership_map = PlanarBooleanFragmentMembershipMap::new(
        source_provenance
            .fragment_membership_map()
            .membership_map_identity()
            .to_string(),
        source_provenance
            .fragment_membership_map()
            .request_identity()
            .to_string(),
        source_provenance
            .fragment_membership_map()
            .fragment_set_identity()
            .to_string(),
        rows,
    );
    PlanarBooleanLoopSourceProvenanceBundle::new(
        source_provenance.bundle_identity().to_string(),
        source_provenance.request_identity().to_string(),
        source_provenance
            .split_ledger_receipt_identity()
            .to_string(),
        source_provenance.source_loop_carriers().clone(),
        fragment_membership_map,
        source_provenance.overlap_chain_lineage_map().clone(),
        source_provenance.counters(),
    )
}

pub(crate) fn source_provenance_without_first_source_loop_carrier(
    source_provenance: &PlanarBooleanLoopSourceProvenanceBundle,
) -> PlanarBooleanLoopSourceProvenanceBundle {
    let first_source_loop_identity = source_provenance
        .source_loop_carriers()
        .rows()
        .first()
        .map(PlanarBooleanLoopSourceCarrierRow::source_loop_identity)
        .expect("test support requires at least one source loop carrier")
        .to_string();
    let rows = source_provenance
        .source_loop_carriers()
        .rows()
        .iter()
        .filter(|row| row.source_loop_identity() != first_source_loop_identity)
        .cloned()
        .collect::<Vec<_>>();
    let source_loop_carriers = PlanarBooleanLoopSourceCarrierSet::new(
        source_provenance
            .source_loop_carriers()
            .carrier_set_identity()
            .to_string(),
        source_provenance
            .source_loop_carriers()
            .request_identity()
            .to_string(),
        source_provenance
            .source_loop_carriers()
            .split_ledger_receipt_identity()
            .to_string(),
        rows,
    );
    PlanarBooleanLoopSourceProvenanceBundle::new(
        source_provenance.bundle_identity().to_string(),
        source_provenance.request_identity().to_string(),
        source_provenance
            .split_ledger_receipt_identity()
            .to_string(),
        source_loop_carriers,
        source_provenance.fragment_membership_map().clone(),
        source_provenance.overlap_chain_lineage_map().clone(),
        source_provenance.counters(),
    )
}
