use super::{
    UiMountedLogicalDamage, UiMountedPaintCommandChange, UiMountedPaintOrderEdit,
    UiMountedPaintOrderIntegrity, UiMountedPresentationAffinity,
    UiMountedPresentationAuxiliaryState,
};

#[derive(Debug, PartialEq)]
pub struct UiMountedPresentationDelta {
    pub(super) affinity: UiMountedPresentationAffinity,
    pub(super) changes: Box<[UiMountedPaintCommandChange]>,
    pub(super) nodes: Box<[super::UiMountedPresentationNodeChange]>,
    pub(super) order: Box<[UiMountedPaintOrderEdit]>,
    pub(super) order_integrity: UiMountedPaintOrderIntegrity,
    pub(super) damage: Box<[UiMountedLogicalDamage]>,
    pub(super) auxiliary: Option<UiMountedPresentationAuxiliaryState>,
    pub(super) production_cost: crate::UiMountedPresentationProductionCost,
}

#[doc(hidden)]
pub struct UiMountedPresentationDeltaInput {
    pub predecessor: crate::UiMountedFrameIdentity,
    pub successor: crate::UiMountedFrameIdentity,
    pub surface: crate::UiSemanticSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub content: crate::UiMountedContentGeneration,
    pub baseline: crate::UiHostSurfaceBaselineIdentity,
    pub changes: Vec<UiMountedPaintCommandChange>,
    pub nodes: Vec<super::UiMountedPresentationNodeChange>,
    pub order: Vec<UiMountedPaintOrderEdit>,
    pub order_integrity: UiMountedPaintOrderIntegrity,
    pub damage: Vec<UiMountedLogicalDamage>,
    pub auxiliary: Option<UiMountedPresentationAuxiliaryState>,
    pub production_cost: crate::UiMountedPresentationProductionCost,
}

impl UiMountedPresentationDelta {
    #[doc(hidden)]
    pub fn from_inert_mechanics(input: UiMountedPresentationDeltaInput) -> Self {
        let affinity = UiMountedPresentationAffinity::from_runtime(
            super::affinity::UiMountedPresentationAffinityInput {
                predecessor: Some(input.predecessor),
                successor: input.successor,
                surface: input.surface,
                binding: input.binding,
                content: input.content,
                baseline: input.baseline,
                receipt_affinity: None,
            },
        );
        Self {
            affinity,
            changes: input.changes.into_boxed_slice(),
            nodes: input.nodes.into_boxed_slice(),
            order: input.order.into_boxed_slice(),
            order_integrity: input.order_integrity,
            damage: input.damage.into_boxed_slice(),
            auxiliary: input.auxiliary,
            production_cost: input.production_cost,
        }
    }

    pub const fn affinity(&self) -> UiMountedPresentationAffinity {
        self.affinity
    }

    #[doc(hidden)]
    pub const fn with_successor_receipt_affinity(
        mut self,
        affinity: Option<crate::UiMountedNodeReceiptAffinity>,
    ) -> Self {
        self.affinity = self.affinity.with_receipt_affinity(affinity);
        self
    }

    pub fn changes(&self) -> &[UiMountedPaintCommandChange] {
        &self.changes
    }

    pub fn nodes(&self) -> &[super::UiMountedPresentationNodeChange] {
        &self.nodes
    }

    pub fn order(&self) -> &[UiMountedPaintOrderEdit] {
        &self.order
    }

    pub const fn order_integrity(&self) -> UiMountedPaintOrderIntegrity {
        self.order_integrity
    }

    pub fn damage(&self) -> &[UiMountedLogicalDamage] {
        &self.damage
    }

    pub fn auxiliary(&self) -> Option<&UiMountedPresentationAuxiliaryState> {
        self.auxiliary.as_ref()
    }

    pub const fn production_cost(&self) -> crate::UiMountedPresentationProductionCost {
        self.production_cost
    }
}
