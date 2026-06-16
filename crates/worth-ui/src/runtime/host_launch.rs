use crate::runtime::{WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeFrameEpoch};
use crate::source::WorthUiArtifact;

/// Launch request for creating an active runtime host from canonical artifact truth.
#[derive(Debug)]
pub struct WorthUiRuntimeLaunch {
    pub(crate) artifact: WorthUiArtifact,
    pub(crate) frame_epoch: WorthUiRuntimeFrameEpoch,
    pub(crate) diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeLaunchDenial {
    StalePendingActivation {
        pending_epoch: WorthUiRuntimeFrameEpoch,
        active_epoch: WorthUiRuntimeFrameEpoch,
    },
}
