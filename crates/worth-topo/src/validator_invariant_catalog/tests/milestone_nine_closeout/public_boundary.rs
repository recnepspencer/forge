use crate::validator_invariant_catalog::worth_topology_legality_catalog_compile_fail_targets;

#[test]
fn compile_fail_targets_include_milestone_nine_boundary_proofs() {
    let target_paths = worth_topology_legality_catalog_compile_fail_targets()
        .iter()
        .map(|target| target.path())
        .collect::<Vec<_>>();
    assert!(target_paths.contains(
        &"tests/ui/validator_invariant_catalog/milestone_nine_closeout_struct_literal.rs"
    ));
    assert!(target_paths
        .contains(&"tests/ui/validator_invariant_catalog/milestone_ten_seed_struct_literal.rs"));
    assert!(target_paths.contains(
        &"tests/ui/validator_invariant_catalog/raw_deletion_row_cannot_mint_closeout.rs"
    ));
    assert!(target_paths
        .contains(&"tests/ui/validator_invariant_catalog/raw_residue_row_cannot_mint_closeout.rs"));
    assert!(target_paths.contains(
        &"tests/ui/validator_invariant_catalog/selected_obligation_digest_cannot_mint_milestone_ten_seed.rs"
    ));
}
