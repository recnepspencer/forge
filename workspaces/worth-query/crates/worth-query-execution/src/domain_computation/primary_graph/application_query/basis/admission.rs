use std::marker::PhantomData;
use std::time::Instant;

use super::{
    authority::WorthQueryApplicationPinnedBasisParts, WorthQueryApplicationPinnedBasis,
    WorthQueryApplicationPinnedBasisDenial, WorthQueryApplicationPinnedBasisDenialKind,
};
use crate::domain_computation::primary_graph::application_query::{
    controls::WorthQueryApplicationQueryBasis, resource_lifecycle::WorthQueryApplicationBasisLease,
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use worth_query_admission::facade::authenticated_principal::{
    WorthQueryRequestInterruption, WorthQueryRequestScope,
};
use worth_query_declaration::facade::application_schema::ApplicationSchema;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn pin_current_application_query_basis(
        &self,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryApplicationPinnedBasis<Schema>, WorthQueryApplicationPinnedBasisDenial>
    {
        admit_pin_request(request)?;
        let lease = admit_current_execution_basis(self).map_err(|denial| {
            WorthQueryApplicationPinnedBasisDenial::new(
                match denial.kind() {
                    WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable => {
                        WorthQueryApplicationPinnedBasisDenialKind::BasisUnavailable
                    }
                    _ => WorthQueryApplicationPinnedBasisDenialKind::RuntimeSupportUnavailable,
                },
                denial.subject(),
            )
        })?;
        admit_pin_request(request)?;
        Ok(WorthQueryApplicationPinnedBasis::new(
            WorthQueryApplicationPinnedBasisParts {
                runtime_authority: self.runtime.authority_identity(),
                schema_binding: self.installed_schema.binding_identity(),
                graph_authority_identity: self
                    .primary_graph_authority
                    .authority_identity()
                    .to_string(),
                provider_identity: self.primary_graph_authority.provider_identity().to_string(),
                expires_at: request.deadline(),
                lease,
                _schema: PhantomData,
            },
        ))
    }
}

pub(in crate::domain_computation::primary_graph::application_query) fn admit_application_query_basis<
    Schema,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    basis: WorthQueryApplicationQueryBasis<Schema>,
) -> Result<WorthQueryApplicationBasisLease, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    match basis {
        WorthQueryApplicationQueryBasis::Current => admit_current_execution_basis(application),
        WorthQueryApplicationQueryBasis::Pinned(pinned) => {
            admit_pinned_execution_basis(application, pinned)
        }
        WorthQueryApplicationQueryBasis::Historical(historical) => {
            admit_historical_execution_basis(application, historical)
        }
        WorthQueryApplicationQueryBasis::Preview(preview) => {
            admit_preview_execution_basis(application, preview)
        }
        WorthQueryApplicationQueryBasis::Continuation(version_id) => {
            let lease = admit_version_execution_basis(application, version_id)?;
            Ok(application.basis_leases.register(lease))
        }
    }
}

fn admit_historical_execution_basis<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    basis: super::WorthQueryApplicationHistoricalBasis<Schema>,
) -> Result<WorthQueryApplicationBasisLease, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    validate_truth_view_authority(application, basis.runtime_authority, &basis.schema_binding)?;
    validate_truth_view_provider(
        application,
        &basis.graph_authority_identity,
        &basis.provider_identity,
    )?;
    validate_truth_view_lease(basis.expires_at, basis.is_live())?;
    Ok(basis.into_lease())
}

fn admit_preview_execution_basis<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    basis: super::WorthQueryApplicationPreviewBasis<Schema>,
) -> Result<WorthQueryApplicationBasisLease, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    validate_truth_view_authority(application, basis.runtime_authority, &basis.schema_binding)?;
    validate_truth_view_provider(
        application,
        &basis.graph_authority_identity,
        &basis.provider_identity,
    )?;
    validate_truth_view_lease(basis.expires_at, basis.is_live())?;
    let guard = basis
        .preview_session_liveness
        .admit_active_session()
        .ok_or_else(|| {
            admission_denial(
                WorthQueryApplicationQueryAdmissionDenialKind::StalePreviewSession,
                "preview session",
            )
        })?;
    drop(guard);
    let (lease, liveness) = basis.into_lease_and_liveness();
    Ok(lease.bind_preview_session(liveness))
}

