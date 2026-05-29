use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationRoutePlanDenialCause,
    ForgeQuerySignalCompatibilityPosture,
};
use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;

use super::contract::ForgeQueryDeclarationSignalExecutionFamily;

pub(crate) fn derive_signal_compatibility_digest<
    D: crate::application::ForgeQueryDomainEntryMarker,
    I: crate::application::ForgeQueryDeclarationInput<D>,
>(
    envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    execution_family: ForgeQueryDeclarationSignalExecutionFamily,
    basis_families: &[BasisFamily],
    signal_posture: ForgeQuerySignalCompatibilityPosture,
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
        format!("signal_posture:{}", signal_posture.as_str()),
        format!("execution_family:{}", execution_family.as_str()),
        format!(
            "basis_families:{}",
            basis_families
                .iter()
                .map(BasisFamily::as_str)
                .collect::<Vec<_>>()
                .join("|")
        ),
        format!(
            "primary_authority:{}",
            I::Family::taxonomy().primary_authority_family().as_str()
        ),
        format!("evidence_origin:{}", envelope.evidence_origin().as_str()),
        format!(
            "route_cause:{}",
            route_cause.map_or("none", ForgeQueryDeclarationRoutePlanDenialCause::as_str)
        ),
        format!(
            "receipt_cause:{}",
            receipt_cause.map_or("none", ForgeQueryDeclarationReceiptDenialCause::as_str)
        ),
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
