use crate::compaction::receipt_construction::published_observation::BlobCompactionPublishedObservation;
use crate::compaction::transitions::execute_rewrite::BlobCompactionRewriteExecution;
use crate::compaction::transitions::publish_rewrite;
use crate::compaction::types::{BlobCompactionIntent, BlobCompactionRewritePlan};
use crate::{BlobCompactionDenial, BlobCompactionEquivalence};
use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_physical_isolation::ReadDuringCompactionVerdict;

/// Store-owned blob compaction authority.
///
/// External callers cannot mint this authority marker directly:
///
/// ```compile_fail
/// let _authority = forge_store_blob_chunks::BlobCompactionAuthority::store_owned();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCompactionAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

impl BlobCompactionAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    #[cfg(test)]
    pub(crate) fn store_owned() -> Self {
        Self::from_current_store_authority(
            crate::lifecycle::generation_registry_test_support::current_authority(
                "phase18.compaction-authority",
                "compaction",
            ),
        )
    }

    pub fn plan_compaction(
        &self,
        intent: BlobCompactionIntent,
    ) -> Result<BlobCompactionRewritePlan, BlobCompactionDenial> {
        BlobCompactionRewritePlan::admit(intent)
    }

    pub fn execute_rewrite(
        &self,
        plan: BlobCompactionRewritePlan,
        equivalence: BlobCompactionEquivalence,
        verdict: ReadDuringCompactionVerdict,
    ) -> Result<BlobCompactionRewriteExecution, BlobCompactionDenial> {
        BlobCompactionRewriteExecution::from_plan(plan, equivalence, verdict)
    }

    pub fn publish_rewrite(
        &self,
        execution: BlobCompactionRewriteExecution,
    ) -> Result<BlobCompactionPublishedObservation, BlobCompactionDenial> {
        publish_rewrite::publish_rewrite(execution)
    }
}
