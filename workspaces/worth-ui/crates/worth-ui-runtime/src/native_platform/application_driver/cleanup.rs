use worth_ui_host_native::{
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
};

use super::{shutdown_observation::UiNativeDriverShutdownEvidence, UiNativeApplicationDriver};

pub(super) enum UiNativeApplicationDriverCleanup {
    RuntimeLaunch(crate::runtime::WorthUiRuntimeLaunchDenial),
    Application {
        cleanup: crate::facade::WorthUiNativeApplicationCleanup,
        evidence: UiNativeDriverShutdownEvidence,
    },
    HostSession(crate::facade::WorthUiHostSessionReleaseRecovery),
    UnresolvedApplication,
}

pub(super) struct UiNativeApplicationDriverCleanupCompletion {
    pub(super) query_close: crate::facade::entry::UiNativeApplicationQueryCloseObservation,
    pub(super) evidence: UiNativeDriverShutdownEvidence,
}

impl UiNativeEventLoopClientCleanup for UiNativeApplicationDriver {
    fn retry(self: Box<Self>) -> UiNativeEventLoopClientClose {
        <Self as UiNativeEventLoopClient>::close(*self)
    }
}

impl UiNativeEventLoopClientCleanup for UiNativeApplicationDriverCleanup {
    fn retry(self: Box<Self>) -> UiNativeEventLoopClientClose {
        match (*self).retry() {
            Ok(completion) => completion.into_client_close(),
            Err(cleanup) => UiNativeEventLoopClientClose::Incomplete(Box::new(cleanup)),
        }
    }
}

impl UiNativeApplicationDriverCleanup {
    pub(super) fn retry(self) -> Result<UiNativeApplicationDriverCleanupCompletion, Self> {
        match self {
            Self::RuntimeLaunch(cleanup) => cleanup
                .retry_host_session_cleanup()
                .map(|_| UiNativeApplicationDriverCleanupCompletion {
                    query_close:
                        crate::facade::entry::UiNativeApplicationQueryCloseObservation::empty_complete(),
                    evidence: UiNativeDriverShutdownEvidence::empty(),
                })
                .map_err(Self::RuntimeLaunch),
            Self::Application { cleanup, evidence } => match cleanup.retry() {
                Ok(query_close) => Ok(UiNativeApplicationDriverCleanupCompletion {
                    query_close,
                    evidence,
                }),
                Err(cleanup) => Err(Self::Application { cleanup, evidence }),
            },
            Self::HostSession(cleanup) => cleanup
                .retry()
                .map(|_| UiNativeApplicationDriverCleanupCompletion {
                    query_close:
                        crate::facade::entry::UiNativeApplicationQueryCloseObservation::empty_complete(),
                    evidence: UiNativeDriverShutdownEvidence::empty(),
                })
                .map_err(Self::HostSession),
            Self::UnresolvedApplication => Err(Self::UnresolvedApplication),
        }
    }
}

impl UiNativeApplicationDriverCleanupCompletion {
    pub(super) fn into_client_close(self) -> UiNativeEventLoopClientClose {
        if self.query_close.query_close_complete()
            && self.query_close.transition_trace_complete()
            && self.query_close.intent_resources_empty()
        {
            UiNativeEventLoopClientClose::CompleteWithObservation(
                self.evidence.finalize(&self.query_close),
            )
        } else {
            UiNativeEventLoopClientClose::Incomplete(Box::new(
                UiNativeApplicationDriverCleanup::UnresolvedApplication,
            ))
        }
    }
}
