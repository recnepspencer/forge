use worth_query_declaration::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinition,
    ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
    ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
    ApplicationQueryOptionalResultFieldRef, ApplicationQueryResultFieldRef,
    ApplicationQueryResultShapeBuilder,
};
use worth_query_declaration::worth_query_application_query;

use super::{
    Account, AccountIdentity, AccountNote, AccountPolicy, AccountSummaryParameters,
    IdentityExecutionSchema,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationProjection, WorthQueryApplicationProjectionDenial,
    WorthQueryApplicationProjectionRow,
};

pub struct AccountSlot;
pub struct NoteSlot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionalAccountFieldResult {
    account: String,
    note: Option<String>,
}

impl OptionalAccountFieldResult {
    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
}

worth_query_application_query!(
    pub OptionalAccountFieldQuery in IdentityExecutionSchema,
    parameters AccountSummaryParameters,
    result OptionalAccountFieldResult,
    scope Account,
    name "optional_account_field"
);

pub(super) fn optional_account_field_definition() -> ApplicationQueryDefinition<
    IdentityExecutionSchema,
    OptionalAccountFieldQuery,
    AccountSummaryParameters,
    OptionalAccountFieldResult,
    Account,
> {
    let shape = ApplicationQueryResultShapeBuilder::new(Account::reference())
        .field(account())
        .optional_field(note())
        .build();
    ApplicationQueryDefinitionBuilder::declare(OptionalAccountFieldQuery::reference())
        .root(Account::reference())
        .scope(Account::reference())
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 2))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .public()
        .build()
        .expect("the optional field fixture is statically canonical")
}

impl WorthQueryApplicationProjection<IdentityExecutionSchema, OptionalAccountFieldQuery>
    for OptionalAccountFieldResult
{
    fn project(
        row: &WorthQueryApplicationProjectionRow<
            '_,
            IdentityExecutionSchema,
            OptionalAccountFieldQuery,
        >,
    ) -> Result<Self, WorthQueryApplicationProjectionDenial> {
        Ok(Self {
            account: row.field(account())?,
            note: row.optional_field(note())?,
        })
    }
}

fn account() -> ApplicationQueryResultFieldRef<
    OptionalAccountFieldQuery,
    AccountSlot,
    IdentityExecutionSchema,
    Account,
    AccountPolicy,
    AccountIdentity,
    String,
    worth_query_declaration::facade::application_schema::ReadOnly,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
> {
    ApplicationQueryResultFieldRef::new("account", AccountIdentity::reference())
}

fn note() -> ApplicationQueryOptionalResultFieldRef<
    OptionalAccountFieldQuery,
    NoteSlot,
    IdentityExecutionSchema,
    Account,
    AccountPolicy,
    AccountNote,
    String,
    worth_query_declaration::facade::application_schema::ReadWrite,
    worth_query_declaration::facade::application_schema::EqualityPredicate,
    worth_query_declaration::facade::application_schema::NoApplicationUnit,
> {
    ApplicationQueryOptionalResultFieldRef::new("note", AccountNote::reference())
}
