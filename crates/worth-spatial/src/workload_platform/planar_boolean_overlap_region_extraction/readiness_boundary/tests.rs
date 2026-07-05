use topology::facade::admit_milestone_seven_five_overlap_readiness_consumer;
use worth_kernel::workload_composition::current_touched_graph_readiness_handoff;

use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_phase_fourteen_subject, LoopFixtureEntryOrder,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopDecisionLog, PlanarBooleanLoopReconstructionLedger,
    PlanarBooleanLoopReconstructionLedgerReceipt,
};

use super::{
    PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind,
    PlanarBooleanOverlapRegionExtractionRequest, PlanarBooleanOverlapRegionExtractionRequestInput,
};

#[test]
fn overlap_request_admits_from_readiness_consumer_and_matching_loop_ledger() {
    run_stack_heavy_readiness_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let loop_ledger_receipt = loop_ledger_receipt_for_tests(LoopFixtureEntryOrder::Canonical);
        let readiness_consumer =
            readiness_consumer_for_loop_ledger(&readiness, &loop_ledger_receipt);

        let request = PlanarBooleanOverlapRegionExtractionRequest::admit(
            PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
                &readiness_consumer,
                &loop_ledger_receipt,
            ),
        )
        .expect("overlap request should admit from readiness and matching loop ledger");

        let binding = request.readiness_loop_ledger_binding();
        assert_eq!(
            binding.selected_route_identity_digest(),
            readiness_consumer.selected_route_identity_digest()
        );
        assert_eq!(
            binding.touched_closure_digest(),
            readiness_consumer.touched_closure_digest()
        );
        assert_eq!(
            binding.selected_plan_digest(),
            readiness_consumer.selected_plan_digest()
        );
        assert_eq!(
            binding.overlap_identity_digests(),
            readiness_consumer.overlap_identity_digests()
        );
        assert_eq!(
            binding.topology_query_posture_digest(),
            readiness_consumer.topology_query_posture_digest()
        );
        assert_eq!(
            binding.spatial_query_posture_digest(),
            readiness_consumer.spatial_query_posture_digest()
        );
        assert_eq!(
            binding.residue_digest(),
            readiness_consumer.residue_digest()
        );
        assert_eq!(
            binding.source_firewall_digest(),
            readiness_consumer.source_firewall_digest()
        );
        assert_eq!(
            binding.architecture_claim_digest(),
            readiness_consumer.architecture_claim_digest()
        );
        assert_eq!(
            binding.loop_ledger_receipt_identity(),
            loop_ledger_receipt.receipt_identity()
        );
        assert_eq!(
            binding.loop_ledger_request_identity(),
            loop_ledger_receipt.request_identity()
        );
        assert_eq!(
            binding.selected_plan_digest(),
            loop_ledger_receipt.selected_plan_digest()
        );
        assert_eq!(
            binding.selected_route_identity_digest(),
            loop_ledger_receipt.selected_route_identity_digest()
        );
        assert_eq!(
            binding.selected_family_identity(),
            loop_ledger_receipt.selected_family_identity()
        );
        assert_eq!(
            binding.selected_product_identity_digest(),
            loop_ledger_receipt.selected_product_identity_digest()
        );
        assert_eq!(
            binding.selected_witness_identity_digest(),
            loop_ledger_receipt.selected_witness_identity_digest()
        );
        assert_eq!(
            binding.touched_closure_digest(),
            loop_ledger_receipt.touched_closure_digest()
        );
        assert_eq!(
            binding.overlap_identity_digests(),
            loop_ledger_receipt.overlap_identity_digests()
        );
        assert_eq!(
            binding.topology_query_posture_digest(),
            loop_ledger_receipt.topology_query_posture_digest()
        );
        assert_eq!(
            binding.spatial_query_posture_digest(),
            loop_ledger_receipt.spatial_query_posture_digest()
        );
        assert_eq!(
            binding.residue_digest(),
            loop_ledger_receipt.residue_digest()
        );
        assert_eq!(
            binding.source_firewall_digest(),
            loop_ledger_receipt.source_firewall_digest()
        );
        assert_eq!(
            binding.architecture_claim_digest(),
            loop_ledger_receipt.architecture_claim_digest()
        );
        assert!(request.certifies_overlap_region_extraction_request());
    });
}

