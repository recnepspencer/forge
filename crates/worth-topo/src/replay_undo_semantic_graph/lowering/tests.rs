use super::test_support::{
    invalidation_receipt_for, loop_cycles_invalidation_receipt, prepare_traversal_replay_request,
    reordered_traversal_touch_proof, selected_materialized_receipt, selected_traversal_receipt,
    selected_traversal_receipt_with_density, selected_traversal_touched_closure,
};
use super::{
    lower_topology_replay_scope_identity_from_admitted_input,
    lower_topology_replay_scope_identity_from_scope_product,
    lower_topology_replay_scope_identity_from_touched_closure,
    lower_topology_replay_scope_product_from_selected_plan,
    lower_topology_undo_scope_identity_from_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    loop_cycles_touched_closure, unrelated_geometry_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationDensityPolicy;
use crate::replay_family_catalog::TopologyReplayFamilyIdentityAuthority;
use crate::replay_undo_semantic_graph::{
    admit_prepared_topology_replay_semantic_graph_input,
    admit_topology_replay_semantic_graph_input, admit_topology_undo_semantic_graph_input,
    lower_topology_undo_scope_product_from_admitted_input, select_topology_replay_plan,
    TopologyReplaySemanticGraphAdmissionError, TopologyReplaySemanticGraphAdmissionRequest,
    TopologyReplaySemanticGraphPreparationRequest,
    TopologyReplaySemanticGraphStageReceiptAuthority, TopologyUndoSemanticGraphAdmissionRequest,
};
use crate::undo_family_catalog::TopologyUndoFamilyIdentityAuthority;

#[test]
fn topology_replay_scope_identity_is_stable_for_same_admitted_input() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);
    let traversal_receipt = selected_traversal_receipt("traversal-views-touch");

    let admitted_input =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &touched_closure,
            &receipt,
            &traversal_receipt,
            Some(&traversal_receipt),
        ))
        .expect("matching replay proof inputs should admit");

    let replay_plan =
        select_topology_replay_plan(&admitted_input).expect("admitted input should select");
    let first_scope_product = lower_topology_replay_scope_product_from_selected_plan(&replay_plan)
        .expect("selected replay plan should lower");
    let second_scope_product = lower_topology_replay_scope_product_from_selected_plan(&replay_plan)
        .expect("selected replay plan should lower");
    let first = lower_topology_replay_scope_identity_from_scope_product(&first_scope_product);
    let second = lower_topology_replay_scope_identity_from_scope_product(&second_scope_product);

    assert_eq!(first.digest(), second.digest());
}

#[test]
fn legacy_topology_replay_scope_identity_remains_stable_for_same_basis_and_receipt() {
    let touched_closure = loop_cycles_touched_closure("loop-touch");
    let receipt = loop_cycles_invalidation_receipt();

    let first =
        lower_topology_replay_scope_identity_from_touched_closure(&touched_closure, &receipt);
    let second =
        lower_topology_replay_scope_identity_from_touched_closure(&touched_closure, &receipt);

    assert_eq!(first.digest(), second.digest());
}

#[test]
fn topology_replay_admission_identity_is_stable_under_subject_ordering_noise() {
    let canonical_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&canonical_closure);
    let reordered_closure =
        crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure::from_declared_touch(
            &reordered_traversal_touch_proof(),
        );
    let traversal_receipt = selected_traversal_receipt("traversal-views-touch");
    let canonical =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &canonical_closure,
            &receipt,
            &traversal_receipt,
            Some(&traversal_receipt),
        ))
        .expect("canonical replay admission should succeed");
    let reordered =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &reordered_closure,
            &receipt,
            &traversal_receipt,
            Some(&traversal_receipt),
        ))
        .expect("reordered replay admission should succeed");

    assert_eq!(canonical.admission_digest(), reordered.admission_digest());
}

