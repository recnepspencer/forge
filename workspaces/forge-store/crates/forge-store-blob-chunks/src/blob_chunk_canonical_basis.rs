use forge_foundational::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest,
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalDerivedDigest, CanonicalDigestAlgorithmId, CanonicalSingleSequenceDigestAlgorithmSlot,
    CanonicalizationRuleVersion, InternedString,
};
use forge_proof::TransitionOutcome;

use crate::{
    AdmittedBlobChunkSequence, BlobChunkProofLeaf, BlobChunkRootCounterSnapshot,
    BlobChunkRootPublicationDenial, ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkRootCanonicalBasis {
    ready_basis: CanonicalBasisReadyArtifact,
    canonical_digest: CanonicalDerivedDigest,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    chunk_identities: Vec<crate::BlobChunkIdentity>,
    chunk_count: u64,
    total_bytes: u64,
    counters: BlobChunkRootCounterSnapshot,
}

impl BlobChunkRootCanonicalBasis {
    pub(crate) fn from_sequence(
        sequence: &AdmittedBlobChunkSequence,
    ) -> Result<Self, BlobChunkRootPublicationDenial> {
        let rule_version = root_canonical_rule_version();
        let entries = canonical_entries_for_sequence(sequence);
        let entry_count = entries.len() as u64;
        let counters = BlobChunkRootCounterSnapshot::start().record_root_publication(entry_count);
        let ready_basis = match prepare_canonical_basis_sequence(
            rule_version.clone(),
            CanonicalBasisDomain::BoundaryArtifact,
            entries,
        ) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(_) => {
                return Err(
                    BlobChunkRootPublicationDenial::CanonicalBasisConstructionDenied {
                        counters: counters.record_denial(),
                    },
                )
            }
        };
        let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::test_stable_fixture(),
            CanonicalBasisDomain::BoundaryArtifact,
            rule_version,
        );
        let digest_ready =
            match admit_canonical_sequence_digest_derivation(ready_basis.clone(), slot) {
                TransitionOutcome::Success(ready) => ready,
                TransitionOutcome::Denied(_) => {
                    return Err(
                        BlobChunkRootPublicationDenial::CanonicalDigestDerivationDenied {
                            counters: counters.record_denial(),
                        },
                    )
                }
            };
        let counters = counters.record_canonical_digest_derivation();

        Ok(Self {
            ready_basis,
            canonical_digest: derive_canonical_digest(digest_ready),
            chunk_tree_root: sequence.chunk_tree_root().clone(),
            logical_content_digest: sequence.logical_content_digest().clone(),
            chunk_identities: sequence
                .proof_frontier()
                .ordered_leaves()
                .iter()
                .map(|leaf| leaf.identity().clone())
                .collect(),
            chunk_count: sequence.proof_frontier().chunk_count(),
            total_bytes: sequence.proof_frontier().total_bytes(),
            counters,
        })
    }

    pub const fn ready_basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.ready_basis
    }

    pub const fn canonical_digest(&self) -> &CanonicalDerivedDigest {
        &self.canonical_digest
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub fn contains_chunk_identity(&self, identity: &crate::BlobChunkIdentity) -> bool {
        self.chunk_identities.iter().any(|chunk| chunk == identity)
    }

    pub const fn chunk_count(&self) -> u64 {
        self.chunk_count
    }

    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub const fn counters(&self) -> BlobChunkRootCounterSnapshot {
        self.counters
    }
}

pub(crate) fn root_canonical_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("s7.blob.chunk-root.canonical-basis.v1")
        .expect("nonempty canonical rule version")
}

fn canonical_entries_for_sequence(
    sequence: &AdmittedBlobChunkSequence,
) -> Vec<CanonicalBasisEntry> {
    let frontier = sequence.proof_frontier();
    let mut entries = vec![
        text_entry("law", "s7.chunk-root-publication"),
        text_entry(
            "chunk_tree_root",
            sequence.chunk_tree_root().digest().as_str(),
        ),
        text_entry(
            "logical_content_digest",
            sequence.logical_content_digest().digest().as_str(),
        ),
        text_entry(
            "chunk_identity_summary",
            sequence.chunk_identity_summary().as_str(),
        ),
        text_entry("chunk_count", &frontier.chunk_count().to_string()),
        text_entry("total_bytes", &frontier.total_bytes().to_string()),
    ];
    for leaf in frontier.ordered_leaves() {
        append_leaf_entries(&mut entries, leaf);
    }
    entries
}

fn text_entry(locus: impl Into<InternedString>, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::BoundaryArtifact,
        CanonicalBasisValue::ExactText(InternedString::from(value)),
    )
}

fn append_leaf_entries(entries: &mut Vec<CanonicalBasisEntry>, leaf: &BlobChunkProofLeaf) {
    let prefix = format!("chunk.{:020}", leaf.ordinal().get());
    entries.push(text_entry(
        owned_locus(&prefix, "identity"),
        leaf.identity().chunk_digest().as_str(),
    ));
    entries.push(text_entry(
        owned_locus(&prefix, "stored"),
        leaf.stored_digest().digest().as_str(),
    ));
    entries.push(text_entry(
        owned_locus(&prefix, "content"),
        leaf.content_digest().digest().as_str(),
    ));
    entries.push(text_entry(
        owned_locus(&prefix, "checksum"),
        leaf.checksum_digest().as_str(),
    ));
    entries.push(text_entry(owned_locus(&prefix, "range"), &range_text(leaf)));
    entries.push(text_entry(
        owned_locus(&prefix, "security"),
        &security_scope_text(leaf),
    ));
}

fn owned_locus(prefix: &str, suffix: &str) -> String {
    format!("{prefix}.{suffix}")
}

fn range_text(leaf: &BlobChunkProofLeaf) -> String {
    format!(
        "{}:{}:{}",
        leaf.ordinal().get(),
        leaf.byte_range().start(),
        leaf.byte_range().len()
    )
}

fn security_scope_text(leaf: &BlobChunkProofLeaf) -> String {
    let metadata = leaf.security_metadata();
    format!(
        "{}:{}:{}:{}:{}",
        metadata.key_scope() as u8,
        metadata.key_version_posture() as u8,
        metadata.tenant_scope() as u8,
        metadata
            .authenticity_requirement()
            .class()
            .map_or(0, |class| class as u8),
        metadata.custody_posture() as u8
    )
}
