use worth_query_admission::facade::{
    application_query::WorthQueryApplicationQueryLane,
    authenticated_principal::{WorthQueryRequestInterruption, WorthQueryRequestScope},
};
use worth_query_declaration::facade::application_schema::ApplicationSchema;

use super::access_receipt::{
    WorthQueryApplicationQueryReceiptBasis, WorthQueryApplicationQueryReceiptIdentity,
};
use super::authorized_read::{execute_authorized_read, refresh_governed_authorization};
use super::read_execution::{
    project_non_live_kernel, read_bounded_root_rows, NonLiveKernelReceiptEvidence,
    RawNonLiveKernelOutcome,
};
use super::{
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationAuthorizationWorkEvidence,
    WorthQueryApplicationProjection, WorthQueryApplicationQueryAccessReceipt,
};
use crate::domain_computation::primary_graph::{
    WorthQueryAuthenticatedPrincipal, WorthQueryPrimaryGraphApplicationRuntime,
};

mod denial;
use denial::{denial, map_authorized_read_denial};
pub use denial::{WorthQueryBoundedLaneDenial, WorthQueryBoundedLaneDenialKind};

pub(super) struct WorthQueryBoundedLaneResult<QueryResult> {
    rows: Vec<QueryResult>,
    receipt: WorthQueryApplicationQueryAccessReceipt,
}

impl<QueryResult> WorthQueryBoundedLaneResult<QueryResult> {
    pub(super) fn into_parts(self) -> (Vec<QueryResult>, WorthQueryApplicationQueryAccessReceipt) {
        (self.rows, self.receipt)
    }
}

struct WorthQueryBoundedReadOutcome {
    kernel: RawNonLiveKernelOutcome,
    authorization_work: WorthQueryApplicationAuthorizationWorkEvidence,
    read_proof: crate::domain_computation::provider_session::WorthQuerySessionGraphReadProof,
}

struct WorthQueryReleasedBoundedProjection<QueryResult> {
    rows: Vec<QueryResult>,
    kernel_receipt: NonLiveKernelReceiptEvidence,
    basis_release: super::WorthQueryApplicationBasisReleaseReceipt,
}

struct WorthQueryBoundedReceiptContext {
    subject: String,
    identity: WorthQueryApplicationQueryReceiptIdentity,
    basis: WorthQueryApplicationQueryReceiptBasis,
    graph_work: crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases,
    disclosure: super::disclosure::WorthQueryApplicationDisclosureReceipt,
}

pub(super) fn execute_bounded_lane<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    mut plan: WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    expected_lane: WorthQueryApplicationQueryLane,
) -> Result<WorthQueryBoundedLaneResult<QueryResult>, WorthQueryBoundedLaneDenial>
where
    Schema: ApplicationSchema,
    QueryResult: WorthQueryApplicationProjection<Schema, Query>,
{
    admit_bounded_lane(application, &mut plan, expected_lane)?;
    let preview_session_guard = if expected_lane == WorthQueryApplicationQueryLane::Preview {
        Some(
            plan.basis
                .preview_session_liveness()
                .and_then(|liveness| liveness.admit_active_session())
                .ok_or_else(|| {
                    denial(
                        WorthQueryBoundedLaneDenialKind::StalePreviewSession,
                        plan.query.name(),
                    )
                })?,
        )
    } else {
        None
    };
    let read = execute_bounded_read(application, &plan)?;
    drop(preview_session_guard);
    finalize_bounded_lane(application, plan, read)
}

fn finalize_bounded_lane<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
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
    read: WorthQueryBoundedReadOutcome,
) -> Result<WorthQueryBoundedLaneResult<QueryResult>, WorthQueryBoundedLaneDenial>
where
    Schema: ApplicationSchema,
    QueryResult: WorthQueryApplicationProjection<Schema, Query>,
{
    let WorthQueryBoundedReadOutcome {
        kernel,
        authorization_work,
        read_proof,
    } = read;
    let basis_identity = plan.basis.identity().clone();
    let basis_version = plan.basis.version_id();
    let basis_release = plan.basis.release();
    if !basis_release.released() {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::BasisReleaseFailed,
            plan.query.name(),
        ));
    }
    let request = plan.controls.request_scope().clone();
    admit_request(&request, plan.query.name())?;
    validate_authentication_lifetime(application, plan.principal, plan.query.name())?;
    let projected = project_non_live_kernel::<Schema, Query, QueryResult, _>(
        kernel,
        &plan.governance,
        || admit_request(&request, plan.query.name()),
        |projection| {
            denial(
                WorthQueryBoundedLaneDenialKind::Projection(projection.kind()),
                projection.subject(),
            )
        },
    )?;
    admit_request(&request, plan.query.name())?;
    validate_authentication_lifetime(application, plan.principal, plan.query.name())?;
    let (rows, kernel_receipt) = projected.into_parts();
    let receipt_context = WorthQueryBoundedReceiptContext {
        subject: plan.query.name().to_string(),
        identity: WorthQueryApplicationQueryReceiptIdentity {
            query_identity: plan.query.identity().clone(),
            parameter_binding_identity: *plan.parameters.identity(),
            graph_authority_identity: plan.graph_authority_identity,
            provider_identity: plan.provider_identity,
        },
        basis: WorthQueryApplicationQueryReceiptBasis {
            identity: basis_identity,
            version: basis_version,
            posture: plan.controls.basis_posture(),
            lane: plan.controls.lane(),
            consistency: plan.controls.consistency(),
            freshness: plan.controls.freshness(),
            released: true,
        },
        graph_work: plan.graph_work,
        canonical_work: plan.canonical_work,
        disclosure: plan.governance.receipt(),
    };
    complete_bounded_lane(
        receipt_context,
        authorization_work,
        read_proof,
        WorthQueryReleasedBoundedProjection {
            rows,
            kernel_receipt,
            basis_release,
        },
    )
}

