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
        reveal_refinement: Option<super::super::super::UiServiceProposalOccupancyScopeIdentity>,
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

/// Reference validation for one staged owner witness: a witness may only carry
/// facts and mounted work for its own family at its own staged scope.
pub(super) fn require_empty_references(
    receipt: &super::UiServiceProposalStageReceipt,
) -> Result<(), super::UiServiceProposalStagingDenial> {
    if receipt.fact_references.is_empty() && receipt.mounted_work_references.is_empty() {
        Ok(())
    } else {
        Err(super::UiServiceProposalStagingDenial::UnexpectedReferences)
    }
}

pub(super) fn validate_references(
    family: crate::capability::UiRuntimeServiceFamily,
    scope: super::super::super::UiServiceProposalOccupancyScopeIdentity,
    receipt: &super::UiServiceProposalStageReceipt,
) -> Result<(), super::UiServiceProposalStagingDenial> {
    if receipt
        .fact_references
        .iter()
        .any(|reference| reference.family() != family)
        || receipt
            .mounted_work_references
            .iter()
            .any(|reference| reference.family() != family)
    {
        return Err(super::UiServiceProposalStagingDenial::ReferenceFamilyMismatch);
    }
    if receipt
        .fact_references
        .iter()
        .any(|reference| reference.scope() != scope)
        || receipt
            .mounted_work_references
            .iter()
            .any(|reference| reference.scope() != scope)
    {
        return Err(super::UiServiceProposalStagingDenial::ReferenceScopeMismatch);
    }
    Ok(())
}
