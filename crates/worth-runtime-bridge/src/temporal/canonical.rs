use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisSequence, CanonicalBasisValue, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

pub(super) fn canonical_version(version: &'static str) -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new(version).expect("temporal canonical version is valid")
}

pub(super) fn transition_canonical_ready(
    version: CanonicalizationRuleVersion,
    entries: impl IntoIterator<Item = CanonicalBasisEntry>,
    denial_context: &'static str,
) -> CanonicalBasisReadyArtifact {
    match prepare_canonical_basis_sequence(version, CanonicalBasisDomain::Transition, entries) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            panic!("{denial_context}: {denial:?}")
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => unreachable!("basis construction uses only denied"),
    }
}

pub(super) fn canonical_digest(
    canonical_ready: CanonicalBasisReadyArtifact,
    denial_context: &'static str,
) -> CanonicalDerivedDigest {
    let derivation = match canonicalization()
        .digest()
        .for_sequence(canonical_ready, CanonicalDigestAlgorithmId::sha256())
    {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            panic!("{denial_context}: {denial:?}")
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => unreachable!("digest admission uses only denied"),
    };

    canonicalization().digest().derive(derivation)
}

pub(super) fn rebuild_ready(sequence: &CanonicalBasisSequence) -> CanonicalBasisReadyArtifact {
    match prepare_canonical_basis_sequence(
        sequence.version().clone(),
        sequence.domain(),
        sequence.entries().iter().cloned(),
    ) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            panic!("stored temporal basis must rebuild cleanly: {denial:?}")
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => unreachable!("basis rebuild uses only denied"),
    }
}

pub(super) fn text_entry(locus: &'static str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Transition,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::TransitionArtifact,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

pub(super) fn u64_entry(locus: &'static str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Transition,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::TransitionArtifact,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

pub(super) fn same_basis(left: &CanonicalBasisSequence, right: &CanonicalBasisSequence) -> bool {
    left.version() == right.version()
        && left.domain() == right.domain()
        && left.entries() == right.entries()
}

pub(super) fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
