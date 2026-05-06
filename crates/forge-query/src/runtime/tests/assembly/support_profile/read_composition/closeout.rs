use super::super::super::super::support::*;

const READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/forge-query/read-composition-phase1-closeout.md"
));

#[test]
fn runtime_public_read_composition_phase_one_closeout_answers_kernel_gate() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.read-composition-closeout")
        .expect("task runtime should open a named workspace");
    let support = workspace.public_read_composition_support_report();
    let closeout = workspace.public_read_composition_phase_one_closeout();

    assert_eq!(
        closeout.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(closeout.read_support_digest(), support.support_digest());
    assert_eq!(
        closeout.support_matrix_digest(),
        workspace.public_support_matrix().matrix_digest()
    );
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("compose_read, compose_read_with_invariant_pack")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("scope classes are kernel-owned")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("operator-owned graph lanes now cover direct_edge")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("query_runtime_current")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("snapshot_indexed_debt")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("descriptor-backed synthetic runtime relationship proof")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("domain_read_family_lowering")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("do not assume Phase 2 Worth topology migration is complete")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("do not assume the side quest is fully closed")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("LoopCycleNeighborhood")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("lowering, invariant-pack, decoder, and certification hook")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("prefer an operator-owned read lane")));
    assert!(closeout.required_verification_commands().iter().any(
        |line| line == "cargo test -p forge-query --test phase_boundaries_compile_fail --quiet"
    ));
    assert!(!closeout.closeout_digest().is_empty());
}

#[test]
fn runtime_public_read_composition_phase_one_closeout_doc_matches_certified_contract() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.read-composition-closeout-doc")
        .expect("task runtime should open a named workspace");
    let support = workspace.public_read_composition_support_report();
    let closeout = workspace.public_read_composition_phase_one_closeout();

    for line in closeout.safe_to_build_now() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.must_not_assume_yet() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.migration_guidance() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.required_verification_commands() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(line));
    }
    for item in support.entry_points() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.scope_classes() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.built_in_operators() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.execution_engines() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.fallback_classes() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.relationship_proof_postures() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.family_admission_modes() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.extension_hook_families() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.boundary_guards() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
    for item in support.denial_lanes() {
        assert!(READ_COMPOSITION_PHASE_ONE_CLOSEOUT_DOC.contains(item));
    }
}
