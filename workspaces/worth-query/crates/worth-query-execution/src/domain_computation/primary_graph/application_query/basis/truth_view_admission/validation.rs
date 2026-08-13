use worth_query_admission::facade::authenticated_principal::{
    WorthQueryRequestInterruption, WorthQueryRequestScope,
};
use worth_query_declaration::facade::application_schema::ApplicationSchema;

use super::super::WorthQueryApplicationPreviewSession;
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
    pub(super) fn ensure_truth_view_indexes(
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

pub(super) fn validate_preview_session<Schema>(
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

pub(super) fn validate_truth_view_request(
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

pub(super) fn denial(
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationQueryAdmissionDenial {
    WorthQueryApplicationQueryAdmissionDenial::new(kind, subject)
}
