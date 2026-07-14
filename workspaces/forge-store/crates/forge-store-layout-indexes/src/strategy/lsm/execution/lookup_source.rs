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
    ) -> Result<BaselineLsmLookupExecution, crate::CounterEnvelopeViolation> {
        let (disposition, comparisons) = self.disposition_for(probe_sequence);
        let retired = self.publication.retired_records();
        let tombstone_blocks_older = disposition == BaselineLsmLookupDisposition::NotFound
            && probe_sequence == retired.value().sequence();
        BaselineLsmLookupExecution::new(
            admission,
            probe_sequence,
            disposition,
            retired.tombstone(),
            retired.value(),
            tombstone_blocks_older,
            comparisons,
        )
    }

    pub const fn replacement_output(&self) -> BlobWalRecordIdentity {
        self.publication.output()
    }

    fn disposition_for(&self, probe_sequence: u64) -> (BaselineLsmLookupDisposition, u16) {
        let retired = self.publication.retired_records();
        if retired.tombstone().sequence() == probe_sequence {
            (BaselineLsmLookupDisposition::Memtable, 1)
        } else if retired.generation().sequence() == probe_sequence {
            (BaselineLsmLookupDisposition::SortedRun, 2)
        } else {
            (BaselineLsmLookupDisposition::NotFound, 3)
        }
    }

    pub const fn publication(
        &self,
    ) -> &forge_store_lsm_authority::PublishedLsmMembershipReplacement {
        &self.publication
    }
}
