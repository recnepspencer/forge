use super::super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[
    ControlledMutation {
        id: 125,
        predicate: "c7-case-observed-wall-headroom-regressed",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign/timing.rs",
        needle: "const CASE_SECONDARY_HANG_GUARD_MS: u64 = 180_000;",
        replacement: "const CASE_SECONDARY_HANG_GUARD_MS: u64 = 120_000;",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::timing::tests::secondary_hang_guard_retains_observed_environment_headroom",
    },
    ControlledMutation {
        id: 126,
        predicate: "c7-case-wall-denial-unlocalized",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign/timing.rs",
        needle: "            return Err(hang_guard_denial(total_elapsed_ms, &stages));",
        replacement: "            return Err(format!(\"Courtroom C case took {total_elapsed_ms}ms\"));",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::timing::tests::hang_guard_denial_localizes_slowest_and_every_stage",
    },
];
