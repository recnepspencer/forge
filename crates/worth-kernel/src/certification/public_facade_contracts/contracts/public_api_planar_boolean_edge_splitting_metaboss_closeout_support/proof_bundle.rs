use super::super::edge_splitting_public_contract_support::completed_split_handoff_for;
use super::super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report,
};
use super::super::metaboss_support::{assert_event_ledger_shape, MetabossEventExtractionSubject};
use super::candidate_manifest_metrics::candidate_rows_have_provenance;
use super::decision_localization_metrics::{
    decision_kind_count, decision_phase_count, localized_decision_rows,
};
use super::event_relation_metrics::{
    event_bearing_pair_count, interval_kind_counts, point_kind_counts, IntervalKindCounts,
    PointKindCounts,
};
use super::topology_closeout_metrics::{topology_closeout_summary, TopologyCloseoutSummary};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitSummumBonumCloseout, PlanarBooleanEdgeSplitSummumBonumCloseoutInput,
    PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy,
};

pub(crate) struct EdgeSplitSummumBonumProofBundle {
    pub(crate) catalog_recipe_query_key: String,
    pub(crate) candidate_index_product_identity: String,
    pub(crate) candidate_index_plan_digest: String,
    pub(crate) production_closeout_identity: String,
    pub(crate) production_closeout_certifies: bool,
    pub(crate) candidate_strategy: PlanarBooleanCandidateIndexStrategy,
    pub(crate) candidate_fallback_posture: PlanarBooleanCandidateIndexFallbackPosture,
    pub(crate) candidate_lifecycle: PlanarBooleanCandidateIndexLifecycleOutcome,
    pub(crate) possible_segment_pairs: usize,
    pub(crate) indexed_candidate_pairs: usize,
    pub(crate) culled_segment_pairs: usize,
    pub(crate) candidate_row_count: usize,
    pub(crate) closeout_candidate_manifest_rows: usize,
    pub(crate) candidate_rows_have_provenance: bool,
    pub(crate) event_bearing_pair_count: usize,
    pub(crate) point_event_count: usize,
    pub(crate) interval_event_count: usize,
    pub(crate) point_kind_counts: PointKindCounts,
    pub(crate) interval_kind_counts: IntervalKindCounts,
    pub(crate) source_edge_carrier_count: usize,
    pub(crate) distinct_source_edge_count: usize,
    pub(crate) point_split_candidate_count: usize,
    pub(crate) interval_split_candidate_count: usize,
    pub(crate) t_junction_boundary_decisions: usize,
    pub(crate) endpoint_noop_decisions: usize,
    pub(crate) closeout_endpoint_noop_decisions: usize,
    pub(crate) micro_interval_policy_required: usize,
    pub(crate) closeout_micro_interval_policy_required: usize,
    pub(crate) split_vertex_count: usize,
    pub(crate) coalesced_vertex_count: usize,
    pub(crate) split_fragment_count: usize,
    pub(crate) interval_attributed_fragment_count: usize,
    pub(crate) overlap_chain_count: usize,
    pub(crate) topology_product_count: usize,
    pub(crate) closeout_topology_product_count: usize,
    pub(crate) source_edge_fragment_lineage_rows: usize,
    pub(crate) closeout_source_edge_lineage_rows: usize,
    pub(crate) decision_phase_count: usize,
    pub(crate) decision_kind_count: usize,
    pub(crate) localized_decision_rows: usize,
    pub(crate) closeout_decision_localization_rows: usize,
    pub(crate) topology_closeout: TopologyCloseoutSummary,
    pub(crate) event_ledger_identity: String,
    pub(crate) split_request_identity: String,
    pub(crate) decision_log_receipt_identity: String,
    pub(crate) split_ledger_receipt_identity: String,
    pub(crate) split_ledger_downstream_identity: String,
    pub(crate) split_ledger_chains: usize,
    pub(crate) persistent_name_rows: usize,
    pub(crate) closeout_persistent_name_rows: usize,
    pub(crate) decision_rows: usize,
    pub(crate) replay_parity_receipt_identity: String,
    pub(crate) closeout_replay_parity_rows: usize,
    pub(crate) replay_candidate_index_reexecutions: usize,
    pub(crate) replay_event_extraction_reexecutions: usize,
    pub(crate) downstream_consumption_identity: String,
    pub(crate) loop_reconstruction_consumption_identity: String,
    pub(crate) loop_consumes_downstream_identity: bool,
}

