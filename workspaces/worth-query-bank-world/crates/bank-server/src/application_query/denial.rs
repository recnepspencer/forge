use worth_query_host::facade::domain::WorthQueryApplicationQueryInstallationDenial;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationContinuationDenial, WorthQueryApplicationLiveOpenDenial,
    WorthQueryApplicationOneShotDenial, WorthQueryApplicationPreviewSessionDenial,
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryBoundedLaneDenial,
    WorthQueryEntityResolutionDenial,
};

#[derive(Debug)]
pub enum BankApplicationQueryDenial {
    Installation(WorthQueryApplicationQueryInstallationDenial),
    ScopeResolution(WorthQueryEntityResolutionDenial),
    Admission(WorthQueryApplicationQueryAdmissionDenial),
    PreviewSession(WorthQueryApplicationPreviewSessionDenial),
    Execution(WorthQueryApplicationOneShotDenial),
    PreviewExecution(WorthQueryBoundedLaneDenial),
    HistoricalExecution(WorthQueryBoundedLaneDenial),
    ContinuationExecution(WorthQueryApplicationContinuationDenial),
    LiveOpen(WorthQueryApplicationLiveOpenDenial),
}
