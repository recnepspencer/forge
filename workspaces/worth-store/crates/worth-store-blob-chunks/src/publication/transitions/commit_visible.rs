use super::super::types::published::{BlobGenerationPublished, BlobPublicationAuthority};
use super::super::types::session_closeout::BlobPublicationSessionCloseout;

pub(crate) fn commit_visible(
    session_closeout: BlobPublicationSessionCloseout,
    authority: BlobPublicationAuthority,
) -> BlobGenerationPublished {
    let _current_authority = authority.into_current_authority();
    let (intent, wal_commit) = session_closeout.into_parts();
    BlobGenerationPublished {
        object_id: intent.object_id().clone(),
        generation: intent.generation(),
        chunk_tree_root: intent.chunk_tree_root().clone(),
        logical_content_digest: intent.logical_content_digest().clone(),
        classification: intent.classification(),
        publication_declaration: wal_commit.publication_declaration().clone(),
        replay_classification_digest: wal_commit.replay_classification_digest().to_owned(),
        replay_counters: wal_commit.replay_counters(),
        staging_identity: wal_commit.staging_identity().clone(),
        security_metadata: wal_commit.security_metadata(),
        counters: intent.counters().with_committed_publication(),
    }
}
