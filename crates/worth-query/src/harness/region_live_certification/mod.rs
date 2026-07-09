mod bundles;
mod canonical_rows;
mod rejection_rows;
mod row_catalog;
mod tests;

use super::live_certification::{LiveCertificationMatrix, MilestoneFiveLiveCertificationArtifact};
pub(crate) use row_catalog::{
    REGION_LIVE_REQUIRED_CANONICAL_ROW_NAMES, REGION_LIVE_REQUIRED_REJECTION_ROW_NAMES,
};

pub struct MilestoneFivePointOneLiveCertificationAdapter;

impl MilestoneFivePointOneLiveCertificationAdapter {
    pub fn region_scoped_live_narrowing_and_stream_contract_certification_artifact(
    ) -> MilestoneFiveLiveCertificationArtifact {
        Self::region_scoped_live_narrowing_and_stream_contract_test().into_milestone_five_artifact()
    }

    pub fn region_scoped_live_narrowing_and_stream_contract_test() -> LiveCertificationMatrix {
        LiveCertificationMatrix {
            suite_name: "Region-Scoped Live Narrowing And Stream Contract Test",
            rows: canonical_rows::canonical_rows(),
            rejection_rows: rejection_rows::rejection_rows(),
        }
    }
}
