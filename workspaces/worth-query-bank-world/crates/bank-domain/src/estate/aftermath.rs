use super::EstateCapabilityOperation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateInverseOperation {
    UnfreezeAccount,
    RevokeDelegatedCapability,
    RestoreRevokedCapability,
    RevokeEmergencyAccess,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateCompensation {
    CompensatingEstateJournal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateReconciliation {
    ConfirmDeathNoticeWithAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstateAftermath {
    NoMutation,
    Reversible(EstateInverseOperation),
    Compensatable(EstateCompensation),
    Reconcilable(EstateReconciliation),
    Irreversible,
}

pub const fn aftermath_for(operation: EstateCapabilityOperation) -> EstateAftermath {
    match operation {
        EstateCapabilityOperation::NotifyDeath => {
            EstateAftermath::Reconcilable(EstateReconciliation::ConfirmDeathNoticeWithAuthority)
        }
        EstateCapabilityOperation::FreezeAccount => {
            EstateAftermath::Reversible(EstateInverseOperation::UnfreezeAccount)
        }
        EstateCapabilityOperation::DelegateCapability => {
            EstateAftermath::Reversible(EstateInverseOperation::RevokeDelegatedCapability)
        }
        EstateCapabilityOperation::RevokeCapability => {
            EstateAftermath::Reversible(EstateInverseOperation::RestoreRevokedCapability)
        }
        EstateCapabilityOperation::RequestEmergencyAccess
        | EstateCapabilityOperation::ApproveEmergencyAccess => {
            EstateAftermath::Reversible(EstateInverseOperation::RevokeEmergencyAccess)
        }
        EstateCapabilityOperation::DisburseEstate => {
            EstateAftermath::Compensatable(EstateCompensation::CompensatingEstateJournal)
        }
        EstateCapabilityOperation::OpenEstateCase
        | EstateCapabilityOperation::RecognizeExecutor
        | EstateCapabilityOperation::RevokeEmergencyAccess
        | EstateCapabilityOperation::CompleteMandatoryReview
        | EstateCapabilityOperation::ReleaseEstate => EstateAftermath::Irreversible,
        EstateCapabilityOperation::ViewRestrictedEstate => EstateAftermath::NoMutation,
    }
}
