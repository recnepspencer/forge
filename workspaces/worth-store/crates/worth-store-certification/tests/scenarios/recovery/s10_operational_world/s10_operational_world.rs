mod certification;
mod closeout;
mod crash_coverage;
mod owner_world;
mod physical_evidence;
mod qos;
mod recovery_publication;
mod replication;

#[test]
fn fresh_process_destroyed_primary_observer_child() {
    use worth_store_operations::certification_scenario::inspect_scenario_truth;
    use worth_store_physical_certification::{
        write_offline_truth_observation_from_environment, OFFLINE_TRUTH_TARGET_ENV,
    };

    let Some(target) = std::env::var_os(OFFLINE_TRUTH_TARGET_ENV) else {
        return;
    };
    let root = std::path::PathBuf::from(target)
        .parent()
        .expect("destroyed primary parent")
        .to_path_buf();
    let truth = inspect_scenario_truth("fresh-process/destroyed-primary", &root);
    assert!(write_offline_truth_observation_from_environment(truth.report()).unwrap());
}

#[test]
fn destroyed_primary_is_classified_by_an_independent_process() {
    use worth_store_certification::courtroom::operational_recovery::S10OperationalScenarioKind;

    let world = owner_world::execute_scenario_world(
        S10OperationalScenarioKind::BurningPrimary,
        "s10-operational-world/fresh-process-destroyed-primary",
    );
    let evidence = world.fresh_process_destroyed_primary_verification();

    assert_ne!(evidence.live_digest(), evidence.damaged_digest());
    assert_ne!(evidence.source_inspection_identity(), [0; 32]);
    assert_ne!(evidence.truth_evidence_identity(), [0; 32]);
    assert_ne!(evidence.observer_process_id(), std::process::id());
    assert_ne!(evidence.evidence_identity(), [0; 32]);
}

#[test]
fn an_unchanged_primary_cannot_claim_destroyed_primary_evidence() {
    use std::process::Command;
    use worth_store_physical_certification::{
        FreshProcessOfflineTruthBaseline, FreshProcessOfflineTruthDenial,
        FreshProcessOfflineTruthRunner,
    };

    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("page.media");
    std::fs::write(&target, b"still-live").unwrap();
    let baseline = FreshProcessOfflineTruthBaseline::capture(&target).unwrap();
    let mut observer = Command::new(std::env::current_exe().unwrap());

    assert_eq!(
        FreshProcessOfflineTruthRunner::new(directory.path().join("evidence"))
            .certify_destroyed_primary(&baseline, &mut observer)
            .unwrap_err(),
        FreshProcessOfflineTruthDenial::TargetNotDamaged
    );
}

