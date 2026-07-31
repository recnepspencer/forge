use std::marker::PhantomData;

use super::{
    WorthQueryApplicationHistoricalBasis, WorthQueryApplicationHistoricalRead,
    WorthQueryApplicationPreviewBasis, WorthQueryApplicationPreviewSession,
};
use crate::domain_computation::primary_graph::application_query::{
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
    pub fn admit_application_historical_basis(
        &self,
        read: WorthQueryApplicationHistoricalRead,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryApplicationHistoricalBasis<Schema>,
        WorthQueryApplicationQueryAdmissionDenial,
    > {
        validate_truth_view_request(request)?;
        let provider_runtime_instance_id = read.provider_runtime_instance_id();
        let evaluation = self
            .bridge
            .evaluate(read.into_evaluation_request())
            .map_err(|error| {
                denial(
                    WorthQueryApplicationQueryAdmissionDenialKind::TruthViewUnavailable,
                    error.to_string(),
                )
            })?;
        let lease = self
            .relational_source
            .admit_truth_view_execution_basis(&evaluation)
            .map_err(|error| {
                denial(
                    WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
                    format!("{error:?}"),
                )
            })?;
        if provider_runtime_instance_id
            .is_some_and(|expected| lease.identity().runtime_instance_id() != expected)
        {
            let _ = lease.release();
            return Err(denial(
                WorthQueryApplicationQueryAdmissionDenialKind::ForeignHistoricalReceipt,
                "application commit receipt",
            ));
        }
        self.ensure_truth_view_indexes(lease.version_id())?;
        let lease = self.basis_leases.register(lease);
        validate_truth_view_request(request)?;
        Ok(WorthQueryApplicationHistoricalBasis {
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
        })
    }

    pub fn admit_application_preview_basis(
        &self,
        session: &WorthQueryApplicationPreviewSession<Schema>,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryApplicationPreviewBasis<Schema>, WorthQueryApplicationQueryAdmissionDenial>
    {
        validate_truth_view_request(request)?;
        validate_preview_session(self, session)?;
        let handle = session.handle().ok_or_else(|| {
            denial(
                WorthQueryApplicationQueryAdmissionDenialKind::StalePreviewSession,
                "closed preview session",
            )
        })?;
        let session_liveness = handle.liveness_observer();
        let evaluation = self
            .bridge
            .evaluate(handle.compare_to_main().speculative_evaluation_request())
            .map_err(|error| {
                denial(
                    WorthQueryApplicationQueryAdmissionDenialKind::TruthViewUnavailable,
                    error.to_string(),
                )
            })?;
        let lease = self
            .relational_source
            .admit_truth_view_execution_basis(&evaluation)
            .map_err(|error| {
                denial(
                    WorthQueryApplicationQueryAdmissionDenialKind::BasisUnavailable,
                    format!("{error:?}"),
                )
            })?;
        self.ensure_truth_view_indexes(lease.version_id())?;
        let lease = self.basis_leases.register(lease);
        validate_truth_view_request(request)?;
        validate_preview_session(self, session)?;
        Ok(WorthQueryApplicationPreviewBasis {
            runtime_authority: self.runtime.authority_identity(),
            schema_binding: self.installed_schema.binding_identity(),
            graph_authority_identity: self
                .primary_graph_authority
                .authority_identity()
                .to_string(),
            provider_identity: self.primary_graph_authority.provider_identity().to_string(),
            preview_session_identity: session.identity().clone(),
            preview_session_liveness: session_liveness,
            expires_at: request.deadline(),
            lease,
            _schema: PhantomData,
        })
    }

    fn ensure_truth_view_indexes(
        &self,
        version: worth_relational::facade::identity::VersionId,
    ) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
                "primary graph",
            )
        })?;
        let handle = graph.integration_handle();
        handle
            .with_runtime_mut(|runtime| handle.ensure_primary_indexes_for_version(runtime, version))
            .map_err(|detail| {
                denial(
                    WorthQueryApplicationQueryAdmissionDenialKind::RuntimeSupportUnavailable,
                    detail,
                )
            })
    }
}

fn validate_preview_session<Schema>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    session: &WorthQueryApplicationPreviewSession<Schema>,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial>
where
    Schema: ApplicationSchema,
{
    if session.runtime_authority != application.runtime.authority_identity()
        || session.schema_binding != application.installed_schema.binding_identity()
    {
        Err(denial(
            WorthQueryApplicationQueryAdmissionDenialKind::ForeignPreviewSession,
            "application preview session",
        ))
    } else {
        Ok(())
    }
}

fn validate_truth_view_request(
    request: &WorthQueryRequestScope,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
    match request.interruption() {
        Some(WorthQueryRequestInterruption::Cancelled) => Err(denial(
            WorthQueryApplicationQueryAdmissionDenialKind::Cancelled,
            "truth-view basis",
        )),
        Some(WorthQueryRequestInterruption::DeadlineExceeded) => Err(denial(
            WorthQueryApplicationQueryAdmissionDenialKind::DeadlineExceeded,
            "truth-view basis",
        )),
        None => Ok(()),
    }
}

fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