#[test]
fn overlap_request_identity_is_replay_stable_for_equivalent_readiness_and_loop_lineage() {
    run_stack_heavy_readiness_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let canonical_receipt = loop_ledger_receipt_for_tests(LoopFixtureEntryOrder::Canonical);
        let replayed_receipt = loop_ledger_receipt_for_tests(LoopFixtureEntryOrder::Replayed);
        let readiness_consumer = readiness_consumer_for_loop_ledger(&readiness, &canonical_receipt);

        let canonical_request = PlanarBooleanOverlapRegionExtractionRequest::admit(
            PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
                &readiness_consumer,
                &canonical_receipt,
            ),
        )
        .expect("canonical overlap request should admit");
        let replayed_request = PlanarBooleanOverlapRegionExtractionRequest::admit(
            PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
                &readiness_consumer,
                &replayed_receipt,
            ),
        )
        .expect("replayed overlap request should admit");

        assert_eq!(
            canonical_request
                .readiness_loop_ledger_binding()
                .binding_identity(),
            replayed_request
                .readiness_loop_ledger_binding()
                .binding_identity()
        );
        assert_eq!(
            canonical_request.request_identity(),
            replayed_request.request_identity()
        );
    });
}

#[test]
fn overlap_request_rejects_missing_loop_ledger_receipt_provenance() {
    run_stack_heavy_readiness_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let hostile_receipt = loop_ledger_receipt_for_tests(LoopFixtureEntryOrder::Canonical)
            .with_receipt_identity_for_tests(String::new());
        let readiness_consumer = readiness_consumer_for_loop_ledger(&readiness, &hostile_receipt);

        let denial = PlanarBooleanOverlapRegionExtractionRequest::admit(
            PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
                &readiness_consumer,
                &hostile_receipt,
            ),
        )
        .expect_err("overlap request should reject missing loop-ledger receipt identity");

        assert_eq!(
            denial.binding_denial().kind(),
            PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind::MissingLoopLedgerReceiptIdentity
        );
    });
}

#[test]
fn overlap_request_does_not_heal_the_supplied_loop_ledger_receipt() {
    run_stack_heavy_readiness_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let receipt = loop_ledger_receipt_for_tests(LoopFixtureEntryOrder::Canonical);
        let readiness_consumer = readiness_consumer_for_loop_ledger(&readiness, &receipt);

        let request = PlanarBooleanOverlapRegionExtractionRequest::admit(
            PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
                &readiness_consumer,
                &receipt,
            ),
        )
        .expect("overlap request should admit from admitted readiness and real loop receipt");

        assert_eq!(
            request
                .readiness_loop_ledger_binding()
                .loop_ledger_receipt_identity(),
            receipt.receipt_identity()
        );
        assert_eq!(
            request
                .readiness_loop_ledger_binding()
                .loop_ledger_downstream_consumption_identity(),
            receipt.downstream_consumption_identity()
        );
        assert_eq!(
            request
                .readiness_loop_ledger_binding()
                .selected_plan_digest(),
            receipt.selected_plan_digest()
        );
    });
}

#[test]
fn overlap_request_rejects_real_loop_ledger_receipt_with_mismatched_selected_route_identity() {
    run_stack_heavy_readiness_test(|| {
        let readiness = current_touched_graph_readiness_handoff()
            .expect("current readiness handoff should assemble");
        let canonical_receipt = loop_ledger_receipt_for_tests(LoopFixtureEntryOrder::Canonical);
        let readiness_consumer = readiness_consumer_for_loop_ledger(&readiness, &canonical_receipt);
        let hostile_receipt = canonical_receipt
            .with_selected_route_identity_digest_for_tests("forged-selected-route");

        let denial = PlanarBooleanOverlapRegionExtractionRequest::admit(
            PlanarBooleanOverlapRegionExtractionRequestInput::from_readiness_consumer_and_loop_ledger(
                &readiness_consumer,
                &hostile_receipt,
            ),
        )
        .expect_err("overlap request should reject mismatched selected-route provenance");

        assert_eq!(
            denial.binding_denial().kind(),
            PlanarBooleanOverlapReadinessLoopLedgerBindingDenialKind::SelectedRouteIdentityMismatch
        );
    });
}

fn loop_ledger_receipt_for_tests(
    order: LoopFixtureEntryOrder,
) -> PlanarBooleanLoopReconstructionLedgerReceipt {
    let fixture = prepared_phase_fourteen_subject(order);
    let decision_log = PlanarBooleanLoopDecisionLog::record(fixture.decision_log_input())
        .expect("phase fourteen products should admit loop decision-log recording");
    let (_, receipt) =
        PlanarBooleanLoopReconstructionLedger::assemble(fixture.ledger_input(&decision_log))
            .expect("phase fourteen products should assemble the loop ledger");
    receipt
}

fn readiness_consumer_for_loop_ledger(
    readiness: &schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput,
    _loop_ledger_receipt: &PlanarBooleanLoopReconstructionLedgerReceipt,
) -> topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer {
    admit_milestone_seven_five_overlap_readiness_consumer(readiness)
        .expect("7.5 readiness consumer should admit")
}

fn run_stack_heavy_readiness_test(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(test)
        .expect("stack-heavy readiness test should spawn")
        .join()
        .expect("stack-heavy readiness test should finish");
}
