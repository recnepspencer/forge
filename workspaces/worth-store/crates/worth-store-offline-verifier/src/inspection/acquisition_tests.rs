use sha2::{Digest, Sha256};
use worth_store_physical_backend::{OfflineMediaClosureEntry, OfflineMediaConsistencyBasis};

use crate::{
    OfflineInspectionBudget, OfflineInspectionCancellation, OfflineInspectionDenial,
    OfflineMediaAcquisitionBudget, OfflineMediaAcquisitionDenial, OfflineMediaAcquisitionDimension,
    OfflineStoreInspection, UntrustedOfflineMediaSet,
};

#[test]
fn cancellation_stops_directory_acquisition_before_topology_is_admitted() {
    let directory = tempfile::tempdir().expect("temp directory");
    let file = directory.path().join("file");
    std::fs::write(&file, b"x").expect("fixture file");
    let cancellation = OfflineInspectionCancellation::new();
    cancellation.cancel();
    let result = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        closure_for_paths(&[file]),
    ))
    .cancellation(cancellation)
    .budget(OfflineInspectionBudget::bounded(16, 1024).expect("budget"))
    .start();
    assert!(matches!(
        result,
        Err(OfflineMediaAcquisitionDenial::Interrupted(
            OfflineInspectionDenial::Cancelled
        ))
    ));
}

#[test]
fn acquisition_file_budget_denies_directory_bombs_before_media_materialization() {
    let directory = tempfile::tempdir().expect("temp directory");
    let paths = (0..3)
        .map(|index| directory.path().join(format!("file-{index}")))
        .collect::<Vec<_>>();
    for path in &paths {
        std::fs::write(path, b"x").expect("fixture file");
    }
    let acquisition = OfflineMediaAcquisitionBudget::bounded(2, 10, 4096, 10).expect("budget");
    let inspection = OfflineInspectionBudget::bounded(16, 1024)
        .expect("inspection budget")
        .with_acquisition_budget(acquisition);
    let denial = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        closure_for_paths(&paths),
    ))
    .budget(inspection)
    .start();
    let denial = match denial {
        Err(denial) => denial,
        Ok(_) => panic!("file discovery budget must apply before media admission"),
    };

    assert!(matches!(
        denial,
        OfflineMediaAcquisitionDenial::BudgetExceeded {
            dimension: OfflineMediaAcquisitionDimension::Files,
            admitted: 3,
            limit: 2
        }
    ));
}

#[test]
fn acquisition_depth_budget_uses_an_iterative_fail_closed_walk() {
    let directory = tempfile::tempdir().expect("temp directory");
    let nested = directory.path().join("one").join("two").join("three");
    std::fs::create_dir_all(&nested).expect("nested directories");
    let file = nested.join("file");
    std::fs::write(&file, b"x").expect("fixture file");
    let acquisition = OfflineMediaAcquisitionBudget::bounded(10, 10, 4096, 2).expect("budget");
    let inspection = OfflineInspectionBudget::bounded(16, 1024)
        .expect("inspection budget")
        .with_acquisition_budget(acquisition);
    let denial = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        closure_for_paths(&[file]),
    ))
    .budget(inspection)
    .start();
    let denial = match denial {
        Err(denial) => denial,
        Ok(_) => panic!("depth budget must deny acquisition"),
    };

    assert!(matches!(
        denial,
        OfflineMediaAcquisitionDenial::BudgetExceeded {
            dimension: OfflineMediaAcquisitionDimension::Depth,
            admitted: 3,
            limit: 2
        }
    ));
}

#[test]
fn owned_allocation_budget_denies_media_topology_before_session_start() {
    let directory = tempfile::tempdir().expect("temp directory");
    let file = directory
        .path()
        .join("artifact-with-owned-path-storage.page");
    std::fs::write(&file, b"physical-bytes").expect("fixture file");
    let inspection = OfflineInspectionBudget::bounded(16, 1024)
        .expect("inspection budget")
        .with_maximum_owned_allocation_bytes(16)
        .expect("buffer-sized owned budget");

    let denial = OfflineStoreInspection::open(UntrustedOfflineMediaSet::from_root(
        directory.path(),
        closure_for_paths(&[file]),
    ))
    .budget(inspection)
    .start();
    let denial = match denial {
        Err(denial) => denial,
        Ok(_) => panic!("basis, paths, and topology must share the owned allocation budget"),
    };

    assert!(matches!(
        denial,
        OfflineMediaAcquisitionDenial::BudgetExceeded {
            dimension: OfflineMediaAcquisitionDimension::OwnedAllocationBytes,
            admitted,
            limit: 16,
        } if admitted > 16
    ));
}

fn closure_for_paths(paths: &[std::path::PathBuf]) -> OfflineMediaConsistencyBasis {
    let entries = paths.iter().map(|path| {
        let bytes = std::fs::read(path).expect("fixture bytes");
        OfflineMediaClosureEntry::new(path, bytes.len() as u64, Sha256::digest(bytes).into())
            .expect("closure entry")
    });
    OfflineMediaConsistencyBasis::content_addressed_closure("acquisition-budget", entries)
        .expect("content closure")
}
