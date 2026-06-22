use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;

use super::validation::validate_planar_local_frame_basis;
use super::{derive_planar_local_frame_axes, PlanarLocalFrameDenial};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalFrameBasis {
    frame_identity: String,
    origin: [f64; 3],
    normal: [f64; 3],
    u_axis: [f64; 3],
    v_axis: [f64; 3],
    w_axis: [f64; 3],
    local_feature_scale_order: i32,
    world_magnitude_order: i32,
    normalization_scale: f64,
    transform_chain_digest: String,
    movement_rotation_posture_identity: String,
    tolerance_policy_identity: String,
    precision_fact_digest: String,
    precision_declaration_digest: String,
    precision_envelope_digest: String,
}

impl PlanarLocalFrameBasis {
    pub fn builder() -> PlanarLocalFrameBasisBuilder {
        PlanarLocalFrameBasisBuilder::default()
    }

    pub fn frame_identity(&self) -> &str {
        &self.frame_identity
    }

    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub fn normal(&self) -> [f64; 3] {
        self.normal
    }

    pub fn u_axis(&self) -> [f64; 3] {
        self.u_axis
    }

    pub fn v_axis(&self) -> [f64; 3] {
        self.v_axis
    }

    pub fn w_axis(&self) -> [f64; 3] {
        self.w_axis
    }

    pub fn local_feature_scale_order(&self) -> i32 {
        self.local_feature_scale_order
    }

    pub fn world_magnitude_order(&self) -> i32 {
        self.world_magnitude_order
    }

    pub fn normalization_scale(&self) -> f64 {
        self.normalization_scale
    }

    pub fn scale_separation_orders(&self) -> i32 {
        self.world_magnitude_order - self.local_feature_scale_order
    }

    pub fn transform_chain_digest(&self) -> &str {
        &self.transform_chain_digest
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        &self.tolerance_policy_identity
    }

    pub fn precision_fact_digest(&self) -> &str {
        &self.precision_fact_digest
    }

    pub fn precision_declaration_digest(&self) -> &str {
        &self.precision_declaration_digest
    }

    pub fn precision_envelope_digest(&self) -> &str {
        &self.precision_envelope_digest
    }

    pub(crate) fn from_builder(
        builder: PlanarLocalFrameBasisBuilder,
    ) -> Result<Self, PlanarLocalFrameDenial> {
        let local_feature_scale_order = builder.local_feature_scale_order.ok_or_else(|| {
            PlanarLocalFrameDenial::new(
                super::PlanarLocalFrameDenialKind::MissingLocalFeatureScaleOrder,
                "local feature scale order is required",
            )
        })?;
        let world_magnitude_order = builder.world_magnitude_order.ok_or_else(|| {
            PlanarLocalFrameDenial::new(
                super::PlanarLocalFrameDenialKind::MissingWorldMagnitudeOrder,
                "world magnitude order is required",
            )
        })?;
        let mut basis = Self {
            frame_identity: builder.frame_identity.unwrap_or_default(),
            origin: builder.origin.unwrap_or([f64::NAN; 3]),
            normal: builder.normal.unwrap_or([f64::NAN; 3]),
            u_axis: [f64::NAN; 3],
            v_axis: [f64::NAN; 3],
            w_axis: [f64::NAN; 3],
            local_feature_scale_order,
            world_magnitude_order,
            normalization_scale: builder.normalization_scale.unwrap_or(f64::NAN),
            transform_chain_digest: builder.transform_chain_digest.unwrap_or_default(),
            movement_rotation_posture_identity: builder
                .movement_rotation_posture_identity
                .unwrap_or_default(),
            tolerance_policy_identity: builder.tolerance_policy_identity.unwrap_or_default(),
            precision_fact_digest: builder.precision_fact_digest.unwrap_or_default(),
            precision_declaration_digest: builder.precision_declaration_digest.unwrap_or_default(),
            precision_envelope_digest: builder.precision_envelope_digest.unwrap_or_default(),
        };
        validate_planar_local_frame_basis(&basis, builder.precision_basis)?;
        let axes = derive_planar_local_frame_axes(&basis)?;
        basis.u_axis = axes.u_axis;
        basis.v_axis = axes.v_axis;
        basis.w_axis = axes.w_axis;
        Ok(basis)
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlanarLocalFrameBasisBuilder {
    frame_identity: Option<String>,
    origin: Option<[f64; 3]>,
    normal: Option<[f64; 3]>,
    local_feature_scale_order: Option<i32>,
    world_magnitude_order: Option<i32>,
    normalization_scale: Option<f64>,
    transform_chain_digest: Option<String>,
    movement_rotation_posture_identity: Option<String>,
    tolerance_policy_identity: Option<String>,
    precision_fact_digest: Option<String>,
    precision_declaration_digest: Option<String>,
    precision_envelope_digest: Option<String>,
    precision_basis: Option<PrecisionBasisSnapshot>,
}

#[derive(Clone, Debug)]
pub(crate) struct PrecisionBasisSnapshot {
    pub(crate) local_frame_identity: String,
    pub(crate) movement_rotation_posture_identity: String,
    pub(crate) tolerance_policy_identity: String,
    pub(crate) local_feature_scale_order: i32,
    pub(crate) world_magnitude_order: i32,
    pub(crate) normalization_scale: f64,
}

impl PlanarLocalFrameBasisBuilder {
    pub fn frame_identity(mut self, identity: impl Into<String>) -> Self {
        self.frame_identity = Some(identity.into());
        self
    }

    pub fn origin(mut self, origin: [f64; 3]) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn normal(mut self, normal: [f64; 3]) -> Self {
        self.normal = Some(normal);
        self
    }

    pub fn local_feature_scale_order(mut self, order: i32) -> Self {
        self.local_feature_scale_order = Some(order);
        self
    }

    pub fn world_magnitude_order(mut self, order: i32) -> Self {
        self.world_magnitude_order = Some(order);
        self
    }

    pub fn normalization_scale(mut self, scale: f64) -> Self {
        self.normalization_scale = Some(scale);
        self
    }

    pub fn transform_chain_digest(mut self, digest: impl Into<String>) -> Self {
        self.transform_chain_digest = Some(digest.into());
        self
    }

    pub fn movement_rotation_posture_identity(mut self, identity: impl Into<String>) -> Self {
        self.movement_rotation_posture_identity = Some(identity.into());
        self
    }

    pub fn tolerance_policy_identity(mut self, identity: impl Into<String>) -> Self {
        self.tolerance_policy_identity = Some(identity.into());
        self
    }

    pub fn precision_receipt(mut self, receipt: &PlanarPrecisionCertificateReceipt) -> Self {
        self.precision_fact_digest = Some(receipt.fact_digest().to_string());
        self.precision_declaration_digest = Some(receipt.declaration_digest().to_string());
        self.precision_envelope_digest = Some(receipt.envelope_digest().to_string());
        self.precision_basis = Some(PrecisionBasisSnapshot {
            local_frame_identity: receipt.basis().local_frame_identity().to_string(),
            movement_rotation_posture_identity: receipt
                .basis()
                .movement_rotation_posture_identity()
                .to_string(),
            tolerance_policy_identity: receipt.basis().tolerance_policy_identity().to_string(),
            local_feature_scale_order: receipt.basis().local_feature_scale_order(),
            world_magnitude_order: receipt.basis().world_magnitude_order(),
            normalization_scale: receipt.basis().normalization_scale(),
        });
        self
    }

    pub fn build(self) -> Result<PlanarLocalFrameBasis, PlanarLocalFrameDenial> {
        PlanarLocalFrameBasis::from_builder(self)
    }
}
