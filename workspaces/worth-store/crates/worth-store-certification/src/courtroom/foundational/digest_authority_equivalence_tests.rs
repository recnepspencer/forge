use worth_foundational::canonicalization_api::lower_lane::{
    basis::CanonicalBasisDomain, comparison::CanonicalComparisonOutcome,
};
use worth_foundational::{
    CanonicalDigestAlgorithmId, CanonicalEquivalenceBasis, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    deny_basis_free_digest_comparison, deny_basis_free_parity, deny_basis_free_reuse,
    deny_basis_free_suppression, StoreCanonicalBasisFamily, StoreCanonicalBasisSourceKind,
    StoreDigestAuthority, StoreDigestAuthorityDenial, StoreDigestEquivalenceBasis,
    StoreDigestEquivalenceDenial, StoreDigestEquivalenceOperation, StoreDigestEvidence,
    StorePhysicalBoundaryWitness,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    prepare_physical_page_header_canonical_basis, PhysicalBinaryEncodingWitness,
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalHeaderAuthority,
    PhysicalHeaderDecodeWitness, PhysicalPageId, PhysicalPageKind, PhysicalSegmentId,
};

#[test]
fn same_native_basis_yields_same_digest() {
    let first = derive_store_digest(page_header_basis());
    let second = derive_store_digest(page_header_basis());

    assert_eq!(
        first.canonical_digest().value().bytes(),
        second.canonical_digest().value().bytes()
    );
    assert_eq!(
        first.family(),
        StoreCanonicalBasisFamily::PhysicalPageHeader
    );
    assert_eq!(
        first.source_kind(),
        StoreCanonicalBasisSourceKind::StorePageHeader
    );
    assert_eq!(
        first.equivalence_basis_identity(),
        StoreDigestEquivalenceBasis::exact_native_basis(
            StoreCanonicalBasisFamily::PhysicalPageHeader,
        )
        .identity()
    );
}

#[test]
fn store_equivalence_requires_named_native_basis() {
    let basis = StoreDigestEquivalenceBasis::exact_native_basis(
        StoreCanonicalBasisFamily::PhysicalPageHeader,
    );
    let decision = match StoreDigestAuthority::compare_native_basis(
        basis,
        page_header_basis(),
        page_header_basis(),
    ) {
        TransitionOutcome::Success(decision) => decision,
        other => panic!("native basis comparison should succeed: {other:?}"),
    };

    assert_eq!(
        decision.basis().family(),
        StoreCanonicalBasisFamily::PhysicalPageHeader
    );
    assert!(matches!(
        decision.outcome(),
        CanonicalComparisonOutcome::Equivalent(_)
    ));
}

#[test]
fn digest_authority_rejects_native_basis_from_wrong_store_family() {
    let outcome = StoreDigestAuthority::for_native_basis(
        StoreCanonicalBasisFamily::AspectBoundaryFact,
        page_header_basis(),
    )
    .derive(CanonicalDigestAlgorithmId::test_stable_fixture());

    let mismatch = match outcome {
        TransitionOutcome::Denied(StoreDigestAuthorityDenial::NativeBasisFamilyDomainMismatch(
            mismatch,
        )) => mismatch,
        other => panic!("wrong-family digest basis should be denied: {other:?}"),
    };

    assert_eq!(
        mismatch.family(),
        StoreCanonicalBasisFamily::AspectBoundaryFact
    );
    assert_eq!(
        mismatch.expected(),
        CanonicalBasisDomain::Future("store.aspect.boundary.fact")
    );
    assert_eq!(
        mismatch.actual(),
        CanonicalBasisDomain::Future("store.physical.page.header")
    );
}

