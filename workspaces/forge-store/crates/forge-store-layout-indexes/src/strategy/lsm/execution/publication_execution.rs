use forge_store_wal::{
    BlobWalRecordEnvelope, DurablePublicationDeclaration, DurablePublicationScope,
};

use super::BaselineLsmCounterObservation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmManifestPublicationExecution {
    membership_replacement: forge_store_lsm_authority::PublishedLsmMembershipReplacement,
    wal_publication: BlobWalRecordEnvelope,
    manifest_publication: DurablePublicationDeclaration,
    published_run_count: u16,
    stale_runs_removed: bool,
    advisory_filter_present: bool,
    counters: BaselineLsmCounterObservation,
}

impl BaselineLsmManifestPublicationExecution {
    pub(super) fn new(
        membership_replacement: forge_store_lsm_authority::PublishedLsmMembershipReplacement,
        wal_publication: BlobWalRecordEnvelope,
        manifest_publication: DurablePublicationDeclaration,
        published_run_count: u16,
        stale_runs_removed: bool,
        advisory_filter_present: bool,
    ) -> Self {
        Self {
            membership_replacement,
            wal_publication,
            manifest_publication,
            published_run_count,
            stale_runs_removed,
            advisory_filter_present,
            counters: BaselineLsmCounterObservation::manifest_publication(published_run_count),
        }
    }

    pub(crate) const fn membership_replacement(
        &self,
    ) -> &forge_store_lsm_authority::PublishedLsmMembershipReplacement {
        &self.membership_replacement
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
