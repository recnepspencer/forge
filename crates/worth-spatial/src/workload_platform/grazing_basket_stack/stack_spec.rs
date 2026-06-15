use super::layer_scope::BasketLayerIndex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrazingOffsetClass {
    CertifiedSeparated,
    NearFeatureScale,
    PredicateHostile,
}

impl GrazingOffsetClass {
    pub const REQUIRED: [Self; 3] = [
        Self::CertifiedSeparated,
        Self::NearFeatureScale,
        Self::PredicateHostile,
    ];

    pub fn human_name(self) -> &'static str {
        match self {
            Self::CertifiedSeparated => "certified separated offset",
            Self::NearFeatureScale => "near feature scale offset",
            Self::PredicateHostile => "predicate hostile near-graze offset",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayerTransformPressure {
    MovementRotationStack { layer: BasketLayerIndex },
    HostileCancellation { layer: BasketLayerIndex },
}

impl LayerTransformPressure {
    pub fn layer(self) -> BasketLayerIndex {
        match self {
            Self::MovementRotationStack { layer } | Self::HostileCancellation { layer } => layer,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketStackCertificationProfile {
    offset_classes: Vec<GrazingOffsetClass>,
    transform_pressure: Vec<LayerTransformPressure>,
}

impl GrazingBasketStackCertificationProfile {
    pub fn hostile_default(layer_count: usize) -> Self {
        Self {
            offset_classes: GrazingOffsetClass::REQUIRED.to_vec(),
            transform_pressure: vec![
                LayerTransformPressure::MovementRotationStack {
                    layer: BasketLayerIndex::new(1.min(layer_count.saturating_sub(1))),
                },
                LayerTransformPressure::HostileCancellation {
                    layer: BasketLayerIndex::new(layer_count.saturating_sub(1)),
                },
            ],
        }
    }

    pub fn offset_classes(&self) -> &[GrazingOffsetClass] {
        &self.offset_classes
    }

    pub fn transform_pressure(&self) -> &[LayerTransformPressure] {
        &self.transform_pressure
    }
}
