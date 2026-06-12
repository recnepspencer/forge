use topology::facade::{NmtTopologyConstructionReceipt, NmtTopologyPattern, NmtTopologyPosture};

use super::denial::{GrazingBasketStackDenial, GrazingBasketStackDenialKind};
use super::layer_scope::{BasketBoundaryScope, BasketLayerIndex};
use super::receipt::{
    GrazingBasketLayerReceipt, GrazingBasketLayerReceiptInput, GrazingBasketStackReceipt,
    GrazingBasketStackReceiptInput,
};
use super::stack_spec::{GrazingBasketStackCertificationProfile, GrazingOffsetClass};
use crate::workload_platform::evidence_ledger::CompleteWorkloadEvidenceLedger;
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;
use crate::workload_platform::transform_workload::TransformReceiptSet;

pub struct GrazingBasketStackWorkload<'a> {
    topology_construction: &'a NmtTopologyConstructionReceipt,
    evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
    projected_workload: &'a ProjectedPlanarWorkload,
    transform_receipts: &'a TransformReceiptSet,
    replay_receipts: &'a ReplayReceiptSet,
    profile: GrazingBasketStackCertificationProfile,
}

impl<'a> GrazingBasketStackWorkload<'a> {
    pub fn from_platform_evidence(
        topology_construction: &'a NmtTopologyConstructionReceipt,
        evidence_ledger: &'a CompleteWorkloadEvidenceLedger,
        projected_workload: &'a ProjectedPlanarWorkload,
        transform_receipts: &'a TransformReceiptSet,
        replay_receipts: &'a ReplayReceiptSet,
    ) -> Self {
        let profile = GrazingBasketStackCertificationProfile::hostile_default(
            topology_construction.counters().layer_count(),
        );
        Self {
            topology_construction,
            evidence_ledger,
            projected_workload,
            transform_receipts,
            replay_receipts,
            profile,
        }
    }

