pub(super) fn phase_seven_responsibility(path: &str) -> Option<&'static str> {
    match path {
        "crates/worth-store-recovery-runtime/src/cleanup/accounting.rs" => {
            Some("exact-cleanup-attempt-accounting")
        }
        "crates/worth-store-recovery-runtime/src/cleanup/attempt.rs" => {
            Some("owner-sampled-cleanup-attempt")
        }
        "crates/worth-store-recovery-runtime/src/cleanup/cancellation.rs" => {
            Some("plan-bound-cleanup-cancellation")
        }
        "crates/worth-store-recovery-runtime/src/cleanup/command_basis.rs" => {
            Some("sealed-cleanup-command-basis")
        }
        "crates/worth-store-recovery-runtime/src/cleanup/disposition.rs" => {
            Some("per-artifact-cleanup-disposition")
        }
        "crates/worth-store-recovery-physics/src/source_precedence/checkpoint_covered_wal.rs" => {
            Some("checkpoint-covered-wal-retention-law")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/mod.rs" => {
            Some("cleanup-coordination-module-boundary")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/admission.rs" => {
            Some("cleanup-work-admission")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/admission/denial.rs" => {
            Some("typed-cleanup-admission-denial")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/freshness.rs" => {
            Some("cleanup-freshness-read-settlement")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/removal.rs" => {
            Some("cleanup-removal-command-and-outcome")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/removal/execution.rs" => {
            Some("scheduled-cleanup-removal-execution")
        }
        "crates/worth-store/src/physical_runtime/recovery_coordination/effect/cleanup.rs" => {
            Some("performed-cleanup-effect-evidence")
        }
        _ => None,
    }
}
