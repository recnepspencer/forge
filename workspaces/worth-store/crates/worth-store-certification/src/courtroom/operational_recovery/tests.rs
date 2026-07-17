use super::{
    S10OperationalScenarioKind, S10OperationalScenarioProgram, S10Phase, ScenarioScaleEvidence,
    ScenarioScaleProfile, ScenarioWorkloadDimensions,
};

#[test]
fn static_phase_topology_meets_the_two_scenario_rule() {
    let programs = [
        S10OperationalScenarioProgram::new(
            S10OperationalScenarioKind::BurningPrimary,
            ScenarioScaleProfile::Ci,
        ),
        S10OperationalScenarioProgram::new(
            S10OperationalScenarioKind::SplitBrainPromotion,
            ScenarioScaleProfile::Ci,
        ),
        S10OperationalScenarioProgram::new(
            S10OperationalScenarioKind::AuthorityRepairRollback,
            ScenarioScaleProfile::Ci,
        ),
    ];
    for phase in S10Phase::all() {
        assert!(
            programs
                .iter()
                .filter(|program| program.covers(phase))
                .count()
                >= 2
        );
    }
}

#[test]
fn release_scale_requires_real_large_media_and_independent_dimensions() {
    let dimensions = ScenarioWorkloadDimensions::new(
        8 * 1024 * 1024,
        2 * 1024 * 1024 * 1024,
        1024 * 1024,
        4096,
        32,
        4,
    );
    let admitted =
        ScenarioScaleEvidence::admit(ScenarioScaleProfile::Release, dimensions, 1024 * 1024, 100)
            .unwrap();
    assert_eq!(admitted.dimensions(), dimensions);
}