    pub fn with_profile(mut self, profile: GrazingBasketStackCertificationProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn certify(self) -> Result<GrazingBasketStackReceipt, GrazingBasketStackDenial> {
        self.require_open_layer_stack()?;
        self.require_layered_open_posture()?;
        self.require_hostile_but_bounded_profile()?;
        self.require_platform_evidence()?;
        self.require_all_offset_classes()?;
        let layers = self.layer_receipts()?;
        Ok(GrazingBasketStackReceipt::new(
            GrazingBasketStackReceiptInput {
                topology_construction_identity: self
                    .topology_construction
                    .pattern_identity()
                    .identity_digest()
                    .to_string(),
                projected_workload_identity: self
                    .projected_workload
                    .receipts()
                    .stage_identity()
                    .receipt_identity(),
                retained_replay_identity: self
                    .replay_receipts
                    .replay_checkpoint_identity()
                    .to_string(),
                transform_posture_identity: self
                    .transform_receipts
                    .transform_posture_receipt()
                    .posture_identity()
                    .to_string(),
                topology_counters: self.topology_construction.counters(),
                strips_per_layer: self.strips_per_layer(),
                open_boundary_breadth: self
                    .topology_construction
                    .open_boundary()
                    .boundary_half_edge_count(),
                layers,
            },
        ))
    }

    fn require_open_layer_stack(&self) -> Result<(), GrazingBasketStackDenial> {
        match self.topology_construction.pattern() {
            NmtTopologyPattern::OpenLayerStack(_) => Ok(()),
            _ => Err(self.denial(
                GrazingBasketStackDenialKind::WrongTopologyPattern,
                None,
                None,
                0,
                "Grazing basket stack certification requires an open layer stack topology construction receipt.",
            )),
        }
    }

    fn require_layered_open_posture(&self) -> Result<(), GrazingBasketStackDenial> {
        if self.topology_construction.topology_posture().posture()
            == NmtTopologyPosture::LayeredOpen
        {
            return Ok(());
        }
        Err(self.denial(
            GrazingBasketStackDenialKind::WrongTopologyPosture,
            None,
            None,
            0,
            "Grazing basket stack certification requires layered open topology posture.",
        ))
    }

    fn require_hostile_but_bounded_profile(&self) -> Result<(), GrazingBasketStackDenial> {
        let counters = self.topology_construction.counters();
        let layer_count = counters.layer_count();
        let strips_per_layer = self.strips_per_layer();
        if !(4..=7).contains(&layer_count) {
            return Err(self.denial(
                GrazingBasketStackDenialKind::UnsupportedLayerProfile,
                None,
                None,
                layer_count,
                format!(
                    "Grazing basket stack admits 4 through 7 layers for the boss; got {layer_count}."
                ),
            ));
        }
        if !(8..=16).contains(&strips_per_layer) {
            return Err(self.denial(
                GrazingBasketStackDenialKind::UnsupportedLayerProfile,
                None,
                None,
                layer_count,
                format!(
                    "Grazing basket stack admits 8 through 16 strips per layer; got {strips_per_layer}."
                ),
            ));
        }
        Ok(())
    }

    fn require_platform_evidence(&self) -> Result<(), GrazingBasketStackDenial> {
        self.evidence_ledger
            .guards()
            .assert_uses_real_topology()
            .and_then(|guard| guard.assert_binding_is_receipt_backed())
            .and_then(|guard| guard.assert_projection_is_receipt_backed())
            .and_then(|guard| guard.assert_transform_changed_geometry())
            .and_then(|guard| guard.assert_replay_consumed_retained_artifact())
            .and_then(|guard| guard.assert_counters_are_receipt_backed())
            .and_then(|guard| guard.assert_no_fixture_arithmetic_as_truth())
            .and_then(|guard| guard.assert_no_synthetic_end_to_end_claim())
            .map(|_| ())
            .map_err(|error| {
                self.denial(
                    GrazingBasketStackDenialKind::MissingPlatformEvidence,
                    None,
                    None,
                    0,
                    format!(
                        "Grazing basket stack requires complete receipt-backed platform evidence: {}.",
                        error.human_reason()
                    ),
                )
            })
    }

    fn require_all_offset_classes(&self) -> Result<(), GrazingBasketStackDenial> {
        for required in GrazingOffsetClass::REQUIRED {
            if !self.profile.offset_classes().contains(&required) {
                return Err(self.denial(
                    GrazingBasketStackDenialKind::MissingLayerEvidence,
                    None,
                    None,
                    0,
                    format!(
                        "Grazing basket stack is missing {} evidence.",
                        required.human_name()
                    ),
                ));
            }
        }
        Ok(())
    }

    fn layer_receipts(&self) -> Result<Vec<GrazingBasketLayerReceipt>, GrazingBasketStackDenial> {
        let stack_identity = self
            .topology_construction
            .pattern_identity()
            .identity_digest()
            .to_string();
        let topology_identity = self.topology_construction.query_declaration_identity();
        let projection_identity = self
            .projected_workload
            .receipts()
            .stage_identity()
            .receipt_identity();
        let retained_replay_identity = self.replay_receipts.replay_checkpoint_identity();
        let transform_posture_identity = self
            .transform_receipts
            .transform_posture_receipt()
            .posture_identity();
        let radial_adjacency_identity = self
            .topology_construction
            .radial_adjacency()
            .radial_digest();
        let mut layers = Vec::new();
        for layer in 0..self.topology_construction.counters().layer_count() {
            let layer_index = BasketLayerIndex::new(layer);
            let transform_pressure = self
                .profile
                .transform_pressure()
                .iter()
                .copied()
                .find(|pressure| pressure.layer() == layer_index);
            layers.push(GrazingBasketLayerReceipt::new(
                GrazingBasketLayerReceiptInput {
                    layer: layer_index,
                    stack_identity: &stack_identity,
                    topology_identity,
                    projection_identity: &projection_identity,
                    retained_replay_identity,
                    transform_posture_identity,
                    radial_adjacency_identity,
                    open_boundary: BasketBoundaryScope::new(layer_index, 0, &stack_identity),
                    offset_class: self.profile.offset_classes()
                        [layer % self.profile.offset_classes().len()],
                    transform_pressure,
                },
            ));
        }
        Ok(layers)
    }

    fn strips_per_layer(&self) -> usize {
        let counters = self.topology_construction.counters();
        counters.face_count() / counters.layer_count().max(1)
    }

    fn denial(
        &self,
        kind: GrazingBasketStackDenialKind,
        source_layer: Option<BasketLayerIndex>,
        target_layer: Option<BasketLayerIndex>,
        touched_layers: usize,
        human_reason: impl Into<String>,
    ) -> GrazingBasketStackDenial {
        GrazingBasketStackDenial::new(
            kind,
            source_layer,
            target_layer,
            None,
            touched_layers,
            self.topology_construction
                .pattern_identity()
                .identity_digest()
                .to_string(),
            human_reason,
        )
    }
}
