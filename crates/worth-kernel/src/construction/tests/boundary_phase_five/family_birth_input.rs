use super::patterns::*;
use super::sources::*;

#[test]
fn phase_five_admitted_scaffold_birth_input_lane_no_longer_passes_parallel_family_parameter() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES,
        &FORBIDDEN_ADMITTED_SCAFFOLD_FAMILY_PARAMETER_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold birth-input helpers still pass a parallel family parameter instead of deriving family from the geometry-specific builder role itself: {violations:?}"
    );
}

#[test]
fn phase_five_shared_birth_scaffold_seam_no_longer_owns_family_parameter_admission() {
    let violations = collect_violations(
        &AUDITED_SHARED_BIRTH_SCAFFOLD_FILE,
        &FORBIDDEN_PLACEMENT_ADMISSION_FAMILY_PARAMETER_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "shared birth-scaffold seam still owns family parameter admission instead of receiving already-admitted family-local geometry and placement truth: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_family_helpers_no_longer_own_request_level_placement_admission() {
    let violations = collect_violations(
        &AUDITED_FAMILY_REALIZATION_FILES,
        &FORBIDDEN_DUPLICATED_PLACEMENT_ADMISSION_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold family helpers still own request-level placement admission instead of receiving one shared admitted placement from the dispatcher: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_sibling_helpers_no_longer_duplicate_realized_birth_bridge() {
    let violations = collect_violations(
        &AUDITED_FAMILY_REALIZATION_FILES,
        &FORBIDDEN_DUPLICATED_REALIZED_BIRTH_BRIDGE_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold sibling helpers still duplicate the shared realized-birth bridge instead of lowering through one explicit module: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_subtree_no_longer_teaches_dual_bridge_entry_protocol() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES,
        &FORBIDDEN_OLD_BIRTH_SCAFFOLD_BRIDGE_ENTRY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input subtree still teaches the deleted dual bridge entry protocol instead of one explicit family birth-scaffold plan lane: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_family_helpers_no_longer_own_embedding_choreography() {
    let violations = collect_violations(
        &AUDITED_FAMILY_REALIZATION_FILES,
        &FORBIDDEN_FAMILY_HELPER_EMBEDDING_CHOREOGRAPHY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold family helpers still own embedding choreography instead of lowering through one shared embedding seam: {violations:?}"
    );
}

#[test]
fn phase_five_birth_input_dispatcher_no_longer_owns_local_request_admission_helpers() {
    let violations = collect_violations(
        &AUDITED_BIRTH_INPUT_FILE,
        &FORBIDDEN_BIRTH_INPUT_LOCAL_REQUEST_ADMISSION_HELPER_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "birth-input dispatcher still defines request-admission helpers locally instead of lowering through one explicit request-admission seam: {violations:?}"
    );
}

#[test]
fn phase_five_birth_input_dispatcher_no_longer_owns_family_strategy_match() {
    let violations = collect_violations(
        &AUDITED_BIRTH_INPUT_FILE,
        &FORBIDDEN_BIRTH_INPUT_FAMILY_STRATEGY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "birth-input dispatcher still owns the raw geometry family strategy table instead of lowering through the admitted family-request seam: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_no_longer_teaches_support_bucket() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES,
        &FORBIDDEN_SUPPORT_BUCKET_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input subtree still teaches a generic support bucket instead of explicit error-mapping and spatial-family bridge seams: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_root_no_longer_mixes_family_cases_with_shared_helpers() {
    let violations = collect_violations(
        &AUDITED_FAMILY_REALIZATION_SEAM_FILES,
        &FORBIDDEN_FLAT_FAMILY_CASE_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input boundary still teaches a flat mixed directory instead of separating per-family cases from shared helper seams: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_error_mapping_seam_no_longer_owns_request_admission_helpers() {
    let violations = collect_violations(
        &AUDITED_FAMILY_BIRTH_INPUT_ERROR_MAPPING_FILE,
        &FORBIDDEN_SUPPORT_REQUEST_ADMISSION_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input error-mapping seam still mixes request-admission helpers with geometry lowering instead of leaving request policing on scalar or family-local admission seams: {violations:?}"
    );
}

#[test]
fn phase_five_lower_layer_family_translation_seam_no_longer_mixes_error_mapping_helpers() {
    let violations = collect_violations(
        &AUDITED_FAMILY_BIRTH_INPUT_LOWER_LAYER_FAMILY_TRANSLATION_FILE,
        &FORBIDDEN_SPATIAL_FAMILY_BRIDGE_ERROR_MAPPING_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input lower-layer family translation seam still mixes geometry error lowering instead of owning only lower-layer family translation: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_boundary_no_longer_splits_family_authority() {
    let violations = collect_violations(
        &AUDITED_FAMILY_REALIZATION_SEAM_FILES,
        &FORBIDDEN_SPLIT_FAMILY_AUTHORITY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family-local construction authority is still split across parallel admitted-request and realization seams instead of one family-owned birth-input lane: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_boundary_no_longer_teaches_old_parameter_bucket() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES,
        &FORBIDDEN_OLD_FAMILY_PARAMETER_BUCKET_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input subtree still teaches the deleted generic parameter bucket instead of the narrower scalar-admission seam plus family-local admitted parameter steps: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_helpers_no_longer_inline_raw_parameter_decoding() {
    let violations = collect_violations(
        &AUDITED_FAMILY_REALIZATION_FILES,
        &FORBIDDEN_RAW_FAMILY_PARAMETER_DECODING_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input helpers still inline raw parameter decoding instead of lowering through named admitted-parameter steps: {violations:?}"
    );
}
