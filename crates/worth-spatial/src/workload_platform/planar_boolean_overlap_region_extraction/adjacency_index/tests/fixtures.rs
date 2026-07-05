use topology::facade::admit_milestone_seven_five_overlap_readiness_consumer;
use worth_kernel::workload_composition::current_touched_graph_readiness_handoff;

use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    admitted_phase_fourteen_identity_products, prepared_phase_fourteen_subject,
    LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopReconstructionLedger,
    PlanarBooleanLoopReconstructionParticipationSupport,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapParticipationRecovery, PlanarBooleanOverlapParticipationRecoveryInput,
    PlanarBooleanOverlapRegionExtractionRequest, PlanarBooleanOverlapRegionExtractionRequestInput,
};

pub(super) fn recovered_participation(
    order: LoopFixtureEntryOrder,
) -> crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapParticipationRecovery
{
    let readiness = current_touched_graph_readiness_handoff()
        .expect("current readiness handoff should assemble");
    let (request, support) = overlap_request_and_support(order, &readiness);
    PlanarBooleanOverlapParticipationRecovery::recover(
        PlanarBooleanOverlapParticipationRecoveryInput::from_request_and_loop_support(
            &request, &support,
        ),
    )
    .expect("phase-four fixture should recover participation")
}

fn overlap_request_and_support(
    order: LoopFixtureEntryOrder,
    readiness: &schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput,
) -> (
    PlanarBooleanOverlapRegionExtractionRequest,
    PlanarBooleanLoopReconstructionParticipationSupport,
) {
    let fixture = prepared_phase_fourteen_subject(order);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("phase fourteen products should admit loop decision-log recording");
    let (identity_map, persistent_name_map, subshape_signature_map) =
        admitted_phase_fourteen_identity_products(&fixture);
    let (ledger, receipt) = PlanarBooleanLoopReconstructionLedger::assemble(
        fixture.ledger_input_with_identity_products(
            &decision_log,
            &identity_map,
            &persistent_name_map,
            &subshape_signature_map,
        ),
    )
    .expect("phase fourteen products should assemble the loop ledger");
    let readiness_consumer = admit_milestone_seven_five_overlap_readiness_consumer(readiness)
        .expect("7.5 readiness consumer should admit");
    let request = PlanarBooleanOverlapRegionExtractionRequest::admit(
        PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
            &readiness_consumer,
            &receipt,
        ),
    )
    .expect("overlap request should admit from readiness and real 7.4 receipt");
    let support =
        PlanarBooleanLoopReconstructionParticipationSupport::admit_from_ledger_and_products(
            &ledger,
            fixture.role_boundary.role_outcomes(),
            &fixture.island_partition,
            &persistent_name_map,
            fixture.source_provenance.fragment_membership_map(),
            fixture.source_provenance.overlap_chain_lineage_map(),
            fixture.source_provenance.source_loop_carriers(),
        )
        .expect("phase fourteen products should admit participation support");
    (request, support)
}
