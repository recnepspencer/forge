use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryDefinition,
    domain::WorthQueryInstalledApplicationQuery,
};

struct Schema;
struct Query;
struct Parameters;
struct QueryResult;
struct Scope;

fn require_installed_query(
    _: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
) {
}

fn host_definition_is_descriptive_only(
    definition: &ApplicationQueryDefinition<Schema, Query, Parameters, QueryResult, Scope>,
) {
    require_installed_query(definition);
}

fn main() {}
