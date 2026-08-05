use bank_domain::estate::{
    aftermath_for, EstateAftermath, EstateCapabilityOperation, EstateCompensation,
    EstateInverseOperation, EstateReconciliation,
};

#[test]
fn estate_operations_declare_honest_aftermath_without_local_undo() {
    let expected = [
        (
            EstateCapabilityOperation::NotifyDeath,
            EstateAftermath::Reconcilable(EstateReconciliation::ConfirmDeathNoticeWithAuthority),
        ),
        (
            EstateCapabilityOperation::FreezeAccount,
            EstateAftermath::Reversible(EstateInverseOperation::UnfreezeAccount),
        ),
        (
            EstateCapabilityOperation::OpenEstateCase,
            EstateAftermath::Irreversible,
        ),
        (
            EstateCapabilityOperation::RecognizeExecutor,
            EstateAftermath::Irreversible,
        ),
        (
            EstateCapabilityOperation::DelegateCapability,
            EstateAftermath::Reversible(EstateInverseOperation::RevokeDelegatedCapability),
        ),
        (
            EstateCapabilityOperation::RevokeCapability,
            EstateAftermath::Reversible(EstateInverseOperation::RestoreRevokedCapability),
        ),
        (
            EstateCapabilityOperation::RequestEmergencyAccess,
            EstateAftermath::Reversible(EstateInverseOperation::RevokeEmergencyAccess),
        ),
        (
            EstateCapabilityOperation::ApproveEmergencyAccess,
            EstateAftermath::Reversible(EstateInverseOperation::RevokeEmergencyAccess),
        ),
        (
            EstateCapabilityOperation::RevokeEmergencyAccess,
            EstateAftermath::Irreversible,
        ),
        (
            EstateCapabilityOperation::CompleteMandatoryReview,
            EstateAftermath::Irreversible,
        ),
        (
            EstateCapabilityOperation::ReleaseEstate,
            EstateAftermath::Irreversible,
        ),
        (
            EstateCapabilityOperation::DisburseEstate,
            EstateAftermath::Compensatable(EstateCompensation::CompensatingEstateJournal),
        ),
        (
            EstateCapabilityOperation::ViewRestrictedEstate,
            EstateAftermath::NoMutation,
        ),
    ];
    for (operation, expected_aftermath) in expected {
        assert_eq!(aftermath_for(operation), expected_aftermath);
    }
}
