use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::undo_family_catalog::TopologyUndoFamilyIdentity;

#[derive(Clone, Copy, Debug)]
pub struct TopologyUndoSemanticGraphAdmissionRequest<'a> {
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
}

impl<'a> TopologyUndoSemanticGraphAdmissionRequest<'a> {
    pub fn new(
        family_identity: TopologyUndoFamilyIdentity,
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
    ) -> Self {
        Self {
            family_identity,
            touched_closure,
            invalidation_receipt,
        }
    }

    pub const fn family_identity(&self) -> TopologyUndoFamilyIdentity {
        self.family_identity
    }

    pub const fn touched_closure(&self) -> &'a DerivedInvalidationTouchedClosure {
        self.touched_closure
    }

    pub const fn invalidation_receipt(&self) -> &'a DerivedInvalidationExecutionReceipt {
        self.invalidation_receipt
    }
}
