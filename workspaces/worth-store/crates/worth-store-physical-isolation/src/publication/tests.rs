use std::io::Write;

use super::store::PhysicalRootPublicationStore;
use super::PhysicalPublicationDenial;
use crate::{
    epoch::{manifest_epoch_from_entry_seed, root_epoch_from_entry_seed},
    CurrentPhysicalRoot, CurrentPhysicalRootBasis, PhysicalOrderingContract,
};

#[test]
fn durable_root_reopens_only_at_the_published_identity() {
    let directory = publication_directory("reopen");
    let first = root(1, false);
    let second = root(2, false);
    let store = PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    store.publish(first, second).unwrap();

    PhysicalRootPublicationStore::open(directory.path(), second).unwrap();
    assert_eq!(
        PhysicalRootPublicationStore::open(directory.path(), first).unwrap_err(),
        PhysicalPublicationDenial::PersistedRootMismatch,
    );
}

#[test]
fn competing_publication_owner_observes_locked_compare_and_swap() {
    let directory = publication_directory("concurrent");
    let first = root(10, false);
    let second = root(11, false);
    let third = root(12, false);
    let winner = PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    let stale = PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    winner.publish(first, second).unwrap();
    assert_eq!(
        stale.publish(first, third).unwrap_err(),
        PhysicalPublicationDenial::ConcurrentRootPublication,
    );
}

#[test]
fn recovery_publication_binding_survives_reopen_and_distinguishes_same_epoch_media() {
    let directory = publication_directory("recovery-binding");
    let first = root(13, false);
    let second = root(14, false);
    let store = PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    let binding = [0x5a; 32];
    store
        .publish_recovery_with_boundary_control(
            first,
            second,
            binding,
            &worth_store_physical_backend::UninterruptedStorageBoundaryControl,
        )
        .unwrap();

    let reopened = PhysicalRootPublicationStore::open(directory.path(), second).unwrap();
    assert_eq!(reopened.current_recovery_binding().unwrap(), Some(binding));
    assert_ne!(
        reopened.current_recovery_binding().unwrap(),
        Some([0xa5; 32])
    );
}

#[test]
fn torn_tail_is_truncated_before_the_next_publication() {
    let directory = publication_directory("torn-tail");
    let first = root(20, false);
    let second = root(21, false);
    let store = PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    store.publish(first, second).unwrap();
    let log = directory.path().join("root-publications.log");
    let durable_bytes = std::fs::metadata(&log).unwrap().len();
    std::fs::OpenOptions::new()
        .append(true)
        .open(&log)
        .unwrap()
        .write_all(b"crash-tail")
        .unwrap();

    PhysicalRootPublicationStore::open(directory.path(), second).unwrap();
    assert_eq!(std::fs::metadata(log).unwrap().len(), durable_bytes);
}

#[test]
fn ordering_contract_is_part_of_persisted_root_identity() {
    let directory = publication_directory("ordering");
    let acquire_release = root(30, false);
    let sequential = root(30, true);
    PhysicalRootPublicationStore::open(directory.path(), acquire_release).unwrap();
    assert_eq!(
        PhysicalRootPublicationStore::open(directory.path(), sequential).unwrap_err(),
        PhysicalPublicationDenial::PersistedRootMismatch,
    );
}

#[test]
fn injected_torn_root_record_reopens_at_the_previous_root() {
    use worth_store_physical_backend::{
        ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
    };

    let directory = publication_directory("injected-torn");
    let first = root(40, false);
    let second = root(41, false);
    let store = PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::RootSwap,
        StorageBoundaryFault::TearWrite { retained_bytes: 11 },
    );
    assert_eq!(
        store
            .publish_with_boundary_control(first, second, &control)
            .unwrap_err(),
        PhysicalPublicationDenial::PublicationStoreIo,
    );
    PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    assert_eq!(control.trace().injected().len(), 1);
}

#[test]
fn interruption_after_root_durability_reopens_at_new_root_without_issuing_outcome() {
    use worth_store_physical_backend::{
        ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
    };

    let directory = publication_directory("durable-interruption");
    let first = root(50, false);
    let second = root(51, false);
    let store = PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    let control = ScriptedStorageBoundaryControl::inject(
        ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        StorageBoundaryFault::Interrupt,
    );
    assert_eq!(
        store
            .publish_with_boundary_control(first, second, &control)
            .unwrap_err(),
        PhysicalPublicationDenial::PublicationStoreIo,
    );
    PhysicalRootPublicationStore::open(directory.path(), second).unwrap();
    assert_eq!(
        PhysicalRootPublicationStore::open(directory.path(), first).unwrap_err(),
        PhysicalPublicationDenial::PersistedRootMismatch,
    );
}

