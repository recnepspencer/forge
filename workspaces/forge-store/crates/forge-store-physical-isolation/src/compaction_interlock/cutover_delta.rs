use super::{CompactionReadInterlockDenial, CompactionReadInterlockPlan};
use crate::{CurrentPhysicalRoot, PhysicalPublicationReceipt};

#[derive(Debug, Clone)]
pub struct CompactionCutoverDelta {
    plan: CompactionReadInterlockPlan,
    rewritten_root: CurrentPhysicalRoot,
}

impl CompactionCutoverDelta {
    pub const fn cutover_state(&self) -> super::CompactionCutoverState {
        super::CompactionCutoverState::RewriteLowered
    }

    pub const fn cutover_transition(&self) -> super::CompactionCutoverTransition {
        super::CompactionCutoverTransitionKind::LowerRewrite.transition()
    }

    pub fn lower(
        plan: CompactionReadInterlockPlan,
        rewritten_root: CurrentPhysicalRoot,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        if rewritten_root.epoch() != plan.target_epoch() {
            return Err(CompactionReadInterlockDenial::StaleEpochReuse {
                source_epoch: plan.source_epoch(),
                reused_epoch: rewritten_root.epoch(),
            });
        }
        Ok(Self {
            plan,
            rewritten_root,
        })
    }

    pub(crate) fn bind_publication(
        self,
        publication: &PhysicalPublicationReceipt,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        if publication.old_root() != self.plan.protected().root()
            || publication.new_root() != self.rewritten_root
        {
            return Err(CompactionReadInterlockDenial::PublicationRootMismatch);
        }
        if !publication.old_reachability().retained_until_release() {
            return Err(CompactionReadInterlockDenial::MissingOldRootPreservation);
        }
        if publication.old_reachability().footprint_basis()
            != self.plan.protected().footprint_basis()
        {
            return Err(
                CompactionReadInterlockDenial::PublicationReachabilityFootprintMismatch {
                    protected: self.plan.protected().footprint_basis(),
                    preserved: publication.old_reachability().footprint_basis(),
                },
            );
        }
        Ok(self)
    }

    pub const fn plan(&self) -> &CompactionReadInterlockPlan {
        &self.plan
    }

    pub const fn rewritten_root(&self) -> CurrentPhysicalRoot {
        self.rewritten_root
    }
}
