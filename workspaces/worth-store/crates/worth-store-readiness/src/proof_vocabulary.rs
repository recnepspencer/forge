use crate::{FoundationalAdoptionDenial, FoundationalAdoptionRow};
use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalizationRuleVersion,
};
use worth_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
};
use worth_proof::TransitionOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalAdoptionDigest {
    digest: CanonicalDerivedDigest,
}

impl FoundationalAdoptionDigest {
    pub const fn canonical_digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofVocabularyAdoptionMap {
    digest: FoundationalAdoptionDigest,
    canonical_entry_count: u32,
}

impl ProofVocabularyAdoptionMap {
    pub(crate) fn from_adoption_rows(
        rows: &[FoundationalAdoptionRow],
    ) -> Result<Self, FoundationalAdoptionDenial> {
        let version = adoption_rule_version();
        let domain = adoption_domain();
        let mut entries: Vec<_> = rows.iter().map(adoption_row_entry).collect();
        entries.sort();
        let sequence = match prepare_canonical_basis_sequence(version.clone(), domain, entries) {
            TransitionOutcome::Success(sequence) => sequence,
            TransitionOutcome::Denied(denial) => {
                return Err(FoundationalAdoptionDenial::CanonicalBasisDenied(denial));
            }
            _ => unreachable!("canonical basis preparation only returns success or denial"),
        };

        let digest_slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::test_stable_fixture(),
            domain,
            version,
        );
        let digest_ready = match admit_canonical_sequence_digest_derivation(sequence, digest_slot) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return Err(FoundationalAdoptionDenial::CanonicalDigestDenied(denial));
            }
            _ => unreachable!("canonical digest admission only returns success or denial"),
        };
        let digest = derive_canonical_digest(digest_ready);
        let canonical_entry_count = digest.metadata().entry_count();

        Ok(Self {
            digest: FoundationalAdoptionDigest { digest },
            canonical_entry_count,
        })
    }

    pub const fn digest(&self) -> &FoundationalAdoptionDigest {
        &self.digest
    }

    pub const fn canonical_entry_count(&self) -> u32 {
        self.canonical_entry_count
    }
}

fn adoption_row_entry(row: &FoundationalAdoptionRow) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        adoption_domain(),
        CanonicalBasisLocus::Named(row.family().canonical_locus().into()),
        CanonicalBasisEntryKind::Future("foundational-adoption-row"),
        CanonicalBasisValue::ExactText(row.public_lane().into()),
    )
}

fn adoption_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("worth-store.physical-vocabulary-adoption.v1")
        .expect("static adoption canonicalization rule version is valid")
}

const fn adoption_domain() -> CanonicalBasisDomain {
    CanonicalBasisDomain::Future("worth-store.physical-vocabulary-adoption")
}
