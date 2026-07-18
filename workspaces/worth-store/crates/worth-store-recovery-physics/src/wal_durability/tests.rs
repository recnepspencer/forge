use std::io::Write;

use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
    SimulatedStrictDurableProfile,
};

use super::{execute_wal_durability, WalAppendPlan, WalDurabilityExecutionError};
use crate::{LogSequenceNumber, WalLsnRange, WalSegmentGeneration, WalSegmentId};

#[test]
fn consecutive_appends_share_one_framed_segment() {
    let root = execution_root("consecutive");
    let planner = open_planner(root.path());
    let first = execute(&planner, 0, 1, "first", b"alpha").unwrap();
    let second = execute(&planner, 1, 2, "second", b"beta").unwrap();
    assert_eq!(
        first.execution().persisted_path(),
        second.execution().persisted_path()
    );
    assert_eq!(
        second.execution().persisted_offset(),
        first.execution().persisted_bytes()
    );
    assert_eq!(
        std::fs::metadata(second.execution().persisted_path())
            .unwrap()
            .len(),
        second.execution().persisted_offset() + second.execution().persisted_bytes(),
    );
}

#[test]
fn torn_trailing_frame_is_removed_before_the_next_acknowledgment() {
    let root = execution_root("torn-tail");
    let planner = open_planner(root.path());
    let first = execute(&planner, 10, 11, "first", b"alpha").unwrap();
    std::fs::OpenOptions::new()
        .append(true)
        .open(first.execution().persisted_path())
        .unwrap()
        .write_all(b"partial-frame")
        .unwrap();
    let second = execute(&planner, 11, 12, "second", b"beta").unwrap();
    assert_eq!(
        second.execution().persisted_offset(),
        first.execution().persisted_bytes()
    );
}

#[test]
fn noncontiguous_lsn_is_a_typed_append_denial() {
    let root = execution_root("lsn-gap");
    let planner = open_planner(root.path());
    execute(&planner, 20, 21, "first", b"alpha").unwrap();
    assert!(matches!(
        execute(&planner, 22, 23, "gap", b"beta"),
        Err(WalDurabilityExecutionError::Artifact(
            worth_store_wal::WalArtifactStoreDenial::NonContiguousLsn
        ))
    ));
}

#[test]
fn corrupted_committed_frame_blocks_later_acknowledgment() {
    let root = execution_root("corruption");
    let planner = open_planner(root.path());
    let first = execute(&planner, 30, 31, "first", b"alpha").unwrap();
    let mut bytes = std::fs::read(first.execution().persisted_path()).unwrap();
    bytes[116] ^= 1;
    std::fs::write(first.execution().persisted_path(), bytes).unwrap();
    assert!(matches!(
        execute(&planner, 31, 32, "second", b"beta"),
        Err(WalDurabilityExecutionError::Artifact(
            worth_store_wal::WalArtifactStoreDenial::DigestMismatch
        ))
    ));
}

#[cfg(windows)]
#[test]
fn windows_wal_append_flushes_its_parent_namespace_without_rename_publication() {
    use worth_store_physical_backend::WindowsFlushFileBuffersProfile;

    let root = execution_root("windows-parent-namespace");
    let planner = open_planner(root.path());
    let payload = b"host-durable";
    let plan = WalAppendPlan::<WindowsFlushFileBuffersProfile>::new(
        WalSegmentId::new(1).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(0), LogSequenceNumber::new(1)).unwrap(),
        "windows-parent-namespace",
        payload.len() as u64,
    )
    .unwrap();
    let backend = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::WindowsFlushFileBuffers,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap();

    let outcome = execute_wal_durability(&planner, payload, plan, &backend).unwrap();

    assert!(outcome.execution().persisted_path().is_file());
    assert!(outcome.durability().counters().directory_syncs_completed() > 0);
    assert_eq!(outcome.durability().counters().renames_completed(), 0);
}

#[cfg(feature = "certification-test-authority")]
#[test]
fn injected_torn_frame_cannot_issue_acknowledgment_and_reopens_at_valid_prefix() {
    use worth_store_physical_backend::{
        ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
    };

    let root = execution_root("injected-torn-tail");
    let planner = open_planner(root.path());
    let first = execute(&planner, 40, 41, "first", b"alpha").unwrap();
    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::WalAppendBeforeFlush,
        StorageBoundaryFault::TearWrite { retained_bytes: 7 },
    );
    let plan: WalAppendPlan<SimulatedStrictDurableProfile> = WalAppendPlan::new(
        WalSegmentId::new(1).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(41), LogSequenceNumber::new(42)).unwrap(),
        "second",
        4,
    )
    .unwrap();
    let denial = super::execute_wal_durability_with_boundary_control(
        &planner,
        b"beta",
        plan,
        &backend(),
        &control,
    )
    .unwrap_err();
    assert!(matches!(
        denial,
        WalDurabilityExecutionError::PhysicalIo(error)
            if error.kind() == std::io::ErrorKind::Interrupted
    ));
    assert_eq!(control.trace().injected().len(), 1);

    let retried = execute(&planner, 41, 42, "second", b"beta").unwrap();
    assert_eq!(
        retried.execution().persisted_offset(),
        first.execution().persisted_bytes()
    );
}

