use crate::authority::{
    FetchedLineageSupportArtifact, FetchedSchemaSupportArtifact, HistoricalIdentityRequest,
    HistoricalIdentityResolution,
};
use crate::failure::StoreError;
use worth_relational::facade::history::CommitId;

use super::{dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn execute_compatibility_authoritative_adapter(
        &self,
        request: crate::CompatibilityAuthoritativeAdapterRequest,
    ) -> Result<crate::CompatibilityAuthoritativeAdapterOutcome, StoreError> {
        dispatch_ref!(self, |backend| backend
            .execute_compatibility_authoritative_adapter(request))
    }

    pub fn fetch_schema_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedSchemaSupportArtifact, StoreError> {
        dispatch_ref!(self, |backend| backend.fetch_schema_support(commit_id))
    }

    pub fn fetch_lineage_support(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedLineageSupportArtifact, StoreError> {
        dispatch_ref!(self, |backend| backend.fetch_lineage_support(commit_id))
    }

    pub fn fetch_lineage_history(
        &self,
        request: HistoricalIdentityRequest,
    ) -> Result<HistoricalIdentityResolution, StoreError> {
        dispatch_ref!(self, |backend| backend.fetch_lineage_history(request))
    }
}
