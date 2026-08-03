use worth_query_admission::facade::{
    application_query::WorthQueryApplicationQueryLane,
    authenticated_principal::{WorthQueryRequestInterruption, WorthQueryRequestScope},
};
use worth_query_declaration::facade::application_schema::ApplicationSchema;

use super::access_receipt::{
    WorthQueryApplicationQueryReceiptBasis, WorthQueryApplicationQueryReceiptIdentity,
};
use super::authorized_read::{
    execute_authorized_read, refresh_governed_authorization,
    WorthQueryAuthorizedApplicationReadDenial,
};
use super::read_execution::{
    project_non_live_kernel, read_bounded_root_rows, WorthQueryApplicationReadExecutionDenialKind,
};
use super::{
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationProjection,
    WorthQueryApplicationProjectionDenialKind, WorthQueryApplicationQueryAccessReceipt,
};
use crate::domain_computation::primary_graph::{
    WorthQueryAuthenticatedPrincipal, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryPrimaryGraphApplicationRuntime,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBoundedLaneDenialKind {
    ForeignPlan,
    StaleInstalledQuery,
    StalePrincipal,
    StaleScope,
    StaleBasisScope(crate::domain_computation::primary_graph::WorthQueryEntityResolutionDenialKind),
    Authorization(WorthQueryOperationAuthorizationDenialKind),
    Cancelled,
    DeadlineExceeded,
    StalePreviewSession,
    BasisUnavailable,
    ExpiredBasis,
    BasisReleaseFailed,
    PredicateIndexUnavailable,
    PredicateLookupOverflow,
    ResultLimitExceeded,
    CardinalityMismatch,
    TraversalUnavailable,
    ProjectionUnavailable,
    Projection(WorthQueryApplicationProjectionDenialKind),
    ResultBufferLimitExceeded,
    WorkLimitExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundedLaneDenial {
    kind: WorthQueryBoundedLaneDenialKind,
    subject: String,
}

pub(super) struct WorthQueryBoundedLaneResult<QueryResult> {
    rows: Vec<QueryResult>,
    receipt: WorthQueryApplicationQueryAccessReceipt,
}

impl<QueryResult> WorthQueryBoundedLaneResult<QueryResult> {
    pub(super) fn into_parts(self) -> (Vec<QueryResult>, WorthQueryApplicationQueryAccessReceipt) {
        (self.rows, self.receipt)
    }
}

impl WorthQueryBoundedLaneDenial {
    pub const fn kind(&self) -> WorthQueryBoundedLaneDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryBoundedLaneDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application query bounded lane denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryBoundedLaneDenial {}

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
    validate_plan_owner(application, &plan)?;
    if plan.controls.lane() != expected_lane {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::ForeignPlan,
            plan.query.name(),
        ));
    }
    let request = plan.controls.request_scope().clone();
    admit_request(&request, plan.query.name())?;
    if plan.controls.basis_is_expired() {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::ExpiredBasis,
            plan.query.name(),
        ));
    }
    validate_authentication_lifetime(plan.principal, plan.query.name())?;
    if !plan.basis.is_live() {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::BasisUnavailable,
            plan.query.name(),
        ));
    }
    refresh_governed_authorization(application, &mut plan)
        .map_err(|read| map_authorized_read_denial(read, plan.query.name()))?;
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
    let graph = application.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryBoundedLaneDenialKind::StaleInstalledQuery,
            plan.query.name(),
        )
    })?;
    let result_buffer = application.result_buffers.reserve(
        plan.graph_read_plan
            .budget_check()
            .max_inline_result_bytes(),
    );
    let (raw, authorization_work) =
        execute_authorized_read(application, graph, &plan, |runtime, graph, plan| {
            read_bounded_root_rows(runtime, graph, plan, result_buffer)
        })
        .map_err(|read| map_authorized_read_denial(read, plan.query.name()))?;
    drop(preview_session_guard);

    let basis_identity = plan.basis.identity().clone();
    let basis_version = plan.basis.version_id();
    let released = plan.basis.release().released();
    if !released {
        return Err(denial(
            WorthQueryBoundedLaneDenialKind::BasisReleaseFailed,
            plan.query.name(),
        ));
    }
    admit_request(&request, plan.query.name())?;
    validate_authentication_lifetime(plan.principal, plan.query.name())?;
    let projected = project_non_live_kernel::<Schema, Query, QueryResult, _>(
        raw,
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
        plan.governance.receipt(),
        kernel_receipt,
    );
    Ok(WorthQueryBoundedLaneResult { rows, receipt })
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
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    subject: &str,
) -> Result<(), WorthQueryBoundedLaneDenial> {
    if principal.is_expired() {
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

fn map_authorized_read_denial(
    value: WorthQueryAuthorizedApplicationReadDenial,
    subject: &str,
) -> WorthQueryBoundedLaneDenial {
    match value {
        WorthQueryAuthorizedApplicationReadDenial::StalePrincipal => {
            denial(WorthQueryBoundedLaneDenialKind::StalePrincipal, subject)
        }
        WorthQueryAuthorizedApplicationReadDenial::StaleScope => {
            denial(WorthQueryBoundedLaneDenialKind::StaleScope, subject)
        }
        WorthQueryAuthorizedApplicationReadDenial::StaleBasisScope(kind) => denial(
            WorthQueryBoundedLaneDenialKind::StaleBasisScope(kind),
            subject,
        ),
        WorthQueryAuthorizedApplicationReadDenial::Authorization(kind, subject) => denial(
            WorthQueryBoundedLaneDenialKind::Authorization(kind),
            subject,
        ),
        WorthQueryAuthorizedApplicationReadDenial::Read(read) => {
            denial(map_read_denial(read.kind()), read.subject())
        }
    }
}

fn map_read_denial(
    kind: WorthQueryApplicationReadExecutionDenialKind,
) -> WorthQueryBoundedLaneDenialKind {
    match kind {
        WorthQueryApplicationReadExecutionDenialKind::PredicateIndexUnavailable => {
            WorthQueryBoundedLaneDenialKind::PredicateIndexUnavailable
        }
        WorthQueryApplicationReadExecutionDenialKind::PredicateLookupOverflow => {
            WorthQueryBoundedLaneDenialKind::PredicateLookupOverflow
        }
        WorthQueryApplicationReadExecutionDenialKind::ResultLimitExceeded => {
            WorthQueryBoundedLaneDenialKind::ResultLimitExceeded
        }
        WorthQueryApplicationReadExecutionDenialKind::CardinalityMismatch => {
            WorthQueryBoundedLaneDenialKind::CardinalityMismatch
        }
        WorthQueryApplicationReadExecutionDenialKind::ProjectionUnavailable => {
            WorthQueryBoundedLaneDenialKind::ProjectionUnavailable
        }
        WorthQueryApplicationReadExecutionDenialKind::ResultBufferLimitExceeded => {
            WorthQueryBoundedLaneDenialKind::ResultBufferLimitExceeded
        }
        WorthQueryApplicationReadExecutionDenialKind::WorkLimitExceeded => {
            WorthQueryBoundedLaneDenialKind::WorkLimitExceeded
        }
        WorthQueryApplicationReadExecutionDenialKind::TargetIdentityIndexUnavailable
        | WorthQueryApplicationReadExecutionDenialKind::TargetIdentityLookupOverflow
        | WorthQueryApplicationReadExecutionDenialKind::TargetIdentityNotFound => {
            WorthQueryBoundedLaneDenialKind::ProjectionUnavailable
        }
        WorthQueryApplicationReadExecutionDenialKind::TraversalUnavailable
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationIndexUnavailable
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationBoundaryRejected
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationGenerationChanged
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationPageWidthInvalid => {
            WorthQueryBoundedLaneDenialKind::TraversalUnavailable
        }
    }
}

fn denial(
    kind: WorthQueryBoundedLaneDenialKind,
    subject: impl Into<String>,
) -> WorthQueryBoundedLaneDenial {
    WorthQueryBoundedLaneDenial {
        kind,
        subject: subject.into(),
    }
}
