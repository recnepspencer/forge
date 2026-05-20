use super::super::{
    plan_foundational_profile_materialization,
    plan_foundational_profile_materialization_with_elision,
    plan_selected_foundational_profile_materialization, BoundaryArtifactTarget,
    FoundationalDescriptiveElisionProfile, FoundationalDescriptiveSurface,
    FoundationalMaterializationPlanningDenial, FoundationalProfileMaterializationPlan,
    ProofBearingArtifactTarget, SupportArtifactTarget,
};
use super::attachment::{
    MaterializedBoundaryArtifactStep, MaterializedProofBearingArtifactStep,
    MaterializedSupportArtifactStep,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalProfileMaterializationFrontDoor;

impl FoundationalProfileMaterializationFrontDoor {
    pub fn for_boundary_artifact<'a, T>(
        self,
        artifact: &'a super::super::BoundaryProfiledArtifact<T>,
    ) -> BoundaryArtifactMaterializationFrontDoor<'a, T> {
        BoundaryArtifactMaterializationFrontDoor(MaterializedBoundaryArtifactStep::new(artifact))
    }

    pub fn for_support_artifact<'a, T>(
        self,
        artifact: &'a super::super::SupportProfiledArtifact<T>,
    ) -> SupportArtifactMaterializationFrontDoor<'a, T> {
        SupportArtifactMaterializationFrontDoor(MaterializedSupportArtifactStep::new(artifact))
    }

    pub fn for_proof_bearing_artifact<'a, T>(
        self,
        artifact: &'a super::super::ProofBearingProfiledArtifact<T>,
    ) -> ProofBearingArtifactMaterializationFrontDoor<'a, T> {
        ProofBearingArtifactMaterializationFrontDoor(MaterializedProofBearingArtifactStep::new(
            artifact,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundaryArtifactMaterializationFrontDoor<'a, T>(MaterializedBoundaryArtifactStep<'a, T>);

impl<'a, T> BoundaryArtifactMaterializationFrontDoor<'a, T> {
    pub fn full_fidelity(self) -> FoundationalProfileMaterializationPlan<BoundaryArtifactTarget> {
        plan_foundational_profile_materialization::<BoundaryArtifactTarget>(self.0.profile())
    }

    pub fn operational_summary(
        self,
    ) -> FoundationalProfileMaterializationPlan<BoundaryArtifactTarget> {
        plan_foundational_profile_materialization_with_elision::<BoundaryArtifactTarget>(
            self.0.profile(),
            FoundationalDescriptiveElisionProfile::OperationalSummary,
        )
    }

    pub fn selected(
        self,
        selected: &[FoundationalDescriptiveSurface],
    ) -> Result<
        FoundationalProfileMaterializationPlan<BoundaryArtifactTarget>,
        FoundationalMaterializationPlanningDenial,
    > {
        plan_selected_foundational_profile_materialization::<BoundaryArtifactTarget>(
            self.0.profile(),
            selected,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SupportArtifactMaterializationFrontDoor<'a, T>(MaterializedSupportArtifactStep<'a, T>);

impl<'a, T> SupportArtifactMaterializationFrontDoor<'a, T> {
    pub fn full_fidelity(self) -> FoundationalProfileMaterializationPlan<SupportArtifactTarget> {
        plan_foundational_profile_materialization::<SupportArtifactTarget>(self.0.profile())
    }

    pub fn operational_summary(
        self,
    ) -> FoundationalProfileMaterializationPlan<SupportArtifactTarget> {
        plan_foundational_profile_materialization_with_elision::<SupportArtifactTarget>(
            self.0.profile(),
            FoundationalDescriptiveElisionProfile::OperationalSummary,
        )
    }

    pub fn selected(
        self,
        selected: &[FoundationalDescriptiveSurface],
    ) -> Result<
        FoundationalProfileMaterializationPlan<SupportArtifactTarget>,
        FoundationalMaterializationPlanningDenial,
    > {
        plan_selected_foundational_profile_materialization::<SupportArtifactTarget>(
            self.0.profile(),
            selected,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProofBearingArtifactMaterializationFrontDoor<'a, T>(
    MaterializedProofBearingArtifactStep<'a, T>,
);

impl<'a, T> ProofBearingArtifactMaterializationFrontDoor<'a, T> {
    pub fn full_fidelity(
        self,
    ) -> FoundationalProfileMaterializationPlan<ProofBearingArtifactTarget> {
        plan_foundational_profile_materialization::<ProofBearingArtifactTarget>(self.0.profile())
    }

    pub fn operational_summary(
        self,
    ) -> FoundationalProfileMaterializationPlan<ProofBearingArtifactTarget> {
        plan_foundational_profile_materialization_with_elision::<ProofBearingArtifactTarget>(
            self.0.profile(),
            FoundationalDescriptiveElisionProfile::OperationalSummary,
        )
    }

    pub fn selected(
        self,
        selected: &[FoundationalDescriptiveSurface],
    ) -> Result<
        FoundationalProfileMaterializationPlan<ProofBearingArtifactTarget>,
        FoundationalMaterializationPlanningDenial,
    > {
        plan_selected_foundational_profile_materialization::<ProofBearingArtifactTarget>(
            self.0.profile(),
            selected,
        )
    }
}
