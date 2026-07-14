use worth_query::facade::runtime::{SavedQueryArtifact, SavedQueryMetadata, SavedQueryPersistenceFamily};

fn main() {
    let _ = SavedQueryArtifact {
        digest: todo!(),
        metadata: todo!(),
        persistence_family: SavedQueryPersistenceFamily::EphemeralProcessOwned,
    };
}
