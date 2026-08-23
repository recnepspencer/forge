use super::super::{ApplicationExternalEffectProtocol, WorthQueryExternalEffectCorrelationFamily};

pub(super) struct DeclaredExternalEffectSlot {
    pub(super) effect: String,
    pub(super) rust_payload_type: String,
    pub(super) protocol: ApplicationExternalEffectProtocol,
    pub(super) maximum_payload_bytes: u64,
    pub(super) correlation_family: WorthQueryExternalEffectCorrelationFamily,
}
