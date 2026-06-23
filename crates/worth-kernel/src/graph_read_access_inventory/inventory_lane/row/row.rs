use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::residue::WorthGraphReadAccessCappedResidueRow;
use super::super::scope::WorthGraphReadAccessScopeBinding;
use super::classification::WorthGraphReadAccessClassification;
use super::contract::validate_classification_contract;
use super::cost_posture::WorthGraphReadAccessCostPosture;
use super::deletion_action::WorthGraphReadAccessDeletionAction;
use super::disposition::WorthGraphReadAccessMilestoneSevenDisposition;
use super::follow_on_work::WorthGraphReadAccessFollowOnWork;
use super::out_of_scope_reason::WorthGraphReadAccessOutOfScopeReason;
use super::owner::WorthGraphReadAccessOwner;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessInventoryRow {
    source_path: String,
    owner: WorthGraphReadAccessOwner,
    current_caller: String,
    classification: WorthGraphReadAccessClassification,
    cost_posture: WorthGraphReadAccessCostPosture,
    deletion_action: WorthGraphReadAccessDeletionAction,
    milestone_seven_disposition: WorthGraphReadAccessMilestoneSevenDisposition,
    scope_binding: WorthGraphReadAccessScopeBinding,
    out_of_scope_reason: Option<WorthGraphReadAccessOutOfScopeReason>,
    capped_residue: Option<WorthGraphReadAccessCappedResidueRow>,
}

