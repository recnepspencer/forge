use crate::capability::DensityTokenId;

use super::WorthUiBoxEdges;

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveMeasurementReceipt {
    padding: WorthUiPrimitiveResolvedInsets,
    radius: WorthUiPrimitiveResolvedMeasurement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveResolvedMeasurement {
    token: String,
    points: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveResolvedInsets {
    token: String,
    edges: WorthUiBoxEdges,
}

impl WorthUiPrimitiveMeasurementReceipt {
    pub(crate) fn new(
        padding: WorthUiPrimitiveResolvedInsets,
        radius: WorthUiPrimitiveResolvedMeasurement,
    ) -> Self {
        Self { padding, radius }
    }

    pub fn padding(&self) -> &WorthUiPrimitiveResolvedInsets {
        &self.padding
    }

    pub fn radius(&self) -> &WorthUiPrimitiveResolvedMeasurement {
        &self.radius
    }
}

impl WorthUiPrimitiveResolvedMeasurement {
    pub(crate) fn new(token: &DensityTokenId, points: f32) -> Self {
        Self {
            token: token.as_str().to_owned(),
            points,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn points(&self) -> f32 {
        self.points
    }
}

impl WorthUiPrimitiveResolvedInsets {
    pub(crate) fn new(token: &DensityTokenId, edges: WorthUiBoxEdges) -> Self {
        Self {
            token: token.as_str().to_owned(),
            edges,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn edges(&self) -> WorthUiBoxEdges {
        self.edges
    }

    pub fn points(&self) -> f32 {
        self.edges.max_axis_point()
    }
}
