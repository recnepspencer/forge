fn compile_fail(glob: &str) {
    let tests = trybuild::TestCases::new();
    tests.compile_fail(glob);
}

#[test]
fn facade_internal_homes_are_private() {
    compile_fail("tests/ui/facade_boundary/*.rs");
}

#[test]
fn diagnostic_primitive_and_category_law_preserves_non_substitution() {
    compile_fail("tests/ui/diagnostics/primitives/*.rs");
}

#[test]
fn diagnostic_row_topology_preserves_family_and_locator_law() {
    compile_fail("tests/ui/diagnostics/rows/*.rs");
}

#[test]
fn diagnostic_materialization_and_support_boundaries_preserve_explicit_seams() {
    compile_fail("tests/ui/diagnostics/materialization/*.rs");
}

#[test]
fn diagnostic_basis_and_comparison_preserve_blind_consumer_canonical_law() {
    compile_fail("tests/ui/diagnostics/basis/*.rs");
}

#[test]
fn diagnostic_certified_bundle_and_attachment_boundaries_reuse_proof_lane() {
    compile_fail("tests/ui/diagnostics/certified/*.rs");
}

#[test]
fn diagnostic_production_readiness_requires_certified_artifact() {
    compile_fail("tests/ui/diagnostics/readiness_boundaries/*.rs");
}

#[test]
fn boundary_artifact_categories_preserve_non_substitution_law() {
    compile_fail("tests/ui/boundary_artifacts/categories/*.rs");
}

#[test]
fn boundary_artifact_roles_and_authority_preserve_claim_legality() {
    compile_fail("tests/ui/boundary_artifacts/authority_admission/*.rs");
    compile_fail("tests/ui/boundary_artifacts/role_legality/*.rs");
}

#[test]
fn boundary_artifact_materialization_contracts_preserve_explicit_seams() {
    compile_fail("tests/ui/boundary_artifacts/materialization_contracts/*.rs");
}

#[test]
fn boundary_artifact_bundle_contracts_preserve_category_honesty() {
    compile_fail("tests/ui/boundary_artifacts/bundle_contracts/*.rs");
}

#[test]
fn boundary_artifact_basis_participation_requires_canonical_readiness() {
    compile_fail("tests/ui/boundary_artifacts/basis_boundaries/*.rs");
}

#[test]
fn boundary_artifact_current_basis_lane_requires_stronger_proof_and_authority() {
    compile_fail("tests/ui/boundary_artifacts/current_basis_boundaries/*.rs");
}

#[test]
fn boundary_artifact_descriptive_extensions_remain_descriptive_and_fail_closed() {
    compile_fail("tests/ui/boundary_artifacts/descriptive_extensions/*.rs");
}

#[test]
fn boundary_artifact_readiness_requires_certified_artifact() {
    compile_fail("tests/ui/boundary_artifacts/readiness_boundaries/*.rs");
}

#[test]
fn branch_local_transition_surfaces_remain_non_authoritative() {
    compile_fail("tests/ui/transitions/branch_local/*.rs");
}

#[test]
fn merge_transition_admission_surfaces_preserve_non_authoritative_boundaries() {
    compile_fail("tests/ui/transitions/merge_admission/*.rs");
}

#[test]
fn committed_authority_transition_lane_requires_proof_bearing_admission() {
    compile_fail("tests/ui/transitions/committed_authority/*.rs");
}

#[test]
fn transition_receipt_and_closeout_boundaries_preserve_authority_separation() {
    compile_fail("tests/ui/transitions/receipt_boundaries/*.rs");
}

#[test]
fn transition_phase5_basis_and_current_basis_boundaries_remain_proof_bearing() {
    compile_fail("tests/ui/transitions/phase5_boundaries/*.rs");
}

#[test]
fn transition_production_readiness_requires_certified_artifact() {
    compile_fail("tests/ui/transitions/readiness_boundaries/*.rs");
}

#[test]
fn value_vocabulary_rejects_generic_document_authority() {
    compile_fail("tests/ui/value_vocabulary/*.rs");
}

#[test]
fn contract_validation_requires_proof_bearing_outputs() {
    compile_fail("tests/ui/contract_validation/*.rs");
}

#[test]
fn aspect_evolution_requires_proof_bearing_classification() {
    compile_fail("tests/ui/aspect_evolution/*.rs");
}

#[test]
fn authoritative_state_requires_validated_admission() {
    compile_fail("tests/ui/authoritative_state/*.rs");
}

#[test]
fn authoritative_patches_do_not_satisfy_state_apis() {
    compile_fail("tests/ui/authoritative_patches/*.rs");
}

#[test]
fn mask_admissibility_preserves_mode_types() {
    compile_fail("tests/ui/mask_admissibility/*.rs");
}

#[test]
fn struct_aspect_values_keep_fields_sealed() {
    compile_fail("tests/ui/struct_aspect_values/*.rs");
}

#[test]
fn identity_categories_are_not_interchangeable() {
    compile_fail("tests/ui/identity_categories/*.rs");
}

#[test]
fn locator_mask_modes_are_not_interchangeable() {
    compile_fail("tests/ui/locators/*.rs");
}

#[test]
fn digest_preparation_requires_readiness_proof() {
    compile_fail("tests/ui/digest_preparation/*.rs");
}

#[test]
fn milestone1_production_readiness_requires_certified_artifact() {
    compile_fail("tests/ui/milestone1_readiness/*.rs");
}

#[test]
fn canonical_basis_requires_readiness_proof() {
    compile_fail("tests/ui/canonicalization/basis/*/*.rs");
}

#[test]
fn canonical_comparison_requires_comparison_readiness_proof() {
    compile_fail("tests/ui/canonicalization/equivalence/*.rs");
}

#[test]
fn canonical_export_requires_readmission_after_trust_boundary() {
    compile_fail("tests/ui/canonicalization/export/*.rs");
}

#[test]
fn canonical_digest_derivation_requires_admitted_input_shape() {
    compile_fail("tests/ui/canonicalization/digest_slots/*.rs");
}

#[test]
fn canonical_production_readiness_requires_certified_artifact() {
    compile_fail("tests/ui/canonicalization/production_readiness/*.rs");
}

#[test]
fn profile_family_boundaries_reject_stringly_or_adjacent_category_substitution() {
    compile_fail("tests/ui/profiles/family_boundaries/*.rs");
}

#[test]
fn profile_set_construction_rejects_partial_duplicate_or_default_smuggling() {
    compile_fail("tests/ui/profiles/set_construction/*.rs");
}

#[test]
fn profile_attachment_boundaries_preserve_target_and_payload_category_law() {
    compile_fail("tests/ui/profiles/attachment_boundaries/*.rs");
}

#[test]
fn profile_identity_boundaries_reject_raw_digest_or_field_forgery() {
    compile_fail("tests/ui/profiles/identity_boundaries/*.rs");
}

#[test]
fn profile_materialization_boundaries_reject_ad_hoc_surface_or_inventory_construction() {
    compile_fail("tests/ui/profiles/materialization_boundaries/*.rs");
}

#[test]
fn profile_certification_and_readiness_boundaries_require_stronger_artifacts() {
    compile_fail("tests/ui/profiles/certification_boundaries/*.rs");
    compile_fail("tests/ui/profiles/readiness_boundaries/*.rs");
}
