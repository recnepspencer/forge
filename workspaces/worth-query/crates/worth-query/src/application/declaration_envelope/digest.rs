use worth_foundational::facade::{
    derive_canonical_digest, prepare_canonical_basis_bundle, prepare_canonical_basis_sequence,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisReadyArtifact, CanonicalBasisValue, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestFrontDoor, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectPublication,
    WorthQueryDeclarationReceiptDenialCause, WorthQueryDeclarationRoutePlanDenialCause,
};

use super::class::{
    WorthQueryDeclarationEnvelopeClass, WorthQueryDeclarationEnvelopeEvidenceOrigin,
};

pub(crate) fn derive_envelope_digest(
    version: CanonicalizationRuleVersion,
    handle_identity_digest: &str,
    operating_context_identity_digest: &str,
    declaration_family_key: &str,
    declaration_digest: &str,
    progression_digest: Option<&str>,
    route_plan_digest: Option<&str>,
    receipt_digest: &str,
    envelope_class: WorthQueryDeclarationEnvelopeClass,
    evidence_origin: WorthQueryDeclarationEnvelopeEvidenceOrigin,
    route_cause: Option<WorthQueryDeclarationRoutePlanDenialCause>,
    receipt_cause: Option<WorthQueryDeclarationReceiptDenialCause>,
    published_aspect_contract: &WorthQueryDeclarationAspectContract,
    published_aspect_publication: &WorthQueryDeclarationAspectPublication,
) -> CanonicalDerivedDigest {
    let mut entries = vec![
        text_entry("envelope.handle", handle_identity_digest),
        text_entry(
            "envelope.operating_context",
            operating_context_identity_digest,
        ),
        text_entry("envelope.family", declaration_family_key),
        text_entry("envelope.declaration", declaration_digest),
        text_entry("envelope.class", envelope_class_token(envelope_class)),
        text_entry("envelope.origin", evidence_origin.as_str()),
        text_entry("envelope.receipt", receipt_digest),
    ];
    if let Some(entry) = optional_text_entry("envelope.progression", progression_digest) {
        entries.push(entry);
    }
    if let Some(entry) = optional_text_entry("envelope.route_plan", route_plan_digest) {
        entries.push(entry);
    }
    if let Some(entry) = route_cause.map(|cause| text_entry("envelope.route_cause", cause.reason()))
    {
        entries.push(entry);
    }
    if let Some(entry) =
        receipt_cause.map(|cause| text_entry("envelope.receipt_cause", cause.reason()))
    {
        entries.push(entry);
    }
    entries.push(text_entry(
        "envelope.published_aspect_contract",
        &format!("{published_aspect_contract:?}"),
    ));
    entries.push(text_entry(
        "envelope.published_aspect_publication",
        &format!("{published_aspect_publication:?}"),
    ));

    let ready = canonical_basis_from_entries(version.clone(), entries);
    let bundle = match prepare_canonical_basis_bundle(version, [ready]) {
        TransitionOutcome::Success(bundle) => bundle,
        outcome => panic!("envelope digest bundle should prepare cleanly: {outcome:?}"),
    };
    let digest_ready = match CanonicalDigestFrontDoor
        .for_bundle(bundle, CanonicalDigestAlgorithmId::test_stable_fixture())
    {
        TransitionOutcome::Success(ready) => ready,
        outcome => panic!("envelope digest derivation should succeed: {outcome:?}"),
    };
    derive_canonical_digest(digest_ready)
}

fn canonical_basis_from_entries(
    version: CanonicalizationRuleVersion,
    entries: impl IntoIterator<Item = CanonicalBasisEntry>,
) -> CanonicalBasisReadyArtifact {
    match prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Future("worth_query.declaration_envelope"),
        entries,
    ) {
        TransitionOutcome::Success(ready) => ready,
        outcome => panic!("envelope digest basis should prepare cleanly: {outcome:?}"),
    }
}

fn text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future("worth_query.declaration_envelope"),
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

fn optional_text_entry(locus: &str, value: Option<&str>) -> Option<CanonicalBasisEntry> {
    value.map(|value| text_entry(locus, value))
}

fn envelope_class_token(class: WorthQueryDeclarationEnvelopeClass) -> &'static str {
    match class {
        WorthQueryDeclarationEnvelopeClass::CoveredCrossing => "covered_crossing",
        WorthQueryDeclarationEnvelopeClass::DeferredCrossing => "deferred_crossing",
        WorthQueryDeclarationEnvelopeClass::DeniedCrossing => "denied_crossing",
        WorthQueryDeclarationEnvelopeClass::FailedCrossing => "failed_crossing",
    }
}
