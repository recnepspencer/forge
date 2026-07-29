use bank_domain::proposals::{BankIdempotencyKey, BankProposalDenial, BankProposalEngine};
use bank_domain::schema::{
    CreateBusinessAccount, CreateBusinessAccountOperation, CreatePersonalAccount,
    CreatePersonalAccountOperation, Institution,
};

use crate::bank_projection::{
    project_business_account_creation, project_personal_account_creation,
};
use crate::{
    BankAdmittedOperation, BankAuthorizedProposal, BankIdentityRuntime, BankOperationProposalError,
    BankOperationProposals,
};

impl BankOperationProposals {
    pub fn prepare_create_personal_account(
        runtime: &BankIdentityRuntime,
        admission: BankAdmittedOperation<
            CreatePersonalAccountOperation,
            CreatePersonalAccount,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        key: &BankIdempotencyKey,
        input: &CreatePersonalAccount,
    ) -> Result<
        BankAuthorizedProposal<
            CreatePersonalAccountOperation,
            CreatePersonalAccount,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        BankOperationProposalError,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime
            .invariant_projection()
            .project_admitted_operation(admission.query(), |reader, institution| {
                project_personal_account_creation(reader, institution, input)
            })?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_create_personal_account(
            &snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }

    pub fn prepare_create_business_account(
        runtime: &BankIdentityRuntime,
        admission: BankAdmittedOperation<
            CreateBusinessAccountOperation,
            CreateBusinessAccount,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        key: &BankIdempotencyKey,
        input: &CreateBusinessAccount,
    ) -> Result<
        BankAuthorizedProposal<
            CreateBusinessAccountOperation,
            CreateBusinessAccount,
            Institution,
            bank_domain::model::InstitutionId,
        >,
        BankOperationProposalError,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch.into());
        }
        let completed = runtime
            .invariant_projection()
            .project_admitted_operation(admission.query(), |reader, institution| {
                project_business_account_creation(reader, institution, input)
            })?;
        let (snapshot, projection, work) = completed.into_parts();
        let snapshot = snapshot?;
        let invariant = BankProposalEngine::prepare_create_business_account(
            &snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new_bounded(
            admission, invariant, projection, work,
        ))
    }
}
