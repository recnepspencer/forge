use forge_foundational::facade::{
    derive_canonical_digest, prepare_canonical_basis_bundle, prepare_canonical_basis_sequence,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisReadyArtifact, CanonicalBasisValue, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestFrontDoor, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;

use super::scheme::ForgeQueryEvidenceIdentityScheme;
use super::scope::ForgeQueryEvidenceScope;
use super::sealed::SealedForgeQueryEvidenceIdentity;

const EVIDENCE_IDENTITY_DOMAIN: &str = "forge_query.evidence_identity";

pub(crate) fn derive_evidence_identity(
    scope: ForgeQueryEvidenceScope,
    scheme: ForgeQueryEvidenceIdentityScheme,
    entries: Vec<CanonicalBasisEntry>,
) -> SealedForgeQueryEvidenceIdentity {
    let version = scheme.canonicalization_rule_version();
    let ready = canonical_basis_from_entries(version.clone(), entries);
    let bundle = match prepare_canonical_basis_bundle(version, [ready]) {
        TransitionOutcome::Success(bundle) => bundle,
        outcome => panic!("evidence identity bundle should prepare cleanly: {outcome:?}"),
    };
    let digest_ready = match CanonicalDigestFrontDoor
        .for_bundle(bundle, CanonicalDigestAlgorithmId::test_stable_fixture())
    {
        TransitionOutcome::Success(ready) => ready,
        outcome => panic!("evidence identity digest derivation should succeed: {outcome:?}"),
    };
    let canonical_digest = derive_canonical_digest(digest_ready);
    SealedForgeQueryEvidenceIdentity::new(
        scope,
        scheme,
        canonical_digest_token(scheme, &canonical_digest),
        canonical_digest,
    )
}

pub(crate) fn text_entry(
    locus: impl Into<String>,
    kind: CanonicalBasisEntryKind,
    value: impl Into<String>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future(EVIDENCE_IDENTITY_DOMAIN),
        CanonicalBasisLocus::Named(locus.into().into()),
        kind,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}

fn canonical_basis_from_entries(
    version: CanonicalizationRuleVersion,
    entries: Vec<CanonicalBasisEntry>,
) -> CanonicalBasisReadyArtifact {
    match prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Future(EVIDENCE_IDENTITY_DOMAIN),
        entries,
    ) {
        TransitionOutcome::Success(ready) => ready,
        outcome => panic!("evidence identity basis should prepare cleanly: {outcome:?}"),
    }
}

fn canonical_digest_token(
    scheme: ForgeQueryEvidenceIdentityScheme,
    digest: &CanonicalDerivedDigest,
) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!(
        "{}:{}:{hex}",
        scheme.as_str(),
        digest.metadata().algorithm().id().as_str()
    )
}
