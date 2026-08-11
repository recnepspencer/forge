use std::path::PathBuf;
use std::process::ExitCode;

use worth_store_recovery_runtime::{
    PhysicalRecoveryLimitDeclaration, PhysicalRecoveryLimits, PhysicalRecoveryOpenRequest,
    PhysicalRecoveryOutcome, PhysicalRecoveryPlatformAuthority,
    PhysicalRecoveryStaticConfiguration, WorthStoreRecovery,
};

const BOUNDED_PROFILE: &str = "c8-phase2-admission-v1";

fn main() -> ExitCode {
    match run(std::env::args_os().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("physical_store_recover: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: Vec<std::ffi::OsString>) -> Result<(), String> {
    let [root, profile] = arguments.as_slice() else {
        return Err(format!(
            "usage: physical_store_recover <store-root> --bounded-profile={BOUNDED_PROFILE}"
        ));
    };
    if profile.to_string_lossy() != format!("--bounded-profile={BOUNDED_PROFILE}") {
        return Err(format!(
            "unsupported bounded profile; expected {BOUNDED_PROFILE}"
        ));
    }
    let root = PathBuf::from(root);
    let configuration = PhysicalRecoveryStaticConfiguration::current();
    let limits = phase_two_admission_limits()?;
    let authority =
        PhysicalRecoveryPlatformAuthority::acquire(root.clone(), configuration.clone(), limits)
            .map_err(|error| format!("platform authority refused: {error:?}"))?;
    let backend_profile = authority.qualified_backend_profile().clone();
    let request = PhysicalRecoveryOpenRequest::declare(
        root,
        configuration,
        backend_profile,
        limits,
        authority,
    );
    match WorthStoreRecovery::recover(request) {
        PhysicalRecoveryOutcome::Recovered(handoff) => {
            eprintln!(
                "recovered Store {:?} into runtime {:?} at root generation {}",
                handoff.core().store_identity().bytes(),
                handoff.core().runtime_identity(),
                handoff.core().root().generation()
            );
            Ok(())
        }
        outcome => Err(format!("physical recovery did not complete: {outcome:?}")),
    }
}

fn phase_two_admission_limits() -> Result<PhysicalRecoveryLimits, String> {
    PhysicalRecoveryLimits::admit(PhysicalRecoveryLimitDeclaration {
        selector_candidates: 4,
        checkpoint_candidates: 64,
        manifest_bytes: 64 * 1024 * 1024,
        manifest_entries: 1_000_000,
        wal_segments: 4_096,
        wal_frames: 16_000_000,
        wal_bytes: u32::MAX as u64,
        redo_targets: 16_000_000,
        redo_bytes: u32::MAX as u64,
        distinct_pages_and_extents: 16_000_000,
        operation_bindings: 16_000_000,
        staging_bytes: u32::MAX as u64,
        dirty_frames: 1_000_000,
        concurrent_commands: 64,
        publication_effects: 64,
        cleanup_candidates: 1_000_000,
        cleanup_bytes: u32::MAX as u64,
        observation_bytes: 64 * 1024 * 1024,
    })
    .map_err(|error| format!("invalid built-in bounded profile: {error:?}"))
}
