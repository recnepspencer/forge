mod bundles;
mod canonical_rows;
mod fixtures;
mod rejection_rows;
mod tests;

use super::collection_matrix::{
    CollectionCertificationMatrix, MilestoneFourCollectionCertificationArtifact,
};

pub struct MilestoneFourCollectionCertificationAdapter;

impl MilestoneFourCollectionCertificationAdapter {
    pub fn collection_cursor_rollup_and_cdc_shape_certification_artifact(
    ) -> MilestoneFourCollectionCertificationArtifact {
        Self::collection_cursor_rollup_and_cdc_shape_test().into_milestone_four_artifact()
    }

    pub fn collection_cursor_rollup_and_cdc_shape_test() -> CollectionCertificationMatrix {
        CollectionCertificationMatrix {
            suite_name: "Collection, Cursor, Rollup, And CDC Shape Parity Test",
            rows: canonical_rows::canonical_rows(),
            rejection_rows: rejection_rows::rejection_rows(),
        }
    }
}
