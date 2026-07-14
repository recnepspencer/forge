use worth_foundational::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest,
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalIntegerWidth, CanonicalSingleSequenceDigestAlgorithmSlot, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

#[test]
fn digest_slot_fixture_has_stable_derived_value() {
    let version = CanonicalizationRuleVersion::new("m2.golden.digest-slot").expect("valid version");
    let sequence = match prepare_canonical_basis_sequence(
        version.clone(),
        CanonicalBasisDomain::Value,
        [CanonicalBasisEntry::new(
            CanonicalBasisDomain::Value,
            CanonicalBasisLocus::Named("digest-golden".into()),
            CanonicalBasisEntryKind::Value,
            CanonicalBasisValue::SignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: 42,
            },
        )],
    ) {
        TransitionOutcome::Success(sequence) => sequence,
        _ => panic!("golden digest sequence should be ready"),
    };
    let ready = match admit_canonical_sequence_digest_derivation(
        sequence,
        CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::test_stable_fixture(),
            CanonicalBasisDomain::Value,
            version,
        ),
    ) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("golden digest derivation should be admitted"),
    };

    let digest = derive_canonical_digest(ready);

    assert_eq!(
        digest.value().bytes(),
        &[
            133, 135, 210, 44, 152, 192, 176, 127, 147, 238, 226, 123, 102, 226, 61, 32, 163, 57,
            216, 15, 126, 192, 78, 48, 80, 80, 249, 41, 207, 139, 52, 120,
        ]
    );
}
