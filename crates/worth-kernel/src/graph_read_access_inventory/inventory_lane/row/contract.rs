use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::residue::WorthGraphReadAccessCappedResidueRow;
use super::super::scope::{WorthGraphReadAccessScopeExpectation, WorthGraphReadAccessScopeKind};
use super::classification::WorthGraphReadAccessClassification;
use super::cost_posture::WorthGraphReadAccessCostPosture;
use super::deletion_action::WorthGraphReadAccessDeletionAction;
use super::disposition::WorthGraphReadAccessMilestoneSevenDisposition;
use super::out_of_scope_reason::WorthGraphReadAccessOutOfScopeReason;

pub(super) fn validate_classification_contract(
    classification: WorthGraphReadAccessClassification,
    cost_posture: WorthGraphReadAccessCostPosture,
    deletion_action: WorthGraphReadAccessDeletionAction,
    disposition: WorthGraphReadAccessMilestoneSevenDisposition,
    scope_kind: WorthGraphReadAccessScopeKind,
    scope_expectation: WorthGraphReadAccessScopeExpectation,
    out_of_scope_reason: Option<WorthGraphReadAccessOutOfScopeReason>,
    capped_residue: Option<&WorthGraphReadAccessCappedResidueRow>,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    match (classification, capped_residue.is_some()) {
        (WorthGraphReadAccessClassification::CappedResidue, false) => {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::CappedResidueMissingResidueRow,
            ));
        }
        (WorthGraphReadAccessClassification::CappedResidue, true) => {}
        (_, true) => {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::ResidueRowOnNonResidueClassification,
            ));
        }
        (_, false) => {}
    }

    let contract = expected_contract(classification);
    if deletion_action != contract.deletion_action || disposition != contract.disposition {
        return Err(error(contract.error_kind));
    }
    validate_scope_contract(classification, scope_kind)?;
    validate_scope_expectation_contract(classification, scope_expectation)?;
    validate_out_of_scope_contract(classification, cost_posture, out_of_scope_reason)?;
    Ok(())
}

fn validate_scope_contract(
    classification: WorthGraphReadAccessClassification,
    scope_kind: WorthGraphReadAccessScopeKind,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    let allowed = match classification {
        WorthGraphReadAccessClassification::QueryDeclarationCandidate => matches!(
            scope_kind,
            WorthGraphReadAccessScopeKind::SelectedObligation
                | WorthGraphReadAccessScopeKind::TouchedAuthorityDigest
                | WorthGraphReadAccessScopeKind::TouchDescriptorDigest
                | WorthGraphReadAccessScopeKind::TopologyReadProof
                | WorthGraphReadAccessScopeKind::SpatialContinuationProof
        ),
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap => matches!(
            scope_kind,
            WorthGraphReadAccessScopeKind::SelectedObligation
                | WorthGraphReadAccessScopeKind::TouchedAuthorityDigest
                | WorthGraphReadAccessScopeKind::TouchDescriptorDigest
                | WorthGraphReadAccessScopeKind::SpatialContinuationProof
        ),
        WorthGraphReadAccessClassification::DeletionTarget
        | WorthGraphReadAccessClassification::CappedResidue => {
            scope_kind == WorthGraphReadAccessScopeKind::DeletedGraphReadSource
        }
        WorthGraphReadAccessClassification::CertificationOnlySupport => {
            scope_kind == WorthGraphReadAccessScopeKind::CertificationOnlyBoundary
        }
        WorthGraphReadAccessClassification::OutOfScopeNonGraphRead => {
            scope_kind == WorthGraphReadAccessScopeKind::OutOfScopeNonGraphRead
        }
    };
    if !allowed {
        return Err(error(
            WorthGraphReadAccessInventoryErrorKind::ScopeClassificationMismatch,
        ));
    }
    Ok(())
}

fn validate_scope_expectation_contract(
    classification: WorthGraphReadAccessClassification,
    scope_expectation: WorthGraphReadAccessScopeExpectation,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    let allowed = match classification {
        WorthGraphReadAccessClassification::QueryDeclarationCandidate => matches!(
            scope_expectation,
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput
                | WorthGraphReadAccessScopeExpectation::FutureExecutionReceiptExpectation
        ),
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap => {
            scope_expectation
                == WorthGraphReadAccessScopeExpectation::QueryAccessRequirementCandidateInput
        }
        WorthGraphReadAccessClassification::DeletionTarget
        | WorthGraphReadAccessClassification::CappedResidue => {
            scope_expectation == WorthGraphReadAccessScopeExpectation::DeletionOnlyResidue
        }
        WorthGraphReadAccessClassification::CertificationOnlySupport => {
            scope_expectation == WorthGraphReadAccessScopeExpectation::CertificationOnlyBoundary
        }
        WorthGraphReadAccessClassification::OutOfScopeNonGraphRead => {
            scope_expectation == WorthGraphReadAccessScopeExpectation::NonGraphReadBoundary
        }
    };
    if !allowed {
        return Err(error(
            WorthGraphReadAccessInventoryErrorKind::ScopeClassificationMismatch,
        ));
    }
    Ok(())
}

