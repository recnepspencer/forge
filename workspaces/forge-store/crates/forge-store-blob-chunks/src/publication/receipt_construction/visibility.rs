use super::super::types::published::{BlobGenerationPublished, BlobVisibleGeneration};

pub(crate) fn from_published(published: &BlobGenerationPublished) -> BlobVisibleGeneration {
    BlobVisibleGeneration {
        object_id: published.object_id().clone(),
        generation: published.generation(),
        chunk_tree_root: published.chunk_tree_root().clone(),
        logical_content_digest: published.logical_content_digest().clone(),
        classification: published.classification(),
        counters: published.counters().with_visible_observation(),
    }
}
