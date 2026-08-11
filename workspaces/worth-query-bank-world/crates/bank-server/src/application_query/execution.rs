use bank_domain::model::BankPrincipalId;
use bank_domain::schema::{BankSchema, Principal};
use worth_query_host::facade::{
    declaration::application_schema::{ApplicationFieldUnit, TypedApplicationValue, WritePosture},
    primary_graph::{
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationProjection,
        WorthQueryApplicationQueryAccessContext, WorthQueryPrimaryGraphApplicationRuntime,
        WorthQueryPrincipalResolutionMode,
    },
    publication::domain_computation::{
        publish_application_result, WorthQueryPublishedApplicationResult,
    },
};

use super::{BankApplicationQueryDenial, BankApplicationQueryInvocation};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

pub(crate) fn execute_one_shot<
    Query,
    Parameters,
    QueryResult,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeIdentity,
    ScopeWrite,
    ScopeUnit,
>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    invocation: BankApplicationQueryInvocation<
        '_,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeUnit,
    >,
) -> Result<WorthQueryPublishedApplicationResult<Query, QueryResult>, BankApplicationQueryDenial>
where
    QueryResult: WorthQueryApplicationProjection<BankSchema, Query>,
    ScopeIdentity: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
{
    execute_with_lane(runtime, principal, invocation, |application, plan| {
        let result = application
            .execute_application_query_one_shot(plan)
            .map_err(BankApplicationQueryDenial::from_execution)?;
        Ok(publish_application_result(result.into_admitted_disclosed()))
    })
}

pub(crate) fn execute_preview<
    Query,
    Parameters,
    QueryResult,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeIdentity,
    ScopeWrite,
    ScopeUnit,
>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    invocation: BankApplicationQueryInvocation<
        '_,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeUnit,
    >,
) -> Result<WorthQueryPublishedApplicationResult<Query, QueryResult>, BankApplicationQueryDenial>
where
    QueryResult: WorthQueryApplicationProjection<BankSchema, Query>,
    ScopeIdentity: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
{
    execute_with_lane(runtime, principal, invocation, |application, plan| {
        let result = application
            .execute_application_query_preview(plan)
            .map_err(BankApplicationQueryDenial::from_preview_execution)?;
        Ok(publish_application_result(result.into_admitted_disclosed()))
    })
}

fn execute_with_lane<
    Query,
    Parameters,
    QueryResult,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeIdentity,
    ScopeWrite,
    ScopeUnit,
    Output,
>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    invocation: BankApplicationQueryInvocation<
        '_,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeUnit,
    >,
    execute: impl FnOnce(
        &WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
        WorthQueryAdmittedApplicationQueryPlan<
            '_,
            BankSchema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            BankPrincipalId,
            Scope,
        >,
    ) -> Result<Output, BankApplicationQueryDenial>,
) -> Result<Output, BankApplicationQueryDenial>
where
    QueryResult: WorthQueryApplicationProjection<BankSchema, Query>,
    ScopeIdentity: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
{
    let BankApplicationQueryInvocation {
        reference,
        scope_field,
        scope_identity,
        parameters,
        controls,
    } = invocation;
    let application = runtime.application_runtime();
    let query = application
        .installed_schema()
        .application_query(reference)
        .map_err(BankApplicationQueryDenial::from_installation)?;
    let scope = application
        .resolve_entity(
            scope_field,
            scope_identity,
            controls.request_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .map_err(BankApplicationQueryDenial::from_scope_resolution)?;
    let access = WorthQueryApplicationQueryAccessContext::<
        BankSchema,
        Principal,
        BankPrincipalId,
        Scope,
    >::new(principal.query(), &scope);
    let plan = application
        .admit_application_query(&query, &access, parameters, controls)
        .map_err(BankApplicationQueryDenial::from_admission)?;
    execute(application, plan)
}
