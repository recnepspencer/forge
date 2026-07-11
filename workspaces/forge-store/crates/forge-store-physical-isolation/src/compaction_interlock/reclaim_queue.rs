use super::{
    CompactionReadInterlockCounters, CompactionReadInterlockDenial, CompactionRewritePublication,
};
use crate::{PhysicalReadPlanReleaseReceipt, ReleasedOldReachability};

#[derive(Debug, Clone)]
pub struct CompactionDeferredReclaimQueue {
    publication: CompactionRewritePublication,
    counters: CompactionReadInterlockCounters,
}

#[derive(Debug, Clone)]
pub struct DrainedCompactionReclaim {
    released: ReleasedOldReachability,
    counters: CompactionReadInterlockCounters,
}

impl CompactionDeferredReclaimQueue {
    pub const fn cutover_state(&self) -> super::CompactionCutoverState {
        super::CompactionCutoverState::ReclaimDeferred
    }

    pub const fn cutover_transition(&self) -> super::CompactionCutoverTransition {
        super::CompactionCutoverTransitionKind::DeferReclaim.transition()
    }

    pub fn admit(
        publication: CompactionRewritePublication,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        if !publication.delta().plan().reclaim_deferred() {
            return Ok(Self {
                counters: publication.counters(),
                publication,
            });
        }
        Ok(Self {
            counters: publication.counters().with_blocked_reclaim(),
            publication,
        })
    }

    pub fn reject_early_reclaim(
        &self,
    ) -> (
        CompactionReadInterlockDenial,
        CompactionReadInterlockCounters,
    ) {
        (
            CompactionReadInterlockDenial::EarlyReclaimBeforeReadRelease {
                protected: self
                    .publication
                    .delta()
                    .plan()
                    .protected()
                    .footprint_basis(),
            },
            self.counters.with_denied_early_reclaim(),
        )
    }

    pub fn drain_after_release(
        self,
        release: PhysicalReadPlanReleaseReceipt,
    ) -> Result<DrainedCompactionReclaim, CompactionReadInterlockDenial> {
        let released = self
            .publication
            .publication()
            .admit_old_reachability_release(release)
            .map_err(
                |_| CompactionReadInterlockDenial::EarlyReclaimBeforeReadRelease {
                    protected: self
                        .publication
                        .delta()
                        .plan()
                        .protected()
                        .footprint_basis(),
                },
            )?;
        Ok(DrainedCompactionReclaim {
            released,
            counters: self.counters,
        })
    }

    pub const fn counters(&self) -> CompactionReadInterlockCounters {
        self.counters
    }

    pub(crate) const fn publication(&self) -> &CompactionRewritePublication {
        &self.publication
    }
}

impl DrainedCompactionReclaim {
    pub const fn cutover_state(&self) -> super::CompactionCutoverState {
        super::CompactionCutoverState::Reclaimed
    }

    pub const fn cutover_transition(&self) -> super::CompactionCutoverTransition {
        super::CompactionCutoverTransitionKind::DrainReclaimAfterReadRelease.transition()
    }

    pub const fn released(&self) -> ReleasedOldReachability {
        self.released
    }

    pub const fn counters(&self) -> CompactionReadInterlockCounters {
        self.counters
    }
}
