#[path = "correspondence_history/composition.rs"]
mod composition;
#[path = "correspondence_history/denied.rs"]
mod denied;
#[path = "correspondence_history/envelope.rs"]
mod envelope;
#[path = "correspondence_history/success.rs"]
mod success;
#[cfg(test)]
#[path = "correspondence_history/tests.rs"]
mod tests;
#[path = "correspondence_history/view.rs"]
mod view;

pub use composition::{
    compose_correspondence_historical_envelope, compose_historical_admission_denied_envelope,
    compose_historical_path_denied_envelope,
};
pub use denied::{
    CorrespondenceHistoricalDeniedEnvelope, HistoricalPathAdmissionDeniedEnvelope,
    HistoricalPathDeniedEnvelope,
};
pub use envelope::CorrespondenceHistoricalEnvelope;
pub use success::{
    CorrespondenceHistoricalAmbiguityEnvelope, CorrespondenceHistoricalDisagreementEnvelope,
    CorrespondenceHistoricalSuccessEnvelope,
};
pub use view::MetadataPreservingHistoricalResultView;
