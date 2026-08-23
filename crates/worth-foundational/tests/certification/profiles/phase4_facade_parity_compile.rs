use worth_foundational::{
    profiles_api::lower_lane::materialization, FoundationalDescriptiveSurface,
    FoundationalMaterializationPlanningDenial, FoundationalObservationDisposition,
    FoundationalProfileMaterializationPlan, MaterializedFoundationalProfileSet,
    ProofBearingArtifactTarget,
};

#[test]
fn lower_lane_materialization_signature_is_compile_visible() {
    let _signature: fn(
        &MaterializedFoundationalProfileSet,
        &[FoundationalDescriptiveSurface],
        FoundationalObservationDisposition,
    ) -> Result<
        FoundationalProfileMaterializationPlan<ProofBearingArtifactTarget>,
        FoundationalMaterializationPlanningDenial,
    > = materialization::plan_selected_foundational_profile_materialization_with_disposition;
}
