use super::super::identity_record::STORE_NAMESPACE_IDENTITY_ENCODING_VERSION;
use super::super::*;
use worth_foundational::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalBasisLocus,
    CanonicalComparisonOutcome, CanonicalEquivalenceBasis, CanonicalMismatchKind,
    CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

fn version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.namespace.identity.v1")
        .expect("valid canonicalization version")
}

fn meaning(
    namespace_version: u16,
    identity: [u8; 16],
    publication: StoreNamespaceIdentityPublicationPosture,
) -> StoreNamespaceIdentityCanonicalMeaning {
    StoreNamespaceIdentityCanonicalMeaning::for_test(
        namespace_version,
        STORE_NAMESPACE_IDENTITY_ENCODING_VERSION,
        identity,
        publication,
    )
}

fn decoded_published_meaning(byte: u8) -> StoreNamespaceIdentityCanonicalMeaning {
    let proposed = ProposedStoreIdentity::from_nonzero_bytes([byte; 16]).expect("nonzero identity");
    let encoded =
        StoreNamespaceIdentityRecord::new(StoreNamespaceVersion::CURRENT, proposed).encode();
    let decoded = StoreNamespaceIdentityRecord::decode(&encoded).expect("valid identity record");
    StoreNamespaceIdentityCanonicalMeaning::from_published_identity(
        StableStoreIdentity::from_published_record(decoded.proposed_identity()),
    )
}

fn ready(
    meaning: StoreNamespaceIdentityCanonicalMeaning,
) -> worth_foundational::CanonicalBasisReadyArtifact {
    match prepare_store_namespace_identity_canonical_basis(version(), meaning) {
        TransitionOutcome::Success(ready) => ready,
        other => panic!("namespace identity basis should be ready: {other:?}"),
    }
}

fn compare(
    left: StoreNamespaceIdentityCanonicalMeaning,
    right: StoreNamespaceIdentityCanonicalMeaning,
    equivalence: CanonicalEquivalenceBasis,
) -> CanonicalComparisonOutcome {
    let comparison = match prepare_canonical_comparison(equivalence, ready(left), ready(right)) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("comparison preparation should be infallible"),
    };
    compare_canonical_basis(&comparison)
}

#[test]
fn equivalent_decoded_identities_lower_to_the_same_canonical_basis() {
    let left = decoded_published_meaning(9);
    let right = decoded_published_meaning(9);

    assert!(matches!(
        compare(left, right, CanonicalEquivalenceBasis::ExactCanonicalBasis),
        CanonicalComparisonOutcome::Equivalent(_)
    ));
}

#[test]
fn canonical_mismatches_are_localized_by_semantic_locus() {
    let baseline = meaning(
        1,
        [3; 16],
        StoreNamespaceIdentityPublicationPosture::Published,
    );
    let cases = [
        (
            meaning(
                2,
                [3; 16],
                StoreNamespaceIdentityPublicationPosture::Published,
            ),
            "namespace.version",
        ),
        (
            StoreNamespaceIdentityCanonicalMeaning::for_test(
                1,
                STORE_NAMESPACE_IDENTITY_ENCODING_VERSION + 1,
                [3; 16],
                StoreNamespaceIdentityPublicationPosture::Published,
            ),
            "encoding.version",
        ),
        (
            meaning(
                1,
                [4; 16],
                StoreNamespaceIdentityPublicationPosture::Published,
            ),
            "identity",
        ),
        (
            meaning(
                1,
                [3; 16],
                StoreNamespaceIdentityPublicationPosture::StagedCandidate,
            ),
            "publication.posture",
        ),
    ];

    for (changed, expected_locus) in cases {
        match compare(
            baseline,
            changed,
            CanonicalEquivalenceBasis::ExactCanonicalBasis,
        ) {
            CanonicalComparisonOutcome::Mismatched(mismatch) => {
                assert_eq!(mismatch.kind(), CanonicalMismatchKind::ValueMismatch);
                assert_eq!(
                    mismatch.left_locus(),
                    Some(&CanonicalBasisLocus::Named(expected_locus.into()))
                );
            }
            other => panic!("expected localized canonical mismatch: {other:?}"),
        }
    }
}

#[test]
fn digest_equivalence_is_not_identity_equivalence() {
    let identity = meaning(
        1,
        [6; 16],
        StoreNamespaceIdentityPublicationPosture::Published,
    );
    assert!(matches!(
        compare(
            identity,
            identity,
            CanonicalEquivalenceBasis::DigestEquivalent
        ),
        CanonicalComparisonOutcome::Unsupported(_)
    ));
}
