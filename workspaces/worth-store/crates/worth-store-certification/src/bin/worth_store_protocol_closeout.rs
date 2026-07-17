use std::num::NonZeroU64;
use std::path::PathBuf;

use worth_store_certification::courtroom::protocol_models::closeout::{
    adjudicate_protocol_law_closeout, run_checked_protocol_program,
};
use worth_store_certification::courtroom::protocol_models::mutants::run_controlled_mutant_program;
use worth_store_formal_models::runner::ProtocolCheckBounds;
#[cfg(not(windows))]
use worth_store_physical_backend::PosixFileFsyncDirFsyncProfile;
#[cfg(windows)]
use worth_store_physical_backend::WindowsFlushFileBuffersProfile;
use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendDurabilityProfile, BackendMediaAssumptionSet, BackendRebindTriggers,
    PhysicalBackendCapabilityAdmissionAuthority,
};

#[cfg(not(windows))]
type HostDurabilityProfile = PosixFileFsyncDirFsyncProfile;
#[cfg(windows)]
type HostDurabilityProfile = WindowsFlushFileBuffersProfile;

fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();
    let [_, java, tool_jar, state_root] = arguments.as_slice() else {
        eprintln!("usage: worth-store-protocol-closeout <java> <tla2tools.jar> <state-root>");
        std::process::exit(2);
    };
    let bounds = ProtocolCheckBounds::new(
        NonZeroU64::new(1_000_000).unwrap(),
        NonZeroU64::new(100).unwrap(),
    );
    let state_root = PathBuf::from(state_root);
    let model_crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("certification crate has a crates directory")
        .join("worth-store-formal-models");
    let runtime_backend = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            HostDurabilityProfile::TARGET,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap_or_else(|denial| {
            eprintln!("backend capability admission failed: {denial:?}");
            std::process::exit(1);
        });
    let checked = run_checked_protocol_program::<HostDurabilityProfile>(
        java,
        tool_jar,
        state_root.join("checked"),
        model_crate_root,
        &runtime_backend,
        bounds,
    )
    .unwrap_or_else(|failure| {
        eprintln!("checked protocol program failed: {failure:?}");
        std::process::exit(1);
    });
    let mutants = run_controlled_mutant_program(java, tool_jar, state_root.join("mutants"), bounds)
        .unwrap_or_else(|failure| {
            eprintln!("controlled protocol mutation program failed: {failure:?}");
            std::process::exit(1);
        });
    let report = adjudicate_protocol_law_closeout(checked, mutants).unwrap_or_else(|denial| {
        eprintln!("protocol closeout denied: {denial:?}");
        std::process::exit(1);
    });

    for row in report.rows() {
        println!(
            "closed {:?}: {} distinct states, localized {:?}",
            row.protocol(),
            row.checked_execution().statistics().distinct_states(),
            row.controlled_defect().mutant(),
        );
    }
}
