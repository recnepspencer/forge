#[derive(Debug, Clone)]
pub(crate) struct BridgeDiagnosticsConfig {
    pub(crate) tier: crate::policy::BridgeDiagnosticsTier,
    pub(crate) records_enabled: bool,
    pub(crate) replay_enabled: bool,
    pub(crate) route_record_limit: usize,
    pub(crate) failure_record_limit: usize,
}
