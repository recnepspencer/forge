use super::*;

#[test]
fn public_closeout_rejects_remaining_second_ontology_dependency() {
    let hostile_inventory = hostile_inventory_with_open_ordinary_dependency();
    let hostile_cutover = ordinary_consumer_cutover_from_inventory_for_tests(&hostile_inventory)
        .expect("hostile cutover should lower from the real inventory path");
    let components = current_public_closeout_components().expect("current closeout components");
    let error = publish_from_parts(
        components.input().expect("current closeout input"),
        &hostile_cutover,
        components.selected_route_packet(),
        components.admitted_public_proof_input(),
    )
    .expect_err("public closeout must reject an open ordinary-consumer dependency");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::OrdinaryConsumerDependencyStillOpen
    );
    assert!(error.detail().contains("second ontology"));
}

#[test]
fn touched_graph_closeout_rejects_foreign_replay_undo_proof_identities() {
    let components = current_public_closeout_components().expect("current closeout components");
    let inventory = current_conflict_batch_admission_inventory().expect("current inventory");
    let foreign_cutover =
        ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override(
            &inventory,
            "foreign-boundary-proof-digest",
            "foreign-transaction-packet-identity",
            "foreign-replay-scope-identity",
            "foreign-undo-scope-identity",
        )
        .expect("foreign cutover fixture should lower from the real inventory path");

    let error = publish_from_parts(
        components.input().expect("current closeout input"),
        &foreign_cutover,
        components.selected_route_packet(),
        components.admitted_public_proof_input(),
    )
    .expect_err("public closeout must reject foreign replay/undo proof joins");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
    assert!(error
        .detail()
        .contains("selected-route packet and cutover proof"));
}

#[test]
fn current_public_closeout_components_fail_when_kernel_consumer_matrix_is_incomplete() {
    let targets = current_coverage_targets().expect("current coverage targets");
    let retained_targets = targets
        .iter()
        .copied()
        .filter(|target| {
            !target
                .covered_reuse_surfaces()
                .contains(&crate::workload_composition::CompiledProductReuseSurfaceIdentity::CurrentEvidenceLookupPublicCloseout)
        })
        .collect::<Vec<_>>();
    let rows = retained_targets
        .iter()
        .map(|target| target.lower_row())
        .collect::<Result<Vec<_>, KernelCompiledProductConsumerDependencyError>>()
        .expect("rows should still lower");
    KernelCompiledProductConsumerDependencyMatrix::new(rows, &retained_targets).expect_err(
        "dropping one covered public-closeout surface must fail matrix coverage before closeout publishes",
    );

    let error = match current_public_closeout_components_with_matrix_targets_loader(|| {
        Ok(retained_targets)
    }) {
        Ok(_) => {
            panic!(
                "current public closeout components must fail when the kernel matrix is incomplete"
            )
        }
        Err(error) => error,
    };

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable
    );
    assert!(error
        .detail()
        .contains("phase 14 kernel consumer dependency matrix did not assemble"));
    assert!(error.detail().contains("missing covered reuse surface"));
}
