#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_plan_inspection_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn runtime_plan_inspection_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn plan_inspection_facade_types_are_importable() {
    runtime_plan_inspection_compile_pass(
        "tests/ui/runtime_authority/pass/runtime_plan_inspection_facade_types.rs",
    );
}

#[test]
fn execution_plan_inspection_fields_are_not_publicly_mintable() {
    runtime_plan_inspection_compile_fail(
        "tests/ui/runtime_authority/fail/execution_plan_inspection_fields_not_public.rs",
    );
}

#[test]
fn plan_node_inspection_fields_are_not_publicly_mintable() {
    runtime_plan_inspection_compile_fail(
        "tests/ui/runtime_authority/fail/plan_node_inspection_fields_not_public.rs",
    );
}

#[test]
fn artifact_to_plan_provenance_fields_are_not_publicly_mintable() {
    runtime_plan_inspection_compile_fail(
        "tests/ui/runtime_authority/fail/artifact_to_plan_provenance_fields_not_public.rs",
    );
}

#[test]
fn lane_inspection_fields_are_not_publicly_mintable() {
    runtime_plan_inspection_compile_fail(
        "tests/ui/runtime_authority/fail/lane_inspection_fields_not_public.rs",
    );
}

#[test]
fn plan_inspection_rejects_unlinked_query_explanation_records() {
    runtime_plan_inspection_compile_fail(
        "tests/ui/runtime_authority/fail/local_query_explanation_record_cannot_replace_query_inspection_link.rs",
    );
}

#[test]
fn plan_inspection_counters_are_not_publicly_mintable() {
    runtime_plan_inspection_compile_fail(
        "tests/ui/runtime_authority/fail/plan_inspection_counters_not_publicly_mintable.rs",
    );
}

#[test]
fn query_inspection_link_fields_are_not_publicly_mintable() {
    runtime_plan_inspection_compile_fail(
        "tests/ui/runtime_authority/fail/query_inspection_links_fields_not_public.rs",
    );
}

