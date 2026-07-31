mod access_and_core;

use bank_domain::model::{AccountId, BankPrincipalId, BusinessId, InstitutionId, PaymentId};
use bank_domain::schema::*;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::declaration::application_schema::TypedMutationPreconditions;
use worth_query_host::facade::domain::WorthQueryApplicationOperationInstallationDenial;
use worth_query_host::facade::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryEntityResolutionDenial,
    WorthQueryOperationAuthorizationDenial,
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

    pub(crate) const fn query(
        &self,
    ) -> &WorthQueryAdmittedApplicationOperation<BankSchema, Operation, Input, Scope> {
        &self.query
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        BankPrincipalId,
        ScopeIdentity,
        WorthQueryAdmittedApplicationOperation<BankSchema, Operation, Input, Scope>,
    ) {
        (self.actor, self.scope, self.query)
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
        preconditions: TypedMutationPreconditions<
            BankSchema,
            CreatePersonalAccountOperation,
            Institution,
        >,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_create_business_account(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        preconditions: TypedMutationPreconditions<
            BankSchema,
            CreateBusinessAccountOperation,
            Institution,
        >,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_opening_funding(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        preconditions: TypedMutationPreconditions<
            BankSchema,
            ApplyOpeningFundingOperation,
            Institution,
        >,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_deposit(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        preconditions: TypedMutationPreconditions<BankSchema, DepositOperation, Institution>,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_withdrawal(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        preconditions: TypedMutationPreconditions<BankSchema, WithdrawOperation, Institution>,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_send_money(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        preconditions: TypedMutationPreconditions<BankSchema, SendMoneyOperation, Account>,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_initiate_business_payment(
        &self,
        actor: &BankAuthenticatedPrincipal,
        business: BusinessId,
        preconditions: TypedMutationPreconditions<
            BankSchema,
            InitiateBusinessPaymentOperation,
            Business,
        >,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_approve_payment(
        &self,
        actor: &BankAuthenticatedPrincipal,
        payment: PaymentId,
        preconditions: TypedMutationPreconditions<
            BankSchema,
            ApprovePaymentOperation,
            PaymentIntent,
        >,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_reject_payment(
        &self,
        actor: &BankAuthenticatedPrincipal,
        payment: PaymentId,
        preconditions: TypedMutationPreconditions<
            BankSchema,
            RejectPaymentOperation,
            PaymentIntent,
        >,
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
            preconditions,
            request,
        )
    }
}
