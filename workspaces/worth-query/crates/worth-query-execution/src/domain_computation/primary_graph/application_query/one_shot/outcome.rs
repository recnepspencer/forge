use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::ApplicationSchema;

use super::{
    admit_request, denial, validate_authentication_lifetime, validate_basis_lifetime,
    WorthQueryApplicationOneShotDenial, WorthQueryApplicationOneShotDenialKind,
    WorthQueryApplicationOneShotResult,
};
use crate::domain_computation::primary_graph::application_query::{
    access_receipt::{
        WorthQueryApplicationQueryReceiptBasis, WorthQueryApplicationQueryReceiptIdentity,
    },
    read_execution::{project_non_live_kernel, RawNonLiveKernelOutcome},
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationAuthorizationWorkEvidence,
    WorthQueryApplicationProjection, WorthQueryApplicationQueryAccessReceipt,
};

pub(super) fn finalize_one_shot<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    plan: WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    kernel: RawNonLiveKernelOutcome,
    authorization_work: WorthQueryApplicationAuthorizationWorkEvidence,
) -> Result<
    WorthQueryApplicationOneShotResult<Query, QueryResult>,
    WorthQueryApplicationOneShotDenial,
>
where
    Schema: ApplicationSchema,
    QueryResult: WorthQueryApplicationProjection<Schema, Query>,
{
    let request = plan.controls.request_scope();
    let basis_identity = plan.basis.identity().clone();
    let basis_version = plan.basis.version_id();
    let released = plan.basis.release().released();
    if !released {
        return Err(denial(
            WorthQueryApplicationOneShotDenialKind::BasisReleaseFailed,
            plan.query.name(),
        ));
    }
    validate_basis_lifetime(&plan.controls, plan.query.name())?;
    admit_request(request, plan.query.name())?;
    validate_authentication_lifetime(plan.principal, plan.query.name())?;

    let projected = project_non_live_kernel::<Schema, Query, QueryResult, _>(
        kernel,
        || admit_request(request, plan.query.name()),
        |projection: crate::domain_computation::primary_graph::WorthQueryApplicationProjectionDenial| {
            denial(
                WorthQueryApplicationOneShotDenialKind::Projection(projection.kind()),
                projection.subject(),
            )
        },
    )?;
    admit_request(request, plan.query.name())?;
    validate_authentication_lifetime(plan.principal, plan.query.name())?;
    let (rows, kernel_receipt) = projected.into_parts();
    let receipt = WorthQueryApplicationQueryAccessReceipt::from_non_live_kernel(
        WorthQueryApplicationQueryReceiptIdentity {
            query_identity: plan.query.identity().clone(),
            parameter_binding_identity: *plan.parameters.identity(),
            graph_authority_identity: plan.graph_authority_identity,
            provider_identity: plan.provider_identity,
        },
        WorthQueryApplicationQueryReceiptBasis {
            identity: basis_identity,
            version: basis_version,
            posture: plan.controls.basis_posture(),
            lane: plan.controls.lane(),
            consistency: plan.controls.consistency(),
            freshness: plan.controls.freshness(),
            released,
        },
        plan.graph_read_plan,
        plan.canonical_work,
        authorization_work,
        kernel_receipt,
    );
    Ok(WorthQueryApplicationOneShotResult {
        rows,
        receipt,
        _query: PhantomData,
    })
}
