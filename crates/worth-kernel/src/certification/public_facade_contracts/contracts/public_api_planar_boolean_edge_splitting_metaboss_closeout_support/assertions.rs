use super::proof_bundle::emit_edge_split_metaboss_proof_bundle;
use super::scene_manifest::EdgeSplitSummumBonumSceneManifest as Manifest;

pub(crate) fn assert_edge_split_summum_bonum_closeout_certifies_real_production_chain() {
    let proof = emit_edge_split_metaboss_proof_bundle();

    assert_eq!(
        proof.catalog_recipe_query_key,
        Manifest::CATALOG_RECIPE_QUERY_KEY
    );
    assert_candidate_index_closeout(&proof);
    assert_scene_relation_manifest(&proof);
    assert_split_stage_closeout(&proof);
    assert_topology_closeout(&proof);
    assert_split_ledger_replay_and_downstream_closeout(&proof);
}

pub(crate) fn assert_edge_split_summum_bonum_candidate_index_closeout_holds() {
    let proof = emit_edge_split_metaboss_proof_bundle();

    assert_candidate_index_closeout(&proof);
}

pub(crate) fn assert_edge_split_summum_bonum_replay_closeout_holds() {
    let proof = emit_edge_split_metaboss_proof_bundle();

    assert_split_ledger_replay_and_downstream_closeout(&proof);
}

pub(crate) fn assert_edge_split_summum_bonum_rejects_cross_product_discovery() {
    let proof = emit_edge_split_metaboss_proof_bundle();

    assert_candidate_index_closeout(&proof);
    assert_eq!(
        proof.indexed_candidate_pairs + proof.culled_segment_pairs,
        proof.possible_segment_pairs
    );
    assert!(
        proof.culled_segment_pairs > proof.indexed_candidate_pairs,
        "closeout scene must prove meaningful spatial culling, not a cosmetic index wrapper"
    );
}

pub(crate) fn assert_edge_split_summum_bonum_decision_localization_holds() {
    let proof = emit_edge_split_metaboss_proof_bundle();

    assert_split_stage_closeout(&proof);
    assert_eq!(proof.localized_decision_rows, proof.decision_rows);
}

pub(crate) fn assert_edge_split_summum_bonum_public_contract_fences_hold() {
    super::super::edge_splitting_public_contract_support::
        assert_split_public_contract_requires_real_ledger_and_rejects_manual_evidence();
}

fn assert_candidate_index_closeout(proof: &super::proof_bundle::EdgeSplitSummumBonumProofBundle) {
    assert!(!proof.candidate_index_product_identity.is_empty());
    assert!(!proof.candidate_index_plan_digest.is_empty());
    assert!(!proof.production_closeout_identity.is_empty());
    assert!(proof.production_closeout_certifies);
    assert_eq!(proof.candidate_strategy, worth_spatial::facade::planar_boolean_events::PlanarBooleanCandidateIndexStrategy::AabbSweep);
    assert_eq!(proof.candidate_fallback_posture, worth_spatial::facade::planar_boolean_events::PlanarBooleanCandidateIndexFallbackPosture::NotUsed);
    assert_eq!(proof.candidate_lifecycle, worth_spatial::facade::planar_boolean_events::PlanarBooleanCandidateIndexLifecycleOutcome::Bound);
    assert_eq!(
        proof.possible_segment_pairs,
        Manifest::POSSIBLE_SEGMENT_PAIRS
    );
    assert_eq!(
        proof.indexed_candidate_pairs,
        Manifest::INDEXED_CANDIDATE_PAIRS
    );
    assert_eq!(proof.culled_segment_pairs, Manifest::CULLED_SEGMENT_PAIRS);
    assert_eq!(proof.candidate_row_count, proof.indexed_candidate_pairs);
    assert_eq!(
        proof.closeout_candidate_manifest_rows,
        proof.indexed_candidate_pairs
    );
    assert!(proof.candidate_rows_have_provenance);
    assert_eq!(
        proof.event_bearing_pair_count,
        Manifest::EVENT_BEARING_CANDIDATE_PAIRS
    );
    assert!(proof.event_bearing_pair_count <= proof.indexed_candidate_pairs);
    assert_ne!(
        proof.indexed_candidate_pairs, proof.possible_segment_pairs,
        "closeout must prove Query indexing rather than full cross-product execution"
    );
}