#[test]
fn exact_plan_single_use_authorization_has_one_durable_race_winner() {
    let receipt =
        worth_store_operations::certification_scenario::certify_scenario_authorization_race(
            "s10-operational-world/authorization-race",
        );

    assert_eq!(receipt.winner_count(), 1);
    assert_eq!(receipt.consumed_replay_denials(), 1);
    assert_ne!(receipt.plan_fingerprint(), [0; 32]);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn post_verification_rejects_a_real_write_outside_the_owner_footprint() {
    let receipt =
        worth_store_operations::certification_scenario::certify_scenario_footprint_mutation_rejection(
            "s10-operational-world/footprint-mutation",
        );

    assert_ne!(receipt.plan_fingerprint(), [0; 32]);
    assert_ne!(receipt.declared_content_fingerprint(), [0; 32]);
    assert_ne!(receipt.injected_content_digest(), [0; 32]);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn every_semantic_staging_boundary_resumes_from_its_durable_handle() {
    let receipt = worth_store_operations::certification_scenario::certify_scenario_staging_resume(
        "s10-operational-world/staging-resume",
    );

    assert_eq!(receipt.recovered_boundaries(), 5);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn rejected_publication_is_terminal_and_a_fresh_retry_reopens_then_readmits() {
    let receipt = recovery_publication::certify_published_readmission_recovery(
        "s10-operational-world/published-readmission-recovery",
    );

    assert_ne!(receipt.rejected_publication_identity(), [0; 32]);
    assert_ne!(receipt.readmitted_publication_identity(), [0; 32]);
    assert_ne!(
        receipt.rejected_publication_identity(),
        receipt.readmitted_publication_identity()
    );
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn repair_sources_deny_stale_authority_and_cross_scope_content() {
    let receipt =
        worth_store_operations::certification_scenario::certify_scenario_repair_source_denials(
            "s10-operational-world/repair-source-denials",
        );

    assert_ne!(receipt.stale_authority_denial_identity(), [0; 32]);
    assert_ne!(receipt.cross_scope_denial_identity(), [0; 32]);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn repair_owner_dag_is_identical_under_caller_permutation() {
    let receipt = worth_store_operations::certification_scenario::certify_scenario_canonical_owner_dag_permutation();

    assert_eq!(receipt.node_count(), 5);
    assert_eq!(receipt.edge_count(), 4);
    assert_ne!(receipt.plan_fingerprint(), [0; 32]);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn every_repair_owner_boundary_recovers_without_replaying_durable_effects() {
    let receipt =
        worth_store_operations::certification_scenario::certify_scenario_repair_owner_recovery(
            "s10-operational-world/repair-owner-recovery",
        );

    assert_eq!(receipt.owner_nodes(), 5);
    assert_eq!(receipt.recovered_cuts(), 15);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn repair_recovery_preserves_safe_actions_across_denial_revocation_and_indeterminacy() {
    let receipt = worth_store_operations::certification_scenario::certify_scenario_repair_cancellation_recovery(
        "s10-operational-world/repair-cancellation-recovery",
    );

    assert_ne!(receipt.scheduler_cancellation_identity(), [0; 32]);
    assert_ne!(receipt.revocation_cancellation_identity(), [0; 32]);
    assert_ne!(receipt.backend_resume_identity(), [0; 32]);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn repair_mutants_cannot_escape_owner_footprints_or_omit_receipts() {
    let receipt =
        worth_store_operations::certification_scenario::certify_scenario_repair_mutant_rejections(
            "s10-operational-world/repair-mutants",
        );

    assert_ne!(receipt.footprint_rejection_identity(), [0; 32]);
    assert_ne!(receipt.omitted_receipt_rejection_identity(), [0; 32]);
    assert_ne!(receipt.evidence_identity(), [0; 32]);
}

#[test]
fn one_control_history_binds_real_owner_media_truth_audit_and_counters() {
    let world = owner_world::execute_authority_repair_rollback_world(
        "s10-operational-world/authority-repair-rollback",
    );

    assert!(world.selected.history_summary().record_count() > 0);
    assert_eq!(
        world.selected.history_summary().record_count() as usize,
        world.trace.control_artifact_identities().len()
    );
    assert!(!world.truth.regions().is_empty());
    assert_eq!(
        world.counters.len(),
        world.trace.operation_identities().len()
    );
    assert_eq!(world.audits.len(), world.distinct_control_operations);
    assert!(
        world
            .authority_repair_classification
            .unwrap()
            .classified_regions()
            >= 256
    );
}

#[test]
fn split_brain_world_reconciles_only_after_independent_survivors_and_lease_expiry() {
    let world = owner_world::execute_scenario_world(
        worth_store_certification::courtroom::operational_recovery::S10OperationalScenarioKind::SplitBrainPromotion,
        "s10-operational-world/split-brain-reconciliation",
    );
    let receipt = world.split_brain_reconciliation.unwrap();
    let control_selection = world.divergent_control_generation_selection();

    assert_eq!(receipt.independent_survivors(), 2);
    assert_eq!(receipt.old_primary_excluded_at_tick(), 50);
    assert_ne!(receipt.receipt_identity(), [0; 32]);
    assert!(control_selection.selected_generation() > 1);
    assert!(control_selection.rejected_generation() > 1);
    assert_ne!(control_selection.receipt_identity(), [0; 32]);
    let authorization = world.revoked_authorization_recovery.unwrap();
    assert_ne!(authorization.promoted_receipt_identity(), [0; 32]);
    assert_ne!(authorization.evidence_identity(), [0; 32]);
}

#[test]
fn a_scenario_cannot_be_relabelled_as_another_owner_topology() {
    use worth_store_certification::courtroom::operational_recovery::{
        certify_s10_operational_scenario, require_s10_structural_preflight,
        S10HostileProgramEvidence, S10OperationalScenarioKind, S10OperationalScenarioProgram,
        S10ScenarioCertificationDenial, S10ScenarioProductionEvidence, ScenarioScaleProfile,
    };

    let identity = "s10-operational-world/relabel-rejection";
    let world =
        owner_world::execute_scenario_world(S10OperationalScenarioKind::BurningPrimary, identity);
    let crash_cuts = crash_coverage::scenario_crash_coverage(
        S10OperationalScenarioKind::BurningPrimary,
        identity,
        &world.trace,
    );
    let execution = physical_evidence::execution_matrix(
        ScenarioScaleProfile::Smoke,
        S10OperationalScenarioKind::SplitBrainPromotion,
        world.trace.clone(),
        [],
    );
    let preflight = require_s10_structural_preflight().unwrap();
    let denial = certify_s10_operational_scenario(
        S10OperationalScenarioProgram::new(
            S10OperationalScenarioKind::SplitBrainPromotion,
            ScenarioScaleProfile::Smoke,
        ),
        &preflight,
        S10ScenarioProductionEvidence::new(&world.selected, &world.truth),
        S10HostileProgramEvidence::burning_primary(
            world.poisoned_backup.as_ref().unwrap(),
            &crash_cuts,
            &world.controlled_selected_prefix_defect(),
            world.fresh_process_destroyed_primary_verification(),
            world.authorization_race.unwrap(),
            world.footprint_mutation_rejection.unwrap(),
            world.staging_resume.unwrap(),
            world.published_readmission_recovery.unwrap(),
            preflight,
        )
        .unwrap(),
        execution,
        qos::operational_qos(),
        world.counters.clone(),
        world.audits.clone(),
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        S10ScenarioCertificationDenial::ScenarioOwnerTopologyMismatch { .. }
    ));
}

#[test]
fn authority_repair_rollback_world_reaches_the_public_s10_certifier() {
    let evidence = certification::certify_authority_repair_rollback_smoke();

    assert_eq!(evidence.phase_invocations().len(), 18);
    assert_ne!(evidence.evidence_identity(), [0; 32]);
    assert_eq!(evidence.hostile_program().missing_requirement(), None);
}

#[test]
fn a_small_real_repair_receipt_cannot_claim_the_hundreds_region_program() {
    use worth_store_certification::courtroom::operational_recovery::{
        S10HostileProgramDenial, S10HostileProgramEvidence, S10OperationalScenarioKind,
    };

    let world = owner_world::execute_scenario_world(
        S10OperationalScenarioKind::SplitBrainPromotion,
        "s10-operational-world/small-repair-hostile-denial",
    );
    assert_eq!(
        S10HostileProgramEvidence::authority_repair(
            world.authority_repair_classification.unwrap(),
            world.restarting_offline_scan.unwrap(),
        )
        .unwrap_err(),
        S10HostileProgramDenial::RepairBreadthBelowHundreds
    );
}

#[test]
fn replica_scenario_evidence_exposes_its_first_unproven_hostile_requirement() {
    use worth_store_certification::courtroom::operational_recovery::{
        S10OperationalScenarioKind, ScenarioScaleProfile,
    };

    for (kind, first_unproven_requirement) in [
        (S10OperationalScenarioKind::BurningPrimary, None),
        (S10OperationalScenarioKind::SplitBrainPromotion, None),
    ] {
        let (evidence, _) = certification::certify_scenario(kind, ScenarioScaleProfile::Smoke);
        assert!(evidence
            .phase_invocations()
            .iter()
            .any(|invocation| invocation.phase().number() == 14));
        assert_eq!(
            evidence.hostile_program().missing_requirement(),
            first_unproven_requirement
        );
    }
}

#[test]
fn ci_and_release_profiles_preserve_the_scenario_topology() {
    use worth_store_certification::courtroom::operational_recovery::{
        S10OperationalScenarioKind, ScenarioScaleProfile,
    };

    let (ci, _) = certification::certify_scenario(
        S10OperationalScenarioKind::AuthorityRepairRollback,
        ScenarioScaleProfile::Ci,
    );
    let (release, _) = certification::certify_scenario(
        S10OperationalScenarioKind::AuthorityRepairRollback,
        ScenarioScaleProfile::Release,
    );
    assert_eq!(
        ci.phase_invocations().len(),
        release.phase_invocations().len()
    );
    assert!(release.scale().store_bytes() > ci.scale().store_bytes());
    assert!(release.scale().blob_bytes() > ci.scale().blob_bytes());
}

#[test]
fn ci_and_release_suites_join_all_three_scenarios() {
    use worth_store_certification::courtroom::operational_recovery::ScenarioScaleProfile;

    let ci = certification::certify_suite(ScenarioScaleProfile::Ci);
    let release = certification::certify_suite(ScenarioScaleProfile::Release);
    assert_eq!(ci.scenarios().count(), 3);
    assert_eq!(release.scenarios().count(), 3);
    assert_ne!(ci.suite_identity(), release.suite_identity());
}

#[test]
fn reached_yieldpoint_labels_cannot_close_s10_without_fresh_process_crash_evidence() {
    use worth_store_certification::courtroom::operational_recovery::{
        S10CloseoutDenial, S10OperationalScenarioKind, ScenarioScaleProfile,
    };
    use worth_store_physical_certification::{
        OperationalRecoveryControlTransitionKind as Control, OperationalRecoveryYieldpoint as Point,
    };

    assert_eq!(
        closeout::closeout_denial(),
        S10CloseoutDenial::MissingFreshProcessCrashCoverage {
            profile: ScenarioScaleProfile::Ci,
            scenario: S10OperationalScenarioKind::BurningPrimary,
            yieldpoint: Point::BeforeDurableControlTransition(Control::BackupSourceLease),
        }
    );
}
