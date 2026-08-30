use super::super::{ApplicationExternalEffectProtocol, WorthQueryExternalEffectCorrelationFamily};
use crate::portable_identity::WorthQueryPortableTypeIdentity;

pub(super) struct DeclaredExternalEffectSlot {
    pub(super) effect: String,
    pub(super) rust_payload_type: WorthQueryPortableTypeIdentity,
    pub(super) protocol: ApplicationExternalEffectProtocol,
    pub(super) maximum_payload_bytes: u64,
    pub(super) correlation_family: WorthQueryExternalEffectCorrelationFamily,
}
