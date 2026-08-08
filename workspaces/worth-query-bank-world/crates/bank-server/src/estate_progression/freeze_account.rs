use bank_domain::{
    estate::EstateAction,
    model::AccountId,
    schema::{
        AccountIdentity, AccountStatus, BankSchema, EstateAccount, EstateCase,
        FreezeEstateAccountCapability, FreezeEstateAccountOperation, Status,
    },
};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    declaration::application_schema::TypedMutationPreconditions,
    primary_graph::{
        WorthQueryAdmittedApplicationOperation, WorthQueryApplicationEffectProgram,
        WorthQueryApplicationIdempotencyBinding,
        WorthQueryApplicationOperationInvariantProjectionReader,
        WorthQueryInvariantDecisionPlanDenial, WorthQueryInvariantEntityIdentity,
        WorthQueryInvariantProjectionTraversalDenial,
    },
};

use super::BankEstateProgressionDenial;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankMutationCommitOutcome};

type AdmittedFreezeOperation = WorthQueryAdmittedApplicationOperation<
    BankSchema,
    FreezeEstateAccountOperation,
    EstateAction,
    EstateCase,
>;
type FreezeEffectProgram = WorthQueryApplicationEffectProgram<
    BankSchema,
    FreezeEstateAccountOperation,
    EstateAction,
    EstateCase,
>;

#[derive(Debug)]
pub enum BankEstateFreezeProjectionDenial {
    RelationCardinality {
        expected: usize,
        observed: usize,
    },
    MissingAccountIdentity,
    RelatedAccountMismatch {
        expected: AccountId,
        observed: AccountId,
    },
    MissingAccountStatus,
    AccountNotOpen(AccountStatus),
    DecisionPlan(WorthQueryInvariantDecisionPlanDenial),
    Traversal(WorthQueryInvariantProjectionTraversalDenial),
}

impl BankIdentityRuntime {
    pub fn freeze_estate_account(
        &self,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<BankMutationCommitOutcome, BankEstateProgressionDenial> {
        let account = freeze_command_account(action)?;
        let admission = self.admit_freeze_operation(principal, action, request)?;
        if let Some(outcome) =
            super::idempotency::resolve_admitted_idempotency(self, &admission, idempotency)?
        {
            return Ok(outcome);
        }
        let program = self.materialize_freeze_effect(admission, account)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_application(program, idempotency)
            .into())
    }

    pub(crate) fn admit_freeze_operation(
        &self,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        request: &WorthQueryRequestScope,
    ) -> Result<AdmittedFreezeOperation, BankEstateProgressionDenial> {
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                FreezeEstateAccountCapability::reference(),
                FreezeEstateAccountOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::CapabilityInstallation)?;
        let access = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::Authorization)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(FreezeEstateAccountOperation::reference())
            .map_err(BankEstateProgressionDenial::OperationInstallation)?;
        self.application_runtime()
            .authorize_capability_operation(
                access,
                &operation,
                TypedMutationPreconditions::<
                    BankSchema,
                    FreezeEstateAccountOperation,
                    EstateCase,
                >::default(),
            )
            .map_err(BankEstateProgressionDenial::Authorization)
    }

    fn materialize_freeze_effect(
        &self,
        admission: AdmittedFreezeOperation,
        account: AccountId,
    ) -> Result<FreezeEffectProgram, BankEstateProgressionDenial> {
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                project_freeze_account(reader, estate, account)
            })
            .map_err(BankEstateProgressionDenial::Projection)?;
        let (projection_result, projection, _) = projected.into_parts();
        projection_result.map_err(BankEstateProgressionDenial::FreezeProjection)?;
        let reads = self
            .application_runtime()
            .begin_projected_application_read_attempt(admission, projection)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        let account = reads
            .resolve_entity(AccountIdentity::reference(), account)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        let mut effects = reads
            .complete_projected_dependencies()
            .map_err(BankEstateProgressionDenial::Attempt)?
            .begin_effect_program();
        let account = effects
            .existing_entity(&account)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        effects
            .write_field(&account, Status::reference(), AccountStatus::Frozen)
            .map_err(BankEstateProgressionDenial::Attempt)?;
        let program = effects
            .finish()
            .map_err(BankEstateProgressionDenial::Attempt)?;
        Ok(program)
    }
}

fn freeze_command_account(action: EstateAction) -> Result<AccountId, BankEstateProgressionDenial> {
    match action {
        EstateAction::FreezeAccount { account, .. } => Ok(account),
        _ => Err(BankEstateProgressionDenial::CommandInput(
            "FreezeEstateAccountOperation",
        )),
    }
}

fn project_freeze_account(
    reader: &mut WorthQueryApplicationOperationInvariantProjectionReader<
        BankSchema,
        FreezeEstateAccountOperation,
    >,
    estate: &WorthQueryInvariantEntityIdentity<BankSchema, EstateCase>,
    expected_account: AccountId,
) -> Result<(), BankEstateFreezeProjectionDenial> {
    let relations = reader.decision_relations_from(EstateAccount::reference(), estate)?;
    let [relation] = relations.as_slice() else {
        return Err(BankEstateFreezeProjectionDenial::RelationCardinality {
            expected: 1,
            observed: relations.len(),
        });
    };
    let account = relation.to().clone();
    let observed_account = reader
        .decision_field(&account, AccountIdentity::reference())?
        .ok_or(BankEstateFreezeProjectionDenial::MissingAccountIdentity)?;
    if observed_account != expected_account {
        return Err(BankEstateFreezeProjectionDenial::RelatedAccountMismatch {
            expected: expected_account,
            observed: observed_account,
        });
    }
    let status = reader
        .decision_field(&account, Status::reference())?
        .ok_or(BankEstateFreezeProjectionDenial::MissingAccountStatus)?;
    if status != AccountStatus::Open {
        return Err(BankEstateFreezeProjectionDenial::AccountNotOpen(status));
    }
    Ok(())
}

impl From<WorthQueryInvariantDecisionPlanDenial> for BankEstateFreezeProjectionDenial {
    fn from(denial: WorthQueryInvariantDecisionPlanDenial) -> Self {
        Self::DecisionPlan(denial)
    }
}

impl From<WorthQueryInvariantProjectionTraversalDenial> for BankEstateFreezeProjectionDenial {
    fn from(denial: WorthQueryInvariantProjectionTraversalDenial) -> Self {
        Self::Traversal(denial)
    }
}

impl std::fmt::Display for BankEstateFreezeProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RelationCardinality { expected, observed } => write!(
                formatter,
                "estate account relation expected {expected} target, observed {observed}"
            ),
            Self::MissingAccountIdentity => {
                write!(formatter, "estate account is missing its typed identity")
            }
            Self::RelatedAccountMismatch { expected, observed } => write!(
                formatter,
                "freeze command account {expected:?} does not match estate account {observed:?}"
            ),
            Self::MissingAccountStatus => {
                write!(formatter, "estate account is missing its current status")
            }
            Self::AccountNotOpen(status) => {
                write!(formatter, "estate account is not open: {status:?}")
            }
            Self::DecisionPlan(denial) => denial.fmt(formatter),
            Self::Traversal(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for BankEstateFreezeProjectionDenial {}
