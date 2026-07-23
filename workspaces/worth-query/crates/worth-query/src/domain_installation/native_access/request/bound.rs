use crate::basis_lifecycle::BasisOperationLane;
use crate::ordinary::read::WorthQueryProjectionDeclaration;

use super::super::{
    WorthQueryNativeAccessKey, WorthQueryNativeKeyResolution,
    WorthQueryNativeKeyResolutionCounters, WorthQueryNativeSelection,
    WorthQueryNativeSelectionDenial, WorthQueryNativeSelectionDenialKind,
};
use crate::domain_installation::WorthQueryConsumerProjectionContract;

pub struct WorthQueryBoundProjectionRequest<D, O, F, L: BasisOperationLane> {
    pub(super) consumer: WorthQueryConsumerProjectionContract<D, O, F, L>,
    pub(super) declaration: WorthQueryProjectionDeclaration,
    pub(super) plan: WorthQueryNativeAccessPlan,
    pub(super) request_identity: u64,
    pub(super) selector_key_slots: Vec<usize>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryBoundProjectionRequest<D, O, F, L> {
    pub fn declared_native_selection_count(&self) -> usize {
        self.plan.keys.len()
    }

    pub fn resolve_native_key(
        &self,
        selection: &WorthQueryNativeSelection,
    ) -> Result<WorthQueryNativeKeyResolution, WorthQueryNativeSelectionDenial> {
        let mut counters = WorthQueryNativeKeyResolutionCounters {
            declaration_checks: 1,
            ..WorthQueryNativeKeyResolutionCounters::default()
        };
        if selection.request_identity() != self.request_identity {
            return Err(WorthQueryNativeSelectionDenial::new(
                WorthQueryNativeSelectionDenialKind::DeclarationMismatch,
                counters,
            ));
        }
        counters.indexed_slot_lookups += 1;
        let key_slot = self.selector_key_slots[selection.declaration_slot()];
        counters.indexed_slot_lookups += 1;
        let key = &self.plan.keys[key_slot];
        Ok(WorthQueryNativeKeyResolution::new(key.clone(), counters))
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryConsumerProjectionContract<D, O, F, L>,
        WorthQueryProjectionDeclaration,
        WorthQueryNativeAccessPlan,
    ) {
        (self.consumer, self.declaration, self.plan)
    }
}

pub(crate) struct WorthQueryNativeAccessPlan {
    pub(crate) keys: Vec<WorthQueryNativeAccessKey>,
}
