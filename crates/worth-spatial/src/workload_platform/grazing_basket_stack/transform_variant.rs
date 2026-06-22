use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::layer_scope::BasketLayerIndex;
use super::receipt::GrazingBasketLayerReceipt;
use super::stack_spec::LayerTransformPressure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketTransformVariantReceipt {
    layer: BasketLayerIndex,
    transform_pressure: LayerTransformPressure,
    layer_identity: String,
    transform_posture_identity: String,
    variant_identity: String,
}

impl GrazingBasketTransformVariantReceipt {
    pub(crate) fn new(
        layer: &GrazingBasketLayerReceipt,
        transform_pressure: LayerTransformPressure,
    ) -> Self {
        let variant_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "grazing-basket-equivalent-transform-variant".to_string(),
                layer.layer_identity().to_string(),
                layer.transform_posture_identity().to_string(),
                format!("{transform_pressure:?}"),
            ],
        );
        Self {
            layer: layer.layer(),
            transform_pressure,
            layer_identity: layer.layer_identity().to_string(),
            transform_posture_identity: layer.transform_posture_identity().to_string(),
            variant_identity,
        }
    }

    pub fn layer(&self) -> BasketLayerIndex {
        self.layer
    }

    pub fn transform_pressure(&self) -> LayerTransformPressure {
        self.transform_pressure
    }

    pub fn layer_identity(&self) -> &str {
        &self.layer_identity
    }

    pub fn transform_posture_identity(&self) -> &str {
        &self.transform_posture_identity
    }

    pub fn variant_identity(&self) -> &str {
        &self.variant_identity
    }
}
