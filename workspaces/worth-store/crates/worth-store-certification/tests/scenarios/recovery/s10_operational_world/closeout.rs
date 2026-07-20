use super::certification;
use worth_store_certification::courtroom::operational_recovery::{
    close_s10_certification, localize_s10_audit_phase_defect,
    localize_s10_closeout_join_phase_defect, localize_s10_control_selection_phase_defect,
    localize_s10_counter_phase_defect, localize_s10_formal_phase_defect,
    localize_s10_harness_phase_defect, localize_s10_observation_join_omission,
    localize_s10_runtime_record_omission, PromotionRemoteExclusionEvidence, S10CloseoutDenial,
    S10OperationalScenarioKind, S10PhaseDefectLocalization, S10PhaseDefectSuite,
    S10ScenarioProductionEvidence, S10ScenarioSuiteEvidence, ScenarioScaleProfile,
};

pub fn closeout_denial() -> S10CloseoutDenial {
    let (burning, burning_world) = certification::certify_scenario(
        S10OperationalScenarioKind::BurningPrimary,
        ScenarioScaleProfile::Ci,
    );
    let (split, split_world) = certification::certify_scenario(
        S10OperationalScenarioKind::SplitBrainPromotion,
        ScenarioScaleProfile::Ci,
    );
    let (repair, repair_world) = certification::certify_scenario(
        S10OperationalScenarioKind::AuthorityRepairRollback,
        ScenarioScaleProfile::Ci,
    );
    let defects = S10PhaseDefectSuite::join(localizations(
        &burning,
        &burning_world,
        &split,
        &split_world,
        &repair,
        &repair_world,
    ))
    .expect("one distinct controlled rejection per S10 phase");
    let ci =
        S10ScenarioSuiteEvidence::join(ScenarioScaleProfile::Ci, [burning, split, repair]).unwrap();
    let release = certification::certify_suite(ScenarioScaleProfile::Release);
    let exclusion = PromotionRemoteExclusionEvidence::from_current_promotion(
        split_world
            .current_promotion
            .as_ref()
            .expect("split-brain scenario must produce a current promotion"),
    )
    .unwrap();
    close_s10_certification(ci, release, defects, &split_world.selected, exclusion)
        .expect_err("reached yieldpoint labels cannot close S10 without crash/reopen receipts")
}

#[allow(clippy::too_many_arguments)]
fn localizations(
    burning: &worth_store_certification::courtroom::operational_recovery::S10OperationalScenarioEvidence,
    burning_world: &super::owner_world::ExecutedOwnerWorld,
    split: &worth_store_certification::courtroom::operational_recovery::S10OperationalScenarioEvidence,
    split_world: &super::owner_world::ExecutedOwnerWorld,
    repair: &worth_store_certification::courtroom::operational_recovery::S10OperationalScenarioEvidence,
    repair_world: &super::owner_world::ExecutedOwnerWorld,
) -> Vec<S10PhaseDefectLocalization> {
    let burning_production =
        S10ScenarioProductionEvidence::new(&burning_world.selected, &burning_world.truth);
    let split_production =
        S10ScenarioProductionEvidence::new(&split_world.selected, &split_world.truth);
    let repair_production =
        S10ScenarioProductionEvidence::new(&repair_world.selected, &repair_world.truth);
    let control_denial = split_world.controlled_selected_prefix_defect();
    let mut defects = vec![
        localize_s10_control_selection_phase_defect(split, &control_denial).unwrap(),
        localize_s10_observation_join_omission(burning, burning_production, 3).unwrap(),
        localize_s10_observation_join_omission(repair, repair_production, 4).unwrap(),
    ];
    for phase in 5..=9 {
        defects.push(
            localize_s10_runtime_record_omission(burning, burning_production, phase).unwrap(),
        );
    }
    defects.push(localize_s10_runtime_record_omission(repair, repair_production, 10).unwrap());
    for phase in 11..=12 {
        defects.push(localize_s10_runtime_record_omission(split, split_production, phase).unwrap());
    }
    defects.extend([
        localize_s10_runtime_record_omission(burning, burning_production, 13).unwrap(),
        localize_s10_runtime_record_omission(split, split_production, 14).unwrap(),
        localize_s10_audit_phase_defect(repair, repair_production).unwrap(),
        localize_s10_harness_phase_defect(
            burning,
            &burning.execution_matrix().controlled_defects()[0],
        )
        .unwrap(),
        localize_s10_formal_phase_defect(split).unwrap(),
        localize_s10_counter_phase_defect(repair).unwrap(),
        localize_s10_closeout_join_phase_defect(burning).unwrap(),
    ]);
    defects
}
