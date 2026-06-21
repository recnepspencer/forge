#[path = "../public_api_planar_boolean_loop_reconstruction_workload_evidence_fixtures.rs"]
mod fixtures;

use topology::facade::PlanarBooleanLoopBlueprintRegistry;

use super::continuation_contract_support::completed_split_handoff_for;
use super::edge_splitting_replay_parity_support::build_edge_split_replay_parity_subject;
use super::metaboss_support::MetabossEventExtractionSubject;
use super::real_handoff_support::{real_loop_handoff_for_branch, ReplayBranch};
use fixtures::assert_runtime_registration_proof;

pub(crate) fn assert_boolean_chain_accepts_only_completed_receipts_and_query_proof() {
    let subject = MetabossEventExtractionSubject::certify("phase7.5 boolean chain handoff");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let loop_handoff =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("real loop handoff should certify before boolean chain integration");

    let boolean_chain = loop_handoff
        .complete_boolean_chain_integration_handoff(&split_handoff)
        .expect("7.5-facing boolean chain handoff should admit from completed split and loop receipts plus Query proof");

    assert_eq!(
        boolean_chain.split_ledger_receipt(),
        split_handoff.split_ledger_receipt()
    );
    assert_eq!(
        boolean_chain.loop_ledger_receipt(),
        loop_handoff.loop_ledger_receipt()
    );
    assert_eq!(
        boolean_chain.query_graph_proof_identity(),
        loop_handoff.runtime_registration_proof().proof_identity()
    );
    assert_eq!(
        boolean_chain.workload_stage_index_identity(),
        loop_handoff.workload_stage_index_identity()
    );
    assert_runtime_registration_proof(
        boolean_chain.runtime_registration_proof(),
        boolean_chain.loop_ledger_receipt(),
        boolean_chain.workload_stage_index_identity(),
        matrix.registry_identity().digest(),
    );
    assert_boolean_chain_counters_match_declared_receipt_breadth(
        &boolean_chain,
        &split_handoff,
        &loop_handoff,
    );
}

pub(crate) fn assert_boolean_chain_query_proof_does_not_rewrite_ledger_identities() {
    let subject =
        MetabossEventExtractionSubject::certify("phase7.5 boolean chain ledger identity replay");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let original =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("original loop handoff should certify");
    let replayed =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Replayed, &matrix, &validators)
            .expect("replayed loop handoff should certify");

    let original_chain = original
        .complete_boolean_chain_integration_handoff(&split_handoff)
        .expect("original boolean chain should admit");
    let replayed_chain = replayed
        .complete_boolean_chain_integration_handoff(&split_handoff)
        .expect("replayed boolean chain should admit");

    assert_eq!(
        original_chain.split_ledger_receipt().ledger_identity(),
        replayed_chain.split_ledger_receipt().ledger_identity()
    );
    assert_eq!(
        original_chain.loop_ledger_receipt().ledger_identity(),
        replayed_chain.loop_ledger_receipt().ledger_identity()
    );
    assert_eq!(
        original_chain.query_graph_proof_identity(),
        replayed_chain.query_graph_proof_identity()
    );
    assert_eq!(original_chain.counters(), replayed_chain.counters());
    assert_eq!(original_chain.counters().query_graph_proofs_consumed(), 1);
    assert_eq!(original_chain.counters().ledger_receipts_consumed(), 2);
}

pub(crate) fn assert_boolean_chain_residue_manifest_is_capped_and_non_authority() {
    let subject = MetabossEventExtractionSubject::certify("phase7.5 boolean chain residue");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let loop_handoff =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("real loop handoff should certify");
    let boolean_chain = loop_handoff
        .complete_boolean_chain_integration_handoff(&split_handoff)
        .expect("boolean chain handoff should admit");

    assert_eq!(boolean_chain.residue_manifest().len(), 2);
    for row in boolean_chain.residue_manifest() {
        assert!(!row.id().is_empty());
        assert!(!row.owner().is_empty());
        assert_eq!(row.cap(), 1);
        assert!(!row.removal_trigger().is_empty());
        assert!(row.boundary().contains("not") || row.boundary().contains("typed"));
    }
}

pub(crate) fn assert_large_admitted_boolean_chain_scales_with_declared_breadth() {
    let subject = MetabossEventExtractionSubject::certify("phase7 boolean chain density");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let loop_handoff =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("real loop handoff should certify");

    let boolean_chain = loop_handoff
        .complete_boolean_chain_integration_handoff(&split_handoff)
        .expect("boolean chain handoff should admit");

    assert_boolean_chain_counters_match_declared_receipt_breadth(
        &boolean_chain,
        &split_handoff,
        &loop_handoff,
    );
    assert!(boolean_chain.counters().declared_split_chain_breadth() > 0);
    assert_eq!(
        boolean_chain.counters().stage_index_lookups(),
        expected_boolean_chain_stage_index_lookups(&split_handoff, &loop_handoff)
    );
}

fn assert_boolean_chain_counters_match_declared_receipt_breadth(
    boolean_chain: &worth_kernel::workload_composition::BooleanChainIntegrationHandoff,
    split_handoff: &worth_kernel::workload_composition::CompletedBooleanSplitHandoff,
    loop_handoff: &worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
) {
    assert_eq!(
        boolean_chain.counters().declared_split_chain_breadth(),
        split_handoff
            .split_ledger_receipt()
            .counters()
            .ledger_chains_emitted()
    );
    assert_eq!(
        boolean_chain.counters().declared_loop_ledger_breadth(),
        loop_handoff
            .loop_ledger_receipt()
            .counters()
            .ledger_rows_emitted()
    );
    assert_eq!(boolean_chain.counters().ledger_receipts_consumed(), 2);
    assert_eq!(boolean_chain.counters().query_graph_proofs_consumed(), 1);
    assert_eq!(boolean_chain.counters().residue_rows(), 2);
}

fn expected_boolean_chain_stage_index_lookups(
    split_handoff: &worth_kernel::workload_composition::CompletedBooleanSplitHandoff,
    loop_handoff: &worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
) -> usize {
    let split_lookup = split_handoff
        .require_boolean_split_lookup()
        .expect("completed split handoff should expose its indexed split receipt lookup");
    let loop_lookup = loop_handoff
        .require_boolean_loop_reconstruction_lookup()
        .expect("completed loop handoff should expose its indexed loop receipt lookup");
    let loop_workload_split_lookup = loop_handoff
        .completed_workload()
        .require_boolean_split_lookup(split_handoff.split_ledger_receipt())
        .expect("completed loop workload should retain the indexed split receipt lookup");

    split_lookup.lookup_counters().indexed_lookup_count()
        + loop_lookup.lookup_counters().indexed_lookup_count()
        + loop_workload_split_lookup
            .lookup_counters()
            .indexed_lookup_count()
}
