use super::artifact::ForgeQueryEvidenceIdentityComparisonError;
use super::encoder::{forge_query_evidence_identity, forge_query_evidence_identity_with_scheme};
use super::scheme::ForgeQueryEvidenceIdentityScheme;
use super::scope::ForgeQueryEvidenceScope;
use super::tag::ForgeQueryEvidenceTag;

#[test]
fn evidence_identity_sequences_do_not_collapse_delimiter_injection() {
    let left = forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentDenialEvidence)
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("invariant_evidence"),
            ["alpha|beta", "gamma"],
        )
        .seal();
    let right = forge_query_evidence_identity(ForgeQueryEvidenceScope::IntentDenialEvidence)
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("invariant_evidence"),
            ["alpha", "beta|gamma"],
        )
        .seal();

    assert_ne!(left.as_str(), right.as_str());
}

#[test]
fn evidence_identity_scope_stays_part_of_the_public_identity_contract() {
    let left = forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentAdmission)
        .field_identity(ForgeQueryEvidenceTag::new("input_digest"), "input:v1")
        .seal();
    let right = forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewIntentReceipt)
        .field_identity(ForgeQueryEvidenceTag::new("input_digest"), "input:v1")
        .seal();

    assert_ne!(left.as_str(), right.as_str());
}

#[test]
fn evidence_identity_keeps_explicit_scheme_metadata() {
    let identity = forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
        .field_shape(ForgeQueryEvidenceTag::new("state_kind"), "ready")
        .seal();

    assert_eq!(identity.scheme(), ForgeQueryEvidenceIdentityScheme::V1);
    assert_eq!(
        identity.scheme().as_str(),
        "forge.query.evidence-identity.v1"
    );
    assert!(identity
        .as_str()
        .starts_with("forge.query.evidence-identity.v1:forge.test.stable-digest-v1:"));
    assert!(!identity.canonical_digest().value().bytes().is_empty());
}

#[test]
fn evidence_identity_cross_scheme_comparison_fails_typed() {
    let left = forge_query_evidence_identity_with_scheme(
        ForgeQueryEvidenceScope::RuntimeStateSnapshot,
        ForgeQueryEvidenceIdentityScheme::V1,
    )
    .field_shape(ForgeQueryEvidenceTag::new("state_kind"), "ready")
    .seal();
    let right = forge_query_evidence_identity_with_scheme(
        ForgeQueryEvidenceScope::RuntimeStateSnapshot,
        ForgeQueryEvidenceIdentityScheme::V2,
    )
    .field_shape(ForgeQueryEvidenceTag::new("state_kind"), "ready")
    .seal();

    assert_eq!(
        left.compare_same_scheme(&right),
        Err(ForgeQueryEvidenceIdentityComparisonError::SchemeMismatch {
            left: ForgeQueryEvidenceIdentityScheme::V1,
            right: ForgeQueryEvidenceIdentityScheme::V2,
        })
    );
}

#[test]
fn evidence_identity_typed_scalar_helpers_preserve_canonical_distinctions() {
    let left = forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicSupportMatrix)
        .field_bool(ForgeQueryEvidenceTag::new("parallel_api_forbidden"), true)
        .field_usize(ForgeQueryEvidenceTag::new("stable_row_count"), 4)
        .seal();
    let same = forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicSupportMatrix)
        .field_bool(ForgeQueryEvidenceTag::new("parallel_api_forbidden"), true)
        .field_usize(ForgeQueryEvidenceTag::new("stable_row_count"), 4)
        .seal();
    let changed_bool =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicSupportMatrix)
            .field_bool(ForgeQueryEvidenceTag::new("parallel_api_forbidden"), false)
            .field_usize(ForgeQueryEvidenceTag::new("stable_row_count"), 4)
            .seal();
    let changed_count =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimePublicSupportMatrix)
            .field_bool(ForgeQueryEvidenceTag::new("parallel_api_forbidden"), true)
            .field_usize(ForgeQueryEvidenceTag::new("stable_row_count"), 5)
            .seal();

    assert_eq!(left.eq_same_scheme(&same), Ok(true));
    assert_eq!(left.eq_same_scheme(&changed_bool), Ok(false));
    assert_eq!(left.eq_same_scheme(&changed_count), Ok(false));
}
