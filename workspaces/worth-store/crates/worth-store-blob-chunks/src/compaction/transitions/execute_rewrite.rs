use crate::compaction::rewrite_binding::BlobCompactionPhysicalRewriteBinding;
use crate::compaction::types::BlobCompactionRewritePlan;
use crate::compaction::{BlobCompactionDenial, BlobCompactionEquivalence};
use worth_store_physical_isolation::ReadDuringCompactionVerdict;

#[derive(Debug)]
pub struct BlobCompactionRewriteExecution {
    plan: BlobCompactionRewritePlan,
    binding: BlobCompactionPhysicalRewriteBinding,
}

impl BlobCompactionRewriteExecution {
    pub(crate) fn from_plan(
        plan: BlobCompactionRewritePlan,
        equivalence: BlobCompactionEquivalence,
        verdict: ReadDuringCompactionVerdict,
    ) -> Result<Self, BlobCompactionDenial> {
        let binding = BlobCompactionPhysicalRewriteBinding::admit(&plan, equivalence, verdict)?;
        Ok(Self { plan, binding })
    }

    pub const fn plan(&self) -> &BlobCompactionRewritePlan {
        &self.plan
    }

    pub const fn equivalence(&self) -> &BlobCompactionEquivalence {
        self.binding.equivalence()
    }

    pub const fn verdict(&self) -> &ReadDuringCompactionVerdict {
        self.binding.verdict()
    }

    pub const fn binding(&self) -> &BlobCompactionPhysicalRewriteBinding {
        &self.binding
    }
}
