use super::edge_splitting_decision_log_support;
use super::edge_splitting_endpoint_boundary_support;
use super::edge_splitting_interval_subdivision_support;
use super::edge_splitting_normalized_schedule_support;
use super::edge_splitting_ordered_schedule_support;
use super::edge_splitting_persistent_naming_support;
use super::edge_splitting_public_contract_support;
use super::edge_splitting_raw_schedule_support;
use super::edge_splitting_replay_parity_support;
use super::edge_splitting_split_vertex_identity_support;
use super::metaboss_support;
use super::reduced_pair_support;

fn with_metaboss_subject(
    scope: &'static str,
    run: impl FnOnce(&metaboss_support::MetabossEventExtractionSubject) + Send + 'static,
) {
    reduced_pair_support::run_with_large_stack(move || {
        let subject = metaboss_support::MetabossEventExtractionSubject::certify(scope);
        run(&subject);
    });
}

#[test]
fn split_public_contract_requires_real_ledger_and_preserves_authority_boundaries() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_public_contract_support::
            assert_split_public_contract_requires_real_ledger_and_preserves_authority_boundaries();
    });
}

#[test]
fn edge_split_raw_schedule_matches_metaboss() {
    with_metaboss_subject("phase4.edge_split.raw_schedule", |subject| {
        edge_splitting_raw_schedule_support::assert_raw_edge_split_schedule_matches_metaboss(
            subject,
        );
    });
}

#[test]
fn edge_split_ordered_schedule_matches_metaboss() {
    with_metaboss_subject("phase4.edge_split.ordered_schedule", |subject| {
        edge_splitting_ordered_schedule_support::assert_ordered_edge_split_schedule_matches_metaboss(
            subject,
        );
    });
}

#[test]
fn edge_split_normalized_schedule_matches_metaboss() {
    with_metaboss_subject("phase4.edge_split.normalized_schedule", |subject| {
        edge_splitting_normalized_schedule_support::assert_normalized_edge_split_schedule_matches_metaboss(
            subject,
        );
    });
}

#[test]
fn edge_split_endpoint_boundary_normalization_matches_metaboss() {
    with_metaboss_subject("phase4.edge_split.endpoint_boundary", |subject| {
        edge_splitting_endpoint_boundary_support::assert_endpoint_boundary_normalization_matches_metaboss(
            subject,
        );
    });
}

#[test]
fn edge_split_interval_subdivision_normalization_matches_metaboss() {
    with_metaboss_subject("phase4.edge_split.interval_subdivision", |subject| {
        edge_splitting_interval_subdivision_support::assert_interval_subdivision_normalization_matches_metaboss(
            subject,
        );
    });
}

#[test]
fn edge_split_vertex_identities_match_metaboss() {
    with_metaboss_subject("phase4.edge_split.vertex_identities", |subject| {
        edge_splitting_split_vertex_identity_support::assert_split_vertex_identities_match_metaboss(
            subject,
        );
    });
}

#[test]
fn edge_split_persistent_naming_matches_metaboss() {
    with_metaboss_subject("phase4.edge_split.persistent_naming", |subject| {
        edge_splitting_persistent_naming_support::assert_split_persistent_naming_matches_metaboss(
            subject,
        );
    });
}

#[test]
fn edge_split_decision_log_matches_metaboss() {
    with_metaboss_subject("phase4.edge_split.decision_log", |subject| {
        edge_splitting_decision_log_support::assert_split_decision_log_matches_metaboss(subject);
    });
}

#[test]
fn edge_split_replay_parity_certifies_split_products() {
    with_metaboss_subject("phase4.edge_split.replay_parity", |subject| {
        let replay_subject =
            edge_splitting_replay_parity_support::build_edge_split_replay_parity_subject(subject);
        let report = edge_splitting_replay_parity_support::replay_parity_report(&replay_subject);
        edge_splitting_replay_parity_support::assert_replay_parity_certifies_split_products(
            &replay_subject,
            &report,
        );
        edge_splitting_replay_parity_support::assert_reversed_source_sense_is_covered(
            &replay_subject,
            &report,
        );
        edge_splitting_replay_parity_support::assert_checkpoint_parity_is_retained_replay_backed(
            &replay_subject,
            &report,
        );
    });
}

#[test]
fn edge_split_replay_parity_rejects_foreign_retained_replay_receipts() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            metaboss_support::MetabossEventExtractionSubject::certify("phase4.edge_split.foreign");
        let foreign_subject =
            metaboss_support::MetabossEventExtractionSubject::certify("phase4.edge_split.foreign.other");
        let replay_subject =
            edge_splitting_replay_parity_support::build_edge_split_replay_parity_subject(&subject);
        let foreign_replay_receipts = foreign_subject
            .pair()
            .left()
            .replay_receipts()
            .expect("foreign metaboss subject should expose retained replay receipts");
        edge_splitting_replay_parity_support::assert_foreign_retained_replay_receipt_is_rejected(
            &replay_subject,
            foreign_replay_receipts,
        );
    });
}

#[test]
fn split_public_contract_support_admits_spatial_touch_authority_from_completed_workload() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_public_contract_support::
            assert_split_handoff_admits_spatial_touch_authority_from_completed_workload();
    });
}

#[test]
fn split_public_contract_support_uses_spatial_facade_proof_product_for_downstream_migration() {
    reduced_pair_support::run_with_large_stack(|| {
        edge_splitting_public_contract_support::
            assert_split_downstream_migration_uses_spatial_facade_proof_product();
    });
}
