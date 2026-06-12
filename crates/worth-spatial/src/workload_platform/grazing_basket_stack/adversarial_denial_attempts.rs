use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::denial::{GrazingBasketStackDenial, GrazingBasketStackDenialKind};
use super::layer_scope::{BasketBoundaryScope, BasketLayerIndex};
use super::receipt::{GrazingBasketStackCounters, GrazingBasketStackReceipt};

impl GrazingBasketStackReceipt {
    pub fn attempt_cross_layer_retained_replay(
        &self,
        source_layer: BasketLayerIndex,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let source = self.require_layer(source_layer)?;
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence =
            joined_evidence(&[source.retained_replay_identity(), target.layer_identity()]);
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::CrossLayerRetainedReplay,
            Some(source_layer),
            Some(target_layer),
            None,
            self.counters().for_attack(2, 2),
            &extra_evidence,
            format!(
                "Retained checkpoint from {} cannot replay onto {}; the checkpoint belongs to a different layer identity.",
                source_layer.human_name(),
                target_layer.human_name()
            ),
        ))
    }

    pub fn attempt_cross_layer_projection_identity(
        &self,
        source_layer: BasketLayerIndex,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let source = self.require_layer(source_layer)?;
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence =
            joined_evidence(&[source.projection_identity(), target.layer_identity()]);
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::CrossLayerProjectionIdentity,
            Some(source_layer),
            Some(target_layer),
            None,
            self.counters().for_attack(2, 2),
            &extra_evidence,
            format!(
                "Projection identity from {} cannot be consumed as retained evidence for {}; it belongs to a different layer identity.",
                source_layer.human_name(),
                target_layer.human_name()
            ),
        ))
    }

    pub fn attempt_surface_support_smuggling(
        &self,
        layer: BasketLayerIndex,
        family: &str,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer(layer)?;
        Err(self.denial(
            GrazingBasketStackDenialKind::UnsupportedSurfaceFamily,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            format!(
                "{} rejected {family} surface support; unsupported non-plane support must stay localized to that layer.",
                layer.human_name()
            ),
        ))
    }

    pub fn attempt_cross_layer_surface_support_smuggling(
        &self,
        source_layer: BasketLayerIndex,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let source = self.require_layer(source_layer)?;
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence = joined_evidence(&[
            source.local_frame_identity(),
            target.radial_adjacency_identity(),
        ]);
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::SurfaceSupportSmuggling,
            Some(source_layer),
            Some(target_layer),
            None,
            self.counters().for_attack(2, 2),
            &extra_evidence,
            format!(
                "Surface support receipt from {} cannot certify {}; the local frame and radial adjacency belong to different layers.",
                source_layer.human_name(),
                target_layer.human_name()
            ),
        ))
    }

    pub fn attempt_cross_layer_parity_lane_smuggling(
        &self,
        source_layer: BasketLayerIndex,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let source = self.require_layer(source_layer)?;
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence = joined_evidence(&[
            source.projection_identity(),
            target.retained_replay_identity(),
        ]);
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::CrossLayerParityLane,
            Some(source_layer),
            Some(target_layer),
            None,
            self.counters().for_attack(2, 2),
            &extra_evidence,
            format!(
                "Parity lane from {} cannot be consumed by {}; projection and retained checkpoint evidence are bound to different layer identities.",
                source_layer.human_name(),
                target_layer.human_name()
            ),
        ))
    }

    pub fn attempt_storm_extraction_smuggling(
        &self,
        layer: BasketLayerIndex,
        storm_digest: &str,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer(layer)?;
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::StormExtractionSmuggling,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            storm_digest,
            format!(
                "Closed storm extraction bundle cannot certify {}; basket stack layers require layer-local open topology receipts.",
                layer.human_name()
            ),
        ))
    }

    pub fn attempt_false_closure(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let receipt = self.require_layer(layer)?;
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::FalseClosure,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            receipt.open_boundary().boundary_identity(),
            format!(
                "{} cannot gain closed-shell or bounded-solid posture from grazing neighbors; its open boundary remains open.",
                layer.human_name()
            ),
        ))
    }

    pub fn attempt_label_only_motion(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer(layer)?;
        Err(self.denial(
            GrazingBasketStackDenialKind::LabelOnlyMotion,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            format!(
                "{} denied label-only movement before overlap extraction or readiness admission.",
                layer.human_name()
            ),
        ))
    }

    pub fn attempt_open_boundary_perturbation(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let receipt = self.require_layer(layer)?;
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::OpenBoundaryPerturbation,
            Some(layer),
            Some(layer),
            Some(receipt.open_boundary().clone()),
            self.counters().for_attack(1, 1),
            receipt.open_boundary().boundary_identity(),
            format!(
                "{} denied radial or open-boundary perturbation before projection success; the boundary remains layer-local.",
                layer.human_name()
            ),
        ))
    }

    pub fn attempt_missing_boundary_evidence(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer(layer)?;
        Err(self.denial(
            GrazingBasketStackDenialKind::MissingLayerEvidence,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            format!(
                "{} has no options without its open-boundary evidence.",
                layer.human_name()
            ),
        ))
    }

    pub fn attempt_missing_projection_evidence(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let receipt = self.require_layer(layer)?;
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::MissingProjectionEvidence,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            receipt.local_frame_identity(),
            format!(
                "{} has no options without projection evidence for its layer-local frame.",
                layer.human_name()
            ),
        ))
    }

    pub fn attempt_missing_retained_checkpoint_evidence(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let receipt = self.require_layer(layer)?;
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::MissingRetainedCheckpointEvidence,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            receipt.retained_replay_identity(),
            format!(
                "{} has no options without retained checkpoint evidence.",
                layer.human_name(),
            ),
        ))
    }

    pub fn attempt_near_graze_predicate_pressure(
        &self,
        layer: BasketLayerIndex,
        boundary: BasketBoundaryScope,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer(layer)?;
        let boundary_index = boundary.boundary_index();
        Err(self.denial(
            GrazingBasketStackDenialKind::PredicateUncertain,
            Some(layer),
            Some(layer),
            Some(boundary),
            self.counters().for_attack(1, 1),
            format!(
                "Near-graze predicate pressure localized to {}, boundary {}, local frame, and precision tier; it cannot admit aggregate closure.",
                layer.human_name(),
                boundary_index
            ),
        ))
    }

    pub fn attempt_whole_stack_broadening(
        &self,
        layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer(layer)?;
        Err(self.denial(
            GrazingBasketStackDenialKind::WholeStackBroadening,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            format!(
                "{} touched one layer; projection, replay, and diagnostics may not relabel the whole stack.",
                layer.human_name()
            ),
        ))
    }

    fn denial(
        &self,
        kind: GrazingBasketStackDenialKind,
        source_layer: Option<BasketLayerIndex>,
        target_layer: Option<BasketLayerIndex>,
        boundary: Option<BasketBoundaryScope>,
        counters: GrazingBasketStackCounters,
        human_reason: impl Into<String>,
    ) -> GrazingBasketStackDenial {
        let human_reason = human_reason.into();
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "grazing-basket-stack-denial".to_string(),
                self.stack_identity().to_string(),
                format!("{kind:?}"),
                human_reason.clone(),
                format!("touched_layers:{}", counters.touched_layers()),
            ],
        );
        GrazingBasketStackDenial::new(
            kind,
            source_layer,
            target_layer,
            boundary,
            counters.touched_layers(),
            evidence_digest,
            human_reason,
        )
    }

    fn denial_with_extra_evidence(
        &self,
        kind: GrazingBasketStackDenialKind,
        source_layer: Option<BasketLayerIndex>,
        target_layer: Option<BasketLayerIndex>,
        boundary: Option<BasketBoundaryScope>,
        counters: GrazingBasketStackCounters,
        extra_evidence: &str,
        human_reason: impl Into<String>,
    ) -> GrazingBasketStackDenial {
        let denial = self.denial(
            kind,
            source_layer,
            target_layer,
            boundary,
            counters,
            human_reason,
        );
        GrazingBasketStackDenial::new(
            denial.kind(),
            denial.source_layer(),
            denial.target_layer(),
            denial.boundary().cloned(),
            denial.touched_layers(),
            truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    denial.evidence_digest().to_string(),
                    extra_evidence.to_string(),
                ],
            ),
            denial.human_reason().to_string(),
        )
    }
}

fn joined_evidence(parts: &[&str]) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &parts
            .iter()
            .map(|part| (*part).to_string())
            .collect::<Vec<_>>(),
    )
}