fn validate_out_of_scope_contract(
    classification: WorthGraphReadAccessClassification,
    cost_posture: WorthGraphReadAccessCostPosture,
    out_of_scope_reason: Option<WorthGraphReadAccessOutOfScopeReason>,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    match (classification, out_of_scope_reason) {
        (WorthGraphReadAccessClassification::OutOfScopeNonGraphRead, None) => Err(error(
            WorthGraphReadAccessInventoryErrorKind::MissingOutOfScopeReason,
        )),
        (WorthGraphReadAccessClassification::OutOfScopeNonGraphRead, Some(_))
            if cost_posture != WorthGraphReadAccessCostPosture::NoGraphTraversal =>
        {
            Err(error(
                WorthGraphReadAccessInventoryErrorKind::OutOfScopeCostPostureMismatch,
            ))
        }
        (WorthGraphReadAccessClassification::OutOfScopeNonGraphRead, Some(_)) => Ok(()),
        (_, Some(_)) => Err(error(
            WorthGraphReadAccessInventoryErrorKind::OutOfScopeReasonOnGraphReadClassification,
        )),
        (_, None) => Ok(()),
    }
}

struct WorthGraphReadAccessInventoryRowContract {
    deletion_action: WorthGraphReadAccessDeletionAction,
    disposition: WorthGraphReadAccessMilestoneSevenDisposition,
    error_kind: WorthGraphReadAccessInventoryErrorKind,
}

const fn expected_contract(
    classification: WorthGraphReadAccessClassification,
) -> WorthGraphReadAccessInventoryRowContract {
    match classification {
        WorthGraphReadAccessClassification::QueryDeclarationCandidate => {
            WorthGraphReadAccessInventoryRowContract {
                deletion_action: WorthGraphReadAccessDeletionAction::MigrateToQueryDeclaration,
                disposition: WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
                error_kind:
                    WorthGraphReadAccessInventoryErrorKind::DeclarationCandidateContractMismatch,
            }
        }
        WorthGraphReadAccessClassification::DeletionTarget => {
            WorthGraphReadAccessInventoryRowContract {
                deletion_action: WorthGraphReadAccessDeletionAction::DeleteAfterConsumerCutover,
                disposition: WorthGraphReadAccessMilestoneSevenDisposition::DeletionOnly,
                error_kind: WorthGraphReadAccessInventoryErrorKind::DeletionTargetContractMismatch,
            }
        }
        WorthGraphReadAccessClassification::CappedResidue => {
            WorthGraphReadAccessInventoryRowContract {
                deletion_action: WorthGraphReadAccessDeletionAction::CapUntilQueryCapabilityExists,
                disposition: WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap,
                error_kind: WorthGraphReadAccessInventoryErrorKind::CappedResidueContractMismatch,
            }
        }
        WorthGraphReadAccessClassification::CertificationOnlySupport => {
            WorthGraphReadAccessInventoryRowContract {
                deletion_action: WorthGraphReadAccessDeletionAction::KeepCertificationOnly,
                disposition: WorthGraphReadAccessMilestoneSevenDisposition::CertificationOnly,
                error_kind:
                    WorthGraphReadAccessInventoryErrorKind::CertificationOnlyContractMismatch,
            }
        }
        WorthGraphReadAccessClassification::QueryAccessCapabilityGap => {
            WorthGraphReadAccessInventoryRowContract {
                deletion_action: WorthGraphReadAccessDeletionAction::CapUntilQueryCapabilityExists,
                disposition: WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap,
                error_kind: WorthGraphReadAccessInventoryErrorKind::CapabilityGapContractMismatch,
            }
        }
        WorthGraphReadAccessClassification::OutOfScopeNonGraphRead => {
            WorthGraphReadAccessInventoryRowContract {
                deletion_action: WorthGraphReadAccessDeletionAction::OutOfScopeNoGraphRead,
                disposition: WorthGraphReadAccessMilestoneSevenDisposition::OutOfScope,
                error_kind: WorthGraphReadAccessInventoryErrorKind::OutOfScopeContractMismatch,
            }
        }
    }
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
