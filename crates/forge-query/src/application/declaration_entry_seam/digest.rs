use crate::application::{
    ForgeQueryDeclarationBridgeContinuationFamily, ForgeQueryDeclarationBridgeContinuationMode,
    ForgeQueryDeclarationBridgeTruthContext, ForgeQueryDeclarationEnvelopeClass,
    ForgeQueryDeclarationPrimaryAuthorityFamily, ForgeQueryDeclarationReceiptClass,
    ForgeQueryDeclarationRelationalAuthorityFamily, ForgeQueryDeclarationRelationalTruthClaim,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryLowerAuthorityRouteFamily,
};
use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;

use super::{
    classification::{
        ForgeQueryDeclarationEntryLowerOwnerCrate, ForgeQueryDeclarationEntrySeamClassification,
    },
    row::ForgeQueryDeclarationEntryCrossingSurface,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn derive_crossing_row_digest(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    declaration_family_key: &str,
    entrypoint_key: &str,
    surface: ForgeQueryDeclarationEntryCrossingSurface,
    primary_authority_family: ForgeQueryDeclarationPrimaryAuthorityFamily,
    lower_owner_crate: ForgeQueryDeclarationEntryLowerOwnerCrate,
    route_family: Option<ForgeQueryLowerAuthorityRouteFamily>,
    receipt_class: Option<ForgeQueryDeclarationReceiptClass>,
    envelope_class: Option<ForgeQueryDeclarationEnvelopeClass>,
    relational_truth_claim: Option<ForgeQueryDeclarationRelationalTruthClaim>,
    relational_authority_family: Option<ForgeQueryDeclarationRelationalAuthorityFamily>,
    bridge_continuation_mode: Option<ForgeQueryDeclarationBridgeContinuationMode>,
    bridge_truth_context: Option<ForgeQueryDeclarationBridgeTruthContext>,
    bridge_continuation_family: Option<ForgeQueryDeclarationBridgeContinuationFamily>,
    signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
    basis_families: &[BasisFamily],
    seam_classification: ForgeQueryDeclarationEntrySeamClassification,
) -> String {
    hash_parts(&[
        format!("handle:{handle_identity_digest}"),
        format!("world:{operating_context_identity_digest}"),
        format!("family:{declaration_family_key}"),
        format!("entrypoint:{entrypoint_key}"),
        format!("surface:{}", surface.as_str()),
        format!("authority:{}", primary_authority_family.as_str()),
        format!("owner:{}", lower_owner_crate.as_str()),
        format!(
            "route:{}",
            route_family.map(|value| value.as_str()).unwrap_or("none")
        ),
        format!(
            "receipt:{}",
            receipt_class.map(receipt_class_str).unwrap_or("none")
        ),
        format!(
            "envelope:{}",
            envelope_class.map(envelope_class_str).unwrap_or("none")
        ),
        format!(
            "relational-claim:{}",
            relational_truth_claim
                .map(|value| value.as_str())
                .unwrap_or("none")
        ),
        format!(
            "relational-family:{}",
            relational_authority_family
                .map(|value| value.as_str())
                .unwrap_or("none")
        ),
        format!(
            "bridge-mode:{}",
            bridge_continuation_mode
                .map(|value| value.as_str())
                .unwrap_or("none")
        ),
        format!(
            "bridge-context:{}",
            bridge_truth_context
                .map(|value| value.as_str())
                .unwrap_or("none")
        ),
        format!(
            "bridge-family:{}",
            bridge_continuation_family
                .map(|value| value.as_str())
                .unwrap_or("none")
        ),
        format!(
            "signal-family:{}",
            signal_execution_family
                .map(|value| value.as_str())
                .unwrap_or("none")
        ),
        format!(
            "basis:{}",
            basis_families
                .iter()
                .map(|value| value.as_str())
                .collect::<Vec<_>>()
                .join("|")
        ),
        format!("classification:{}", seam_classification.as_str()),
    ])
}

pub(crate) fn derive_readiness_digest(parts: &[String]) -> String {
    hash_parts(parts)
}

pub(crate) fn derive_inspection_digest(parts: &[String]) -> String {
    hash_parts(parts)
}

fn receipt_class_str(value: ForgeQueryDeclarationReceiptClass) -> &'static str {
    match value {
        ForgeQueryDeclarationReceiptClass::CoveredCrossing => "covered_crossing",
        ForgeQueryDeclarationReceiptClass::DeferredCrossing => "deferred_crossing",
        ForgeQueryDeclarationReceiptClass::DeniedCrossing => "denied_crossing",
        ForgeQueryDeclarationReceiptClass::FailedCrossing => "failed_crossing",
    }
}

fn envelope_class_str(value: ForgeQueryDeclarationEnvelopeClass) -> &'static str {
    match value {
        ForgeQueryDeclarationEnvelopeClass::CoveredCrossing => "covered_crossing",
        ForgeQueryDeclarationEnvelopeClass::DeferredCrossing => "deferred_crossing",
        ForgeQueryDeclarationEnvelopeClass::DeniedCrossing => "denied_crossing",
        ForgeQueryDeclarationEnvelopeClass::FailedCrossing => "failed_crossing",
    }
}
