use super::{CompactionCutoverDelta, CompactionReadInterlockDenial};
use crate::PhysicalPublicationReceipt;
use forge_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmCompactionPublicationReceipt;

#[derive(Debug, Clone)]
pub struct LsmCompactionCutoverDelta {
    delta: CompactionCutoverDelta,
    receipt: BaselineLsmCompactionPublicationReceipt,
}

impl LsmCompactionCutoverDelta {
    pub fn admit(
        delta: CompactionCutoverDelta,
        receipt: BaselineLsmCompactionPublicationReceipt,
    ) -> LsmCompactionCutoverAdmission {
        let binding = receipt.physical_publication();
        let rewritten = delta.rewritten_root();
        if binding.target_epoch() != rewritten.epoch().get()
            || binding.root_scope() != rewritten.scope()
            || binding.manifest_epoch() != rewritten.manifest_epoch().get()
        {
            return LsmCompactionCutoverAdmission::physical_target_denied(
                CompactionReadInterlockDenial::LsmPhysicalTargetMismatch,
            );
        }
        LsmCompactionCutoverAdmission {
            outcome: LsmCompactionCutoverOutcome::Admitted(Self { delta, receipt }),
        }
    }

    pub const fn cutover_state(&self) -> super::CompactionCutoverState {
        super::CompactionCutoverState::LsmTombstoneRetentionAdmitted
    }

    pub const fn cutover_transition(&self) -> super::CompactionCutoverTransition {
        super::CompactionCutoverTransitionKind::AdmitLsmTombstoneRetention.transition()
    }

    pub const fn delta(&self) -> &CompactionCutoverDelta {
        &self.delta
    }

    pub const fn receipt(&self) -> &BaselineLsmCompactionPublicationReceipt {
        &self.receipt
    }

    pub(crate) fn bind_publication(
        self,
        publication: &PhysicalPublicationReceipt,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        self.delta.clone().bind_publication(publication)?;
        Ok(self)
    }
}

#[derive(Debug)]
pub struct LsmCompactionCutoverAdmission {
    outcome: LsmCompactionCutoverOutcome,
}

macro_rules! define_lsm_compaction_cutover_outcomes {
    ($( $variant:ident($payload:ty) => $fact:ident ),+ $(,)?) => {
        #[derive(Debug)]
        enum LsmCompactionCutoverOutcome {
            $($variant($payload)),+
        }

        impl LsmCompactionCutoverOutcome {
            const fn production_transition(&self) -> super::CompactionCutoverTransition {
                match self {
                    $(Self::$variant(_) => super::CompactionCutoverTransitionKind::$fact.transition()),+
                }
            }
        }
    };
}

define_lsm_compaction_cutover_outcomes!(
    Admitted(LsmCompactionCutoverDelta) => AdmitLsmTombstoneRetention,
    PhysicalTargetDenied(CompactionReadInterlockDenial) => DenyLsmPhysicalTarget,
);

impl LsmCompactionCutoverAdmission {
    const fn physical_target_denied(denial: CompactionReadInterlockDenial) -> Self {
        Self {
            outcome: LsmCompactionCutoverOutcome::PhysicalTargetDenied(denial),
        }
    }

    pub fn into_result(self) -> Result<LsmCompactionCutoverDelta, CompactionReadInterlockDenial> {
        match self.outcome {
            LsmCompactionCutoverOutcome::Admitted(delta) => Ok(delta),
            LsmCompactionCutoverOutcome::PhysicalTargetDenied(denial) => Err(denial),
        }
    }

    pub const fn production_transition(&self) -> super::CompactionCutoverTransition {
        self.outcome.production_transition()
    }
}

#[cfg(any(test, feature = "certification-authority"))]
pub fn execute_baseline_lsm_compaction_for_certification(
    root: crate::CurrentPhysicalRoot,
) -> BaselineLsmCompactionPublicationReceipt {
    let binding = forge_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmPhysicalPublicationBinding::new(
        root.scope(),
        root.epoch().get(),
        root.manifest_epoch().get(),
    )
    .expect("physical publication binding");
    forge_store_wal::layout_access::execute_baseline_lsm_persisted_fixture(binding)
        .compaction_publication_receipt()
        .clone()
}
