use worth_query_decl::facade::{
    application_query::{
        ApplicationQueryBasisSupport, ApplicationQueryCardinality,
        ApplicationQueryDefinitionBuilder, ApplicationQueryDependencyCeiling,
        ApplicationQueryDisclosureContract, ApplicationQueryLaneEligibility,
        ApplicationQueryReference, TypedApplicationQueryResultShape,
    },
    application_schema::{ApplicationAbilityRef, ApplicationEntityRef},
};

struct Schema;
struct Query;
struct Parameters;
struct Result;
struct Account;
struct Institution;
struct ViewInstitution;

fn mismatched_scope(
    reference: ApplicationQueryReference<Schema, Query, Parameters, Result, Account>,
    account: ApplicationEntityRef<Schema, Account>,
    shape: TypedApplicationQueryResultShape<Schema, Query, Account, Result>,
    ability: ApplicationAbilityRef<Schema, ViewInstitution, Institution>,
) {
    let _ = ApplicationQueryDefinitionBuilder::declare(reference)
        .root(account)
        .scope(account)
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 0))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .requires_ability(ability);
}

fn main() {}
