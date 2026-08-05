use crate::{
    estate::{EstateAction, EstateDenial},
    model::EmployeeRole,
};

pub(super) fn validate(role: EmployeeRole, action: EstateAction) -> Result<(), EstateDenial> {
    let allowed = match action {
        EstateAction::NotifyDeath { .. }
        | EstateAction::RecognizeExecutor { .. }
        | EstateAction::CompleteMandatoryReview { .. } => matches!(
            role,
            EmployeeRole::EstateSpecialist | EmployeeRole::Compliance | EmployeeRole::Legal
        ),
        EstateAction::FreezeAccount { .. }
        | EstateAction::OpenEstateCase { .. }
        | EstateAction::DelegateCapability { .. }
        | EstateAction::RevokeCapability { .. }
        | EstateAction::RequestEmergencyAccess { .. }
        | EstateAction::RevokeEmergencyAccess { .. } => matches!(
            role,
            EmployeeRole::BranchManager
                | EmployeeRole::EstateSpecialist
                | EmployeeRole::Compliance
                | EmployeeRole::Legal
        ),
        EstateAction::ApproveEmergencyAccess { .. }
        | EstateAction::ReleaseEstate { .. }
        | EstateAction::DisburseEstate(_) => matches!(
            role,
            EmployeeRole::EstateSpecialist | EmployeeRole::Compliance | EmployeeRole::Legal
        ),
        EstateAction::ViewRestrictedEstate { field, .. }
        | EstateAction::ViewRestrictedEstateWithEmergencyAccess { field, .. } => {
            match field.classification() {
                crate::estate::BankDisclosureClassification::Restricted => {
                    !matches!(role, EmployeeRole::Teller | EmployeeRole::Auditor)
                }
                crate::estate::BankDisclosureClassification::HighlyRestricted => matches!(
                    role,
                    EmployeeRole::EstateSpecialist | EmployeeRole::Compliance | EmployeeRole::Legal
                ),
                crate::estate::BankDisclosureClassification::LegalSealed => {
                    matches!(role, EmployeeRole::Compliance | EmployeeRole::Legal)
                }
            }
        }
    };
    if allowed {
        Ok(())
    } else {
        Err(EstateDenial::EmployeeRoleMismatch)
    }
}
