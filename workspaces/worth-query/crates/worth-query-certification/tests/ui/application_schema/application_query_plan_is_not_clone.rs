use worth_query_execution::facade::primary_graph::WorthQueryAdmittedApplicationQueryPlan;

struct Schema;
struct Query;
struct Parameters;
struct QueryResult;
struct Principal;
struct PrincipalIdentity;
struct Scope;

fn assert_clone<T: Clone>() {}

fn main() {
    assert_clone::<
        WorthQueryAdmittedApplicationQueryPlan<
            'static,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
    >();
}