#[cfg(feature = "certification-test-authority")]
#[test]
fn dropped_flush_cannot_mint_a_durability_proof_or_acknowledgment() {
    use worth_store_physical_backend::{
        ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
    };

    let root = execution_root("dropped-flush");
    let planner = open_planner(root.path());
    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::WalFlush,
        StorageBoundaryFault::AbortBeforeDurabilityBarrier,
    );
    let plan: WalAppendPlan<SimulatedStrictDurableProfile> = WalAppendPlan::new(
        WalSegmentId::new(1).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(0), LogSequenceNumber::new(1)).unwrap(),
        "unacknowledged",
        5,
    )
    .unwrap();
    assert!(matches!(
        super::execute_wal_durability_with_boundary_control(
            &planner,
            b"value",
            plan,
            &backend(),
            &control,
        ),
        Err(WalDurabilityExecutionError::PhysicalIo(error))
            if error.kind() == std::io::ErrorKind::Interrupted
    ));
    assert_eq!(
        control.trace().injected(),
        &[(
            (ProductionStorageBoundarySeam::WalFlush),
            StorageBoundaryFault::AbortBeforeDurabilityBarrier
        )]
    );
}

#[cfg(feature = "certification-test-authority")]
#[test]
fn generated_torn_wal_prefixes_never_acknowledge_or_break_lsn_continuity() {
    use worth_store_physical_backend::{
        ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
    };

    let probe_root = execution_root("generated-tear-size-probe");
    let probe_planner = open_planner(probe_root.path());
    execute(&probe_planner, 0, 1, "first", b"alpha").unwrap();
    let encoded_second_frame_bytes = execute(&probe_planner, 1, 2, "second", b"beta")
        .unwrap()
        .execution()
        .persisted_bytes();

    for retained_bytes in 0..encoded_second_frame_bytes {
        let root = execution_root(&format!("generated-tear-{retained_bytes}"));
        let planner = open_planner(root.path());
        let first = execute(&planner, 0, 1, "first", b"alpha").unwrap();
        let control = ScriptedStorageBoundaryControl::inject(
            ProductionStorageBoundarySeam::WalAppendBeforeFlush,
            StorageBoundaryFault::TearWrite { retained_bytes },
        );
        let plan: WalAppendPlan<SimulatedStrictDurableProfile> = WalAppendPlan::new(
            WalSegmentId::new(1).unwrap(),
            WalSegmentGeneration::new(1).unwrap(),
            WalLsnRange::new(LogSequenceNumber::new(1), LogSequenceNumber::new(2)).unwrap(),
            "second",
            4,
        )
        .unwrap();
        assert!(super::execute_wal_durability_with_boundary_control(
            &planner,
            b"beta",
            plan,
            &backend(),
            &control,
        )
        .is_err());

        let retried = execute(&planner, 1, 2, "second", b"beta").unwrap();
        assert_eq!(
            retried.execution().persisted_offset(),
            first.execution().persisted_bytes(),
            "retained bytes {retained_bytes}",
        );
        execute(&planner, 2, 3, "third", b"gamma").unwrap();
    }
}

fn execute(
    planner: &worth_store_wal::WalAppendPlanner,
    start: u64,
    end: u64,
    digest: &str,
    payload: &[u8],
) -> Result<
    super::ExecutedWalDurabilityOutcome<SimulatedStrictDurableProfile>,
    WalDurabilityExecutionError,
> {
    let plan = WalAppendPlan::new(
        WalSegmentId::new(1).unwrap(),
        WalSegmentGeneration::new(1).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(start), LogSequenceNumber::new(end)).unwrap(),
        digest,
        payload.len() as u64,
    )
    .unwrap();
    execute_wal_durability(planner, payload, plan, &backend())
}

fn open_planner(root: &std::path::Path) -> worth_store_wal::WalAppendPlanner {
    worth_store_wal::WalAppendPlanner::open(root, 1, 1).expect("open WAL planner")
}

fn backend() -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::SimulatedStrictDurable,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}

fn execution_root(label: &str) -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix(&format!("worth-store-wal-{label}-"))
        .tempdir()
        .unwrap()
}