impl WorthGraphReadAccessInventoryRow {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn builder(
    ) -> WorthGraphReadAccessInventoryRowBuilder {
        WorthGraphReadAccessInventoryRowBuilder::default()
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn owner(&self) -> WorthGraphReadAccessOwner {
        self.owner
    }

    pub fn current_caller(&self) -> &str {
        &self.current_caller
    }

    pub const fn classification(&self) -> WorthGraphReadAccessClassification {
        self.classification
    }

    pub const fn cost_posture(&self) -> WorthGraphReadAccessCostPosture {
        self.cost_posture
    }

    pub const fn deletion_action(&self) -> WorthGraphReadAccessDeletionAction {
        self.deletion_action
    }

    pub const fn milestone_seven_disposition(
        &self,
    ) -> WorthGraphReadAccessMilestoneSevenDisposition {
        self.milestone_seven_disposition
    }

    pub const fn follow_on_work(&self) -> WorthGraphReadAccessFollowOnWork {
        follow_on_work_for_classification(self.classification)
    }

    pub const fn scope_binding(&self) -> &WorthGraphReadAccessScopeBinding {
        &self.scope_binding
    }

    pub fn capped_residue(&self) -> Option<&WorthGraphReadAccessCappedResidueRow> {
        self.capped_residue.as_ref()
    }

    pub const fn out_of_scope_reason(&self) -> Option<WorthGraphReadAccessOutOfScopeReason> {
        self.out_of_scope_reason
    }

    pub const fn claims_execution_authority(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(in crate::graph_read_access_inventory::inventory_lane) struct WorthGraphReadAccessInventoryRowBuilder
{
    source_path: Option<String>,
    owner: Option<WorthGraphReadAccessOwner>,
    current_caller: Option<String>,
    classification: Option<WorthGraphReadAccessClassification>,
    cost_posture: Option<WorthGraphReadAccessCostPosture>,
    deletion_action: Option<WorthGraphReadAccessDeletionAction>,
    milestone_seven_disposition: Option<WorthGraphReadAccessMilestoneSevenDisposition>,
    scope_binding: Option<WorthGraphReadAccessScopeBinding>,
    out_of_scope_reason: Option<WorthGraphReadAccessOutOfScopeReason>,
    capped_residue: Option<WorthGraphReadAccessCappedResidueRow>,
}

impl WorthGraphReadAccessInventoryRowBuilder {
    pub fn source_path(mut self, source_path: impl Into<String>) -> Self {
        self.source_path = Some(source_path.into());
        self
    }

    pub const fn owner(mut self, owner: WorthGraphReadAccessOwner) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn current_caller(mut self, current_caller: impl Into<String>) -> Self {
        self.current_caller = Some(current_caller.into());
        self
    }

    pub const fn classification(
        mut self,
        classification: WorthGraphReadAccessClassification,
    ) -> Self {
        self.classification = Some(classification);
        self
    }

    pub const fn cost_posture(mut self, cost_posture: WorthGraphReadAccessCostPosture) -> Self {
        self.cost_posture = Some(cost_posture);
        self
    }

    pub const fn deletion_action(
        mut self,
        deletion_action: WorthGraphReadAccessDeletionAction,
    ) -> Self {
        self.deletion_action = Some(deletion_action);
        self
    }

    pub const fn milestone_seven_disposition(
        mut self,
        disposition: WorthGraphReadAccessMilestoneSevenDisposition,
    ) -> Self {
        self.milestone_seven_disposition = Some(disposition);
        self
    }

    pub fn scope_binding(mut self, scope_binding: WorthGraphReadAccessScopeBinding) -> Self {
        self.scope_binding = Some(scope_binding);
        self
    }

    pub const fn out_of_scope_reason(
        mut self,
        reason: WorthGraphReadAccessOutOfScopeReason,
    ) -> Self {
        self.out_of_scope_reason = Some(reason);
        self
    }

    pub fn capped_residue(mut self, residue: WorthGraphReadAccessCappedResidueRow) -> Self {
        self.capped_residue = Some(residue);
        self
    }

    pub fn build(
        self,
    ) -> Result<WorthGraphReadAccessInventoryRow, WorthGraphReadAccessInventoryError> {
        let source_path = require_non_empty_string(
            self.source_path,
            WorthGraphReadAccessInventoryErrorKind::MissingSourcePath,
        )?;
        let owner = self
            .owner
            .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingOwner))?;
        let current_caller = require_non_empty_string(
            self.current_caller,
            WorthGraphReadAccessInventoryErrorKind::MissingCurrentCaller,
        )?;
        let classification = self
            .classification
            .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingClassification))?;
        let cost_posture = self
            .cost_posture
            .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingCostPosture))?;
        let deletion_action = self
            .deletion_action
            .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingDeletionAction))?;
        let milestone_seven_disposition = self.milestone_seven_disposition.ok_or_else(|| {
            error(WorthGraphReadAccessInventoryErrorKind::MissingMilestoneSevenDisposition)
        })?;
        let scope_binding = self
            .scope_binding
            .ok_or_else(|| error(WorthGraphReadAccessInventoryErrorKind::MissingScopeBinding))?;
        if scope_binding.source_path() != source_path {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::ScopeSourcePathMismatch,
            ));
        }

        validate_classification_contract(
            classification,
            cost_posture,
            deletion_action,
            milestone_seven_disposition,
            scope_binding.scope_kind(),
            scope_binding.scope_expectation(),
            self.out_of_scope_reason,
            self.capped_residue.as_ref(),
        )?;

        Ok(WorthGraphReadAccessInventoryRow {
            source_path,
            owner,
            current_caller,
            classification,
            cost_posture,
            deletion_action,
            milestone_seven_disposition,
            scope_binding,
            out_of_scope_reason: self.out_of_scope_reason,
            capped_residue: self.capped_residue,
        })
    }
}

const fn follow_on_work_for_classification(
    classification: WorthGraphReadAccessClassification,
) -> WorthGraphReadAccessFollowOnWork {
    match classification {
        WorthGraphReadAccessClassification::QueryDeclarationCandidate => {
            WorthGraphReadAccessFollowOnWork::MilestoneSevenDeclaration
        }
        WorthGraphReadAccessClassification::CappedResidue
        | WorthGraphReadAccessClassification::QueryAccessCapabilityGap => {
            WorthGraphReadAccessFollowOnWork::MilestoneEightAccessPlanAdoption
        }
        WorthGraphReadAccessClassification::DeletionTarget => {
            WorthGraphReadAccessFollowOnWork::DeletionOnlyCleanup
        }
        WorthGraphReadAccessClassification::CertificationOnlySupport => {
            WorthGraphReadAccessFollowOnWork::CertificationOnly
        }
        WorthGraphReadAccessClassification::OutOfScopeNonGraphRead => {
            WorthGraphReadAccessFollowOnWork::OutOfScope
        }
    }
}

fn require_non_empty_string(
    value: Option<String>,
    error_kind: WorthGraphReadAccessInventoryErrorKind,
) -> Result<String, WorthGraphReadAccessInventoryError> {
    let value = value.ok_or_else(|| error(error_kind))?;
    if value.is_empty() {
        return Err(error(error_kind));
    }
    Ok(value)
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
