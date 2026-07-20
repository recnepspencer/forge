use std::path::Path;

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    MediaOwnedPhysicalRuntime, MediaShutdownOutcome, ObservationError,
};
use worth_store_physical_backend::{MediaCounterSnapshot, MediaOperationRole};

use super::child_dispatch::{emit, run_role, ChildReport};
use super::{admit_runtime, media_admission};

#[path = "namespace_discovery/canonical_equivalence.rs"]
mod canonical_equivalence;
#[path = "namespace_discovery/observer_report.rs"]
mod observer_report;
use canonical_equivalence::canonical_mismatch_loci;
use observer_report::{assert_namespace_shape, observe_namespace};

#[test]
fn namespace_creation_and_fresh_process_discovery() {
    let parent = tempfile::tempdir().unwrap();
    let absent = parent.path().join("absent-store");
    let existing_empty = parent.path().join("existing-empty-store");
    std::fs::create_dir(&existing_empty).unwrap();

    let absent_writer = run_role("namespace-writer", &absent, &[]);
    let empty_writer = run_role("namespace-writer", &existing_empty, &[]);
    assert_initialization_counters(&absent_writer);
    assert_initialization_counters(&empty_writer);

    let relocated = parent.path().join("relocated-store");
    std::fs::rename(&absent, &relocated).unwrap();
    let before_reopen = observe_namespace(&relocated);
    assert_namespace_shape(&before_reopen);
    assert_eq!(
        before_reopen.value("identity"),
        absent_writer.value("store")
    );

    let observer_environment = [
        (
            "WORTH_STORE_C4_OBSERVED_NAMESPACE_VERSION",
            before_reopen.value("namespace_version"),
        ),
        (
            "WORTH_STORE_C4_OBSERVED_ENCODING_VERSION",
            before_reopen.value("encoding_version"),
        ),
        (
            "WORTH_STORE_C4_OBSERVED_IDENTITY",
            before_reopen.value("identity"),
        ),
    ];
    let first_reopener = run_role("namespace-reopener", &relocated, &observer_environment);
    let second_reopener = run_role("namespace-reopener", &relocated, &observer_environment);
    assert_reopener_identity_independence(&absent_writer, &first_reopener);
    assert_reopener_identity_independence(&first_reopener, &second_reopener);
    for reopener in [&first_reopener, &second_reopener] {
        assert_eq!(reopener.value("store"), absent_writer.value("store"));
        assert_eq!(reopener.value("canonical"), "equivalent");
        assert_eq!(
            reopener.value("mutant_loci"),
            "namespace.version,encoding.version,identity,publication.posture"
        );
        assert_reopen_has_no_initialization_effects(reopener);
    }

    let after_reopen = observe_namespace(&relocated);
    assert_namespace_shape(&after_reopen);
    assert_eq!(
        before_reopen.value("identity_record_sha256"),
        after_reopen.value("identity_record_sha256")
    );
    assert_eq!(before_reopen.path_kinds(), after_reopen.path_kinds());

    let empty_observation = observe_namespace(&existing_empty);
    assert_namespace_shape(&empty_observation);
    assert_eq!(
        empty_observation.value("identity"),
        empty_writer.value("store")
    );
    assert_ne!(empty_writer.value("store"), absent_writer.value("store"));
    assert_eq!(
        sorted_child_paths(parent.path()),
        vec![
            "existing-empty-store".to_owned(),
            "relocated-store".to_owned()
        ]
    );
}

pub(super) fn run_child(role: &str, root: &Path) {
    let media = match role {
        "namespace-writer" => admit_writer(root),
        "namespace-reopener" => admit_media(root, media_admission()),
        _ => panic!("unsupported namespace role"),
    };
    let observation = media.observer().snapshot().unwrap();
    let canonical_loci = (role == "namespace-reopener").then(|| canonical_mismatch_loci(&media));
    let store_identity = media.store_identity();
    let runtime_identity = media.runtime_identity();
    let observer = media.observer();
    assert!(matches!(media.close(), MediaShutdownOutcome::Released(_)));
    assert!(matches!(
        observer.snapshot(),
        Err(ObservationError::Closed { .. })
    ));
    let mut fields = report_fields(
        store_identity,
        runtime_identity,
        observation.mutation_owner(),
        observer.media_counters(),
    );
    if let Some(loci) = canonical_loci {
        fields.push(("canonical", "equivalent".into()));
        fields.push(("mutant_loci", loci));
    }
    emit(&fields);
}

fn admit_writer(root: &Path) -> MediaOwnedPhysicalRuntime {
    #[cfg(feature = "certification-test-authority")]
    let admission = worth_store::physical_runtime::FilesystemMediaAdmission::certification(
        worth_store_physical_backend::FilesystemAccessPosture::CoordinatedServiceAccount,
    );
    #[cfg(not(feature = "certification-test-authority"))]
    let admission = media_admission();
    admit_media(root, admission)
}

fn admit_media(
    root: &Path,
    admission: worth_store::physical_runtime::FilesystemMediaAdmission,
) -> MediaOwnedPhysicalRuntime {
    match admit_runtime(root)
        .try_admit_filesystem_media(admission)
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("fresh-process media admission failed"),
    }
}

