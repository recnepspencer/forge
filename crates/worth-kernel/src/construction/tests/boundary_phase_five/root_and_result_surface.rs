use super::patterns::*;
use super::sources::*;

#[test]
fn phase_five_admitted_scaffold_subtree_owns_scaffold_only_modules() {
    let violations = collect_violations(
        &AUDITED_CONSTRUCTION_ROOT_FILES,
        &FORBIDDEN_PEER_SCAFFOLD_DECLARATION_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "construction root still teaches scaffold-only modules as peer phases instead of admitted-scaffold subtree details: {violations:?}"
    );
}

#[test]
fn phase_five_result_surfaces_no_longer_depend_on_broad_scaffold_type() {
    let violations = collect_violations(
        &AUDITED_RESULT_SURFACE_FILES,
        &FORBIDDEN_RESULT_SURFACE_SCAFFOLD_DEPENDENCY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "result/evidence/artifact production files still depend on the broad scaffold type instead of the narrowed admitted artifact seam: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_inlines_admitted_artifact_cloning() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES,
        &FORBIDDEN_ADMITTED_SCAFFOLD_ROOT_ADMITTED_ARTIFACT_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still owns the old admitted-result-input struct or re-clones that seam's truth instead of delegating the phase boundary honestly: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_teaches_transient_wrapper_phase() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES,
        &FORBIDDEN_ADMITTED_SCAFFOLD_WRAPPER_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still teaches a transient admitted-scaffold wrapper phase instead of producing one admitted artifact as its honest output: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_subtree_no_longer_teaches_transient_admitted_geometry_phase() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES,
        &FORBIDDEN_TRANSIENT_ADMITTED_GEOMETRY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold subtree still teaches a transient admitted-geometry phase instead of one direct request-to-birth-input bridge: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_subtree_no_longer_teaches_deleted_split_bridge_modules() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES,
        &FORBIDDEN_DELETED_BRIDGE_MODULE_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold subtree still teaches the deleted embedded-birth or realized-birth split instead of one explicit birth-scaffold bridge seam: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_subtree_no_longer_teaches_old_mixed_request_buckets() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES,
        &FORBIDDEN_OLD_REQUEST_ADMISSION_BUCKET_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold subtree still teaches the deleted mixed request buckets or the removed admitted-family-request subtree instead of one placement seam plus one family-owned realization lane: {violations:?}"
    );
}

#[test]
fn phase_five_query_backed_entry_surfaces_no_longer_call_direct_local_preparation() {
    let violations = collect_violations(
        &AUDITED_QUERY_BACKED_ENTRY_FILES,
        &FORBIDDEN_QUERYLESS_RUNTIME_ENTRY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "query-backed runtime/public construction entry surfaces still call direct local preparation helpers instead of crossing the declaration-authoring query front door: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_artifact_no_longer_retains_broad_birth_input_bag() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_ARTIFACT_FILE,
        &FORBIDDEN_BROAD_BIRTH_INPUT_RETAINED_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted artifact seam still retains the full lower-layer birth-input bag instead of only the birth-side facts the post-handoff kernel result lane genuinely needs: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_artifact_no_longer_retains_local_request_or_intent_digest_sidecars() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_ARTIFACT_FILE,
        &FORBIDDEN_LOCAL_DIGEST_SIDECAR_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted artifact seam still retains local request or intent digest sidecars after topology admitted handoff instead of keeping only the remaining post-handoff birth witness facts: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_artifact_no_longer_retains_local_family_or_scaffold_sidecars() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_ARTIFACT_FILE,
        &FORBIDDEN_LOCAL_BIRTH_SIDE_SIDECAR_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted artifact seam still retains local family or scaffold sidecars even though topology admitted handoff already carries birth-plan truth: {violations:?}"
    );
}

#[test]
fn phase_five_result_surfaces_no_longer_depend_on_admitted_artifact_request_or_intent_digest() {
    let violations = collect_violations(
        &AUDITED_RESULT_SURFACE_FILES,
        &FORBIDDEN_RESULT_SURFACE_LOCAL_DIGEST_DEPENDENCY_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "result/evidence/artifact production files still depend on admitted-artifact request or intent digest sidecars instead of the remaining post-handoff semantic seam: {violations:?}"
    );
}

#[test]
fn phase_five_request_no_longer_retains_parallel_family_sidecar() {
    let violations = collect_violations(
        &AUDITED_REQUEST_FILE,
        &FORBIDDEN_REQUEST_FAMILY_SIDECAR_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "primitive construction request still retains a parallel family sidecar instead of deriving family truth from geometry itself: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_teaches_category_bucket_family_helpers() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES,
        &FORBIDDEN_ROOT_CATEGORY_BUCKET_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still teaches broad category-bucket family helper files instead of the explicit family-birth-input subtree: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_owns_topology_handoff_sequencing() {
    let violations = collect_violations(
        &AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES,
        &FORBIDDEN_KERNEL_TOPOLOGY_HANDOFF_SEQUENCING_PATTERNS,
    );
    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still sequences topology handoff creation locally instead of crossing the topology-owned synopsis-to-admitted-handoff seam: {violations:?}"
    );
}
