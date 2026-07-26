use super::{WorthUiDslProtocolIdentity, WorthUiSealedSemanticPackage};

/// Produces an otherwise valid package with an unsupported protocol solely for
/// cross-crate denial certification. This module is feature-gated and is never
/// re-exported through the product facade.
pub fn with_unsupported_protocol(
    package: WorthUiSealedSemanticPackage,
) -> WorthUiSealedSemanticPackage {
    package.with_protocol_for_certification(WorthUiDslProtocolIdentity::unsupported_for_test())
}