pub(crate) fn emit_edge_split_metaboss_proof_bundle() -> EdgeSplitSummumBonumProofBundle {
    let subject =
        MetabossEventExtractionSubject::certify("phase7.3 summum bonum edge split closeout");
    assert_event_ledger_shape(&subject);
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    let downstream = completed_split_handoff
        .admit_batch_execution_cluster()
        .expect("summum bonum split ledger must admit batch execution cluster")
        .admit_downstream_split_consumption(
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
        )
        .expect("summum bonum split ledger must admit downstream consumption");
    let loop_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
            &downstream,
        ),
    )
    .expect("loop reconstruction must consume downstream split authority");
    let segment_pairs = &subject.inputs().pair_worklist;
    let candidate_product = segment_pairs.candidate_index_product();
    let candidate_counters = candidate_product.counters();
    let replay_receipt = replay_report.receipt();
    let ledger_receipt = replay_subject.original_ledger.receipt();
    let products = &replay_subject.original_products;
    let request_counters = products.request.counters();
    let endpoint_counters = products.endpoint_boundary.counters();
    let interval_counters = products.interval_subdivision.counters();
    let vertex_counters = products.vertices.counters();
    let fragment_counters = products.fragments.counters();
    let overlap_counters = products.chains.counters();
    let decision_rows = replay_subject
        .original_decision_log
        .receipt()
        .decision_rows();
    let closeout = PlanarBooleanEdgeSplitSummumBonumCloseout::certify(
        PlanarBooleanEdgeSplitSummumBonumCloseoutInput::new(
            candidate_product,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.chains,
            &products.naming,
            replay_subject.original_decision_log.receipt(),
            ledger_receipt,
            replay_receipt,
            &downstream,
            &loop_consumption,
        ),
    )
    .expect("summum bonum production closeout certificate must certify");
    let closeout_counters = closeout.counters();

    EdgeSplitSummumBonumProofBundle {
        catalog_recipe_query_key: subject.pair().recipe().query_key().to_string(),
        candidate_index_product_identity: candidate_product.product_identity().to_string(),
        candidate_index_plan_digest: candidate_product.plan_digest().to_string(),
        production_closeout_identity: closeout.closeout_identity().to_string(),
        production_closeout_certifies: closeout.certifies_milestone_7_3_summum_bonum_closeout(),
        candidate_strategy: candidate_product.strategy(),
        candidate_fallback_posture: candidate_product.fallback_posture(),
        candidate_lifecycle: candidate_product.lifecycle_outcome(),
        possible_segment_pairs: candidate_counters.expected_pair_breadth(),
        indexed_candidate_pairs: candidate_counters.query_index_candidate_count(),
        culled_segment_pairs: candidate_counters.query_index_culled_pair_count(),
        candidate_row_count: candidate_product.rows().len(),
        closeout_candidate_manifest_rows: closeout_counters.candidate_rows(),
        candidate_rows_have_provenance: candidate_rows_have_provenance(candidate_product.rows()),
        event_bearing_pair_count: event_bearing_pair_count(&subject),
        point_event_count: subject.ledger().point_events().len(),
        interval_event_count: subject.ledger().interval_events().len(),
        point_kind_counts: point_kind_counts(&subject),
        interval_kind_counts: interval_kind_counts(&subject),
        source_edge_carrier_count: request_counters.segment_carrier_count(),
        distinct_source_edge_count: fragment_counters.source_edges_covered(),
        point_split_candidate_count: request_counters.point_event_count().saturating_mul(2),
        interval_split_candidate_count: request_counters.interval_event_count().saturating_mul(2),
        t_junction_boundary_decisions: endpoint_counters.t_junction_boundary_decisions(),
        endpoint_noop_decisions: endpoint_counters.endpoint_noop_decisions(),
        closeout_endpoint_noop_decisions: closeout_counters.endpoint_noop_decisions(),
        micro_interval_policy_required: interval_counters.micro_intervals_policy_required(),
        closeout_micro_interval_policy_required: closeout_counters.micro_interval_policy_required(),
        split_vertex_count: vertex_counters.split_vertices_minted(),
        coalesced_vertex_count: vertex_counters.split_vertices_coalesced(),
        split_fragment_count: fragment_counters.fragments_emitted(),
        interval_attributed_fragment_count: fragment_counters.interval_attributed_fragments(),
        overlap_chain_count: overlap_counters.chains_emitted(),
        topology_product_count: overlap_counters.topology_products_emitted(),
        closeout_topology_product_count: closeout_counters.topology_products_emitted(),
        source_edge_fragment_lineage_rows: products.fragments.schedules().len(),
        closeout_source_edge_lineage_rows: closeout_counters.lineage_rows(),
        decision_phase_count: decision_phase_count(decision_rows),
        decision_kind_count: decision_kind_count(decision_rows),
        localized_decision_rows: localized_decision_rows(decision_rows),
        closeout_decision_localization_rows: closeout_counters.decision_rows(),
        topology_closeout: topology_closeout_summary(),
        event_ledger_identity: subject.ledger().event_ledger_identity().to_string(),
        split_request_identity: replay_subject
            .original_products
            .request
            .split_request_identity()
            .to_string(),
        decision_log_receipt_identity: replay_subject
            .original_decision_log
            .receipt()
            .receipt_identity()
            .to_string(),
        split_ledger_receipt_identity: ledger_receipt.receipt_identity().to_string(),
        split_ledger_downstream_identity: ledger_receipt
            .downstream_consumption_identity()
            .to_string(),
        split_ledger_chains: ledger_receipt.chain_identities().len(),
        persistent_name_rows: replay_subject
            .original_products
            .naming
            .persistent_name_rows()
            .len(),
        closeout_persistent_name_rows: closeout_counters.persistent_name_rows(),
        decision_rows: replay_subject
            .original_decision_log
            .receipt()
            .decision_rows()
            .len(),
        replay_parity_receipt_identity: replay_receipt.receipt_identity().to_string(),
        closeout_replay_parity_rows: closeout_counters.replay_parity_rows(),
        replay_candidate_index_reexecutions: replay_receipt
            .counters()
            .candidate_index_reexecutions(),
        replay_event_extraction_reexecutions: replay_receipt
            .counters()
            .event_extraction_reexecutions(),
        downstream_consumption_identity: downstream.consumption_identity().to_string(),
        loop_reconstruction_consumption_identity: loop_consumption
            .consumption_identity()
            .to_string(),
        loop_consumes_downstream_identity: loop_consumption.downstream_consumption_identity()
            == downstream.consumption_identity(),
    }
}
