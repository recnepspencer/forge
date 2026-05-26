use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationReceiptDenialCause,
    ForgeQueryDeclarationRoutePlanDenialCause,
};
use crate::identity::hash_parts;

use super::{
    artifact::ForgeQueryDeclarationRelationalRoutingClass,
    contract::{
        ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalTruthClaim,
    },
};

pub(crate) fn derive_relational_routing_digest<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    class: ForgeQueryDeclarationRelationalRoutingClass,
    truth_claim: ForgeQueryDeclarationRelationalTruthClaim,
    authority_family: ForgeQueryDeclarationRelationalAuthorityFamily,
    binding_surface: &'static str,
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
        format!("truth_claim:{}", truth_claim.as_str()),
        format!("authority_family:{}", authority_family.as_str()),
        format!("binding_surface:{binding_surface}"),
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
