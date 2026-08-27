impl super::UiServiceProposalStageReceipt {
    pub(in crate::runtime) fn from_family_owner(
        proposal: super::super::UiServiceProposalIdentity,
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::super::UiServiceProposalOccupancyScopeIdentity,
        fact_references: Vec<super::super::UiServiceProducedFactReference>,
        mounted_work_references: Vec<super::super::UiServiceMountedWorkReference>,
    ) -> Self {
        Self {
            proposal,
            completed: super::super::UiServiceProposalStage::FamilyOwnedStaging,
            issuer: super::UiServiceProposalStageIssuer::FamilyOwner { family, scope },
            fact_references: fact_references.into_boxed_slice(),
            mounted_work_references: mounted_work_references.into_boxed_slice(),
        }
    }

    pub(in crate::runtime) fn existing_preparation(
        proposal: super::super::UiServiceProposalIdentity,
    ) -> Self {
        Self {
            proposal,
            completed: super::super::UiServiceProposalStage::AssembleSuccessor,
            issuer: super::UiServiceProposalStageIssuer::ExistingPreparation,
            fact_references: Box::new([]),
            mounted_work_references: Box::new([]),
        }
    }

    pub(in crate::runtime) fn focus_resolution(
        proposal: super::super::UiServiceProposalIdentity,
        reveal_refinement: bool,
    ) -> Self {
        Self {
            proposal,
            completed: super::super::UiServiceProposalStage::ResolveFocusAndReveal,
            issuer: super::UiServiceProposalStageIssuer::FocusOwner { reveal_refinement },
            fact_references: Box::new([]),
            mounted_work_references: Box::new([]),
        }
    }

    pub(in crate::runtime) fn motion_derivation(
        proposal: super::super::UiServiceProposalIdentity,
    ) -> Self {
        Self {
            proposal,
            completed: super::super::UiServiceProposalStage::DeriveMotion,
            issuer: super::UiServiceProposalStageIssuer::MotionOwner,
            fact_references: Box::new([]),
            mounted_work_references: Box::new([]),
        }
    }

    #[cfg(test)]
    pub(in crate::runtime) fn recorded_family_fixture(
        proposal: super::super::UiServiceProposalIdentity,
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::super::UiServiceProposalOccupancyScopeIdentity,
        fact_references: Vec<super::super::UiServiceProducedFactReference>,
        mounted_work_references: Vec<super::super::UiServiceMountedWorkReference>,
    ) -> Self {
        Self::from_family_owner(
            proposal,
            family,
            scope,
            fact_references,
            mounted_work_references,
        )
    }

    #[cfg(test)]
    pub(in crate::runtime) fn recorded_stage_fixture(
        proposal: super::super::UiServiceProposalIdentity,
        completed: super::super::UiServiceProposalStage,
        issuer: super::UiServiceProposalStageIssuer,
    ) -> Self {
        Self {
            proposal,
            completed,
            issuer,
            fact_references: Box::new([]),
            mounted_work_references: Box::new([]),
        }
    }
}
