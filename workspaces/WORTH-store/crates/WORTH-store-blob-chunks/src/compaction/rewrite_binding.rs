use super::{BlobCompactionDenial, BlobCompactionEquivalence, BlobCompactionRewritePlan};
use crate::ChunkTreeRoot;
use worth_store_physical_isolation::ReadDuringCompactionVerdict;

#[derive(Debug, Clone)]
pub struct BlobCompactionPhysicalRewriteBinding {
    equivalence: BlobCompactionEquivalence,
    verdict: ReadDuringCompactionVerdict,
    expected_manifest_epoch: u64,
}

impl BlobCompactionPhysicalRewriteBinding {
    pub(crate) fn admit(
        plan: &BlobCompactionRewritePlan,
        equivalence: BlobCompactionEquivalence,
        verdict: ReadDuringCompactionVerdict,
    ) -> Result<Self, BlobCompactionDenial> {
        let expected_manifest_epoch = physical_rewrite_manifest_epoch_for_root(
            equivalence.new_root(),
            plan.physical().protected().root().manifest_epoch().get(),
        );
        if !equivalence.matches_plan_basis(plan)
            || verdict.proof().publication().delta().plan() != plan.physical()
            || !verdict.post_cutover_reader_observed_new_epoch()
            || verdict.proof().post_cutover_root().manifest_epoch().get() != expected_manifest_epoch
        {
            return Err(BlobCompactionDenial::MixedChunkTreePublication {
                counters: plan.counters().record_denial(),
            });
        }
        Ok(Self {
            equivalence,
            verdict,
            expected_manifest_epoch,
        })
    }

    pub const fn equivalence(&self) -> &BlobCompactionEquivalence {
        &self.equivalence
    }

    pub const fn verdict(&self) -> &ReadDuringCompactionVerdict {
        &self.verdict
    }

    pub const fn expected_manifest_epoch(&self) -> u64 {
        self.expected_manifest_epoch
    }
}

pub(crate) fn physical_rewrite_manifest_epoch_for_root(
    root: &ChunkTreeRoot,
    old_manifest_epoch: u64,
) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    mix_bytes(&mut hash, b"s7.blob-compaction.rewritten-physical-root");
    mix_bytes(&mut hash, root.digest().as_str().as_bytes());
    old_manifest_epoch.saturating_add((hash % 1_000_000).saturating_add(1))
}

fn mix_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}
