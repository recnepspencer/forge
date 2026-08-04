use super::super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[
    ControlledMutation {
        id: 112,
        predicate: "c7-cold-source-binding-headroom-regressed",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/timing/mod.rs",
        needle: "const SOURCE_BINDING_WALL_BUDGET_MS: u64 = 10_000;",
        replacement: "const SOURCE_BINDING_WALL_BUDGET_MS: u64 = 2_000;",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::timing::tests::bounded_cold_source_binding_retains_secondary_wall_headroom",
    },
    ControlledMutation {
        id: 113,
        predicate: "c7-case-wall-headroom-regressed",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign/timing.rs",
        needle: "const CASE_SECONDARY_HANG_GUARD_MS: u64 = 180_000;",
        replacement: "const CASE_SECONDARY_HANG_GUARD_MS: u64 = 60_000;",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::timing::tests::secondary_hang_guard_retains_canonical_multi_case_headroom",
    },
    ControlledMutation {
        id: 114,
        predicate: "c7-nested-case-cost-class-collapsed",
        source: "tools/store-test-runner/src/mutation_campaign/evidence.rs",
        needle: "const NESTED_EXECUTABLE_COLD_LIMIT_MS: u64 = 300_000;",
        replacement: "const NESTED_EXECUTABLE_COLD_LIMIT_MS: u64 = 180_000;",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "mutation_campaign::evidence::tests::nested_executable_cases_retain_distinct_cold_build_headroom",
    },
];
