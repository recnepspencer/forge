use super::super::super::WorthUiBoxEdges;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveHitFrameDerivationBasis {
    VisualBounds,
    FlowPadding,
    ExplicitHitSlop,
    DisabledNone,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthUiPrimitiveHitFrameDerivationReceipt {
    basis: WorthUiPrimitiveHitFrameDerivationBasis,
    edges: WorthUiBoxEdges,
}

impl WorthUiPrimitiveHitFrameDerivationReceipt {
    pub(super) fn new(
        basis: WorthUiPrimitiveHitFrameDerivationBasis,
        edges: WorthUiBoxEdges,
    ) -> Self {
        Self { basis, edges }
    }

    pub fn basis(self) -> WorthUiPrimitiveHitFrameDerivationBasis {
        self.basis
    }

    pub fn edges(self) -> WorthUiBoxEdges {
        self.edges
    }
}
