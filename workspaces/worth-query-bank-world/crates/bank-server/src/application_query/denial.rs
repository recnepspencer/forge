use worth_query_host::facade::domain::{
    WorthQueryApplicationCapabilityInstallationDenial, WorthQueryApplicationQueryInstallationDenial,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationContinuationDenial, WorthQueryApplicationLiveOpenDenial,
    WorthQueryApplicationOneShotDenial, WorthQueryApplicationPreviewSessionDenial,
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryBoundedLaneDenial,
    WorthQueryEntityResolutionDenial, WorthQueryOperationAuthorizationDenial,
};

#[derive(Debug)]
pub enum BankApplicationQueryDenial {
    Installation(WorthQueryApplicationQueryInstallationDenial),
    CapabilityInstallation(WorthQueryApplicationCapabilityInstallationDenial),
    CapabilityAdmission(WorthQueryOperationAuthorizationDenial),
    ScopeResolution(WorthQueryEntityResolutionDenial),
    Admission(WorthQueryApplicationQueryAdmissionDenial),
    PreviewSession(WorthQueryApplicationPreviewSessionDenial),
    Execution(WorthQueryApplicationOneShotDenial),
    PreviewExecution(WorthQueryBoundedLaneDenial),
    HistoricalExecution(WorthQueryBoundedLaneDenial),
    ContinuationExecution(WorthQueryApplicationContinuationDenial),
    LiveOpen(WorthQueryApplicationLiveOpenDenial),
}
