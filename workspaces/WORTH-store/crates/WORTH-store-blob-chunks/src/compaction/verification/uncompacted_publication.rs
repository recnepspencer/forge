use crate::compaction::classification::CompactionEligibilityCase;
use crate::{BlobChunkRootPublication, LifecycleReceipt};

pub(crate) fn require_uncompacted_publication(
    lifecycle: &LifecycleReceipt,
    publication: &BlobChunkRootPublication,
) -> Option<CompactionEligibilityCase> {
    let declaration = lifecycle.declaration();
    if publication.chunk_tree_root() == declaration.chunk_tree_root()
        && publication.logical_content_digest() == declaration.logical_content_digest()
        && publication.canonical_basis().chunk_tree_root() == declaration.chunk_tree_root()
        && publication.canonical_basis().logical_content_digest()
            == declaration.logical_content_digest()
    {
        None
    } else {
        Some(CompactionEligibilityCase::EquivalenceBasisMismatch)
    }
}
