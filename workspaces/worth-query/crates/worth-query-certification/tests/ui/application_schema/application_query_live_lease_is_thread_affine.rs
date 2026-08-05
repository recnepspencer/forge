use worth_query_decl::facade::{
    application_query::ApplicationQueryLiveCauseBinding, application_schema::ApplicationSchema,
};
use worth_query_execution::facade::primary_graph::WorthQueryApplicationLiveLease;

fn require_send<T: Send>() {}

fn live_lease_cannot_cross_signal_owner_threads<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
    Target,
    Binding,
>()
where
    Schema: ApplicationSchema + Send + Sync + 'static,
    Query: Send + Sync + 'static,
    Parameters: Send + Sync + 'static,
    QueryResult: Send + Sync + 'static,
    Principal: Send + Sync + 'static,
    PrincipalIdentity: Send + Sync + 'static,
    Scope: Send + Sync + 'static,
    Target: Send + Sync + 'static,
    Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target> + Send + Sync + 'static,
{
    require_send::<
        WorthQueryApplicationLiveLease<
            'static,
            'static,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
            Target,
            Binding,
        >,
    >();
}

fn main() {}
