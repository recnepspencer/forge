use worth_foundational::facade::{
    derive_canonical_digest, prepare_canonical_basis_bundle, prepare_canonical_basis_sequence,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisReadyArtifact, CanonicalBasisValue, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestFrontDoor, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectPublication, WorthQueryDeclarationFoundationalEvidenceClass,
};

use super::artifact::{WorthQueryDeclarationReceiptClass, WorthQueryDeclarationReceiptKind};

pub(crate) fn derive_receipt_digest(
    version: CanonicalizationRuleVersion,
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    declaration_family_key: &str,
    declaration_digest: &str,
    progression_digest: Option<&str>,
    route_plan_digest: Option<&str>,
    class: WorthQueryDeclarationReceiptClass,
    kind: WorthQueryDeclarationReceiptKind,
    foundational_evidence_class: WorthQueryDeclarationFoundationalEvidenceClass,
    foundational_support_digest: &str,
    legality_digest: Option<&str>,
    route_contract_reason: Option<&str>,
    route_intent_token: Option<&str>,
    route_cause_reason: Option<&str>,
    receipt_cause_reason: Option<&str>,
    crossing_aspect_contract: &WorthQueryDeclarationAspectContract,
    crossing_aspect_coverage: &WorthQueryDeclarationAspectCoverage,
    crossing_aspect_publication: &WorthQueryDeclarationAspectPublication,
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
    entries.push(text_entry(
        "receipt.crossing_aspect_contract",
        &format!("{crossing_aspect_contract:?}"),
    ));
    entries.push(text_entry(
        "receipt.crossing_aspect_coverage",
        &format!("{crossing_aspect_coverage:?}"),
    ));
    entries.push(text_entry(
        "receipt.crossing_aspect_publication",
        &format!("{crossing_aspect_publication:?}"),
    ));
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
        CanonicalBasisDomain::Future("worth_query.declaration_receipt"),
        entries,
    ) {
        TransitionOutcome::Success(ready) => ready,
        outcome => panic!("receipt digest basis should prepare cleanly: {outcome:?}"),
    }
}

fn text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future("worth_query.declaration_receipt"),
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

fn optional_text_entry(locus: &str, value: Option<&str>) -> Option<CanonicalBasisEntry> {
    value.map(|value| text_entry(locus, value))
}

fn receipt_kind_token(kind: WorthQueryDeclarationReceiptKind) -> &'static str {
    match kind {
        WorthQueryDeclarationReceiptKind::Relational => "relational",
        WorthQueryDeclarationReceiptKind::Bridge => "bridge",
        WorthQueryDeclarationReceiptKind::Mixed => "mixed",
        WorthQueryDeclarationReceiptKind::Deferred => "deferred",
        WorthQueryDeclarationReceiptKind::Denied => "denied",
        WorthQueryDeclarationReceiptKind::Failed => "failed",
    }
}

fn receipt_class_token(class: WorthQueryDeclarationReceiptClass) -> &'static str {
    match class {
        WorthQueryDeclarationReceiptClass::CoveredCrossing => "covered_crossing",
        WorthQueryDeclarationReceiptClass::DeferredCrossing => "deferred_crossing",
        WorthQueryDeclarationReceiptClass::DeniedCrossing => "denied_crossing",
        WorthQueryDeclarationReceiptClass::FailedCrossing => "failed_crossing",
    }
}

fn foundational_evidence_class_token(
    class: WorthQueryDeclarationFoundationalEvidenceClass,
) -> &'static str {
    match class {
        WorthQueryDeclarationFoundationalEvidenceClass::LegalityAdmitted => "legality_admitted",
        WorthQueryDeclarationFoundationalEvidenceClass::LegalityDenied => "legality_denied",
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionAdmitted => {
            "progression_admitted"
        }
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDeferred => {
            "progression_deferred"
        }
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionDenied => "progression_denied",
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionStale => "progression_stale",
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionRebindRequired => {
            "progression_rebind_required"
        }
        WorthQueryDeclarationFoundationalEvidenceClass::ProgressionFailed => "progression_failed",
    }
}
