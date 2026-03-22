use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::diagnostics::facade::RelationalDiagnosticsFacade;
use crate::logic::runtime::{RelationalReplayRecord, RelationalRuntime};
use crate::publication::bundle::PublicationBundle;
use crate::publication::cdc::data::{
    SubscriberResumeRequest, SubscriberStreamBatch, SubscriberStreamFailure,
};
use crate::publication::patch::data::{PatchStreamBatch, PatchStreamReadError, PatchStreamRequest};
use crate::validation::data::InvariantExecutionPoint;

pub struct PublicationAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn publication_access(&self) -> PublicationAccess<'_> {
        PublicationAccess::new(self)
    }
}

impl<'runtime> PublicationAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn diagnostics(&self) -> RelationalDiagnosticsFacade {
        RelationalDiagnosticsFacade {
            artifacts: self.diagnostic_artifacts().to_vec(),
        }
    }

    pub fn diagnostic_artifacts(
        &self,
    ) -> &[crate::diagnostics::data::RelationalDiagnosticArtifact] {
        &self.runtime.publication.diagnostics
    }

    pub fn diagnostics_since(
        &self,
        start: usize,
    ) -> Vec<crate::diagnostics::data::RelationalDiagnosticArtifact> {
        self.runtime.publication.diagnostics[start..].to_vec()
    }

    pub fn latest_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.runtime.publication.latest_bundle.as_ref()
    }

    pub fn latest_patch(&self) -> Option<&crate::publication::patch::data::RelationalPatchRecord> {
        self.latest_bundle().map(|bundle| &bundle.patch)
    }

    pub fn latest_replay(&self) -> Option<&RelationalReplayRecord> {
        self.latest_bundle().map(|bundle| &bundle.replay)
    }

    pub fn read_patch_stream(
        &self,
        request: PatchStreamRequest,
    ) -> Result<PatchStreamBatch, PatchStreamReadError> {
        crate::publication::patch::logic::read_patch_stream(self.runtime, request)
    }

    pub fn read_subscriber_stream(
        &self,
        request: SubscriberResumeRequest,
    ) -> Result<SubscriberStreamBatch, SubscriberStreamFailure> {
        crate::publication::cdc::access::read_subscriber_stream(self.runtime, request)
    }
}

pub(crate) fn publication_failure_diagnostic(
    code: crate::diagnostics::data::DiagnosticCode,
    detail: String,
    fields: serde_json::Value,
) -> RelationalDiagnosticsEntry {
    RelationalDiagnosticsEntry {
        code,
        message: detail,
        fields: serde_json::json!({
            "execution_point": InvariantExecutionPoint::SnapshotPublication.diagnostic_label(),
            "failure": fields,
        }),
    }
}
