use std::collections::BTreeSet;

use sha2::{Digest, Sha256};
use worth_store_authority::ControlStoreFencingAuthority;
use worth_store_certification::courtroom::operational_recovery::{
    S10OperationalScenarioKind, ScenarioScaleProfile,
};
use worth_store_offline_verifier::{
    ForensicAcquisitionIntent, ForensicAcquisitionProgress, ForensicAcquisitionSession,
    OperationalTruthReport,
};
use worth_store_operations::certification_scenario::{
    certify_scenario_truth_restarts, execute_scenario_pitr_staging,
    execute_scenario_restore_staging, execute_scenario_rollback_staging, inspect_scenario_truth,
    ExactScenarioControlSelection, OwnerBackedBackupScenario, RejectedPoisonedBackupScenario,
};
use worth_store_operations::{
    AuditCompletenessReceipt, CurrentReplicaPromotion, OperationalCounterReceipt,
    OperationalOperationId, SelectedOperationalControlState,
};
use worth_store_physical_backend::{
    OfflineMediaClosureEntry, OfflineMediaConsistencyBasis, ReadOnlyOfflineMediaCapability,
};
use worth_store_physical_certification::{
    DrivenOperationalControlStore, DrivenOperationalTransition, OperationalRecoveryDriverTrace,
    OperationalRecoveryProductionDriver,
};

mod audit;
mod control_selection_defect;
mod crash_probe;
mod fresh_process_verification;
mod repair;
mod topology;
pub use crash_probe::execute_scenario_crash_probe;
use topology::ScenarioOwnerTopology;

pub struct ExecutedOwnerWorld {
    pub selected: SelectedOperationalControlState,
    pub truth: OperationalTruthReport,
    pub trace: OperationalRecoveryDriverTrace,
    pub counters: Vec<OperationalCounterReceipt>,
    pub audits: Vec<AuditCompletenessReceipt>,
    pub distinct_control_operations: usize,
    pub current_promotion: Option<CurrentReplicaPromotion>,
    pub revoked_authorization_recovery: Option<
        worth_store_certification::courtroom::operational_recovery::RevokedAuthorizationRecoveryReceipt,
    >,
    pub authority_repair_classification:
        Option<worth_store_physical_integrity::IntegrityRepairClassificationReceipt>,
    pub poisoned_backup: Option<RejectedPoisonedBackupScenario>,
    pub split_brain_rejection: Option<worth_store_replication::ReplicaPromotionRejectionReceipt>,
    pub split_brain_reconciliation:
        Option<worth_store_replication::SplitBrainReconciliationReceipt>,
    pub restarting_offline_scan: Option<worth_store_offline_verifier::RestartingOfflineScanReceipt>,
    pub authorization_race:
        Option<worth_store_operations::certification_scenario::ScenarioAuthorizationRaceReceipt>,
    pub footprint_mutation_rejection: Option<
        worth_store_operations::certification_scenario::ScenarioFootprintMutationRejectionReceipt,
    >,
    pub staging_resume:
        Option<worth_store_operations::certification_scenario::ScenarioStagingResumeReceipt>,
    pub published_readmission_recovery: Option<
        worth_store_certification::courtroom::operational_recovery::PublishedReadmissionRecoveryReceipt,
    >,
    pub retained_authority_rollback:
        Option<worth_store_operations::CompletedRetainedAuthorityRollback>,
    pub repair_source_denials:
        Option<worth_store_operations::certification_scenario::ScenarioRepairSourceDenialReceipt>,
    pub canonical_repair_dag: Option<
        worth_store_operations::certification_scenario::ScenarioCanonicalOwnerDagPermutationReceipt,
    >,
    pub repair_owner_recovery:
        Option<worth_store_operations::certification_scenario::ScenarioRepairOwnerRecoveryReceipt>,
    pub repair_cancellation_recovery: Option<
        worth_store_operations::certification_scenario::ScenarioRepairCancellationRecoveryReceipt,
    >,
    pub repair_mutant_rejections: Option<
        worth_store_operations::certification_scenario::ScenarioRepairMutantRejectionReceipt,
    >,
    media: OwnerBackedBackupScenario,
}

pub fn execute_authority_repair_rollback_world(identity: &str) -> ExecutedOwnerWorld {
    execute_scenario_world(
        S10OperationalScenarioKind::AuthorityRepairRollback,
        identity,
    )
}

pub fn execute_scenario_world(
    kind: S10OperationalScenarioKind,
    identity: &str,
) -> ExecutedOwnerWorld {
    execute_scenario_world_for_profile(kind, ScenarioScaleProfile::Smoke, identity)
}

