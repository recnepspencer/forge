use crate::planar_contracts::local_frame::PlanarLocalFrameCertificateReceipt;

use super::projection_math::project_point_to_certified_plane_2d;
use super::validation::validate_project_point_to_certified_plane_2d_basis;
use super::ProjectPointToCertifiedPlane2DDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DBasis {
    source_point_identity: String,
    source_point: [f64; 3],
    source_point_basis_digest: String,
    local_delta_from_frame_origin: [f64; 3],
    frame_identity: String,
    frame_origin: [f64; 3],
    u_axis: [f64; 3],
    v_axis: [f64; 3],
    w_axis: [f64; 3],
    transform_chain_digest: String,
    movement_rotation_posture_identity: String,
    tolerance_policy_identity: String,
    local_frame_fact_digest: String,
    local_frame_declaration_digest: String,
    local_frame_envelope_digest: String,
    point_2d: [f64; 2],
    signed_distance_to_plane_bits: u64,
    local_frame_snapshot: LocalFrameReceiptSnapshot,
}

impl ProjectPointToCertifiedPlane2DBasis {
    pub fn builder() -> ProjectPointToCertifiedPlane2DBasisBuilder {
        ProjectPointToCertifiedPlane2DBasisBuilder::default()
    }

    pub fn source_point_identity(&self) -> &str {
        &self.source_point_identity
    }

    pub fn source_point(&self) -> [f64; 3] {
        self.source_point
    }

    pub fn source_point_basis_digest(&self) -> &str {
        &self.source_point_basis_digest
    }

    pub fn local_delta_from_frame_origin(&self) -> [f64; 3] {
        self.local_delta_from_frame_origin
    }

    pub fn frame_identity(&self) -> &str {
        &self.frame_identity
    }

