use worth_store_physical_format::RecordArtifactFile;

use super::{
    catalog_candidate_progression::SettledPublicationArtifacts,
    catalog_cutover_preflight::{PreparedCatalogResidency, ValidatedCatalogFrameSet},
};

pub(in crate::physical_runtime::record_serving) struct CatalogReplacementEligibility {
    candidate: RecordArtifactFile,
}

impl CatalogReplacementEligibility {
    pub(super) fn join(
        settled: SettledPublicationArtifacts,
        frame_set: ValidatedCatalogFrameSet,
        residency: PreparedCatalogResidency,
    ) -> Option<Self> {
        let candidate = settled.candidate();
        (frame_set.candidate() == candidate && residency.candidate() == candidate)
            .then_some(Self { candidate })
    }

    pub(in crate::physical_runtime::record_serving) fn matches(
        &self,
        candidate: RecordArtifactFile,
    ) -> bool {
        self.candidate == candidate
    }
}