#[test]
fn store_equivalence_rejects_native_basis_from_wrong_store_family() {
    let basis = StoreDigestEquivalenceBasis::exact_native_basis(
        StoreCanonicalBasisFamily::AspectBoundaryFact,
    );

    let mismatch = match StoreDigestAuthority::compare_native_basis(
        basis,
        page_header_basis(),
        page_header_basis(),
    ) {
        TransitionOutcome::Denied(
            StoreDigestEquivalenceDenial::NativeBasisFamilyDomainMismatch(mismatch),
        ) => mismatch,
        other => panic!("wrong-family equivalence basis should be denied: {other:?}"),
    };

    assert_eq!(
        mismatch.family(),
        StoreCanonicalBasisFamily::AspectBoundaryFact
    );
    assert_eq!(
        mismatch.expected(),
        CanonicalBasisDomain::Future("store.aspect.boundary.fact")
    );
    assert_eq!(
        mismatch.actual(),
        CanonicalBasisDomain::Future("store.physical.page.header")
    );
}

#[test]
fn basis_free_reuse_parity_suppression_and_comparison_are_denied() {
    assert_eq!(
        deny_basis_free_reuse(),
        StoreDigestEquivalenceDenial::BasisRequired {
            operation: StoreDigestEquivalenceOperation::Reuse,
        }
    );
    assert_eq!(
        deny_basis_free_parity(),
        StoreDigestEquivalenceDenial::BasisRequired {
            operation: StoreDigestEquivalenceOperation::Parity,
        }
    );
    assert_eq!(
        deny_basis_free_suppression(),
        StoreDigestEquivalenceDenial::BasisRequired {
            operation: StoreDigestEquivalenceOperation::Suppression,
        }
    );
    assert_eq!(
        deny_basis_free_digest_comparison(),
        StoreDigestEquivalenceDenial::BasisRequired {
            operation: StoreDigestEquivalenceOperation::DigestComparison,
        }
    );
}

#[test]
fn projection_and_digest_equivalence_are_not_store_authority_basis() {
    for foundational_basis in [
        CanonicalEquivalenceBasis::DeclaredAspectEquivalence,
        CanonicalEquivalenceBasis::CompatibilityLoweredNativeEquivalence,
        CanonicalEquivalenceBasis::ProjectionEquivalent,
        CanonicalEquivalenceBasis::DigestEquivalent,
    ] {
        assert_eq!(
            StoreDigestEquivalenceBasis::from_foundational_basis(
                StoreCanonicalBasisFamily::PhysicalPageHeader,
                foundational_basis,
            ),
            Err(StoreDigestEquivalenceDenial::NonNativeEquivalenceRejected {
                family: StoreCanonicalBasisFamily::PhysicalPageHeader,
                foundational_basis,
            })
        );
    }
}

fn derive_store_digest(
    basis: worth_foundational::canonicalization_api::lower_lane::basis::CanonicalBasisReadyArtifact,
) -> StoreDigestEvidence {
    match StoreDigestAuthority::for_native_basis(
        StoreCanonicalBasisFamily::PhysicalPageHeader,
        basis,
    )
    .derive(CanonicalDigestAlgorithmId::test_stable_fixture())
    {
        TransitionOutcome::Success(evidence) => evidence,
        other => panic!("Store digest derivation should succeed: {other:?}"),
    }
}

fn page_header_basis(
) -> worth_foundational::canonicalization_api::lower_lane::basis::CanonicalBasisReadyArtifact {
    let outcome = prepare_physical_page_header_canonical_basis(
        basis_version(),
        decoded_page_header(),
        physical_witness(),
    );

    match outcome {
        TransitionOutcome::Success(ready) => ready,
        other => panic!("page header basis should be ready: {other:?}"),
    }
}

fn decoded_page_header() -> PhysicalHeaderDecodeWitness {
    let generation = generation(7);
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(segment(11), page(13))
        .with_page_generation(generation);
    let bytes = crate::physical_fixture_encoding::data_page_bytes(cell, b"digest-authority");
    header_authority()
        .decode_page_header(cell, &bytes, PhysicalPageKind::DataPage)
        .unwrap()
        .witness()
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(
        PhysicalBinaryEncodingWitness::physical_format_canonical()
            .expect("static S.1 fixture encoding witness is valid"),
    )
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}

fn basis_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("store.native-basis.test.v1").unwrap()
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
