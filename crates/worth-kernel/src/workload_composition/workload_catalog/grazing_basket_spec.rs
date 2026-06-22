use topology::facade::{OpenLayerPattern, OpenLayerStackSpec, OpenSheetPatchSpec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrazingBasketStackSpec {
    layer_count: usize,
    strips_per_layer: usize,
}

impl GrazingBasketStackSpec {
    pub fn new() -> Self {
        Self {
            layer_count: 4,
            strips_per_layer: 8,
        }
    }

    pub fn layers(mut self, layer_count: usize) -> Self {
        self.layer_count = layer_count;
        self
    }

    pub fn strips_per_layer(mut self, strips_per_layer: usize) -> Self {
        self.strips_per_layer = strips_per_layer;
        self
    }

    pub fn layer_count(self) -> usize {
        self.layer_count
    }

    pub fn strip_count_per_layer(self) -> usize {
        self.strips_per_layer
    }

    pub fn into_open_layer_stack_spec(self) -> OpenLayerStackSpec {
        OpenLayerStackSpec::new()
            .layers(self.layer_count)
            .layer_pattern(OpenLayerPattern::SheetPatch(
                OpenSheetPatchSpec::new().strips(self.strips_per_layer),
            ))
            .with_layer_identity()
            .with_open_boundary_receipts()
            .with_radial_adjacency_receipts()
    }
}

impl Default for GrazingBasketStackSpec {
    fn default() -> Self {
        Self::new()
    }
}