#[test]
fn topology_replay_selected_plan_and_scope_product_stay_stable_for_same_admitted_input() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);
    let traversal_receipt = selected_traversal_receipt("traversal-views-touch");
    let admitted_input =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &touched_closure,
            &receipt,
            &traversal_receipt,
            Some(&traversal_receipt),
        ))
        .expect("matching replay proof inputs should admit");

    let replay_plan =
        select_topology_replay_plan(&admitted_input).expect("admitted input should select");
    let first_scope_product = lower_topology_replay_scope_product_from_selected_plan(&replay_plan)
        .expect("selected replay plan should lower");
    let second_scope_product = lower_topology_replay_scope_product_from_selected_plan(&replay_plan)
        .expect("selected replay plan should lower");

    assert_eq!(
        first_scope_product.scope_identity().digest(),
        second_scope_product.scope_identity().digest()
    );
    assert_eq!(
        first_scope_product.equivalence_basis(),
        second_scope_product.equivalence_basis()
    );
    assert_eq!(
        replay_plan.selected_plan_identity().digest(),
        admitted_input.selected_plan_identity().digest()
    );
    assert_eq!(
        replay_plan.stage_identity().digest(),
        admitted_input.stage_identity().digest()
    );
    assert_eq!(
        first_scope_product.selected_plan_identity().digest(),
        admitted_input.selected_plan_identity().digest()
    );
    assert_eq!(
        first_scope_product.stage_identity().digest(),
        admitted_input.stage_identity().digest()
    );
}

#[test]
fn topology_replay_and_undo_scope_identities_are_distinct_for_same_basis() {
    let touched_closure = loop_cycles_touched_closure("loop-touch");
    let receipt = loop_cycles_invalidation_receipt();

    let replay =
        lower_topology_replay_scope_identity_from_touched_closure(&touched_closure, &receipt);
    let undo = lower_topology_undo_scope_identity_from_touched_closure(
        TopologyUndoFamilyIdentityAuthority::traversal_views().identity(),
        &touched_closure,
        &receipt,
    );

    assert_ne!(replay.digest(), undo.digest());
}

#[test]
fn topology_replay_admission_rejects_missing_required_stage_receipt_authority() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);

    let error = admit_topology_replay_semantic_graph_input(
        TopologyReplaySemanticGraphAdmissionRequest::new(
            TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
            &touched_closure,
            &receipt,
            None,
        ),
    )
    .expect_err("missing stage receipt authority should deny replay admission");

    assert_eq!(
        error,
        TopologyReplaySemanticGraphAdmissionError::MissingRequiredStageReceiptAuthority {
            family_identity: TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
        }
    );
}

#[test]
fn topology_replay_admission_rejects_wrong_stage_receipt_family() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);
    let materialized_receipt = selected_materialized_receipt("traversal-views-touch");

    let error = admit_topology_replay_semantic_graph_input(
        TopologyReplaySemanticGraphAdmissionRequest::new(
            TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
            &touched_closure,
            &receipt,
            Some(
                TopologyReplaySemanticGraphStageReceiptAuthority::MaterializedGraph(
                    &materialized_receipt,
                ),
            ),
        ),
    )
    .expect_err("wrong stage receipt family should deny replay admission");

    assert_eq!(
        error,
        TopologyReplaySemanticGraphAdmissionError::StageReceiptFamilyMismatch {
            family_identity: TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
            stage_receipt_family_identity:
                TopologyReplayFamilyIdentityAuthority::materialized_graph().identity(),
        }
    );
}

#[test]
fn topology_replay_admission_rejects_mismatched_touched_closure_and_receipt() {
    let foreign_touched_closure = unrelated_geometry_touched_closure();
    let canonical_touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&canonical_touched_closure);
    let traversal_receipt = selected_traversal_receipt("traversal-views-touch");

    let error = admit_topology_replay_semantic_graph_input(
        TopologyReplaySemanticGraphAdmissionRequest::new(
            TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
            &foreign_touched_closure,
            &receipt,
            Some(
                TopologyReplaySemanticGraphStageReceiptAuthority::TraversalViews(
                    &traversal_receipt,
                ),
            ),
        ),
    )
    .expect_err("foreign touched closure should deny replay admission");

    assert_eq!(
        error,
        TopologyReplaySemanticGraphAdmissionError::InvalidationReceiptTouchedClosureMismatch {
            touched_closure_digest: foreign_touched_closure.closure_digest().to_string(),
            receipt_touched_closure_digest: receipt.touched_closure_digest().to_string(),
        }
    );
}

#[test]
fn topology_replay_admission_rejects_wrong_stage_receipt_selected_plan() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);
    let dense_traversal_receipt = selected_traversal_receipt_with_density(
        "traversal-views-touch",
        DerivedInvalidationDensityPolicy::Dense,
    );

    let error =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &touched_closure,
            &receipt,
            &dense_traversal_receipt,
            Some(&dense_traversal_receipt),
        ))
        .expect_err("wrong stage receipt selected plan should deny replay admission");

    assert!(matches!(
        error,
        TopologyReplaySemanticGraphAdmissionError::StageReceiptSelectedPlanMismatch { .. }
    ));
}