fn validate_truth_view_authority<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    schema_binding: &worth_query_installation::facade::ApplicationSchemaBindingIdentity,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    if runtime_authority != application.runtime.authority_identity() {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::ForeignBasis,
            "runtime authority",
        ));
    }
    if schema_binding != &application.installed_schema.binding_identity() {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::StaleBasis,
            "installed schema",
        ));
    }
    Ok(())
}

fn validate_truth_view_provider<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    graph_authority_identity: &str,
    provider_identity: &str,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    if graph_authority_identity != application.primary_graph_authority.authority_identity()
        || provider_identity != application.primary_graph_authority.provider_identity()
    {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::WrongProviderBasis,
            "primary graph provider",
        ));
    }
    Ok(())
}

fn validate_truth_view_lease(
    expires_at: Instant,
    is_live: bool,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
    if Instant::now() >= expires_at {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::ExpiredBasis,
            "truth-view basis deadline",
        ));
    }
    if !is_live {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
            "truth-view basis lease",
        ));
    }
    Ok(())
}

fn admit_current_execution_basis<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
) -> Result<WorthQueryApplicationBasisLease, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    let graph = application.runtime.primary_graph().ok_or_else(|| {
        admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
            "primary graph",
        )
    })?;
    let handle = graph.integration_handle();
    let version = handle
        .with_runtime_mut(|runtime| {
            handle.ensure_primary_indexes_current(runtime).map(|()| {
                runtime
                    .history()
                    .latest_commit()
                    .map_or(worth_relational::facade::identity::VersionId(0), |commit| {
                        commit.version_id
                    })
            })
        })
        .map_err(|detail| {
            admission_denial(
                WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
                detail,
            )
        })?;
    let lease = admit_version_execution_basis(application, version)?;
    Ok(application.basis_leases.register(lease))
}

fn admit_version_execution_basis<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    version: worth_relational::facade::identity::VersionId,
) -> Result<
    worth_relational::facade::runtime::RelationalExecutionBasisLease,
    WorthQueryApplicationQueryAdmissionDenial,
>
where
    Schema: ApplicationSchema,
{
    let branch = application
        .relational_source
        .resolve_execution_basis_branch(version)
        .ok_or_else(|| {
            admission_denial(
                WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
                "Relational version branch",
            )
        })?;
    application
        .relational_source
        .admit_execution_basis(&branch, version)
        .map_err(|basis| {
            admission_denial(
                WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
                basis.detail(),
            )
        })
}

fn admit_pinned_execution_basis<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    pinned: WorthQueryApplicationPinnedBasis<Schema>,
) -> Result<WorthQueryApplicationBasisLease, WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    if pinned.runtime_authority() != application.runtime.authority_identity() {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::ForeignBasis,
            "runtime authority",
        ));
    }
    if pinned.schema_binding() != &application.installed_schema.binding_identity() {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::StaleBasis,
            "installed schema",
        ));
    }
    if pinned.graph_authority_identity() != application.primary_graph_authority.authority_identity()
        || pinned.provider_identity() != application.primary_graph_authority.provider_identity()
    {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::WrongProviderBasis,
            "primary graph provider",
        ));
    }
    if Instant::now() >= pinned.expires_at() {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::ExpiredBasis,
            "pinned basis deadline",
        ));
    }
    if !pinned.is_live() {
        return Err(admission_denial(
            WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
            "pinned basis lease",
        ));
    }
    Ok(pinned.into_lease())
}

fn admit_pin_request(
    request: &WorthQueryRequestScope,
) -> Result<(), WorthQueryApplicationPinnedBasisDenial> {
    match request.interruption() {
        Some(WorthQueryRequestInterruption::Cancelled) => {
            Err(WorthQueryApplicationPinnedBasisDenial::new(
                WorthQueryApplicationPinnedBasisDenialKind::Cancelled,
                "request",
            ))
        }
        Some(WorthQueryRequestInterruption::DeadlineExceeded) => {
            Err(WorthQueryApplicationPinnedBasisDenial::new(
                WorthQueryApplicationPinnedBasisDenialKind::DeadlineExceeded,
                "request",
            ))
        }
        None => Ok(()),
    }
}

fn admission_denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
