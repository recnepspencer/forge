#[path = "trybuild_support.rs"]
mod trybuild_support;
fn runtime_measurement_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn runtime_measurement_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn measurement_facade_types_are_importable() {
    runtime_measurement_pass("tests/ui/runtime_measurement/pass/measurement_facade_types.rs");
}

#[test]
fn measurement_counter_fields_are_not_publicly_mintable() {
    runtime_measurement_fail(
        "tests/ui/runtime_measurement/fail/measurement_counter_fields_not_public.rs",
    );
}

#[test]
fn certified_measurement_packet_fields_are_not_publicly_mintable() {
    runtime_measurement_fail(
        "tests/ui/runtime_measurement/fail/certified_measurement_packet_fields_not_public.rs",
    );
}

#[test]
fn unattributed_work_bucket_is_not_public_facade_dx() {
    runtime_measurement_fail(
        "tests/ui/runtime_measurement/fail/unattributed_work_bucket_not_public.rs",
    );
}

#[test]
fn uncertified_counter_packet_cannot_lower_to_foundational() {
    runtime_measurement_fail(
        "tests/ui/runtime_measurement/fail/uncertified_counter_packet_cannot_lower_to_foundational.rs",
    );
}

