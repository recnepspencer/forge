use bank_domain::model::{AccountId, BankPrincipalId, InstitutionId, PaymentId};
use bank_domain::schema::*;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;

use super::{BankAdmittedOperation, BankOperationAdmissionError};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub(crate) fn authorize_account_discovery(
        &self,
        actor: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            DiscoverAccountsOperation,
            DiscoverAccounts,
            Principal,
            BankPrincipalId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            PrincipalIdentityField::reference(),
            actor.principal_id(),
            DiscoverAccountsOperation::reference(),
            request,
        )
    }

    pub(crate) fn authorize_account_summary(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<ReadAccountSummaryOperation, ReadAccountSummary, Account, AccountId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            AccountIdentity::reference(),
            account,
            ReadAccountSummaryOperation::reference(),
            request,
        )
    }

    pub(crate) fn authorize_account_detail(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<ReadAccountDetailOperation, ReadAccountDetail, Account, AccountId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            AccountIdentity::reference(),
            account,
            ReadAccountDetailOperation::reference(),
            request,
        )
    }

    pub(crate) fn authorize_account_users(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            ReadAccountAuthorizedUsersOperation,
            ReadAccountAuthorizedUsers,
            Account,
            AccountId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            AccountIdentity::reference(),
            account,
            ReadAccountAuthorizedUsersOperation::reference(),
            request,
        )
    }

    pub(crate) fn authorize_account_activity(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            ReadAccountActivityOperation,
            ReadAccountActivity,
            Account,
            AccountId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            AccountIdentity::reference(),
            account,
            ReadAccountActivityOperation::reference(),
            request,
        )
    }

    pub(crate) fn authorize_pending_payments(
        &self,
        actor: &BankAuthenticatedPrincipal,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            ReadPendingPaymentsOperation,
            ReadPendingPayments,
            Principal,
            BankPrincipalId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            PrincipalIdentityField::reference(),
            actor.principal_id(),
            ReadPendingPaymentsOperation::reference(),
            request,
        )
    }

    pub(crate) fn authorize_payment(
        &self,
        actor: &BankAuthenticatedPrincipal,
        payment: PaymentId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<ReadPaymentOperation, ReadPayment, PaymentIntent, PaymentId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            PaymentIdentityField::reference(),
            payment,
            ReadPaymentOperation::reference(),
            request,
        )
    }

    pub(crate) fn authorize_institution_audit(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            AuditInstitutionActivityOperation,
            AuditInstitutionActivity,
            Institution,
            InstitutionId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            InstitutionIdentityField::reference(),
            institution,
            AuditInstitutionActivityOperation::reference(),
            request,
        )
    }
}
