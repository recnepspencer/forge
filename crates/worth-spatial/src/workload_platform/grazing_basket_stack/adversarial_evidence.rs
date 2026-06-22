use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityFactError;
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;
use crate::workload_platform::surface_support::{
    SurfaceFamily, UnsupportedSurfaceSupport, UnsupportedSurfaceSupportReasonCode,
};
use crate::workload_platform::transform_workload::{
    UnsupportedTransformReasonCode, UnsupportedTransformWorkload,
};

use super::layer_scope::BasketLayerIndex;
use super::receipt::GrazingBasketLayerReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrazingBasketLayerEvidenceKind {
    Projection,
    RetainedReplay,
    SurfaceSupport,
    ParityLane,
    OpenBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketLayerAuthorityEvidence {
    layer: BasketLayerIndex,
    kind: GrazingBasketLayerEvidenceKind,
    evidence_identity: String,
}

impl GrazingBasketLayerAuthorityEvidence {
    pub fn projection_from_layer(layer: &GrazingBasketLayerReceipt) -> Self {
        Self::from_layer(
            layer,
            GrazingBasketLayerEvidenceKind::Projection,
            layer.projection_identity(),
        )
    }

    pub fn retained_replay_from_layer(layer: &GrazingBasketLayerReceipt) -> Self {
        Self::from_layer(
            layer,
            GrazingBasketLayerEvidenceKind::RetainedReplay,
            layer.retained_replay_identity(),
        )
    }

    pub fn surface_support_from_layer(layer: &GrazingBasketLayerReceipt) -> Self {
        Self::from_layer(
            layer,
            GrazingBasketLayerEvidenceKind::SurfaceSupport,
            layer.local_frame_identity(),
        )
    }

    pub fn parity_lane_from_layer(layer: &GrazingBasketLayerReceipt) -> Self {
        Self::from_layer(
            layer,
            GrazingBasketLayerEvidenceKind::ParityLane,
            layer.projection_identity(),
        )
    }

    pub fn open_boundary_from_layer(layer: &GrazingBasketLayerReceipt) -> Self {
        Self::from_layer(
            layer,
            GrazingBasketLayerEvidenceKind::OpenBoundary,
            layer.open_boundary().boundary_identity(),
        )
    }

    fn from_layer(
        layer: &GrazingBasketLayerReceipt,
        kind: GrazingBasketLayerEvidenceKind,
        evidence_identity: &str,
    ) -> Self {
        Self {
            layer: layer.layer(),
            kind,
            evidence_identity: evidence_identity.to_string(),
        }
    }

    pub fn layer(&self) -> BasketLayerIndex {
        self.layer
    }

    pub fn kind(&self) -> GrazingBasketLayerEvidenceKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketUnsupportedSurfaceEvidence {
    family: SurfaceFamily,
    unsupported_stage_identity: String,
    human_reason: String,
}

impl GrazingBasketUnsupportedSurfaceEvidence {
    pub fn from_unsupported_surface_support(
        unsupported: &UnsupportedSurfaceSupport,
    ) -> Option<Self> {
        if unsupported.reason_code() != UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted {
            return None;
        }
        let receipt = unsupported.receipt()?;
        Some(Self {
            family: unsupported.family()?,
            unsupported_stage_identity: receipt.stage_identity().receipt_identity(),
            human_reason: unsupported.human_reason().to_string(),
        })
    }

    pub fn family(&self) -> SurfaceFamily {
        self.family
    }

    pub fn unsupported_stage_identity(&self) -> &str {
        &self.unsupported_stage_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketDeniedMotionEvidence {
    reason_code: UnsupportedTransformReasonCode,
    human_reason: String,
}

impl GrazingBasketDeniedMotionEvidence {
    pub fn from_unsupported_transform(unsupported: &UnsupportedTransformWorkload) -> Option<Self> {
        if unsupported.reason_code() != UnsupportedTransformReasonCode::LabelOnlyMotionEvidence {
            return None;
        }
        Some(Self {
            reason_code: unsupported.reason_code(),
            human_reason: unsupported.human_reason().to_string(),
        })
    }

    pub fn reason_code(&self) -> UnsupportedTransformReasonCode {
        self.reason_code
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketStormExtractionEvidence {
    projection_stage_identity: String,
}

impl GrazingBasketStormExtractionEvidence {
    pub fn from_projected_workload(projected: &ProjectedPlanarWorkload) -> Self {
        Self {
            projection_stage_identity: projected.receipts().stage_identity().receipt_identity(),
        }
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketPredicateUncertaintyEvidence {
    evidence_identity: String,
    human_reason: String,
}

impl GrazingBasketPredicateUncertaintyEvidence {
    pub fn from_predicate_error(error: &PlanarPredicateAuthorityFactError) -> Option<Self> {
        match error {
            PlanarPredicateAuthorityFactError::PredicateUncertain {
                denial,
                certified_sign,
                precision_escalation,
                counters,
            } => Some(Self {
                evidence_identity: format!(
                    "predicate={denial:?};sign={:?};resolved_at={:?};basis_parts={}",
                    certified_sign.sign(),
                    precision_escalation.get_resolved_at(),
                    counters.canonical_basis_part_count()
                ),
                human_reason: format!(
                    "predicate authority returned certified zero with {:?} precision and requires a user policy before repair",
                    precision_escalation.get_resolved_at()
                ),
            }),
            _ => None,
        }
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
