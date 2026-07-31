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
    let _ = ApplicationQueryDefinitionBuilder::requires_ability(
        reference,
        account,
        account,
        shape,
        ApplicationQueryCardinality::ExactlyOne,
        ApplicationQueryDependencyCeiling::bounded(0, 0, 0),
        ApplicationQueryDisclosureContract::public(),
        ApplicationQueryBasisSupport::current_and_pinned(),
        ApplicationQueryLaneEligibility::one_shot(),
        ability,
    );
}

fn main() {}
