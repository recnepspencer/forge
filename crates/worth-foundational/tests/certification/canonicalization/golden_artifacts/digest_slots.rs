use worth_foundational::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest,
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalIntegerWidth, CanonicalSingleSequenceDigestAlgorithmSlot, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

#[test]
fn sha256_digest_slot_has_stable_derived_value() {
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
            CanonicalDigestAlgorithmId::sha256(),
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
            89, 41, 204, 117, 202, 52, 125, 39, 24, 4, 120, 254, 17, 43, 108, 97, 249, 128, 96, 1,
            220, 139, 212, 48, 18, 203, 195, 190, 118, 82, 94, 135,
        ]
    );
}
