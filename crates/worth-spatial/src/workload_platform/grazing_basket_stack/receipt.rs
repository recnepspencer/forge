use topology::facade::NmtTopologyConstructionCounters;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::layer_scope::{BasketBoundaryScope, BasketLayerIndex};
use super::stack_spec::{GrazingOffsetClass, LayerTransformPressure};
use super::transform_variant::GrazingBasketTransformVariantReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrazingBasketStackCounters {
    total_layers: usize,
    strips_per_layer: usize,
    touched_layers: usize,
    open_boundary_breadth: usize,
    projection_breadth: usize,
    retained_checkpoint_breadth: usize,
    local_frame_breadth: usize,
    radial_adjacency_breadth: usize,
    precision_escalation_breadth: usize,
    localization_breadth: usize,
}

impl GrazingBasketStackCounters {
    pub(crate) fn new(
        total_layers: usize,
        strips_per_layer: usize,
        open_boundary_breadth: usize,
    ) -> Self {
        Self {
            total_layers,
            strips_per_layer,
            touched_layers: total_layers,
            open_boundary_breadth,
            projection_breadth: total_layers,
            retained_checkpoint_breadth: total_layers,
            local_frame_breadth: total_layers,
            radial_adjacency_breadth: total_layers,
            precision_escalation_breadth: GrazingOffsetClass::REQUIRED.len(),
            localization_breadth: total_layers,
        }
    }

    pub(crate) fn for_attack(self, touched_layers: usize, localization_breadth: usize) -> Self {
        Self {
            touched_layers,
            localization_breadth,
            ..self
        }
    }

    pub fn total_layers(self) -> usize {
        self.total_layers
    }

    pub fn strips_per_layer(self) -> usize {
        self.strips_per_layer
    }

    pub fn touched_layers(self) -> usize {
        self.touched_layers
    }

    pub fn open_boundary_breadth(self) -> usize {
        self.open_boundary_breadth
    }

    pub fn projection_breadth(self) -> usize {
        self.projection_breadth
    }

    pub fn projection_consumption_breadth(self) -> usize {
        self.projection_breadth
    }

    pub fn retained_checkpoint_breadth(self) -> usize {
        self.retained_checkpoint_breadth
    }

    pub fn local_frame_breadth(self) -> usize {
        self.local_frame_breadth
    }

    pub fn radial_adjacency_breadth(self) -> usize {
        self.radial_adjacency_breadth
    }

    pub fn precision_escalation_breadth(self) -> usize {
        self.precision_escalation_breadth
    }

    pub fn localization_breadth(self) -> usize {
        self.localization_breadth
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketLayerReceipt {
    layer: BasketLayerIndex,
    layer_identity: String,
    topology_posture_identity: String,
    projection_identity: String,
    retained_replay_identity: String,
    transform_posture_identity: String,
    local_frame_identity: String,
    radial_adjacency_identity: String,
    open_boundary: BasketBoundaryScope,
    offset_class: GrazingOffsetClass,
    transform_pressure: Option<LayerTransformPressure>,
}

impl GrazingBasketLayerReceipt {
    pub(crate) fn new(input: GrazingBasketLayerReceiptInput<'_>) -> Self {
        let layer_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "grazing-basket-layer".to_string(),
                input.stack_identity.to_string(),
                format!("layer:{}", input.layer.get()),
                input.topology_identity.to_string(),
                input.projection_identity.to_string(),
                input.retained_replay_identity.to_string(),
                format!("offset:{:?}", input.offset_class),
                format!("pressure:{:?}", input.transform_pressure),
            ],
        );
        Self {
            layer: input.layer,
            layer_identity,
            topology_posture_identity: input.topology_identity.to_string(),
            projection_identity: input.projection_identity.to_string(),
            retained_replay_identity: input.retained_replay_identity.to_string(),
            transform_posture_identity: input.transform_posture_identity.to_string(),
            local_frame_identity: input.local_frame_identity.to_string(),
            radial_adjacency_identity: input.radial_adjacency_identity.to_string(),
            open_boundary: input.open_boundary,
            offset_class: input.offset_class,
            transform_pressure: input.transform_pressure,
        }
    }

    pub fn layer(&self) -> BasketLayerIndex {
        self.layer
    }

    pub fn layer_identity(&self) -> &str {
        &self.layer_identity
    }

    pub fn topology_posture_identity(&self) -> &str {
        &self.topology_posture_identity
    }

    pub fn projection_identity(&self) -> &str {
        &self.projection_identity
    }

    pub fn retained_replay_identity(&self) -> &str {
        &self.retained_replay_identity
    }

