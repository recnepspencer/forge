use forge_store_wal::{
    BlobWalRecordEnvelope, DurablePublicationDeclaration, DurablePublicationScope,
};

use super::super::BaselineLsmCounterObservation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmManifestPublicationExecution {
    maintenance_mode: crate::maintenance::IndexMaintenanceMode,
    membership_replacement: forge_store_lsm_authority::PublishedLsmMembershipReplacement,
    wal_publication: BlobWalRecordEnvelope,
    manifest_publication: DurablePublicationDeclaration,
    published_run_count: u16,
    stale_runs_removed: bool,
    advisory_filter_present: bool,
    counters: BaselineLsmCounterObservation,
}

impl BaselineLsmManifestPublicationExecution {
    pub(super) fn from_published(published: &super::PublishedLsmCompaction) -> Self {
        Self {
            maintenance_mode: published.maintenance_mode,
            membership_replacement: published.membership_replacement.clone(),
            wal_publication: published.wal_publication.clone(),
            manifest_publication: published.manifest_publication.clone(),
            published_run_count: published.sorted_run_records.len() as u16,
            stale_runs_removed: published.sorted_run_records.len() > 1,
            advisory_filter_present: false,
            counters: BaselineLsmCounterObservation::manifest_publication(
                published.sorted_run_records.len() as u16,
            ),
        }
    }

    pub(crate) const fn membership_replacement(
        &self,
    ) -> &forge_store_lsm_authority::PublishedLsmMembershipReplacement {
        &self.membership_replacement
    }

    pub const fn maintenance_mode(&self) -> crate::maintenance::IndexMaintenanceMode {
        self.maintenance_mode
    }

    pub const fn wal_publication(&self) -> &BlobWalRecordEnvelope {
        &self.wal_publication
    }

    pub const fn manifest_publication(&self) -> &DurablePublicationDeclaration {
        &self.manifest_publication
    }

    pub fn manifest_sequence_advanced(&self) -> bool {
        matches!(
            self.manifest_publication.scope(),
            DurablePublicationScope::Manifest(scope) if scope.covered_lsn_end() > scope.covered_lsn_start()
        )
    }

    pub const fn published_run_count(&self) -> u16 {
        self.published_run_count
    }

    pub const fn stale_runs_removed(&self) -> bool {
        self.stale_runs_removed
    }

    pub const fn advisory_filter_present(&self) -> bool {
        self.advisory_filter_present
    }

    pub const fn counters(&self) -> BaselineLsmCounterObservation {
        self.counters
    }
}