fn complete_bounded_lane<QueryResult>(
    context: WorthQueryBoundedReceiptContext,
    authorization_work: WorthQueryApplicationAuthorizationWorkEvidence,
    read_proof: crate::domain_computation::provider_session::WorthQuerySessionGraphReadProof,
    projected: WorthQueryReleasedBoundedProjection<QueryResult>,
) -> Result<WorthQueryBoundedLaneResult<QueryResult>, WorthQueryBoundedLaneDenial> {
    let read_completion = context
        .graph_work
        .complete_query_read(
            read_proof,
            projected.kernel_receipt.observed_graph_read_work(),
            projected.basis_release,
        )
        .map_err(|_| {
            denial(
                WorthQueryBoundedLaneDenialKind::ForeignPlan,
                context.subject,
            )
        })?;
    let receipt = WorthQueryApplicationQueryAccessReceipt::from_non_live_kernel(
        context.identity,
        context.basis,
        read_completion,
        context.canonical_work,
        authorization_work,
        context.disclosure,
        projected.kernel_receipt,
    );
    Ok(WorthQueryBoundedLaneResult {
        rows: projected.rows,
        receipt,
    })
}

fn admit_bounded_lane<Schema, Query, Parameters, QueryResult, Principal, PrincipalIdentity, Scope>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    plan: &mut WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    expected_lane: WorthQueryApplicationQueryLane,
) -> Result<(), WorthQueryBoundedLaneDenial>
where
    Schema: ApplicationSchema,
{
    validate_plan_owner(application, plan)?;
    if plan.controls.lane() != expected_lane {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::ForeignPlan,
            plan.query.name(),
        ));
    }
    admit_request(plan.controls.request_scope(), plan.query.name())?;
    if plan.controls.basis_is_expired() {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::ExpiredBasis,
            plan.query.name(),
        ));
    }
    validate_authentication_lifetime(application, plan.principal, plan.query.name())?;
    if !plan.basis.is_live() {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::BasisUnavailable,
            plan.query.name(),
        ));
    }
    refresh_governed_authorization(application, plan)
        .map_err(|read| map_authorized_read_denial(read, plan.query.name()))?;
    Ok(())
}

fn execute_bounded_read<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    plan: &WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
) -> Result<WorthQueryBoundedReadOutcome, WorthQueryBoundedLaneDenial>
where
    Schema: ApplicationSchema,
{
    application.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryBoundedLaneDenialKind::StaleInstalledQuery,
            plan.query.name(),
        )
    })?;
    let result_buffer = application.result_buffers.reserve(
        plan.graph_read_plan()
            .budget_check()
            .max_inline_result_bytes(),
    );
    let (kernel, authorization_work, read_proof) =
        execute_authorized_read(application, plan, |runtime, graph, plan| {
            read_bounded_root_rows(runtime, graph, plan, result_buffer)
        })
        .map_err(|read| map_authorized_read_denial(read, plan.query.name()))?;
    Ok(WorthQueryBoundedReadOutcome {
        kernel,
        authorization_work,
        read_proof,
    })
}

fn validate_plan_owner<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    plan: &WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
) -> Result<(), WorthQueryBoundedLaneDenial>
where
    Schema: ApplicationSchema,
{
    if plan.runtime_authority != application.runtime.authority_identity() {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::ForeignPlan,
            plan.query.name(),
        ));
    }
    application
        .runtime
        .installed_packages()
        .validate_application_schema(&application.installed_schema)
        .map_err(|_| {
            denial(
                WorthQueryBoundedLaneDenialKind::StaleInstalledQuery,
                plan.query.name(),
            )
        })?;
    application
        .installed_schema
        .validate_installed_query(plan.query)
        .map_err(|_| {
            denial(
                WorthQueryBoundedLaneDenialKind::StaleInstalledQuery,
                plan.query.name(),
            )
        })
}

fn validate_authentication_lifetime<Schema, Principal, PrincipalIdentity>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    subject: &str,
) -> Result<(), WorthQueryBoundedLaneDenial> {
    if application.authentication_is_expired(principal.valid_until()) {
        Err(denial(
            WorthQueryBoundedLaneDenialKind::StalePrincipal,
            subject,
        ))
    } else {
        Ok(())
    }
}

fn admit_request(
    request: &WorthQueryRequestScope,
    subject: &str,
) -> Result<(), WorthQueryBoundedLaneDenial> {
    match request.interruption() {
        Some(WorthQueryRequestInterruption::Cancelled) => {
            Err(denial(WorthQueryBoundedLaneDenialKind::Cancelled, subject))
        }
        Some(WorthQueryRequestInterruption::DeadlineExceeded) => Err(denial(
            WorthQueryBoundedLaneDenialKind::DeadlineExceeded,
            subject,
        )),
        None => Ok(()),
    }
}
