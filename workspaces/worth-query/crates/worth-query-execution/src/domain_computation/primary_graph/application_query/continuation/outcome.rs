use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::ApplicationSchema;

use super::{
    authority::WorthQueryApplicationQueryContinuation,
    denial::WorthQueryApplicationContinuationDenial,
    denial::WorthQueryApplicationContinuationDenialKind,
    WorthQueryApplicationContinuationPageResult,
};
use crate::domain_computation::primary_graph::application_query::{
    access_receipt::{
        WorthQueryApplicationQueryReceiptBasis, WorthQueryApplicationQueryReceiptIdentity,
    },
    execution_validation::validate_request,
    read_execution::{project_non_live_kernel, RawNonLiveKernelOutcome},
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationAuthorizationWorkEvidence,
    WorthQueryApplicationProjection, WorthQueryApplicationQueryAccessReceipt,
};

pub(super) fn finalize_continuation_page<
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
    WorthQueryApplicationContinuationPageResult<Schema, Query, Parameters, QueryResult, Scope>,
    WorthQueryApplicationContinuationDenial,
>
where
    Schema: ApplicationSchema,
    QueryResult: WorthQueryApplicationProjection<Schema, Query>,
{
    let request = plan.controls.request_scope();
    let continuation_contract = plan
        .query
        .continuation()
        .expect("continuation plans retain an installed continuation contract");
    let index_id = plan
        .continuation_index_id
        .expect("continuation plans retain an installed ordered index");
    let basis_identity = plan.basis.identity().clone();
    let basis_version = plan.basis.version_id();
    let next_page_ordinal = plan
        .continuation_state
        .as_ref()
        .map_or(2, |state| state.page_ordinal.saturating_add(1));
    let released = plan.basis.release().released();
    if !released {
        return Err(denial(
            WorthQueryApplicationContinuationDenialKind::BasisReleaseFailed,
            plan.query.name(),
        ));
    }
    validate_request(request)
        .map_err(|validation| denial(map_validation_denial(validation), plan.query.name()))?;
    if plan.principal.is_expired() {
        return Err(denial(
            WorthQueryApplicationContinuationDenialKind::StalePrincipal,
            plan.query.name(),
        ));
    }

    let projected = project_non_live_kernel::<Schema, Query, QueryResult, _>(
        kernel,
        &plan.governance,
        || {
            validate_request(request).map_err(|validation| {
                denial(map_validation_denial(validation), plan.query.name())
            })
        },
        |projection: crate::domain_computation::primary_graph::WorthQueryApplicationProjectionDenial| {
            denial(
                WorthQueryApplicationContinuationDenialKind::Projection(projection.kind()),
                projection.subject(),
            )
        },
    )?;
    let (rows, kernel_receipt) = projected.into_parts();
    let continuation = match (
        kernel_receipt.read.has_more,
        kernel_receipt.read.next_boundary.clone(),
        kernel_receipt.read.ordered_index_generation,
    ) {
        (true, Some(boundary), Some(index_generation)) => {
            Some(WorthQueryApplicationQueryContinuation {
                runtime_authority: plan.runtime_authority.as_u64(),
                schema_binding: plan.query.binding_identity().clone(),
                query_identity: plan.query.identity().clone(),
                query_authority_identity: plan.query.authority_identity().to_string(),
                parameter_basis: plan.parameters.canonical_basis().clone(),
                scope_entity_id: plan.scope.entity_id(),
                continuation_contract_digest: *continuation_contract.digest(),
                graph_authority_identity: plan.graph_authority_identity.clone(),
                provider_identity: plan.provider_identity.clone(),
                basis_version,
                index_id,
                index_generation,
                boundary,
                page_ordinal: next_page_ordinal,
                _marker: PhantomData,
            })
        }
        (false, None, Some(_)) => None,
        _ => {
            return Err(denial(
                WorthQueryApplicationContinuationDenialKind::ContinuationIndexUnavailable,
                plan.query.name(),
            ))
        }
    };
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
        plan.governance.receipt(),
        kernel_receipt,
    );
    Ok(WorthQueryApplicationContinuationPageResult {
        rows,
        continuation,
        receipt,
        _query: PhantomData,
    })
}

fn map_validation_denial(
    denial: crate::domain_computation::primary_graph::application_query::execution_validation::WorthQueryApplicationQueryExecutionValidationDenial,
) -> WorthQueryApplicationContinuationDenialKind {
    use crate::domain_computation::primary_graph::application_query::execution_validation::WorthQueryApplicationQueryExecutionValidationDenial as Validation;
    match denial {
        Validation::Cancelled => WorthQueryApplicationContinuationDenialKind::Cancelled,
        Validation::DeadlineExceeded => {
            WorthQueryApplicationContinuationDenialKind::DeadlineExceeded
        }
        Validation::StalePrincipal => WorthQueryApplicationContinuationDenialKind::StalePrincipal,
        Validation::ExpiredBasis => WorthQueryApplicationContinuationDenialKind::ExpiredBasis,
        Validation::BasisUnavailable => {
            WorthQueryApplicationContinuationDenialKind::BasisUnavailable
        }
        Validation::ForeignPlan => WorthQueryApplicationContinuationDenialKind::ForeignPlan,
        Validation::StaleInstalledQuery => {
            WorthQueryApplicationContinuationDenialKind::StaleInstalledQuery
        }
    }
}

fn denial(
    kind: WorthQueryApplicationContinuationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationContinuationDenial {
    WorthQueryApplicationContinuationDenial::new(kind, subject)
}
