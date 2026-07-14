use super::compile_fail_support;

mod runtime_authority_additional_cases;
mod runtime_authority_cases;

const EXTERN_CRATES: &[&str] = &[
    "forge_store_authority",
    "forge_store_budgets",
    "forge_store_compatibility",
    "forge_store_lsm_authority",
    "forge_store_physical_format",
    "forge_store_physical_isolation",
    "forge_store_recovery_physics",
    "forge_store_security",
    "forge_store_wal",
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
