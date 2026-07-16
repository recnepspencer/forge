use crate::application::{
    WorthQueryDeclarationBridgeContinuationFamily, WorthQueryDeclarationBridgeContinuationMode,
    WorthQueryDeclarationBridgeTruthContext, WorthQueryDeclarationEnvelopeClass,
    WorthQueryDeclarationPrimaryAuthorityFamily, WorthQueryDeclarationReceiptClass,
    WorthQueryDeclarationRelationalAuthorityFamily, WorthQueryDeclarationRelationalTruthClaim,
    WorthQueryDeclarationSignalExecutionFamily, WorthQueryLowerAuthorityRouteFamily,
};
use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;

use super::{
    classification::{
        WorthQueryDeclarationEntryLowerOwnerCrate, WorthQueryDeclarationEntrySeamClassification,
    },
    row::WorthQueryDeclarationEntryCrossingSurface,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn derive_crossing_row_digest(
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    declaration_family_key: &str,
    entrypoint_key: &str,
    surface: WorthQueryDeclarationEntryCrossingSurface,
    primary_authority_family: WorthQueryDeclarationPrimaryAuthorityFamily,
    lower_owner_crate: WorthQueryDeclarationEntryLowerOwnerCrate,
    route_family: Option<WorthQueryLowerAuthorityRouteFamily>,
    receipt_class: Option<WorthQueryDeclarationReceiptClass>,
    envelope_class: Option<WorthQueryDeclarationEnvelopeClass>,
    relational_truth_claim: Option<WorthQueryDeclarationRelationalTruthClaim>,
    relational_authority_family: Option<WorthQueryDeclarationRelationalAuthorityFamily>,
    bridge_continuation_mode: Option<WorthQueryDeclarationBridgeContinuationMode>,
    bridge_truth_context: Option<WorthQueryDeclarationBridgeTruthContext>,
    bridge_continuation_family: Option<WorthQueryDeclarationBridgeContinuationFamily>,
    signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
    basis_families: &[BasisFamily],
    seam_classification: WorthQueryDeclarationEntrySeamClassification,
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

#[cfg(test)]
pub(crate) fn derive_inspection_digest(parts: &[String]) -> String {
    hash_parts(parts)
}

fn receipt_class_str(value: WorthQueryDeclarationReceiptClass) -> &'static str {
    match value {
        WorthQueryDeclarationReceiptClass::CoveredCrossing => "covered_crossing",
        WorthQueryDeclarationReceiptClass::DeferredCrossing => "deferred_crossing",
        WorthQueryDeclarationReceiptClass::DeniedCrossing => "denied_crossing",
        WorthQueryDeclarationReceiptClass::FailedCrossing => "failed_crossing",
    }
}

fn envelope_class_str(value: WorthQueryDeclarationEnvelopeClass) -> &'static str {
    match value {
        WorthQueryDeclarationEnvelopeClass::CoveredCrossing => "covered_crossing",
        WorthQueryDeclarationEnvelopeClass::DeferredCrossing => "deferred_crossing",
        WorthQueryDeclarationEnvelopeClass::DeniedCrossing => "denied_crossing",
        WorthQueryDeclarationEnvelopeClass::FailedCrossing => "failed_crossing",
    }
}
