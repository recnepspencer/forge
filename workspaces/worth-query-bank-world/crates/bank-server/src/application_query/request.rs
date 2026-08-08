use bank_domain::schema::BankSchema;
use worth_query_host::facade::declaration::{
    application_query::{ApplicationQueryParameterSet, ApplicationQueryReference},
    application_schema::{
        ApplicationFieldRef, ApplicationFieldUnit, EqualityPredicate, TypedApplicationValue,
        WritePosture,
    },
};
use worth_query_host::facade::primary_graph::WorthQueryApplicationQueryControls;

pub(crate) struct BankApplicationQueryInvocation<
    'request,
    Query,
    Parameters,
    QueryResult,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeIdentity,
    ScopeWrite,
    ScopeUnit,
> {
    pub(super) reference:
        ApplicationQueryReference<BankSchema, Query, Parameters, QueryResult, Scope>,
    pub(super) scope_field: ApplicationFieldRef<
        BankSchema,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        EqualityPredicate,
        ScopeUnit,
    >,
    pub(super) scope_identity: ScopeIdentity,
    pub(super) parameters: ApplicationQueryParameterSet<Query>,
    pub(super) controls: WorthQueryApplicationQueryControls<'request, BankSchema>,
}

impl<
        'request,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeUnit,
    >
    BankApplicationQueryInvocation<
        'request,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeUnit,
    >
where
    ScopeIdentity: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
{
    pub(crate) const fn new(
        reference: ApplicationQueryReference<BankSchema, Query, Parameters, QueryResult, Scope>,
        scope_field: ApplicationFieldRef<
            BankSchema,
            Scope,
            ScopeAspect,
            ScopeField,
            ScopeIdentity,
            ScopeWrite,
            EqualityPredicate,
            ScopeUnit,
        >,
        scope_identity: ScopeIdentity,
        parameters: ApplicationQueryParameterSet<Query>,
        controls: WorthQueryApplicationQueryControls<'request, BankSchema>,
    ) -> Self {
        Self {
            reference,
            scope_field,
            scope_identity,
            parameters,
            controls,
        }
    }
}
