use worth_query_decl::facade::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinitionBuilder,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryReference, TypedApplicationQueryResultShape,
};
use worth_query_decl::facade::application_schema::ApplicationEntityRef;

struct Schema;
struct Query;
struct Parameters;
struct Result;
struct Account;

fn omit_lanes(
    reference: ApplicationQueryReference<Schema, Query, Parameters, Result, Account>,
    account: ApplicationEntityRef<Schema, Account>,
    shape: TypedApplicationQueryResultShape<Schema, Query, Account, Result>,
) {
    let _ = ApplicationQueryDefinitionBuilder::declare(reference)
        .root(account)
        .scope(account)
        .result_shape(shape)
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 0))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .public();
}

fn main() {}
