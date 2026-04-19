use super::denied::{
    CorrespondenceHistoricalDeniedEnvelope, HistoricalPathAdmissionDeniedEnvelope,
    HistoricalPathDeniedEnvelope,
};
use super::success::{
    CorrespondenceHistoricalAmbiguityEnvelope, CorrespondenceHistoricalDisagreementEnvelope,
    CorrespondenceHistoricalSuccessEnvelope,
};
use super::view::MetadataPreservingHistoricalResultView;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CorrespondenceHistoricalEnvelope {
    Success(CorrespondenceHistoricalSuccessEnvelope),
    Ambiguity(CorrespondenceHistoricalAmbiguityEnvelope),
    Disagreement(CorrespondenceHistoricalDisagreementEnvelope),
    CorrespondenceDenied(CorrespondenceHistoricalDeniedEnvelope),
    HistoricalPathDenied(HistoricalPathDeniedEnvelope),
    HistoricalPathAdmissionDenied(HistoricalPathAdmissionDeniedEnvelope),
}

impl CorrespondenceHistoricalEnvelope {
    pub fn result_view(&self) -> Option<MetadataPreservingHistoricalResultView<'_>> {
        match self {
            Self::Success(envelope) => Some(envelope.result_view()),
            Self::Ambiguity(envelope) => Some(envelope.result_view()),
            Self::Disagreement(envelope) => Some(envelope.result_view()),
            Self::CorrespondenceDenied(_)
            | Self::HistoricalPathDenied(_)
            | Self::HistoricalPathAdmissionDenied(_) => None,
        }
    }
}
