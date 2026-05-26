use forge_foundational::facade::{
    derive_canonical_digest, prepare_canonical_basis_bundle, prepare_canonical_basis_sequence,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisReadyArtifact, CanonicalBasisValue, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestFrontDoor, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;

use crate::application::ForgeQueryDeclarationFoundationalEvidenceClass;

use super::artifact::{ForgeQueryDeclarationReceiptClass, ForgeQueryDeclarationReceiptKind};

pub(crate) fn derive_receipt_digest(
    version: CanonicalizationRuleVersion,
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    declaration_family_key: &str,
    declaration_digest: &str,
    progression_digest: Option<&str>,
    route_plan_digest: Option<&str>,
    class: ForgeQueryDeclarationReceiptClass,
    kind: ForgeQueryDeclarationReceiptKind,
    foundational_evidence_class: ForgeQueryDeclarationFoundationalEvidenceClass,
    foundational_support_digest: &str,
    legality_digest: Option<&str>,
    route_contract_reason: Option<&str>,
    route_intent_token: Option<&str>,
    route_cause_reason: Option<&str>,
    receipt_cause_reason: Option<&str>,
) -> CanonicalDerivedDigest {
    let mut entries = vec![
        text_entry("receipt.handle", handle_identity_digest),
        text_entry(
            "receipt.operating_context",
            operating_context_identity_digest,
        ),
        text_entry("receipt.family", declaration_family_key),
        text_entry("receipt.declaration", declaration_digest),
        text_entry("receipt.class", receipt_class_token(class)),
        text_entry("receipt.kind", receipt_kind_token(kind)),
        text_entry(
            "receipt.evidence_class",
            foundational_evidence_class_token(foundational_evidence_class),
        ),
        text_entry("receipt.support", foundational_support_digest),
    ];
    if let Some(entry) = optional_text_entry("receipt.legality", legality_digest) {
        entries.push(entry);
    }
    if let Some(entry) = optional_text_entry("receipt.progression", progression_digest) {
        entries.push(entry);
    }
    if let Some(entry) = optional_text_entry("receipt.route_plan", route_plan_digest) {
        entries.push(entry);
    }
    if let Some(entry) = optional_text_entry("receipt.route_contract", route_contract_reason) {
        entries.push(entry);
    }
    if let Some(entry) = optional_text_entry("receipt.route_intent", route_intent_token) {
        entries.push(entry);
    }
    if let Some(entry) = optional_text_entry("receipt.route_cause", route_cause_reason) {
        entries.push(entry);
    }
    if let Some(entry) = optional_text_entry("receipt.receipt_cause", receipt_cause_reason) {
        entries.push(entry);
    }
    let ready = canonical_basis_from_entries(version.clone(), entries);
    let bundle = match prepare_canonical_basis_bundle(version, [ready]) {
        TransitionOutcome::Success(bundle) => bundle,
        outcome => panic!("receipt digest bundle should prepare cleanly: {outcome:?}"),
    };
    let digest_ready = match CanonicalDigestFrontDoor
        .for_bundle(bundle, CanonicalDigestAlgorithmId::test_stable_fixture())
    {
        TransitionOutcome::Success(ready) => ready,
        outcome => panic!("receipt digest derivation should succeed: {outcome:?}"),
    };
    derive_canonical_digest(digest_ready)
}

fn canonical_basis_from_entries(
    version: CanonicalizationRuleVersion,
    entries: impl IntoIterator<Item = CanonicalBasisEntry>,
) -> CanonicalBasisReadyArtifact {
    match prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Future("forge_query.declaration_receipt"),
        entries,
    ) {
        TransitionOutcome::Success(ready) => ready,
        outcome => panic!("receipt digest basis should prepare cleanly: {outcome:?}"),
    }
}

fn text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future("forge_query.declaration_receipt"),
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

fn optional_text_entry(locus: &str, value: Option<&str>) -> Option<CanonicalBasisEntry> {
    value.map(|value| text_entry(locus, value))
}

fn receipt_kind_token(kind: ForgeQueryDeclarationReceiptKind) -> &'static str {
    match kind {
        ForgeQueryDeclarationReceiptKind::Relational => "relational",
        ForgeQueryDeclarationReceiptKind::Bridge => "bridge",
        ForgeQueryDeclarationReceiptKind::Mixed => "mixed",
        ForgeQueryDeclarationReceiptKind::Deferred => "deferred",
        ForgeQueryDeclarationReceiptKind::Denied => "denied",
        ForgeQueryDeclarationReceiptKind::Failed => "failed",
    }
}

fn receipt_class_token(class: ForgeQueryDeclarationReceiptClass) -> &'static str {
    match class {
        ForgeQueryDeclarationReceiptClass::CoveredCrossing => "covered_crossing",
        ForgeQueryDeclarationReceiptClass::DeferredCrossing => "deferred_crossing",
        ForgeQueryDeclarationReceiptClass::DeniedCrossing => "denied_crossing",
        ForgeQueryDeclarationReceiptClass::FailedCrossing => "failed_crossing",
    }
}

fn foundational_evidence_class_token(
    class: ForgeQueryDeclarationFoundationalEvidenceClass,
) -> &'static str {
    match class {
        ForgeQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted => "legality_admitted",
        ForgeQueryDeclarationFoundationalEvidenceClass::LegalityDenied => "legality_denied",
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted => {
            "progression_admitted"
        }
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred => {
            "progression_deferred"
        }
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionDenied => "progression_denied",
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionStale => "progression_stale",
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired => {
            "progression_rebind_required"
        }
        ForgeQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => "progression_failed",
    }
}
