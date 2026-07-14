use super::compile_fail_support;

mod runtime_authority_additional_cases;
mod runtime_authority_cases;

const EXTERN_CRATES: &[&str] = &[
    "worth_store_authority",
    "worth_store_budgets",
    "worth_store_compatibility",
    "worth_store_lsm_authority",
    "worth_store_physical_format",
    "worth_store_physical_isolation",
    "worth_store_recovery_physics",
    "worth_store_security",
    "worth_store_wal",
];

#[test]
fn parallel_runtime_and_compatibility_modules_are_removed() {
    for (fixture, expected) in runtime_authority_cases::CASES
        .iter()
        .copied()
        .chain(runtime_authority_additional_cases::CASES)
    {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "runtime_authority",
            fixture,
            &[expected],
            EXTERN_CRATES,
        );
    }
}
