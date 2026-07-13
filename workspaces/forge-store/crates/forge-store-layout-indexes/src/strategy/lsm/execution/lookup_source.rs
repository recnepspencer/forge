use super::{BaselineLsmLookupAdmission, BaselineLsmLookupDisposition, BaselineLsmLookupExecution};
use forge_store_wal::BlobWalRecordIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmLookupSource {
    publication: forge_store_lsm_authority::PublishedLsmMembershipReplacement,
}

impl BaselineLsmLookupSource {
    pub(in crate::strategy::lsm) fn from_published_replacement(
        replacement: &forge_store_lsm_authority::PublishedLsmMembershipReplacement,
    ) -> Self {
        Self {
            publication: replacement.clone(),
        }
    }

    pub(crate) fn execute_latest_visible(
        self,
        admission: BaselineLsmLookupAdmission,
        probe_sequence: u64,
    ) -> BaselineLsmLookupExecution {
        let disposition = self.disposition_for(probe_sequence);
        let retired = self.publication.retired();
        let tombstone_blocks_older = disposition == BaselineLsmLookupDisposition::NotFound
            && probe_sequence == retired[0].sequence();
        BaselineLsmLookupExecution::new(
            admission,
            probe_sequence,
            disposition,
            retired[2],
            retired[0],
            tombstone_blocks_older,
        )
    }

    pub const fn replacement_output(&self) -> BlobWalRecordIdentity {
        self.publication.output()
    }

    fn disposition_for(&self, probe_sequence: u64) -> BaselineLsmLookupDisposition {
        let retired = self.publication.retired();
        if retired[2].sequence() == probe_sequence {
            BaselineLsmLookupDisposition::Memtable
        } else if retired[1].sequence() == probe_sequence {
            BaselineLsmLookupDisposition::SortedRun
        } else {
            BaselineLsmLookupDisposition::NotFound
        }
    }

    pub const fn publication(
        &self,
    ) -> &forge_store_lsm_authority::PublishedLsmMembershipReplacement {
        &self.publication
    }
}