    pub fn transform_posture_identity(&self) -> &str {
        &self.transform_posture_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn radial_adjacency_identity(&self) -> &str {
        &self.radial_adjacency_identity
    }

    pub fn open_boundary(&self) -> &BasketBoundaryScope {
        &self.open_boundary
    }

    pub fn offset_class(&self) -> GrazingOffsetClass {
        self.offset_class
    }

    pub fn transform_pressure(&self) -> Option<LayerTransformPressure> {
        self.transform_pressure
    }
}

pub(crate) struct GrazingBasketLayerReceiptInput<'a> {
    pub layer: BasketLayerIndex,
    pub stack_identity: &'a str,
    pub topology_identity: &'a str,
    pub projection_identity: &'a str,
    pub retained_replay_identity: &'a str,
    pub transform_posture_identity: &'a str,
    pub local_frame_identity: &'a str,
    pub radial_adjacency_identity: &'a str,
    pub open_boundary: BasketBoundaryScope,
    pub offset_class: GrazingOffsetClass,
    pub transform_pressure: Option<LayerTransformPressure>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketStackReceipt {
    stack_identity: String,
    topology_construction_identity: String,
    projected_workload_identity: String,
    retained_replay_identity: String,
    transform_posture_identity: String,
    topology_counters: NmtTopologyConstructionCounters,
    layers: Vec<GrazingBasketLayerReceipt>,
    counters: GrazingBasketStackCounters,
}

impl GrazingBasketStackReceipt {
    pub(crate) fn new(input: GrazingBasketStackReceiptInput) -> Self {
        let counters = GrazingBasketStackCounters::new(
            input.layers.len(),
            input.strips_per_layer,
            input.open_boundary_breadth,
        );
        let stack_identity = stack_identity(&input, counters);
        Self {
            stack_identity,
            topology_construction_identity: input.topology_construction_identity,
            projected_workload_identity: input.projected_workload_identity,
            retained_replay_identity: input.retained_replay_identity,
            transform_posture_identity: input.transform_posture_identity,
            topology_counters: input.topology_counters,
            layers: input.layers,
            counters,
        }
    }

    pub fn stack_identity(&self) -> &str {
        &self.stack_identity
    }

    pub fn topology_construction_identity(&self) -> &str {
        &self.topology_construction_identity
    }

    pub fn projected_workload_identity(&self) -> &str {
        &self.projected_workload_identity
    }

    pub fn retained_replay_identity(&self) -> &str {
        &self.retained_replay_identity
    }

    pub fn transform_posture_identity(&self) -> &str {
        &self.transform_posture_identity
    }

    pub fn topology_counters(&self) -> NmtTopologyConstructionCounters {
        self.topology_counters
    }

    pub fn counters(&self) -> GrazingBasketStackCounters {
        self.counters
    }

    pub fn layers(&self) -> &[GrazingBasketLayerReceipt] {
        &self.layers
    }

    pub fn layer(&self, layer: BasketLayerIndex) -> Option<&GrazingBasketLayerReceipt> {
        self.layers.iter().find(|receipt| receipt.layer() == layer)
    }

    pub fn admit_equivalent_transform_variant(
        &self,
        layer: BasketLayerIndex,
        transform_pressure: LayerTransformPressure,
    ) -> Result<GrazingBasketTransformVariantReceipt, super::denial::GrazingBasketStackDenial> {
        let receipt = self.require_layer(layer)?;
        if transform_pressure.layer() != layer {
            return Err(super::denial::GrazingBasketStackDenial::new(
                super::denial::GrazingBasketStackDenialKind::LabelOnlyMotion,
                Some(transform_pressure.layer()),
                Some(layer),
                None,
                1,
                self.stack_identity.clone(),
                format!(
                    "Equivalent transform pressure for {} cannot certify {}.",
                    transform_pressure.layer().human_name(),
                    layer.human_name()
                ),
            ));
        }
        Ok(GrazingBasketTransformVariantReceipt::new(
            receipt,
            transform_pressure,
        ))
    }

    pub(crate) fn require_layer(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<&GrazingBasketLayerReceipt, super::denial::GrazingBasketStackDenial> {
        self.layer(layer).ok_or_else(|| {
            super::denial::GrazingBasketStackDenial::new(
                super::denial::GrazingBasketStackDenialKind::MissingLayerEvidence,
                Some(layer),
                Some(layer),
                None,
                1,
                self.stack_identity.clone(),
                format!(
                    "{} is missing from the grazing basket stack.",
                    layer.human_name()
                ),
            )
        })
    }
}

pub(crate) struct GrazingBasketStackReceiptInput {
    pub topology_construction_identity: String,
    pub projected_workload_identity: String,
    pub retained_replay_identity: String,
    pub transform_posture_identity: String,
    pub topology_counters: NmtTopologyConstructionCounters,
    pub layers: Vec<GrazingBasketLayerReceipt>,
    pub strips_per_layer: usize,
    pub open_boundary_breadth: usize,
}

fn stack_identity(
    input: &GrazingBasketStackReceiptInput,
    counters: GrazingBasketStackCounters,
) -> String {
    let mut parts = vec![
        "grazing-basket-stack".to_string(),
        input.topology_construction_identity.clone(),
        input.projected_workload_identity.clone(),
        input.retained_replay_identity.clone(),
        format!("layers:{}", counters.total_layers()),
        format!("strips:{}", counters.strips_per_layer()),
    ];
    parts.extend(
        input
            .layers
            .iter()
            .map(|layer| layer.layer_identity().to_string()),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
