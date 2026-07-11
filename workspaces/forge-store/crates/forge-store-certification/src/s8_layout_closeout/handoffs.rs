use super::S8LayoutCloseoutDenial;
use forge_store_layout_indexes::layout_certification::{
    S8LayoutHazardRow, S9FormalModelTarget, S9LayoutMachineContract, S9LayoutStateMachine,
    StorageFoundationS9LayoutHandoff,
};
use forge_store_readiness::S8LayoutHandoffReadiness;

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutCourtroomGrammar {
    handoff: StorageFoundationS9LayoutHandoff,
}

pub(crate) fn preserve_s8_layout_handoff_grammar(
    readiness: S8LayoutHandoffReadiness,
) -> Result<S8LayoutCourtroomGrammar, S8LayoutCloseoutDenial> {
    // This is a non-certifying projection. Executed lane completion remains
    // behind certify_s8_layout_closeout_suite and cannot be inferred here.
    Ok(S8LayoutCourtroomGrammar {
        handoff: readiness.into_handoff(),
    })
}

impl S8LayoutCourtroomGrammar {
    /// The Store-owned S.9 modeling grammar preserved through the courtroom.
    pub const fn grammar(&self) -> &StorageFoundationS9LayoutHandoff {
        &self.handoff
    }

    /// Per-machine requirements remain inspectable by S.9 without reducing
    /// their evidence to copied labels or counter summaries.
    pub fn obligations_for(
        &self,
        machine: S9LayoutStateMachine,
    ) -> impl Iterator<Item = S8LayoutHazardRow> + '_ {
        self.handoff.obligations_for(machine)
    }

    pub fn machine_contract(
        &self,
        machine: S9LayoutStateMachine,
    ) -> Option<S9LayoutMachineContract> {
        self.handoff.machine_contract(machine)
    }

    pub fn declares_pending_protocol_target(&self, target: S9FormalModelTarget) -> bool {
        self.handoff.declares_pending_protocol_target(target)
    }
}
