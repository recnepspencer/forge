use bank_domain::proposals::{
    BankIdempotencyKey, BankProposalDenial, BankProposalEngine, BankSnapshot,
};
use bank_domain::schema::{
    CreateBusinessAccount, CreateBusinessAccountOperation, CreatePersonalAccount,
    CreatePersonalAccountOperation, Institution,
};

use crate::{BankAdmittedOperation, BankAuthorizedProposal, BankOperationProposals};

impl BankOperationProposals {
    pub fn prepare_create_personal_account(
        snapshot: &BankSnapshot,
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
        BankProposalDenial,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_create_personal_account(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }

    pub fn prepare_create_business_account(
        snapshot: &BankSnapshot,
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
        BankProposalDenial,
    > {
        if admission.scope() != input.institution {
            return Err(BankProposalDenial::ScopeInputMismatch);
        }
        let invariant = BankProposalEngine::prepare_create_business_account(
            snapshot,
            admission.idempotency_binding(),
            key,
            input,
        )?;
        Ok(BankAuthorizedProposal::new(admission, invariant))
    }
}
