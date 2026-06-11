use hadwiger_research::facade::*;

use super::fixtures::{
    conflict_graph, conflict_graph_with_required_color_count, contact_witness, handle,
    terminal_relation,
};

#[test]
fn tiling_equivalence_declarations_have_query_readiness() {
    let handle = handle();

    assert!(
        !research_declaration_entry_readiness::<TilingEquivalenceClassificationDeclaration>(
            &handle
        )
        .rows()
        .is_empty()
    );
    assert!(
        !research_declaration_entry_readiness::<TilingSuppressionDeclaration>(&handle)
            .rows()
            .is_empty()
    );
    assert!(
        !research_declaration_entry_readiness::<TilingReactivationDeclaration>(&handle)
            .rows()
            .is_empty()
    );
}

#[test]
fn tile_contact_equivalence_blocks_duplicate_checker_work_without_authority() {
    let handle = handle();
    let proof = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_tile_equivalence_witness(
            "contact-equivalence",
            contact_witness("same-contact-class"),
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        proof.equivalence_scope(),
        TilingEquivalenceScope::TileContactGraph
    );
    assert!(proof.blocks_duplicate_checker_work());
    assert!(!proof.blocks_duplicate_proof_admission());
    assert!(!proof.admits_theorem_authority());
    assert!(!proof.registers_query_invariant_authority());
    assert_eq!(
        proof.query_declaration_reference().declaration_family_key(),
        "hadwiger.tiling.equivalence_classification"
    );
    assert_eq!(proof.counters().candidate_breadth_inspected(), 2);
    assert_eq!(proof.counters().equivalence_scopes_evaluated(), 1);
    assert_eq!(proof.counters().exact_equality_hits(), 1);
    assert_eq!(proof.counters().tile_equivalence_hits(), 1);
    assert_eq!(proof.counters().novelty_fingerprint_hits(), 0);
    assert_eq!(proof.counters().query_declarations_performed(), 1);
    assert_eq!(proof.counters().hidden_broad_scan_refusals(), 0);
}

#[test]
fn changed_tile_contact_signature_changes_equivalence_digest() {
    let handle = handle();
    let left = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_tile_equivalence_witness(
            "contact-equivalence-digest",
            contact_witness("same-contact-class"),
        )
        .unwrap(),
    )
    .unwrap();
    let changed = TileEquivalenceWitness::builder(
        "different-contact-class",
        TileEquivalenceScope::ContactConstraint,
    )
    .with_left_contact_signature(
        TileContactGraphSignature::from_edges("tile-a", [("center", "north")]).unwrap(),
    )
    .with_right_contact_signature(
        TileContactGraphSignature::from_edges("tile-b", [("center", "south")]).unwrap(),
    )
    .finish()
    .unwrap();
    let right = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_tile_equivalence_witness(
            "contact-equivalence-digest",
            changed,
        )
        .unwrap(),
    )
    .unwrap();

    assert_ne!(left.artifact_digest(), right.artifact_digest());
    assert_eq!(
        right.posture(),
        TilingCandidateEquivalencePosture::Unsupported
    );
}

#[test]
fn exact_conflict_graph_equivalence_converges_across_query_declarations() {
    let handle = handle();
    let left = conflict_graph(&handle, "equiv-conflict-left");
    let right = conflict_graph(&handle, "equiv-conflict-right");
    assert_ne!(
        left.query_declaration_digest(),
        right.query_declaration_digest()
    );

    let proof = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_conflict_graphs(
            "exact-conflict-graph-equivalence",
            &left,
            &right,
            TilingEquivalenceScope::ExactConflictGraph,
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        proof.equivalence_scope(),
        TilingEquivalenceScope::ExactConflictGraph
    );
    assert_eq!(
        proof.posture(),
        TilingCandidateEquivalencePosture::BlocksDuplicateCheckerWork
    );
    assert!(proof.blocks_duplicate_checker_work());
    assert!(!proof.admits_theorem_authority());
}

#[test]
fn checker_input_reuse_keeps_color_target_in_equivalence_basis() {
    let handle = handle();
    let left = conflict_graph_with_required_color_count(&handle, "checker-reuse-left", Some(5));
    let right = conflict_graph_with_required_color_count(&handle, "checker-reuse-right", Some(6));

    let proof = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_conflict_graphs(
            "checker-reuse-color-target",
            &left,
            &right,
            TilingEquivalenceScope::CheckerInputReuse,
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        proof.equivalence_scope(),
        TilingEquivalenceScope::CheckerInputReuse
    );
    assert_eq!(
        proof.posture(),
        TilingCandidateEquivalencePosture::Unsupported
    );
    assert!(!proof.blocks_duplicate_checker_work());
}

#[test]
fn conflict_graph_equivalence_rejects_wrong_scope_inputs() {
    let handle = handle();
    let left = conflict_graph(&handle, "wrong-scope-left");
    let right = conflict_graph(&handle, "wrong-scope-right");

    assert!(matches!(
        TilingCandidateEquivalenceRequest::from_conflict_graphs(
            "wrong-scope-conflict-graph",
            &left,
            &right,
            TilingEquivalenceScope::ConflictCore,
        ),
        Err(TilingEquivalenceError::ScopeInputMismatch {
            scope: "conflict_core"
        })
    ));
}

#[test]
fn terminal_relation_equivalence_is_pairwise_and_blocks_proof_admission_only() {
    let handle = handle();
    let (_motif, left) = terminal_relation(&handle, "terminal-equivalence", "left-relation");
    let (_same_motif, right) = terminal_relation(&handle, "terminal-equivalence", "right-relation");

    let proof = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_terminal_forcing_relations(
            "terminal-relation-equivalence",
            &left,
            &right,
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        proof.equivalence_scope(),
        TilingEquivalenceScope::MotifTerminalBehavior
    );
    assert_eq!(
        proof.posture(),
        TilingCandidateEquivalencePosture::BlocksDuplicateProofAdmission
    );
    assert!(!proof.blocks_duplicate_checker_work());
    assert!(proof.blocks_duplicate_proof_admission());
}

#[test]
fn terminal_relation_equivalence_keeps_motif_owner_in_basis() {
    let handle = handle();
    let (_left_motif, left) = terminal_relation(&handle, "terminal-owner-left", "owner-left");
    let (_right_motif, right) = terminal_relation(&handle, "terminal-owner-right", "owner-right");

    let proof = classify_tiling_candidate_equivalence_checked(
        &handle,
        TilingCandidateEquivalenceRequest::from_terminal_forcing_relations(
            "terminal-owner-separation",
            &left,
            &right,
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        proof.equivalence_scope(),
        TilingEquivalenceScope::MotifTerminalBehavior
    );
    assert_eq!(
        proof.posture(),
        TilingCandidateEquivalencePosture::Unsupported
    );
    assert!(!proof.blocks_duplicate_proof_admission());
}