pub fn execute_scenario_world_for_profile(
    kind: S10OperationalScenarioKind,
    profile: ScenarioScaleProfile,
    identity: &str,
) -> ExecutedOwnerWorld {
    let minimum = if kind == S10OperationalScenarioKind::AuthorityRepairRollback {
        256
    } else {
        1
    };
    let blob_count = minimum
        + match profile {
            ScenarioScaleProfile::Smoke => 0,
            ScenarioScaleProfile::Ci => 8,
            ScenarioScaleProfile::Release => 32,
        };
    let scenario = OwnerBackedBackupScenario::materialize_with_blob_count(identity, blob_count);
    execute_materialized_scenario_world(
        kind,
        identity,
        scenario,
        OperationalRecoveryProductionDriver::uninterrupted(),
        true,
    )
}

fn execute_materialized_scenario_world(
    kind: S10OperationalScenarioKind,
    identity: &str,
    scenario: OwnerBackedBackupScenario,
    driver: OperationalRecoveryProductionDriver,
    include_restart_matrix: bool,
) -> ExecutedOwnerWorld {
    let topology = ScenarioOwnerTopology::for_kind(kind);
    let control = scenario.control_store();
    let driven = DrivenOperationalControlStore::new(&control, &driver);
    let mut counters = Vec::new();

    if topology.abandoned_backup {
        let abandoned = scenario.abandon(identity, &driven);
        counters.push(abandoned.counters());
    }
    let poisoned_backup = topology.abandoned_backup.then(|| {
        let rejection = scenario.reject_poisoned_materialization(identity, &driven);
        counters.push(rejection.counters());
        rejection
    });

    let restore = scenario.execute_named(identity, "restore-source", &driven);
    counters.push(restore.counters());
    let restore = execute_scenario_restore_staging(
        &format!("{identity}/restore"),
        restore.into_restore_source(),
        &scenario.workspace_root().join("restore"),
        scenario.authority(),
        &control,
        &driven,
    );
    counters.push(OperationalCounterReceipt::from_backup_restore(&restore));
    super::recovery_publication::publish_restore(identity, restore, &scenario, &control, &driven);

    let pitr = scenario.execute_named(identity, "pitr-source", &driven);
    counters.push(pitr.counters());
    let pitr = execute_scenario_pitr_staging(
        &format!("{identity}/pitr"),
        pitr.into_restore_source(),
        &scenario.workspace_root().join("pitr"),
        &scenario.workspace_root().join("pitr-leases"),
        scenario.authority(),
        &control,
        &driven,
    );
    counters.push(OperationalCounterReceipt::from_point_in_time_recovery(
        &pitr,
    ));
    super::recovery_publication::publish_pitr(identity, pitr, &scenario, &control, &driven);

    let retained_authority_rollback = if topology.rollback {
        let rollback = scenario.execute_named(identity, "rollback-source", &driven);
        counters.push(rollback.counters());
        let rollback = execute_scenario_rollback_staging(
            &format!("{identity}/rollback"),
            rollback.into_restore_source(),
            &scenario.workspace_root().join("rollback"),
            &scenario.workspace_root().join("rollback-leases"),
            scenario.authority(),
            &control,
            &driven,
        );
        counters.push(OperationalCounterReceipt::from_rollback(&rollback));
        Some(super::recovery_publication::publish_rollback(
            identity, rollback, &scenario, &control, &driven,
        ))
    } else {
        None
    };

    let authority_repair_classification = if topology.repair {
        Some(repair::execute(
            identity,
            &scenario,
            &control,
            &driven,
            &mut counters,
        ))
    } else {
        None
    };

    let replica = topology.replica.then(|| {
        super::replication::execute_replica_lifecycle(
            identity,
            &scenario,
            &control,
            &driver,
            &driven,
            topology.old_primary_rejoin,
        )
    });
    if let Some(replica) = &replica {
        counters.extend(replica.counters);
    }
    let split_brain_rejection = replica
        .as_ref()
        .map(|replica| replica.rejected_highest_observed.clone());
    let split_brain_reconciliation = replica
        .as_ref()
        .and_then(|replica| replica.split_brain_reconciliation);
    let revoked_authorization_recovery = replica
        .as_ref()
        .map(|replica| replica.revoked_authorization_recovery);

    let restarting_offline_scan = (topology.repair && include_restart_matrix).then(|| {
        certify_scenario_truth_restarts(
            &format!("{identity}/offline-verification"),
            scenario.source_root(),
        )
    });
    let authorization_race = topology.abandoned_backup.then(|| {
        worth_store_operations::certification_scenario::certify_scenario_authorization_race(
            &format!("{identity}/authorization-race"),
        )
    });
    let footprint_mutation_rejection = topology.abandoned_backup.then(|| {
        worth_store_operations::certification_scenario::certify_scenario_footprint_mutation_rejection(
            &format!("{identity}/footprint-mutation"),
        )
    });
    let staging_resume = topology.abandoned_backup.then(|| {
        worth_store_operations::certification_scenario::certify_scenario_staging_resume(&format!(
            "{identity}/staging-resume"
        ))
    });
    let published_readmission_recovery = topology.abandoned_backup.then(|| {
        super::recovery_publication::certify_published_readmission_recovery(&format!(
            "{identity}/published-readmission-recovery"
        ))
    });
    let complete_repair_program = kind == S10OperationalScenarioKind::AuthorityRepairRollback;
    let repair_source_denials = complete_repair_program.then(|| {
        worth_store_operations::certification_scenario::certify_scenario_repair_source_denials(
            &format!("{identity}/repair-source-denials"),
        )
    });
    let canonical_repair_dag = complete_repair_program.then(
        worth_store_operations::certification_scenario::certify_scenario_canonical_owner_dag_permutation,
    );
    let repair_owner_recovery = complete_repair_program.then(|| {
        worth_store_operations::certification_scenario::certify_scenario_repair_owner_recovery(
            &format!("{identity}/repair-owner-recovery"),
        )
    });
    let repair_cancellation_recovery = complete_repair_program.then(|| {
        worth_store_operations::certification_scenario::certify_scenario_repair_cancellation_recovery(
            &format!("{identity}/repair-cancellation-recovery"),
        )
    });
    let repair_mutant_rejections = complete_repair_program.then(|| {
        worth_store_operations::certification_scenario::certify_scenario_repair_mutant_rejections(
            &format!("{identity}/repair-mutants"),
        )
    });
    let inspected = inspect_scenario_truth(
        &format!("{identity}/offline-verification"),
        scenario.source_root(),
    );
    driver.observe_completed_truth_composition(inspected.operation(), inspected.report());
    counters.push(inspected.counters());
    let truth = inspected.into_report();

    counters.push(execute_forensics(identity, &scenario, &driver));
    let selected = select_control(&scenario, &control);
    let distinct_control_operations = selected
        .durable_records()
        .iter()
        .map(|record| record.operation_id().as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let audits = audit::derive_audits(&driver, &selected);

    ExecutedOwnerWorld {
        selected,
        truth,
        trace: driver.trace(),
        counters,
        audits,
        distinct_control_operations,
        current_promotion: replica.map(|replica| replica.current),
        revoked_authorization_recovery,
        authority_repair_classification,
        poisoned_backup,
        split_brain_rejection,
        split_brain_reconciliation,
        restarting_offline_scan,
        authorization_race,
        footprint_mutation_rejection,
        staging_resume,
        published_readmission_recovery,
        retained_authority_rollback,
        repair_source_denials,
        canonical_repair_dag,
        repair_owner_recovery,
        repair_cancellation_recovery,
        repair_mutant_rejections,
        media: scenario,
    }
}

fn execute_forensics(
    identity: &str,
    scenario: &OwnerBackedBackupScenario,
    driver: &OperationalRecoveryProductionDriver,
) -> OperationalCounterReceipt {
    let paths = std::fs::read_dir(scenario.source_root())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    let closure = paths
        .iter()
        .map(|path| {
            let bytes = std::fs::read(path).unwrap();
            OfflineMediaClosureEntry::new(path, bytes.len() as u64, Sha256::digest(bytes).into())
                .unwrap()
        })
        .collect::<Vec<_>>();
    let basis = OfflineMediaConsistencyBasis::content_addressed_closure(
        format!("{identity}/forensic-closure"),
        closure,
    )
    .unwrap();
    let media = ReadOnlyOfflineMediaCapability::open(paths, basis).unwrap();
    let plan = ForensicAcquisitionIntent::new(
        scenario.workspace_root().join("forensics"),
        "s10-certification-observer",
        "read-only-os-handle",
        "deterministic-tick",
        1,
        64 * 1024,
    )
    .unwrap()
    .plan(&media)
    .unwrap();
    let mut session = ForensicAcquisitionSession::open(plan, media).unwrap();
    let operation = OperationalOperationId::new(format!("{identity}/forensics")).unwrap();
    while let ForensicAcquisitionProgress::SourceRecorded { .. } = completed(
        driver
            .forensic_acquire_next(&operation, &mut session)
            .unwrap(),
    ) {}
    let (_, counters) = completed(driver.forensic_finish(session, 2).unwrap());
    OperationalCounterReceipt::from_forensic_acquisition(&operation, counters)
}

fn select_control(
    scenario: &OwnerBackedBackupScenario,
    control: &worth_store_operations::OperationalControlStore,
) -> SelectedOperationalControlState {
    let provider = ExactScenarioControlSelection::current(scenario.authority(), control);
    let fencing = ControlStoreFencingAuthority::for_current_store(scenario.authority(), &provider);
    let worth_store_operations::ControlStoreTrustPosture::Selected(selected) =
        control.inspect_generations(&fencing)
    else {
        panic!("one current control generation must be selected");
    };
    selected
}

fn completed<T: std::fmt::Debug>(transition: DrivenOperationalTransition<T>) -> T {
    match transition {
        DrivenOperationalTransition::Completed(value) => value,
        other => panic!("uninterrupted driver returned {other:?}"),
    }
}
