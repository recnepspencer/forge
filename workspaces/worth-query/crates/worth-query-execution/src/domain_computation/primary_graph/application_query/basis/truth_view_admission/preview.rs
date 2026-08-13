use std::marker::PhantomData;

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_schema::ApplicationSchema;

use super::super::{WorthQueryApplicationPreviewBasis, WorthQueryApplicationPreviewSession};
use super::validation::{denial, validate_preview_session, validate_truth_view_request};
use crate::domain_computation::primary_graph::{
    application_query::{
        WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
    },
    WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
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
            .ordinary()
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
}
