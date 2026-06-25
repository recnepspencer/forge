use crate::capability::CapabilitySnapshot;
use crate::runtime::{
    WorthUiObservedAuthoredEdit, WorthUiObservedAuthoredEditDenial, WorthUiRuntimeHost,
    WorthUiSourceWatcher, WorthUiWatchedCandidateSubmission,
    WorthUiWatchedCandidateSubmissionDenial, WorthUiWatcherEvent,
};

impl WorthUiRuntimeHost {
    pub fn source_ingress(
        &self,
        provider: crate::runtime::WorthUiSourceProvider,
    ) -> WorthUiSourceWatcher {
        WorthUiSourceWatcher::new(provider)
    }

    pub fn observe_authored_edit(
        &self,
        snapshot: &CapabilitySnapshot,
        edit: WorthUiObservedAuthoredEdit,
    ) -> Result<WorthUiWatchedCandidateSubmission, WorthUiObservedAuthoredEditResultDenial> {
        let (provider, provider_revision_id) = edit.into_parts();
        let mut session = self.source_ingress(provider).start();
        let batch = session
            .ingest([WorthUiWatcherEvent::provider_revision(provider_revision_id)])
            .map_err(WorthUiObservedAuthoredEditResultDenial::SourceIngress)?;
        batch
            .lower_to_candidate_submission(snapshot, self.active_authoring_snapshot())
            .map_err(WorthUiObservedAuthoredEditResultDenial::CandidateSubmission)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiObservedAuthoredEditResultDenial {
    SourceIngress(crate::runtime::WorthUiSourceIngressDenial),
    CandidateSubmission(WorthUiWatchedCandidateSubmissionDenial),
    InvalidObservedEdit(WorthUiObservedAuthoredEditDenial),
}

impl From<WorthUiObservedAuthoredEditDenial> for WorthUiObservedAuthoredEditResultDenial {
    fn from(value: WorthUiObservedAuthoredEditDenial) -> Self {
        Self::InvalidObservedEdit(value)
    }
}