    pub fn frame_origin(&self) -> [f64; 3] {
        self.frame_origin
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

    pub fn transform_chain_digest(&self) -> &str {
        &self.transform_chain_digest
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        &self.tolerance_policy_identity
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        &self.local_frame_fact_digest
    }

    pub fn local_frame_declaration_digest(&self) -> &str {
        &self.local_frame_declaration_digest
    }

    pub fn local_frame_envelope_digest(&self) -> &str {
        &self.local_frame_envelope_digest
    }

    pub fn point_2d(&self) -> [f64; 2] {
        self.point_2d
    }

    pub fn signed_distance_to_plane_bits(&self) -> u64 {
        self.signed_distance_to_plane_bits
    }

    pub(crate) fn local_frame_snapshot(&self) -> &LocalFrameReceiptSnapshot {
        &self.local_frame_snapshot
    }

    pub(crate) fn from_builder(
        builder: ProjectPointToCertifiedPlane2DBasisBuilder,
    ) -> Result<Self, ProjectPointToCertifiedPlane2DDenial> {
        let snapshot = builder
            .local_frame_snapshot
            .unwrap_or_else(LocalFrameReceiptSnapshot::missing);
        let mut basis = Self {
            source_point_identity: builder.source_point_identity.unwrap_or_default(),
            source_point: builder.source_point.unwrap_or([f64::NAN; 3]),
            source_point_basis_digest: builder.source_point_basis_digest.unwrap_or_default(),
            local_delta_from_frame_origin: builder
                .local_delta_from_frame_origin
                .unwrap_or([f64::NAN; 3]),
            frame_identity: builder.frame_identity.unwrap_or_default(),
            frame_origin: snapshot.frame_origin,
            u_axis: snapshot.u_axis,
            v_axis: snapshot.v_axis,
            w_axis: snapshot.w_axis,
            transform_chain_digest: builder.transform_chain_digest.unwrap_or_default(),
            movement_rotation_posture_identity: builder
                .movement_rotation_posture_identity
                .unwrap_or_default(),
            tolerance_policy_identity: builder.tolerance_policy_identity.unwrap_or_default(),
            local_frame_fact_digest: builder.local_frame_fact_digest.unwrap_or_default(),
            local_frame_declaration_digest: builder
                .local_frame_declaration_digest
                .unwrap_or_default(),
            local_frame_envelope_digest: builder.local_frame_envelope_digest.unwrap_or_default(),
            point_2d: [f64::NAN; 2],
            signed_distance_to_plane_bits: f64::NAN.to_bits(),
            local_frame_snapshot: snapshot,
        };
        validate_project_point_to_certified_plane_2d_basis(&basis)?;
        let projection = project_point_to_certified_plane_2d(&basis)?;
        basis.point_2d = projection.point_2d;
        basis.signed_distance_to_plane_bits = projection.signed_distance_to_plane.to_bits();
        Ok(basis)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProjectPointToCertifiedPlane2DBasisBuilder {
    source_point_identity: Option<String>,
    source_point: Option<[f64; 3]>,
    source_point_basis_digest: Option<String>,
    local_delta_from_frame_origin: Option<[f64; 3]>,
    frame_identity: Option<String>,
    transform_chain_digest: Option<String>,
    movement_rotation_posture_identity: Option<String>,
    tolerance_policy_identity: Option<String>,
    local_frame_fact_digest: Option<String>,
    local_frame_declaration_digest: Option<String>,
    local_frame_envelope_digest: Option<String>,
    local_frame_snapshot: Option<LocalFrameReceiptSnapshot>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LocalFrameReceiptSnapshot {
    pub(crate) frame_identity: String,
    pub(crate) frame_origin: [f64; 3],
    pub(crate) u_axis: [f64; 3],
    pub(crate) v_axis: [f64; 3],
    pub(crate) w_axis: [f64; 3],
    pub(crate) transform_chain_digest: String,
    pub(crate) movement_rotation_posture_identity: String,
    pub(crate) tolerance_policy_identity: String,
}

impl LocalFrameReceiptSnapshot {
    fn missing() -> Self {
        Self {
            frame_identity: String::new(),
            frame_origin: [f64::NAN; 3],
            u_axis: [f64::NAN; 3],
            v_axis: [f64::NAN; 3],
            w_axis: [f64::NAN; 3],
            transform_chain_digest: String::new(),
            movement_rotation_posture_identity: String::new(),
            tolerance_policy_identity: String::new(),
        }
    }
}

impl ProjectPointToCertifiedPlane2DBasisBuilder {
    pub fn source_point_identity(mut self, identity: impl Into<String>) -> Self {
        self.source_point_identity = Some(identity.into());
        self
    }

    pub fn source_point(mut self, point: [f64; 3]) -> Self {
        self.source_point = Some(point);
        self
    }

    pub fn source_point_basis_digest(mut self, digest: impl Into<String>) -> Self {
        self.source_point_basis_digest = Some(digest.into());
        self
    }

    pub fn local_delta_from_frame_origin(mut self, delta: [f64; 3]) -> Self {
        self.local_delta_from_frame_origin = Some(delta);
        self
    }

    pub fn frame_identity(mut self, identity: impl Into<String>) -> Self {
        self.frame_identity = Some(identity.into());
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

    pub fn local_frame_receipt(mut self, receipt: &PlanarLocalFrameCertificateReceipt) -> Self {
        self.frame_identity = Some(receipt.frame_identity().to_string());
        self.transform_chain_digest = Some(receipt.basis().transform_chain_digest().to_string());
        self.movement_rotation_posture_identity = Some(
            receipt
                .basis()
                .movement_rotation_posture_identity()
                .to_string(),
        );
        self.tolerance_policy_identity =
            Some(receipt.basis().tolerance_policy_identity().to_string());
        self.local_frame_fact_digest = Some(receipt.fact_digest().to_string());
        self.local_frame_declaration_digest = Some(receipt.declaration_digest().to_string());
        self.local_frame_envelope_digest = Some(receipt.envelope_digest().to_string());
        self.local_frame_snapshot = Some(LocalFrameReceiptSnapshot {
            frame_identity: receipt.frame_identity().to_string(),
            frame_origin: receipt.basis().origin(),
            u_axis: receipt.basis().u_axis(),
            v_axis: receipt.basis().v_axis(),
            w_axis: receipt.basis().w_axis(),
            transform_chain_digest: receipt.basis().transform_chain_digest().to_string(),
            movement_rotation_posture_identity: receipt
                .basis()
                .movement_rotation_posture_identity()
                .to_string(),
            tolerance_policy_identity: receipt.basis().tolerance_policy_identity().to_string(),
        });
        self
    }

    pub fn build(
        self,
    ) -> Result<ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DDenial> {
        ProjectPointToCertifiedPlane2DBasis::from_builder(self)
    }
}
