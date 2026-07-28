use bank_domain::model::{AccountId, BankPrincipalId, BusinessId, InstitutionId, PaymentId};
use bank_domain::schema::*;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::declaration::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldRef, ApplicationOperationRef, EqualityPredicate,
    TypedApplicationValue, WritePosture,
};
use worth_query_host::facade::domain::WorthQueryApplicationOperationInstallationDenial;
use worth_query_host::facade::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryEntityResolutionDenial,
    WorthQueryOperationAuthorizationDenial, WorthQueryPrincipalResolutionMode,
};

use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

/// Bank-owned retention of Query's exact admitted-operation authority plus
/// the typed bank actor and scope identities used by proposal semantics.
///
/// ```compile_fail
/// use bank_server::BankAdmittedOperation;
///
/// let _ = BankAdmittedOperation::<(), (), (), u64> {
///     actor: todo!(),
///     scope: 1,
///     query: todo!(),
/// };
/// ```
pub struct BankAdmittedOperation<Operation, Input, Scope, ScopeIdentity> {
    actor: BankPrincipalId,
    scope: ScopeIdentity,
    query: WorthQueryAdmittedApplicationOperation<BankSchema, Operation, Input, Scope>,
}

impl<Operation, Input, Scope, ScopeIdentity: Copy>
    BankAdmittedOperation<Operation, Input, Scope, ScopeIdentity>
{
    pub const fn actor(&self) -> BankPrincipalId {
        self.actor
    }

    pub const fn scope(&self) -> ScopeIdentity {
        self.scope
    }

    pub fn operation(&self) -> &str {
        self.query.operation()
    }

    /// Descriptive canonical bytes retained from Query's authenticated
    /// operation, principal, and typed scope binding. The bytes grant no
    /// authority and cannot reconstruct this admission.
    pub fn operation_scope_fingerprint(&self) -> [u8; 32] {
        *self.query.operation_scope_fingerprint().bytes()
    }

    pub(crate) fn idempotency_binding(&self) -> bank_domain::proposals::BankOperationScopeBinding {
        bank_domain::proposals::BankOperationScopeBinding::from_fingerprint_bytes(
            self.operation_scope_fingerprint(),
        )
    }
}

#[derive(Debug)]
pub enum BankOperationAdmissionError {
    ScopeResolution(WorthQueryEntityResolutionDenial),
    OperationInstallation(WorthQueryApplicationOperationInstallationDenial),
    Authorization(WorthQueryOperationAuthorizationDenial),
}

impl std::fmt::Display for BankOperationAdmissionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScopeResolution(error) => error.fmt(formatter),
            Self::OperationInstallation(error) => error.fmt(formatter),
            Self::Authorization(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for BankOperationAdmissionError {}

impl BankIdentityRuntime {
    pub fn authorize_create_personal_account(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            CreatePersonalAccountOperation,
            CreatePersonalAccount,
            Institution,
            InstitutionId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            InstitutionIdentityField::reference(),
            institution,
            CreatePersonalAccountOperation::reference(),
            request,
        )
    }

    pub fn authorize_create_business_account(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            CreateBusinessAccountOperation,
            CreateBusinessAccount,
            Institution,
            InstitutionId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            InstitutionIdentityField::reference(),
            institution,
            CreateBusinessAccountOperation::reference(),
            request,
        )
    }

    pub fn authorize_opening_funding(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            ApplyOpeningFundingOperation,
            ApplyOpeningFunding,
            Institution,
            InstitutionId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            InstitutionIdentityField::reference(),
            institution,
            ApplyOpeningFundingOperation::reference(),
            request,
        )
    }

    pub fn authorize_deposit(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<DepositOperation, Deposit, Institution, InstitutionId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            InstitutionIdentityField::reference(),
            institution,
            DepositOperation::reference(),
            request,
        )
    }

    pub fn authorize_withdrawal(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<WithdrawOperation, Withdraw, Institution, InstitutionId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            InstitutionIdentityField::reference(),
            institution,
            WithdrawOperation::reference(),
            request,
        )
    }

    pub fn authorize_send_money(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<SendMoneyOperation, SendMoney, Account, AccountId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            AccountIdentity::reference(),
            account,
            SendMoneyOperation::reference(),
            request,
        )
    }

    pub fn authorize_initiate_business_payment(
        &self,
        actor: &BankAuthenticatedPrincipal,
        business: BusinessId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            InitiateBusinessPaymentOperation,
            InitiateBusinessPayment,
            Business,
            BusinessId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            BusinessIdentityField::reference(),
            business,
            InitiateBusinessPaymentOperation::reference(),
            request,
        )
    }

    pub fn authorize_approve_payment(
        &self,
        actor: &BankAuthenticatedPrincipal,
        payment: PaymentId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<ApprovePaymentOperation, ApprovePayment, PaymentIntent, PaymentId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            PaymentIdentityField::reference(),
            payment,
            ApprovePaymentOperation::reference(),
            request,
        )
    }

    pub fn authorize_reject_payment(
        &self,
        actor: &BankAuthenticatedPrincipal,
        payment: PaymentId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<RejectPaymentOperation, RejectPayment, PaymentIntent, PaymentId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            PaymentIdentityField::reference(),
            payment,
            RejectPaymentOperation::reference(),
            request,
        )
    }

    pub fn authorize_grant_account_access(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            GrantAccountAuthorizationOperation,
            GrantAccountAuthorization,
            Account,
            AccountId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            AccountIdentity::reference(),
            account,
            GrantAccountAuthorizationOperation::reference(),
            request,
        )
    }

    pub fn authorize_revoke_account_access(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<
            RevokeAccountAuthorizationOperation,
            RevokeAccountAuthorization,
            Account,
            AccountId,
        >,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            AccountIdentity::reference(),
            account,
            RevokeAccountAuthorizationOperation::reference(),
            request,
        )
    }

    pub fn authorize_reverse_journal(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        request: &WorthQueryRequestScope,
    ) -> Result<
        BankAdmittedOperation<ReverseJournalOperation, ReverseJournal, Institution, InstitutionId>,
        BankOperationAdmissionError,
    > {
        self.authorize(
            actor,
            InstitutionIdentityField::reference(),
            institution,
            ReverseJournalOperation::reference(),
            request,
        )
    }

    fn authorize<Aspect, Scope, Field, Value, Write, Currency, Operation, Input>(
        &self,
        actor: &BankAuthenticatedPrincipal,
        field: ApplicationFieldRef<
            BankSchema,
            Scope,
            Aspect,
            Field,
            Value,
            Write,
            EqualityPredicate,
            Currency,
        >,
        value: Value,
        operation: ApplicationOperationRef<BankSchema, Operation, Input>,
        request: &WorthQueryRequestScope,
    ) -> Result<BankAdmittedOperation<Operation, Input, Scope, Value>, BankOperationAdmissionError>
    where
        Value: TypedApplicationValue + Clone + Copy,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        let identity = self
            .application_runtime()
            .resolve_entity(
                field,
                value,
                request,
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(BankOperationAdmissionError::ScopeResolution)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(operation)
            .map_err(BankOperationAdmissionError::OperationInstallation)?;
        let query = self
            .application_runtime()
            .authorize_operation(actor.query(), &identity, &operation, request)
            .map_err(BankOperationAdmissionError::Authorization)?;
        Ok(BankAdmittedOperation {
            actor: actor.principal_id(),
            scope: value,
            query,
        })
    }
}
