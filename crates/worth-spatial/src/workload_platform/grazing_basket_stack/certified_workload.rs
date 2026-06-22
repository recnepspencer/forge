use super::denial::{GrazingBasketStackDenial, GrazingBasketStackDenialKind};
use super::layer_scope::{BasketBoundaryScope, BasketLayerIndex};
use super::receipt::{
    GrazingBasketLayerReceipt, GrazingBasketLayerReceiptInput, GrazingBasketStackReceipt,
    GrazingBasketStackReceiptInput,
};
use super::stack_spec::{GrazingBasketStackCertificationProfile, GrazingOffsetClass};
use crate::workload_platform::nmt_certification_context::{
    NmtCertifiedScopeContext, NmtCertifiedScopeSet,
};

pub struct CertifiedGrazingBasketStackWorkload<'a> {
    pub(super) certified_scopes: &'a NmtCertifiedScopeSet,
    pub(super) profile: GrazingBasketStackCertificationProfile,
}

impl<'a> CertifiedGrazingBasketStackWorkload<'a> {
    pub fn with_profile(mut self, profile: GrazingBasketStackCertificationProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn certify(self) -> Result<GrazingBasketStackReceipt, GrazingBasketStackDenial> {
        let layer_count = self.certified_scopes.scopes().len();
        let first_scope = self.certified_scopes.scopes().first().ok_or_else(|| {
            self.denial(
                GrazingBasketStackDenialKind::MissingLayerEvidence,
                None,
                None,
                0,
                "Grazing basket stack requires certified NMT layer scopes.",
            )
        })?;
        self.require_layer_profile(layer_count, first_scope)?;
        self.require_all_offset_classes()?;
        let stack_identity = self
            .certified_scopes
            .parent_construction_identity()
            .to_string();
        let layers = self
            .certified_scopes
            .scopes()
            .iter()
            .enumerate()
            .map(|(index, scope)| self.layer_receipt(index, scope, &stack_identity))
            .collect::<Vec<_>>();
        Ok(GrazingBasketStackReceipt::new(
            GrazingBasketStackReceiptInput {
                topology_construction_identity: stack_identity.clone(),
                projected_workload_identity: first_scope
                    .projection()
                    .parent_projection_identity()
                    .to_string(),
                retained_replay_identity: first_scope
                    .retained_replay()
                    .parent_replay_identity()
                    .to_string(),
                transform_posture_identity: first_scope
                    .motion()
                    .transform_posture_identity()
                    .to_string(),
                topology_counters: self.certified_scopes.topology_counters(),
                layers,
                strips_per_layer: first_scope.topology_scope().counters().face_count(),
                open_boundary_breadth: layer_count,
            },
        ))
    }

    fn require_layer_profile(
        &self,
        layer_count: usize,
        first_scope: &NmtCertifiedScopeContext,
    ) -> Result<(), GrazingBasketStackDenial> {
        if !(4..=7).contains(&layer_count) {
            return Err(self.denial(
                GrazingBasketStackDenialKind::UnsupportedLayerProfile,
                None,
                None,
                layer_count,
                format!(
                    "Grazing basket stack admits 4 through 7 certified layers; got {layer_count}."
                ),
            ));
        }
        let strips_per_layer = first_scope.topology_scope().counters().face_count();
        if !(8..=16).contains(&strips_per_layer) {
            return Err(self.denial(
                GrazingBasketStackDenialKind::UnsupportedLayerProfile,
                None,
                None,
                layer_count,
                format!(
                    "Grazing basket stack admits 8 through 16 strips per certified layer; got {strips_per_layer}."
                ),
            ));
        }
        Ok(())
    }

    fn layer_receipt(
        &self,
        index: usize,
        scope: &NmtCertifiedScopeContext,
        stack_identity: &str,
    ) -> GrazingBasketLayerReceipt {
        let layer = BasketLayerIndex::new(index);
        let transform_pressure = self
            .profile
            .transform_pressure()
            .iter()
            .copied()
            .find(|pressure| pressure.layer() == layer);
        GrazingBasketLayerReceipt::new(GrazingBasketLayerReceiptInput {
            layer,
            stack_identity,
            topology_identity: scope.topology_scope().scope_identity(),
            projection_identity: scope.projection().scope_projection_identity(),
            retained_replay_identity: scope.retained_replay().scope_replay_identity(),
            transform_posture_identity: scope.motion().scope_motion_identity(),
            local_frame_identity: scope.projection().local_frame_identity(),
            radial_adjacency_identity: scope.topology_scope().radial_adjacency_identity(),
            open_boundary: BasketBoundaryScope::new(layer, 0, stack_identity),
            offset_class: self.profile.offset_classes()
                [index % self.profile.offset_classes().len()],
            transform_pressure,
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
            self.certified_scopes
                .parent_construction_identity()
                .to_string(),
            human_reason,
        )
    }
}
