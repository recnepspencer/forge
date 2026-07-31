mod account_access;
mod account_balance;
mod account_creation;
mod bounded;
mod business_payment;
mod money_movement;
#[cfg(test)]
mod operation_shape_tests;
mod reversal;
mod send_money;
#[cfg(test)]
mod tests;

use bank_domain::proposals::BankProposalDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankProjectionDenial {
    InvalidSnapshotVersion,
    MissingField(&'static str),
    MissingRelation(&'static str),
    AmbiguousRelation(&'static str),
    EntityResolution(worth_query_host::facade::primary_graph::WorthQueryEntityResolutionDenialKind),
    Traversal {
        kind:
            worth_query_host::facade::primary_graph::WorthQueryInvariantProjectionTraversalDenialKind,
        relation: String,
    },
    Aggregate(
        worth_query_host::facade::primary_graph::WorthQueryInvariantAggregateDenialKind,
    ),
    DecisionPlan(
        worth_query_host::facade::primary_graph::WorthQueryInvariantDecisionPlanDenialKind,
    ),
    AccountingRevisionMismatch(bank_domain::model::AccountId),
    InvalidDomainState(BankProposalDenial),
}

pub(crate) use account_access::{
    project_account_authorization_grant, project_account_authorization_revoke,
};
pub(crate) use account_creation::{
    project_business_account_creation, project_personal_account_creation,
};
pub(crate) use business_payment::{
    project_business_payment_initiation, project_payment_approval, project_payment_rejection,
};
pub(crate) use money_movement::project_institution_money_movement;
pub(crate) use reversal::project_journal_reversal;
pub(crate) use send_money::project_send_money_decision;

pub(super) fn missing_field<T>(
    value: Option<T>,
    field: &'static str,
) -> Result<T, BankProjectionDenial> {
    value.ok_or(BankProjectionDenial::MissingField(field))
}

impl std::fmt::Display for BankProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "bank invariant projection denied: {self:?}")
    }
}

impl std::error::Error for BankProjectionDenial {}

impl From<worth_query_host::facade::primary_graph::WorthQueryEntityResolutionDenial>
    for BankProjectionDenial
{
    fn from(
        denial: worth_query_host::facade::primary_graph::WorthQueryEntityResolutionDenial,
    ) -> Self {
        Self::EntityResolution(denial.kind())
    }
}

impl From<worth_query_host::facade::primary_graph::WorthQueryInvariantProjectionTraversalDenial>
    for BankProjectionDenial
{
    fn from(
        denial: worth_query_host::facade::primary_graph::WorthQueryInvariantProjectionTraversalDenial,
    ) -> Self {
        Self::Traversal {
            kind: denial.kind(),
            relation: denial.relation().to_string(),
        }
    }
}

impl From<worth_query_host::facade::primary_graph::WorthQueryInvariantDecisionPlanDenial>
    for BankProjectionDenial
{
    fn from(
        denial: worth_query_host::facade::primary_graph::WorthQueryInvariantDecisionPlanDenial,
    ) -> Self {
        Self::DecisionPlan(denial.kind())
    }
}

impl From<worth_query_host::facade::primary_graph::WorthQueryInvariantAggregateDenial>
    for BankProjectionDenial
{
    fn from(
        denial: worth_query_host::facade::primary_graph::WorthQueryInvariantAggregateDenial,
    ) -> Self {
        Self::Aggregate(denial.kind())
    }
}
