use bank_domain::estate::EstateAction;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::provisional_aftermath::progress_admitted_reconciliation;

use super::super::{
    BankEstateProgressionDenial, BankEstateProgressionFailure, BankRecordedInverseUndoAdmission,
    BankRecoveryTransitionReceipt,
};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub fn progress_undo_reconciliation(
        &self,
        admission: BankRecordedInverseUndoAdmission,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<BankRecoveryTransitionReceipt, BankEstateProgressionDenial> {
        self.progress_undo_reconciliation_retaining(admission, principal, request)
            .map_err(BankEstateProgressionFailure::into_denial)
    }

    pub fn progress_undo_reconciliation_retaining(
        &self,
        admission: BankRecordedInverseUndoAdmission,
        principal: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankRecoveryTransitionReceipt,
        BankEstateProgressionFailure<BankRecordedInverseUndoAdmission>,
    > {
        let action = match admission.query.original_input::<EstateAction>() {
            Some(action) => *action,
            None => {
                return Err(BankEstateProgressionFailure::retained(
                    BankEstateProgressionDenial::CommandInput("retained reconciliation input"),
                    admission,
                ));
            }
        };
        let operation = admission.installed_operation().to_owned();
        let current = match self.admit_notification_operation(principal, action, request) {
            Ok(current) => current,
            Err(denial) => {
                return Err(BankEstateProgressionFailure::retained(denial, admission));
            }
        };
        let authority = match self
            .application_runtime()
            .admit_undo_recovery_effect_authority(&admission.query, &current)
            .map_err(BankEstateProgressionDenial::from_recovery)
        {
            Ok(authority) => authority,
            Err(denial) => {
                return Err(BankEstateProgressionFailure::retained(denial, admission));
            }
        };
        progress_admitted_reconciliation(admission.query, &authority)
            .map_err(BankEstateProgressionDenial::from_recovery)
            .map_err(BankEstateProgressionFailure::consumed)?;
        Ok(BankRecoveryTransitionReceipt::new(operation))
    }
}