#[test]
fn topology_replay_scope_product_localizes_denial_to_exact_mismatched_topology_proof_input() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let sparse_receipt = invalidation_receipt_for(&touched_closure);
    let traversal_receipt = selected_traversal_receipt("traversal-views-touch");
    let dense_receipt = super::test_support::invalidation_receipt_for_density(
        &touched_closure,
        DerivedInvalidationDensityPolicy::Dense,
    );

    let error =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &touched_closure,
            &dense_receipt,
            &traversal_receipt,
            Some(&traversal_receipt),
        ))
        .expect_err("mismatched invalidation receipt should deny before scope lowering");

    assert!(matches!(
        error,
        TopologyReplaySemanticGraphAdmissionError::StageReceiptSelectedPlanMismatch { .. }
    ));

    let admitted_input =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &touched_closure,
            &sparse_receipt,
            &traversal_receipt,
            Some(&traversal_receipt),
        ))
        .expect("matching replay proof inputs should admit");
    let replay_plan =
        select_topology_replay_plan(&admitted_input).expect("admitted input should select");
    let scope_product = lower_topology_replay_scope_product_from_selected_plan(&replay_plan)
        .expect("selected replay plan should lower");

    assert_eq!(
        scope_product.scope_identity().digest(),
        lower_topology_replay_scope_identity_from_admitted_input(&admitted_input).digest()
    );
}

#[test]
fn topology_replay_admission_rejects_wrong_stage_identity() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);
    let sparse_traversal_receipt = selected_traversal_receipt("traversal-views-touch");
    let foreign_materialized_receipt = selected_materialized_receipt("loop-touch");

    let error =
        admit_prepared_topology_replay_semantic_graph_input(prepare_traversal_replay_request(
            &touched_closure,
            &receipt,
            &sparse_traversal_receipt,
            None,
        ))
        .expect_err("missing declared stage identity should deny replay admission");

    assert_eq!(
        error,
        TopologyReplaySemanticGraphAdmissionError::MissingRequiredStageIdentity {
            family_identity: TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
        }
    );

    let error = admit_prepared_topology_replay_semantic_graph_input(
        crate::replay_undo_semantic_graph::prepare_topology_replay_semantic_graph_request(
            TopologyReplaySemanticGraphPreparationRequest::new(
                TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
                &touched_closure,
                &receipt,
                Some(TopologyReplaySemanticGraphStageReceiptAuthority::TraversalViews(
                    &sparse_traversal_receipt,
                )),
                Some(
                    crate::replay_undo_semantic_graph::prepare_topology_replay_semantic_graph_stage_identity(
                        TopologyReplaySemanticGraphStageReceiptAuthority::MaterializedGraph(
                            &foreign_materialized_receipt,
                        ),
                    ),
                ),
            ),
        ),
    )
    .expect_err("foreign real stage identity should deny replay admission");

    assert!(matches!(
        error,
        TopologyReplaySemanticGraphAdmissionError::StageIdentityMismatch { .. }
    ));
}

#[test]
fn topology_undo_scope_product_preserves_stage_and_prior_proof_identity() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);
    let admitted =
        admit_topology_undo_semantic_graph_input(TopologyUndoSemanticGraphAdmissionRequest::new(
            TopologyUndoFamilyIdentityAuthority::traversal_views().identity(),
            &touched_closure,
            &receipt,
        ))
        .expect("undo input should admit");
    let scope_product =
        lower_topology_undo_scope_product_from_admitted_input(&admitted).expect("scope product");

    assert_eq!(scope_product.family_identity(), admitted.family_identity());
    assert_eq!(
        scope_product.stage_index_identity().digest(),
        admitted.stage_index_identity().digest()
    );
    assert_eq!(
        scope_product.prior_proof_identity().digest(),
        admitted.prior_proof_identity().digest()
    );
}

#[test]
fn topology_undo_scope_identity_changes_with_undo_family() {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let receipt = invalidation_receipt_for(&touched_closure);

    let traversal_scope = lower_topology_undo_scope_identity_from_touched_closure(
        TopologyUndoFamilyIdentityAuthority::traversal_views().identity(),
        &touched_closure,
        &receipt,
    );
    let materialized_scope = lower_topology_undo_scope_identity_from_touched_closure(
        TopologyUndoFamilyIdentityAuthority::materialized_graph().identity(),
        &touched_closure,
        &receipt,
    );

    assert_ne!(traversal_scope.digest(), materialized_scope.digest());
}