fn report_fields(
    store_identity: worth_store_physical_format::store_namespace::StableStoreIdentity,
    runtime_identity: worth_store::physical_runtime::RuntimeIdentity,
    owner: worth_store_physical_backend::MutationOwnerObservation,
    counters: MediaCounterSnapshot,
) -> Vec<(&'static str, String)> {
    vec![
        ("store", hex(&store_identity.bytes())),
        ("runtime", runtime_identity.get().to_string()),
        ("process", owner.process_id().to_string()),
        ("owner", hex(&owner.owner().bytes())),
        ("attempt", hex(&owner.attempt().bytes())),
        ("attempted", counters.attempted_operations().to_string()),
        ("completed", counters.completed_operations().to_string()),
        ("denied", counters.denied_before_effect().to_string()),
        ("partial", counters.partial_effects().to_string()),
        (
            "indeterminate",
            counters.indeterminate_effects().to_string(),
        ),
        ("requested_bytes", counters.requested_bytes().to_string()),
        ("completed_bytes", counters.completed_bytes().to_string()),
        (
            "qualification",
            counters.qualification_transactions().to_string(),
        ),
        ("file_syncs", counters.file_syncs().to_string()),
        ("directory_syncs", counters.directory_syncs().to_string()),
        (
            "file_state_sync_attempts",
            counters
                .attempts_for(MediaOperationRole::SynchronizeFileState)
                .to_string(),
        ),
        (
            "directory_publication_sync_attempts",
            counters
                .attempts_for(MediaOperationRole::SynchronizeDirectoryPublication)
                .to_string(),
        ),
        ("replacements", counters.replacements().to_string()),
        ("deletions", counters.deletions().to_string()),
        ("cleanup", counters.cleanup_actions().to_string()),
        (
            "ownership_releases",
            counters.ownership_releases().to_string(),
        ),
        ("live_files", counters.live_file_handles().to_string()),
        (
            "live_directories",
            counters.live_directory_handles().to_string(),
        ),
        (
            "create_new",
            counters
                .attempts_for(MediaOperationRole::CreateNew)
                .to_string(),
        ),
        (
            "positioned_write",
            counters
                .attempts_for(MediaOperationRole::PositionedWrite)
                .to_string(),
        ),
        (
            "append",
            counters
                .attempts_for(MediaOperationRole::Append)
                .to_string(),
        ),
        (
            "truncate",
            counters
                .attempts_for(MediaOperationRole::Truncate)
                .to_string(),
        ),
        (
            "allocate",
            counters
                .attempts_for(MediaOperationRole::Allocate)
                .to_string(),
        ),
        (
            "atomic_replace",
            counters
                .attempts_for(MediaOperationRole::AtomicReplace)
                .to_string(),
        ),
        (
            "delete",
            counters
                .attempts_for(MediaOperationRole::Delete)
                .to_string(),
        ),
        ("conserved", counters.is_conserved().to_string()),
    ]
}

fn assert_initialization_counters(report: &ChildReport) {
    assert_eq!(report.value("conserved"), "true");
    assert_eq!(
        report.number("attempted"),
        report.number("completed")
            + report.number("denied")
            + report.number("partial")
            + report.number("indeterminate")
    );
    assert!(report.number("completed_bytes") <= report.number("requested_bytes"));
    assert!(report.number("file_syncs") >= 1);
    assert!(report.number("directory_syncs") >= 1);
    assert!(report.number("replacements") >= 1);
    assert_closed_resource_conservation(report);
    #[cfg(feature = "certification-test-authority")]
    {
        assert_eq!(report.number("qualification"), 1);
        assert_eq!(report.number("cleanup"), 3);
        assert_eq!(report.number("positioned_write"), 18);
        assert_eq!(report.number("append"), 1);
        assert_eq!(report.number("truncate"), 1);
        assert_eq!(report.number("allocate"), 1);
        assert_eq!(report.number("file_state_sync_attempts"), 3);
        assert_eq!(report.number("directory_publication_sync_attempts"), 4);
    }
    #[cfg(not(feature = "certification-test-authority"))]
    assert_eq!(report.number("qualification"), 0);
}

fn assert_reopen_has_no_initialization_effects(report: &ChildReport) {
    assert_eq!(report.value("conserved"), "true");
    assert_eq!(report.number("create_new"), 0);
    assert_eq!(report.number("positioned_write"), 0);
    assert_eq!(report.number("append"), 0);
    assert_eq!(report.number("truncate"), 0);
    assert_eq!(report.number("allocate"), 0);
    assert_eq!(report.number("atomic_replace"), 0);
    assert_eq!(report.number("delete"), 0);
    assert_eq!(report.number("qualification"), 0);
    assert_eq!(report.number("file_syncs"), 0);
    assert_eq!(report.number("directory_syncs"), 3);
    assert_eq!(report.number("replacements"), 0);
    assert_eq!(report.number("deletions"), 0);
    assert_eq!(report.number("cleanup"), 0);
    assert_closed_resource_conservation(report);
}

fn assert_closed_resource_conservation(report: &ChildReport) {
    assert_eq!(report.number("ownership_releases"), 1);
    assert_eq!(report.number("live_files"), 0);
    assert_eq!(report.number("live_directories"), 0);
}

fn assert_reopener_identity_independence(first: &ChildReport, second: &ChildReport) {
    for identity in ["process", "runtime", "owner", "attempt"] {
        assert_ne!(
            first.value(identity),
            second.value(identity),
            "fresh process reused {identity} identity"
        );
    }
}

fn sorted_child_paths(root: &Path) -> Vec<String> {
    let mut names = std::fs::read_dir(root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
