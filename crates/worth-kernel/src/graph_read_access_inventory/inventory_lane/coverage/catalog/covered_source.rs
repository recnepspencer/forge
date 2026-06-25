use super::super::super::row::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessDeletionAction, WorthGraphReadAccessInventoryRow,
    WorthGraphReadAccessInventoryRowBuilder, WorthGraphReadAccessMilestoneSevenDisposition,
    WorthGraphReadAccessOwner,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessCoveredSource {
    source_path: &'static str,
    owner: WorthGraphReadAccessOwner,
    current_caller: &'static str,
    classification: WorthGraphReadAccessClassification,
    cost_posture: WorthGraphReadAccessCostPosture,
    deletion_action: WorthGraphReadAccessDeletionAction,
    disposition: WorthGraphReadAccessMilestoneSevenDisposition,
}

impl WorthGraphReadAccessCoveredSource {
    pub(crate) const fn declaration_candidate(
        source_path: &'static str,
        owner: WorthGraphReadAccessOwner,
        current_caller: &'static str,
        cost_posture: WorthGraphReadAccessCostPosture,
    ) -> Self {
        Self {
            source_path,
            owner,
            current_caller,
            classification: WorthGraphReadAccessClassification::QueryDeclarationCandidate,
            cost_posture,
            deletion_action: WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration,
            disposition: WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
        }
    }

    pub(crate) const fn deletion_target(
        source_path: &'static str,
        owner: WorthGraphReadAccessOwner,
        current_caller: &'static str,
        cost_posture: WorthGraphReadAccessCostPosture,
    ) -> Self {
        Self {
            source_path,
            owner,
            current_caller,
            classification: WorthGraphReadAccessClassification::DeletionTarget,
            cost_posture,
            deletion_action: WorthGraphReadAccessDeletionAction::DeleteAfterConsumerCutover,
            disposition: WorthGraphReadAccessMilestoneSevenDisposition::DeletionOnly,
        }
    }

    pub(crate) const fn certification_only(
        source_path: &'static str,
        owner: WorthGraphReadAccessOwner,
        current_caller: &'static str,
        cost_posture: WorthGraphReadAccessCostPosture,
    ) -> Self {
        Self {
            source_path,
            owner,
            current_caller,
            classification: WorthGraphReadAccessClassification::CertificationOnlySupport,
            cost_posture,
            deletion_action: WorthGraphReadAccessDeletionAction::KeepCertificationOnly,
            disposition: WorthGraphReadAccessMilestoneSevenDisposition::CertificationOnly,
        }
    }

    pub(crate) const fn access_capability_gap(
        source_path: &'static str,
        owner: WorthGraphReadAccessOwner,
        current_caller: &'static str,
        cost_posture: WorthGraphReadAccessCostPosture,
    ) -> Self {
        Self {
            source_path,
            owner,
            current_caller,
            classification: WorthGraphReadAccessClassification::QueryAccessCapabilityGap,
            cost_posture,
            deletion_action: WorthGraphReadAccessDeletionAction::CapUntilQueryCapabilityExists,
            disposition: WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap,
        }
    }

    pub(crate) const fn source_path(&self) -> &'static str {
        self.source_path
    }

    #[cfg(test)]
    pub(crate) const fn owner(&self) -> WorthGraphReadAccessOwner {
        self.owner
    }

    #[cfg(test)]
    pub(crate) const fn classification(&self) -> WorthGraphReadAccessClassification {
        self.classification
    }

    pub(crate) fn into_row_builder(self) -> WorthGraphReadAccessInventoryRowBuilder {
        WorthGraphReadAccessInventoryRow::builder()
            .source_path(self.source_path)
            .owner(self.owner)
            .current_caller(self.current_caller)
            .classification(self.classification)
            .cost_posture(self.cost_posture)
            .deletion_action(self.deletion_action)
            .milestone_seven_disposition(self.disposition)
    }
}
