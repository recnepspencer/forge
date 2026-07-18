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
    let driver = OperationalRecoveryProductionDriver::pause_once_at(
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

#[test]
fn external_process_death_and_independent_reopen_mint_a_real_control_cut() {
    use std::process::Command;

    use worth_store_operations::certification_scenario::{
        reopen_owner_backed_control_store_at, OwnerBackedBackupScenario,
    };

    use crate::{
        admit_current_process_probe, write_reopen_observation_from_environment,
        DrivenOperationalControlStore, OperationalRecoveryControlTransitionKind as Control,
        OperationalRecoveryFreshProcessRunner, OperationalRecoveryProcessCrashConfig, ProcessRole,
        PROCESS_CRASH_ROLE_ENV,
    };

    const ROOT_ENV: &str = "WORTH_STORE_S10_PROCESS_CRASH_MEDIA_ROOT";
    const CASE: &str = "s10-real-process-crash-cut";
    if let Some(root) = std::env::var_os(ROOT_ENV).map(std::path::PathBuf::from) {
        if std::env::var(PROCESS_CRASH_ROLE_ENV).ok().as_deref() == Some("reopen") {
            let admission = admit_current_process_probe(ProcessRole::RecoveredRuntime).unwrap();
            let control = reopen_owner_backed_control_store_at(&root);
            assert!(write_reopen_observation_from_environment(&admission, &control).unwrap());
            return;
        }
        let config = OperationalRecoveryProcessCrashConfig::from_environment()
            .unwrap()
            .expect("cut child configuration");
        let scenario = OwnerBackedBackupScenario::materialize_at(CASE, 1, &root);
        let control = scenario.control_store();
        let driver = OperationalRecoveryProductionDriver::crash_once_at(config);
        let driven = DrivenOperationalControlStore::new(&control, &driver);
        let _ = scenario.execute(CASE, &driven);
        panic!("the cut child must die at its configured durable yieldpoint");
    }

    let uninterrupted = OwnerBackedBackupScenario::materialize(CASE);
    let control = uninterrupted.control_store();
    let driver = OperationalRecoveryProductionDriver::uninterrupted();
    let driven = DrivenOperationalControlStore::new(&control, &driver);
    let _ = uninterrupted.execute(CASE, &driven);
    let trace = driver.trace();

    let directory = tempfile::tempdir().unwrap();
    let media_root = directory.path().join("media");
    let evidence_root = directory.path().join("evidence");
    let executable = std::env::current_exe().unwrap();
    let exact = "operational_recovery_driver_tests::external_process_death_and_independent_reopen_mint_a_real_control_cut";
    let mut cut = Command::new(&executable);
    cut.arg("--exact")
        .arg(exact)
        .arg("--nocapture")
        .env(ROOT_ENV, &media_root);
    let mut reopen = Command::new(&executable);
    reopen
        .arg("--exact")
        .arg(exact)
        .arg("--nocapture")
        .env(ROOT_ENV, &media_root);
    let point =
        OperationalRecoveryYieldpoint::AfterDurableControlTransition(Control::BackupSourceLease);
    let evidence = OperationalRecoveryFreshProcessRunner::new(evidence_root)
        .certify_control_cut_with_process_evidence(
            &media_root,
            CASE,
            &mut cut,
            &mut reopen,
            point,
            &trace,
        )
        .unwrap();

    assert_eq!(evidence.crash_cut().yieldpoint(), point);
    assert!(!evidence.crash_cut().operation_identities().is_empty());
    assert_ne!(evidence.crash_cut().evidence_identity(), [0; 32]);
    assert_ne!(
        evidence.cut_process().process.process_id,
        evidence.reopen_process().process.process_id
    );
    assert_ne!(
        evidence.cut_process().process.runtime_identity,
        evidence.reopen_process().process.runtime_identity
    );
    assert_eq!(
        evidence.cut_process().process.executable_identity,
        evidence.reopen_process().process.executable_identity
    );
    assert!(matches!(
        evidence.cut_process().termination,
        crate::ProcessTermination::ParentKill { .. }
    ));
    assert!(matches!(
        evidence.reopen_process().termination,
        crate::ProcessTermination::GracefulExit { code: Some(0) }
    ));
}
