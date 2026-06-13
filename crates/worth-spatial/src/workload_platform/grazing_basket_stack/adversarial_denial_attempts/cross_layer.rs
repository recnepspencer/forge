use super::joined_evidence;
use crate::workload_platform::grazing_basket_stack::{
    BasketLayerIndex, GrazingBasketLayerAuthorityEvidence, GrazingBasketLayerEvidenceKind,
    GrazingBasketStackDenial, GrazingBasketStackDenialKind, GrazingBasketStackReceipt,
    GrazingBasketStormExtractionEvidence, GrazingBasketUnsupportedSurfaceEvidence,
};

impl GrazingBasketStackReceipt {
    pub fn attempt_cross_layer_retained_replay(
        &self,
        source_layer: BasketLayerIndex,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let source = GrazingBasketLayerAuthorityEvidence::retained_replay_from_layer(
            self.require_layer(source_layer)?,
        );
        self.attempt_cross_layer_retained_replay_evidence(&source, target_layer)
    }

    pub fn attempt_cross_layer_retained_replay_evidence(
        &self,
        source: &GrazingBasketLayerAuthorityEvidence,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer_evidence_kind(source, GrazingBasketLayerEvidenceKind::RetainedReplay)?;
        let source_layer = source.layer();
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence =
            joined_evidence(&[source.evidence_identity(), target.layer_identity()]);
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
        let source = GrazingBasketLayerAuthorityEvidence::projection_from_layer(
            self.require_layer(source_layer)?,
        );
        self.attempt_cross_layer_projection_identity_evidence(&source, target_layer)
    }

    pub fn attempt_cross_layer_projection_identity_evidence(
        &self,
        source: &GrazingBasketLayerAuthorityEvidence,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer_evidence_kind(source, GrazingBasketLayerEvidenceKind::Projection)?;
        let source_layer = source.layer();
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence =
            joined_evidence(&[source.evidence_identity(), target.layer_identity()]);
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

    pub fn attempt_unsupported_surface_support(
        &self,
        layer: BasketLayerIndex,
        evidence: &GrazingBasketUnsupportedSurfaceEvidence,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer(layer)?;
        Err(self.denial_with_extra_evidence(
            GrazingBasketStackDenialKind::UnsupportedSurfaceFamily,
            Some(layer),
            Some(layer),
            None,
            self.counters().for_attack(1, 1),
            evidence.unsupported_stage_identity(),
            format!(
                "{} rejected {}; {}.",
                layer.human_name(),
                evidence.family().human_label(),
                evidence.human_reason()
            ),
        ))
    }

    pub fn attempt_cross_layer_surface_support_smuggling(
        &self,
        source_layer: BasketLayerIndex,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        let source = GrazingBasketLayerAuthorityEvidence::surface_support_from_layer(
            self.require_layer(source_layer)?,
        );
        self.attempt_cross_layer_surface_support_smuggling_evidence(&source, target_layer)
    }

    pub fn attempt_cross_layer_surface_support_smuggling_evidence(
        &self,
        source: &GrazingBasketLayerAuthorityEvidence,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer_evidence_kind(source, GrazingBasketLayerEvidenceKind::SurfaceSupport)?;
        let source_layer = source.layer();
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence = joined_evidence(&[
            source.evidence_identity(),
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
        let source = GrazingBasketLayerAuthorityEvidence::parity_lane_from_layer(
            self.require_layer(source_layer)?,
        );
        self.attempt_cross_layer_parity_lane_smuggling_evidence(&source, target_layer)
    }

    pub fn attempt_cross_layer_parity_lane_smuggling_evidence(
        &self,
        source: &GrazingBasketLayerAuthorityEvidence,
        target_layer: BasketLayerIndex,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.require_layer_evidence_kind(source, GrazingBasketLayerEvidenceKind::ParityLane)?;
        let source_layer = source.layer();
        let target = self.require_layer(target_layer)?;
        if source_layer == target_layer {
            return Ok(());
        }
        let extra_evidence = joined_evidence(&[
            source.evidence_identity(),
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

    pub fn attempt_storm_extraction_smuggling_evidence(
        &self,
        layer: BasketLayerIndex,
        evidence: &GrazingBasketStormExtractionEvidence,
    ) -> Result<(), GrazingBasketStackDenial> {
        self.attempt_storm_extraction_smuggling(layer, evidence.projection_stage_identity())
    }
}
