use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause,
};
use crate::identity::hash_parts;

use super::{
    artifact::ForgeQueryDeclarationBridgeRoutingClass,
    contract::ForgeQueryDeclarationBridgeContinuationFamily,
    request::ForgeQueryDeclarationBridgeContinuationRequest,
};

pub(crate) fn derive_bridge_routing_digest<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    class: ForgeQueryDeclarationBridgeRoutingClass,
    continuation_request: ForgeQueryDeclarationBridgeContinuationRequest,
    continuation_family: ForgeQueryDeclarationBridgeContinuationFamily,
    binding_surface: &'static str,
    aspect_contract: &ForgeQueryDeclarationAspectContract,
    aspect_coverage: &ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    aspect_fit: ForgeQueryDeclarationAspectFit,
    mapped_aspects: &ForgeQueryDeclarationAspectCoverage,
    mapping_fit: ForgeQueryDeclarationAspectFit,
    route_cause: Option<ForgeQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<ForgeQueryDeclarationReceiptDenialCause>,
) -> String {
    hash_parts(&[
        format!("handle:{}", envelope.handle_identity_digest()),
        format!(
            "operating_context:{}",
            envelope.operating_context_identity_digest()
        ),
        format!("family:{}", envelope.declaration_family_key()),
        format!("declaration:{}", envelope.declaration_digest()),
        format!(
            "progression:{}",
            envelope.progression_digest().unwrap_or("none")
        ),
        format!(
            "route_plan:{}",
            envelope.route_plan_digest().unwrap_or("none")
        ),
        format!(
            "receipt:{}",
            canonical_digest_token(envelope.receipt_digest())
        ),
        format!(
            "envelope:{}",
            canonical_digest_token(envelope.envelope_digest())
        ),
        format!("class:{class:?}"),
        format!("mode:{}", continuation_request.mode().as_str()),
        format!(
            "truth_context:{}",
            continuation_request.truth_context().as_str()
        ),
        format!("continuation_family:{}", continuation_family.as_str()),
        format!("binding_surface:{binding_surface}"),
        format!("aspect_contract:{aspect_contract:?}"),
        format!("aspect_coverage:{aspect_coverage:?}"),
        format!("aspect_coverage_basis:{aspect_coverage_basis:?}"),
        format!("aspect_fit:{aspect_fit:?}"),
        format!("mapped_aspects:{mapped_aspects:?}"),
        format!("mapping_fit:{mapping_fit:?}"),
        format!("evidence_origin:{:?}", envelope.evidence_origin()),
        format!("route_cause:{route_cause:?}"),
        format!("receipt_cause:{receipt_cause:?}"),
    ])
}

fn canonical_digest_token(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
