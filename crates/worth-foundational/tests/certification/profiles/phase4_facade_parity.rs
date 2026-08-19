use worth_foundational::profiles_api::FoundationalProfilePublicLane;
use worth_foundational::{
    foundational_profile_milestone10_readiness_report, profiles,
    profiles_api::{
        common_path, lower_lane::materialization as lower_materialization,
        stronger_lane::readiness as stronger_readiness,
    },
    FoundationalDescriptiveSurface, FoundationalObservationDisposition,
    FoundationalProfileMilestone10PhaseGate,
};

#[test]
fn lower_lane_materialization_matches_root_and_common_paths() {
    let _root: fn(
        &worth_foundational::MaterializedFoundationalProfileSet,
        &[FoundationalDescriptiveSurface],
        FoundationalObservationDisposition,
    ) -> Result<
        worth_foundational::FoundationalProfileMaterializationPlan<
            worth_foundational::ProofBearingArtifactTarget,
        >,
        worth_foundational::FoundationalMaterializationPlanningDenial,
    > = worth_foundational::plan_selected_foundational_profile_materialization_with_disposition;
    let _lower: fn(
        &worth_foundational::MaterializedFoundationalProfileSet,
        &[FoundationalDescriptiveSurface],
        FoundationalObservationDisposition,
    ) -> Result<
        worth_foundational::FoundationalProfileMaterializationPlan<
            worth_foundational::ProofBearingArtifactTarget,
        >,
        worth_foundational::FoundationalMaterializationPlanningDenial,
    > = lower_materialization::plan_selected_foundational_profile_materialization_with_disposition;

    let _common = common_path::profiles().materialization();
    let _grouped = profiles().materialization();
    assert_eq!(
        FoundationalObservationDisposition::Inactive,
        FoundationalObservationDisposition::Inactive
    );
}

#[test]
fn readiness_and_facade_lanes_remain_distinct() {
    let report = foundational_profile_milestone10_readiness_report();
    assert!(report.passes_readiness_checklist());
    assert_eq!(
        report.phase_gates(),
        &[
            FoundationalProfileMilestone10PhaseGate::BoundaryFreeze,
            FoundationalProfileMilestone10PhaseGate::ObjectiveAndActivationVocabulary,
            FoundationalProfileMilestone10PhaseGate::ObservationDispositionAndWorkDisclosure,
            FoundationalProfileMilestone10PhaseGate::FoundationalFacade,
            FoundationalProfileMilestone10PhaseGate::SignalPolicyCompilerHandoff,
        ]
    );
    assert!(report
        .certified_surfaces()
        .contains(&"observation-activation-profile"));
    assert_eq!(
        report.store_handoff(),
        "Store owns durability; Foundational owns only shared profile and work meaning"
    );

    assert_eq!(
        worth_foundational::profiles_api::profile_public_surface_inventory()
            .iter()
            .filter(|entry| entry.lane() == FoundationalProfilePublicLane::LowerLane)
            .count(),
        6
    );
    assert!(
        worth_foundational::profiles_api::profile_public_surface_inventory()
            .iter()
            .find(|entry| entry.path().ends_with("lower_lane::materialization"))
            .expect("lower-lane materialization inventory row")
            .teaches()
            .contains("observation-disposition")
    );
    let stronger = stronger_readiness::foundational_profile_milestone3_readiness_report();
    assert!(stronger.passes_readiness_checklist());
}
