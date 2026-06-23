use super::WorthUiBoxEdges;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveAlign {
    Start,
    Center,
    End,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveContainerReceipt {
    align: WorthUiPrimitiveAlign,
    padding_edges: WorthUiBoxEdges,
    radius_points: f32,
}

impl WorthUiPrimitiveContainerReceipt {
    pub(crate) fn new(
        align: WorthUiPrimitiveAlign,
        padding_edges: WorthUiBoxEdges,
        radius_points: f32,
    ) -> Self {
        Self {
            align,
            padding_edges,
            radius_points,
        }
    }

    pub fn align(&self) -> WorthUiPrimitiveAlign {
        self.align
    }

    pub fn padding_points(&self) -> f32 {
        self.padding_edges.max_axis_point()
    }

    pub fn padding_edges(&self) -> WorthUiBoxEdges {
        self.padding_edges
    }

    pub fn radius_points(&self) -> f32 {
        self.radius_points
    }
}
