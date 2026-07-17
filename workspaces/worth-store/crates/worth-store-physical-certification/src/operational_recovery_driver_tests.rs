use sha2::{Digest, Sha256};
use worth_store_offline_verifier::{ForensicAcquisitionIntent, ForensicAcquisitionSession};
use worth_store_physical_backend::{
    OfflineMediaClosureEntry, OfflineMediaConsistencyBasis, ReadOnlyOfflineMediaCapability,
};

use crate::{
    DrivenOperationalTransition, OperationalRecoveryProductionDriver, OperationalRecoveryYieldpoint,
};

#[test]
fn interrupted_after_forensic_record_reopens_real_production_state() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("source.bin");
    let target = directory.path().join("forensic");
    let bytes = vec![7; 4096];
    std::fs::write(&source, &bytes).unwrap();
    let basis = OfflineMediaConsistencyBasis::content_addressed_closure(
        "operational-driver",
        [OfflineMediaClosureEntry::new(
            source.clone(),
            bytes.len() as u64,
            Sha256::digest(&bytes).into(),
        )
        .unwrap()],
    )
    .unwrap();
    let media = || ReadOnlyOfflineMediaCapability::open([source.clone()], basis.clone()).unwrap();
    let plan = ForensicAcquisitionIntent::new(
        &target,
        "observer",
        "read-only-handle",
        "monotonic-test-clock",
        1,
        256,
    )
    .unwrap()
    .plan(&media())
    .unwrap();
    let mut session = ForensicAcquisitionSession::open(plan.clone(), media()).unwrap();
    let mut driver = OperationalRecoveryProductionDriver::pause_once_at(
        OperationalRecoveryYieldpoint::AfterForensicSourceRecord,
    );

    assert!(matches!(
        driver
            .forensic_acquire_next(
                &worth_store_operations::OperationalOperationId::new("forensic-driver").unwrap(),
                &mut session,
            )
            .unwrap(),
        DrivenOperationalTransition::InterruptedAfter(_)
    ));
    drop(session);

    let (_, counters) = ForensicAcquisitionSession::open(plan, media())
        .unwrap()
        .acquire(2)
        .unwrap();
    assert_eq!(counters.recovered_source_records(), 1);
}

#[test]
fn existing_harness_contract_registry_binds_s10_yieldpoints() {
    let contracts = crate::AdmittedDriverContractSet::developer_smoke().unwrap();
    for point in OperationalRecoveryYieldpoint::ALL {
        assert!(
            contracts.binds_yieldpoint(point.token()),
            "missing {}",
            point.token()
        );
    }
}