#[test]
fn concurrent_publishers_execute_against_the_same_locked_root() {
    let directory = publication_directory("threaded-race");
    let first = root(60, false);
    let second = root(61, false);
    let third = root(62, false);
    PhysicalRootPublicationStore::open(directory.path(), first).unwrap();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let publish = |candidate| {
        let directory = directory.path().to_owned();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            let store = PhysicalRootPublicationStore::open(&directory, first).unwrap();
            barrier.wait();
            store.publish(first, candidate)
        })
    };
    let left = publish(second);
    let right = publish(third);
    barrier.wait();
    let outcomes = [left.join().unwrap(), right.join().unwrap()];
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| {
                matches!(
                    outcome,
                    Err(PhysicalPublicationDenial::ConcurrentRootPublication)
                )
            })
            .count(),
        1,
    );
}

#[test]
fn fresh_process_crash_after_root_durability_reopens_at_new_root() {
    const CHILD_FLAG: &str = "WORTH_STORE_ROOT_CRASH_CHILD";
    const DIRECTORY: &str = "WORTH_STORE_ROOT_CRASH_DIRECTORY";
    if std::env::var_os(CHILD_FLAG).is_some() {
        let directory = std::path::PathBuf::from(std::env::var_os(DIRECTORY).unwrap());
        let store = PhysicalRootPublicationStore::open(&directory, root(70, false)).unwrap();
        let control = worth_store_physical_backend::ProcessCrashStorageBoundaryControl::at(
            worth_store_physical_backend::ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
        );
        let _ = store.publish_with_boundary_control(root(70, false), root(71, false), &control);
        panic!("process-crash control returned instead of terminating the child");
    }

    let directory = publication_directory("fresh-process-crash");
    PhysicalRootPublicationStore::open(directory.path(), root(70, false)).unwrap();
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "publication::tests::fresh_process_crash_after_root_durability_reopens_at_new_root",
            "--nocapture",
        ])
        .env(CHILD_FLAG, "1")
        .env(DIRECTORY, directory.path())
        .status()
        .unwrap();
    assert!(!status.success());
    PhysicalRootPublicationStore::open(directory.path(), root(71, false)).unwrap();
}

#[test]
fn every_torn_root_offset_reopens_to_exactly_one_complete_root() {
    use worth_store_physical_backend::{
        ProductionStorageBoundarySeam, ScriptedStorageBoundaryControl, StorageBoundaryFault,
    };

    for retained_bytes in 0..=128 {
        let directory = publication_directory(&format!("tear-{retained_bytes}"));
        let previous = root(80, false);
        let candidate = root(81, false);
        let store = PhysicalRootPublicationStore::open(directory.path(), previous).unwrap();
        let control = ScriptedStorageBoundaryControl::inject(
            ProductionStorageBoundarySeam::RootSwap,
            StorageBoundaryFault::TearWrite { retained_bytes },
        );
        assert_eq!(
            store
                .publish_with_boundary_control(previous, candidate, &control)
                .unwrap_err(),
            PhysicalPublicationDenial::PublicationStoreIo,
        );

        let previous_reopens =
            PhysicalRootPublicationStore::open(directory.path(), previous).is_ok();
        let candidate_reopens =
            PhysicalRootPublicationStore::open(directory.path(), candidate).is_ok();
        assert_ne!(
            previous_reopens, candidate_reopens,
            "offset {retained_bytes}"
        );
    }
}

fn root(seed: u64, sequential: bool) -> CurrentPhysicalRoot {
    let ordering = if sequential {
        PhysicalOrderingContract::sequentially_consistent_for(crate::PhysicalOrderingSite::RootSwap)
    } else {
        PhysicalOrderingContract::root_swap_acquire_release()
    };
    CurrentPhysicalRoot::from_physical_isolation_entry(
        CurrentPhysicalRootBasis::new(
            root_epoch_from_entry_seed(seed),
            manifest_epoch_from_entry_seed(seed),
            worth_store_physical_format::PhysicalStoreIdentity::physical_format_default()
                .authority_identity(),
        ),
        ordering,
    )
    .unwrap()
}

fn publication_directory(label: &str) -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix(&format!("worth-store-root-{label}-"))
        .tempdir()
        .unwrap()
}
