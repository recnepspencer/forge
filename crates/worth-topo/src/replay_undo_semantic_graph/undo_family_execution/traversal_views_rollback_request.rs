use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;

#[derive(Clone, Copy, Debug)]
pub struct TraversalViewsRollbackRequest<'a> {
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
}

impl<'a> TraversalViewsRollbackRequest<'a> {
    pub fn new(
        touched_closure: &'a DerivedInvalidationTouchedClosure,
        invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
    ) -> Self {
        Self {
            touched_closure,
            invalidation_receipt,
        }
    }

    pub const fn touched_closure(&self) -> &'a DerivedInvalidationTouchedClosure {
        self.touched_closure
    }

    pub const fn invalidation_receipt(&self) -> &'a DerivedInvalidationExecutionReceipt {
        self.invalidation_receipt
    }
}
