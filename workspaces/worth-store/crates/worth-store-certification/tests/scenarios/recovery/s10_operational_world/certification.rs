use std::path::Path;

use worth_store_certification::courtroom::operational_recovery::{
    certify_s10_operational_scenario, execute_s10_structural_preflight,
    S10OperationalScenarioEvidence, S10OperationalScenarioKind, S10OperationalScenarioProgram,
    S10ScenarioProductionEvidence, S10ScenarioSuiteEvidence, ScenarioScaleProfile,
};

use super::{owner_world, physical_evidence, qos};

pub fn certify_authority_repair_rollback_smoke() -> S10OperationalScenarioEvidence {
    certify_scenario(
        S10OperationalScenarioKind::AuthorityRepairRollback,
        ScenarioScaleProfile::Smoke,
    )
    .0
}

pub fn certify_scenario(
    kind: S10OperationalScenarioKind,
    profile: ScenarioScaleProfile,
) -> (
    S10OperationalScenarioEvidence,
    owner_world::ExecutedOwnerWorld,
) {
    let identity = format!("s10-certifier/{}/{}", kind.token(), profile.token());
    let world = owner_world::execute_scenario_world_for_profile(kind, profile, &identity);
    let preflight =
        execute_s10_structural_preflight(forge_root()).expect("S10 structural preflight");
    let crash_coverage =
        super::crash_coverage::scenario_crash_coverage(kind, &identity, &world.trace);
    let execution =
        physical_evidence::execution_matrix(profile, kind, world.trace.clone(), crash_coverage);
    let production = S10ScenarioProductionEvidence::new(&world.selected, &world.truth);
    certify_s10_operational_scenario(
        S10OperationalScenarioProgram::new(kind, profile),
        &preflight,
        production,
        hostile_program(kind, &world, &execution, preflight),
        execution,
        qos::operational_qos(),
        world.counters.clone(),
        world.audits.clone(),
    )
    .map(|evidence| (evidence, world))
    .expect("ordinary owner world must reach the public S10 certifier")
}

fn hostile_program(
    kind: S10OperationalScenarioKind,
    world: &owner_world::ExecutedOwnerWorld,
    execution: &worth_store_certification::courtroom::operational_recovery::S10ScenarioExecutionMatrix,
    preflight: worth_store_certification::courtroom::operational_recovery::S10StructuralPreflightEvidence,
) -> worth_store_certification::courtroom::operational_recovery::S10HostileProgramEvidence {
    use worth_store_certification::courtroom::operational_recovery::S10HostileProgramEvidence;

    match kind {
        S10OperationalScenarioKind::BurningPrimary => S10HostileProgramEvidence::burning_primary(
            world
                .poisoned_backup
                .as_ref()
                .expect("burning-primary poison probe"),
            execution.crash_reopen_coverage(),
            &world.controlled_selected_prefix_defect(),
            world.fresh_process_destroyed_primary_verification(),
            world
                .authorization_race
                .expect("burning-primary authorization race"),
            world
                .footprint_mutation_rejection
                .expect("burning-primary footprint mutant"),
            world
                .staging_resume
                .expect("burning-primary staging recovery"),
            world
                .published_readmission_recovery
                .expect("burning-primary publication recovery"),
            preflight,
        ),
        S10OperationalScenarioKind::SplitBrainPromotion => S10HostileProgramEvidence::split_brain(
            world
                .split_brain_rejection
                .as_ref()
                .expect("split-brain candidate rejection"),
            world
                .current_promotion
                .as_ref()
                .expect("split-brain admitted promotion"),
            world
                .split_brain_reconciliation
                .expect("split-brain reconciliation receipt"),
            world.divergent_control_generation_selection(),
            world
                .revoked_authorization_recovery
                .expect("split-brain revoked authorization recovery"),
        ),
        S10OperationalScenarioKind::AuthorityRepairRollback => {
            S10HostileProgramEvidence::authority_repair_complete(
                world
                    .authority_repair_classification
                    .expect("authority-repair production classification"),
                world
                    .restarting_offline_scan
                    .expect("repair restarting offline scan"),
                world
                    .repair_source_denials
                    .expect("repair source authority denials"),
                world.canonical_repair_dag.expect("canonical repair DAG"),
                world
                    .repair_owner_recovery
                    .expect("every repair owner crash cut"),
                world
                    .repair_cancellation_recovery
                    .expect("repair cancellation and recovery"),
                world
                    .repair_mutant_rejections
                    .expect("repair mutant rejections"),
                world
                    .retained_authority_rollback
                    .as_ref()
                    .expect("retained-authority rollback closure"),
                preflight,
            )
        }
    }
    .expect("ordinary hostile program evidence")
}

pub fn certify_suite(profile: ScenarioScaleProfile) -> S10ScenarioSuiteEvidence {
    S10ScenarioSuiteEvidence::join(
        profile,
        [
            S10OperationalScenarioKind::BurningPrimary,
            S10OperationalScenarioKind::SplitBrainPromotion,
            S10OperationalScenarioKind::AuthorityRepairRollback,
        ]
        .map(|kind| certify_scenario(kind, profile).0),
    )
    .expect("all three ordinary scenario programs form the profile suite")
}

fn forge_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("worth-store certification lives under the Forge root")
}
