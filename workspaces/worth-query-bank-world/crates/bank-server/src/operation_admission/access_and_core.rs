use bank_domain::model::{AccountId, InstitutionId};
use bank_domain::schema::*;
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::declaration::application_schema::{
    ApplicationFieldRef, ApplicationFieldUnit, ApplicationOperationRef, EqualityPredicate,
    TypedApplicationValue, TypedMutationPreconditions, WritePosture,
};
use worth_query_host::facade::primary_graph::WorthQueryPrincipalResolutionMode;

use super::{BankAdmittedOperation, BankOperationAdmissionError};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub fn authorize_grant_account_access(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        preconditions: TypedMutationPreconditions<
            BankSchema,
            GrantAccountAuthorizationOperation,
            Account,
        >,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_revoke_account_access(
        &self,
        actor: &BankAuthenticatedPrincipal,
        account: AccountId,
        preconditions: TypedMutationPreconditions<
            BankSchema,
            RevokeAccountAuthorizationOperation,
            Account,
        >,
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
            preconditions,
            request,
        )
    }

    pub fn authorize_reverse_journal(
        &self,
        actor: &BankAuthenticatedPrincipal,
        institution: InstitutionId,
        preconditions: TypedMutationPreconditions<BankSchema, ReverseJournalOperation, Institution>,
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
            preconditions,
            request,
        )
    }

    pub(super) fn authorize<Aspect, Scope, Field, Value, Write, Unit, Operation, Input>(
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
            Unit,
        >,
        value: Value,
        operation: ApplicationOperationRef<BankSchema, Operation, Input>,
        preconditions: TypedMutationPreconditions<BankSchema, Operation, Scope>,
        request: &WorthQueryRequestScope,
    ) -> Result<BankAdmittedOperation<Operation, Input, Scope, Value>, BankOperationAdmissionError>
    where
        Value: TypedApplicationValue + Clone + Copy,
        Write: WritePosture,
        Unit: ApplicationFieldUnit,
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
            .authorize_operation(actor.query(), &identity, &operation, preconditions, request)
            .map_err(BankOperationAdmissionError::Authorization)?;
        Ok(BankAdmittedOperation {
            actor: actor.principal_id(),
            scope: value,
            query,
        })
    }
}