fn assert_split_stage_closeout(proof: &super::proof_bundle::EdgeSplitSummumBonumProofBundle) {
    assert_eq!(proof.source_edge_carrier_count, 48);
    assert_eq!(proof.distinct_source_edge_count, 46);
    assert!(proof.source_edge_carrier_count >= proof.distinct_source_edge_count);
    assert_eq!(
        proof.point_split_candidate_count,
        proof.point_event_count.saturating_mul(2)
    );
    assert_eq!(
        proof.interval_split_candidate_count,
        proof.interval_event_count.saturating_mul(2)
    );
    assert!(proof.t_junction_boundary_decisions > 0);
    assert_eq!(proof.endpoint_noop_decisions, 0);
    assert_eq!(
        proof.closeout_endpoint_noop_decisions,
        proof.endpoint_noop_decisions
    );
    assert_eq!(proof.micro_interval_policy_required, 0);
    assert_eq!(
        proof.closeout_micro_interval_policy_required,
        proof.micro_interval_policy_required
    );
    assert!(proof.split_vertex_count > 0);
    assert!(proof.coalesced_vertex_count > 0);
    assert!(proof.split_fragment_count >= proof.distinct_source_edge_count);
    assert!(proof.interval_attributed_fragment_count > 0);
    assert!(proof.overlap_chain_count > 0);
    assert_eq!(proof.topology_product_count, 0);
    assert_eq!(
        proof.closeout_topology_product_count,
        proof.topology_product_count
    );
    assert_eq!(
        proof.source_edge_fragment_lineage_rows,
        proof.distinct_source_edge_count
    );
    assert_eq!(
        proof.closeout_source_edge_lineage_rows,
        proof.distinct_source_edge_count
    );
    assert!(proof.decision_phase_count >= 7);
    assert!(proof.decision_kind_count >= 6);
    assert_eq!(proof.localized_decision_rows, proof.decision_rows);
    assert_eq!(
        proof.closeout_decision_localization_rows,
        proof.decision_rows
    );
}

fn assert_topology_closeout(proof: &super::proof_bundle::EdgeSplitSummumBonumProofBundle) {
    assert_eq!(proof.topology_closeout.required_operator_rows, 46);
    assert_eq!(proof.topology_closeout.required_validator_rows, 29);
    assert_eq!(proof.topology_closeout.phase_27_query_graph_rows, 3);
    assert_eq!(proof.topology_closeout.phase_27_query_invariant_rows, 2);
    assert_eq!(proof.topology_closeout.phase_27_runtime_validators, 5);
}

fn assert_scene_relation_manifest(proof: &super::proof_bundle::EdgeSplitSummumBonumProofBundle) {
    assert_eq!(proof.point_event_count, Manifest::POINT_EVENTS);
    assert_eq!(proof.interval_event_count, Manifest::INTERVAL_EVENTS);
    assert_eq!(
        proof.point_kind_counts.proper_crossings,
        Manifest::PROPER_CROSSINGS
    );
    assert_eq!(
        proof.point_kind_counts.a_endpoint_on_b_interior,
        Manifest::A_ENDPOINT_ON_B_INTERIOR
    );
    assert_eq!(
        proof.point_kind_counts.b_endpoint_on_a_interior,
        Manifest::B_ENDPOINT_ON_A_INTERIOR
    );
    assert_eq!(
        proof.point_kind_counts.shared_endpoints,
        Manifest::SHARED_ENDPOINTS
    );
    assert_eq!(
        proof.interval_kind_counts.partial_overlaps,
        Manifest::PARTIAL_OVERLAPS
    );
    assert_eq!(
        proof.interval_kind_counts.containment_overlaps,
        Manifest::CONTAINMENT_OVERLAPS
    );
    assert_eq!(
        proof.interval_kind_counts.identical_same_direction,
        Manifest::IDENTICAL_SAME_DIRECTION
    );
    assert_eq!(
        proof.interval_kind_counts.identical_anti_parallel,
        Manifest::IDENTICAL_ANTI_PARALLEL
    );
}

fn assert_split_ledger_replay_and_downstream_closeout(
    proof: &super::proof_bundle::EdgeSplitSummumBonumProofBundle,
) {
    assert!(!proof.event_ledger_identity.is_empty());
    assert!(!proof.split_request_identity.is_empty());
    assert!(!proof.decision_log_receipt_identity.is_empty());
    assert!(!proof.split_ledger_receipt_identity.is_empty());
    assert!(!proof.split_ledger_downstream_identity.is_empty());
    assert!(proof.split_ledger_chains > 0);
    assert!(proof.persistent_name_rows > 0);
    assert_eq!(
        proof.closeout_persistent_name_rows,
        proof.persistent_name_rows
    );
    assert!(proof.decision_rows > 0);
    assert!(!proof.replay_parity_receipt_identity.is_empty());
    assert!(proof.closeout_replay_parity_rows > 0);
    assert_eq!(proof.replay_candidate_index_reexecutions, 0);
    assert_eq!(proof.replay_event_extraction_reexecutions, 0);
    assert!(!proof.downstream_consumption_identity.is_empty());
    assert!(!proof.loop_reconstruction_consumption_identity.is_empty());
    assert!(proof.loop_consumes_downstream_identity);
}
