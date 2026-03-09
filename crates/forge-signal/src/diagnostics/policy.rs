use serde::{Deserialize, Serialize};

use crate::diagnostics::profile::DiagnosticsProfile;

/// Enforced operational policy derived from a public diagnostics profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsPolicy {
    pub profile: DiagnosticsProfile,
    pub history_limit: usize,
    pub detail_limit: usize,
    pub retain_history_details: bool,
    pub retain_flow_explanation: bool,
    pub retain_latest_failure_context: bool,
    pub retain_stage_details: bool,
    pub capture_forensic_failure_context: bool,
}

impl DiagnosticsPolicy {
    pub fn from_profile(profile: DiagnosticsProfile) -> Self {
        Self {
            profile,
            history_limit: profile.history_limit(),
            detail_limit: profile.detail_limit(),
            retain_history_details: profile.retains_history_details(),
            retain_flow_explanation: profile.retains_flow_explanation(),
            retain_latest_failure_context: !matches!(profile, DiagnosticsProfile::Operational),
            retain_stage_details: matches!(
                profile,
                DiagnosticsProfile::Development | DiagnosticsProfile::Forensic
            ),
            capture_forensic_failure_context: matches!(profile, DiagnosticsProfile::Forensic),
        }
    }
}
