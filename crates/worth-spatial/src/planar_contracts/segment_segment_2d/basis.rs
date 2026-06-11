use worth_math::sign::TriSign;

use crate::planar_contracts::predicate_authority::{
    PlanarPredicateFactReceipt, PlanarPredicateKind,
};
use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt;

use super::classification::{classify_segment_segment_2d, CertifiedSegmentSegment2DClassification};
use super::validation::{
    validate_certified_segment_segment_2d_basis, validate_orientation_receipts,
};
use super::CertifiedSegmentSegment2DDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSegmentSegment2DBasis {
    first_segment_identity: String,
    second_segment_identity: String,
    topology_basis_identity: String,
    contact_policy_identity: String,
    first_start: ProjectedEndpointSnapshot,
    first_end: ProjectedEndpointSnapshot,
    second_start: ProjectedEndpointSnapshot,
    second_end: ProjectedEndpointSnapshot,
    orientations: Vec<OrientationReceiptSnapshot>,
    classification: CertifiedSegmentSegment2DClassification,
}

impl CertifiedSegmentSegment2DBasis {
    pub fn builder() -> CertifiedSegmentSegment2DBasisBuilder {
        CertifiedSegmentSegment2DBasisBuilder::default()
    }

    pub(crate) fn from_builder(
        builder: CertifiedSegmentSegment2DBasisBuilder,
    ) -> Result<Self, CertifiedSegmentSegment2DDenial> {
        let basis = Self {
            first_segment_identity: builder.first_segment_identity.unwrap_or_default(),
            second_segment_identity: builder.second_segment_identity.unwrap_or_default(),
            topology_basis_identity: builder.topology_basis_identity.unwrap_or_default(),
            contact_policy_identity: builder.contact_policy_identity.unwrap_or_default(),
            first_start: builder
                .first_start
                .unwrap_or_else(ProjectedEndpointSnapshot::missing),
            first_end: builder
                .first_end
                .unwrap_or_else(ProjectedEndpointSnapshot::missing),
            second_start: builder
                .second_start
                .unwrap_or_else(ProjectedEndpointSnapshot::missing),
            second_end: builder
                .second_end
                .unwrap_or_else(ProjectedEndpointSnapshot::missing),
            orientations: Vec::new(),
            classification: CertifiedSegmentSegment2DClassification::PolicyRequiredOrUncertain,
        };
        validate_certified_segment_segment_2d_basis(&basis)?;
        Ok(basis)
    }

    pub fn first_segment_identity(&self) -> &str {
        &self.first_segment_identity
    }

    pub fn second_segment_identity(&self) -> &str {
        &self.second_segment_identity
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn contact_policy_identity(&self) -> &str {
        &self.contact_policy_identity
    }

    pub fn classification(&self) -> CertifiedSegmentSegment2DClassification {
        self.classification
    }

    pub fn first_start_point_2d(&self) -> [f64; 2] {
        self.first_start.point_2d
    }

    pub fn first_end_point_2d(&self) -> [f64; 2] {
        self.first_end.point_2d
    }

    pub fn second_start_point_2d(&self) -> [f64; 2] {
        self.second_start.point_2d
    }

    pub fn second_end_point_2d(&self) -> [f64; 2] {
        self.second_end.point_2d
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        &self.first_start.local_frame_fact_digest
    }

    pub fn local_frame_declaration_digest(&self) -> &str {
        &self.first_start.local_frame_declaration_digest
    }

    pub fn local_frame_envelope_digest(&self) -> &str {
        &self.first_start.local_frame_envelope_digest
    }

    pub fn frame_identity(&self) -> &str {
        &self.first_start.frame_identity
    }

    pub fn transform_chain_digest(&self) -> &str {
        &self.first_start.transform_chain_digest
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.first_start.movement_rotation_posture_identity
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        &self.first_start.tolerance_policy_identity
    }

    pub fn endpoint_source_identities(&self) -> [&str; 4] {
        [
            &self.first_start.source_point_identity,
            &self.first_end.source_point_identity,
            &self.second_start.source_point_identity,
            &self.second_end.source_point_identity,
        ]
    }

    pub fn endpoint_projection_fact_digests(&self) -> [&str; 4] {
        [
            &self.first_start.projection_fact_digest,
            &self.first_end.projection_fact_digest,
            &self.second_start.projection_fact_digest,
            &self.second_end.projection_fact_digest,
        ]
    }

    pub(crate) fn orientation_fact_digests(&self) -> [&str; 4] {
        [
            &self.orientations[0].fact_digest,
            &self.orientations[1].fact_digest,
            &self.orientations[2].fact_digest,
            &self.orientations[3].fact_digest,
        ]
    }

    pub(crate) fn orientation_signs(&self) -> [TriSign; 4] {
        [
            self.orientations[0].sign,
            self.orientations[1].sign,
            self.orientations[2].sign,
            self.orientations[3].sign,
        ]
    }

    pub(crate) fn endpoints(&self) -> [&ProjectedEndpointSnapshot; 4] {
        [
            &self.first_start,
            &self.first_end,
            &self.second_start,
            &self.second_end,
        ]
    }

    pub(crate) fn orientations(&self) -> &[OrientationReceiptSnapshot] {
        &self.orientations
    }

    pub(crate) fn expected_orientation_points(&self) -> [[[f64; 2]; 3]; 4] {
        [
            [
                self.first_start_point_2d(),
                self.first_end_point_2d(),
                self.second_start_point_2d(),
            ],
            [
                self.first_start_point_2d(),
                self.first_end_point_2d(),
                self.second_end_point_2d(),
            ],
            [
                self.second_start_point_2d(),
                self.second_end_point_2d(),
                self.first_start_point_2d(),
            ],
            [
                self.second_start_point_2d(),
                self.second_end_point_2d(),
                self.first_end_point_2d(),
            ],
        ]
    }

    pub(crate) fn with_orientation_receipts(
        mut self,
        receipts: [&PlanarPredicateFactReceipt; 4],
    ) -> Result<Self, CertifiedSegmentSegment2DDenial> {
        self.orientations = receipts
            .into_iter()
            .map(OrientationReceiptSnapshot::from_receipt)
            .collect();
        validate_certified_segment_segment_2d_basis(&self)?;
        validate_orientation_receipts(&self)?;
        self.classification = classify_segment_segment_2d(&self);
        Ok(self)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CertifiedSegmentSegment2DBasisBuilder {
    first_segment_identity: Option<String>,
    second_segment_identity: Option<String>,
    topology_basis_identity: Option<String>,
    contact_policy_identity: Option<String>,
    first_start: Option<ProjectedEndpointSnapshot>,
    first_end: Option<ProjectedEndpointSnapshot>,
    second_start: Option<ProjectedEndpointSnapshot>,
    second_end: Option<ProjectedEndpointSnapshot>,
}

impl CertifiedSegmentSegment2DBasisBuilder {
    pub fn first_segment_identity(mut self, identity: impl Into<String>) -> Self {
        self.first_segment_identity = Some(identity.into());
        self
    }

    pub fn second_segment_identity(mut self, identity: impl Into<String>) -> Self {
        self.second_segment_identity = Some(identity.into());
        self
    }

    pub fn topology_basis_identity(mut self, identity: impl Into<String>) -> Self {
        self.topology_basis_identity = Some(identity.into());
        self
    }

    pub fn contact_policy_identity(mut self, identity: impl Into<String>) -> Self {
        self.contact_policy_identity = Some(identity.into());
        self
    }

    pub fn first_segment_endpoints(
        mut self,
        start: &ProjectPointToCertifiedPlane2DReceipt,
        end: &ProjectPointToCertifiedPlane2DReceipt,
    ) -> Self {
        self.first_start = Some(ProjectedEndpointSnapshot::from_receipt(start));
        self.first_end = Some(ProjectedEndpointSnapshot::from_receipt(end));
        self
    }

    pub fn second_segment_endpoints(
        mut self,
        start: &ProjectPointToCertifiedPlane2DReceipt,
        end: &ProjectPointToCertifiedPlane2DReceipt,
    ) -> Self {
        self.second_start = Some(ProjectedEndpointSnapshot::from_receipt(start));
        self.second_end = Some(ProjectedEndpointSnapshot::from_receipt(end));
        self
    }

    pub fn build(self) -> Result<CertifiedSegmentSegment2DBasis, CertifiedSegmentSegment2DDenial> {
        CertifiedSegmentSegment2DBasis::from_builder(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProjectedEndpointSnapshot {
    pub(crate) source_point_identity: String,
    pub(crate) point_2d: [f64; 2],
    pub(crate) projection_declaration_digest: String,
    pub(crate) projection_envelope_digest: String,
    pub(crate) projection_fact_digest: String,
    pub(crate) local_frame_fact_digest: String,
    pub(crate) local_frame_declaration_digest: String,
    pub(crate) local_frame_envelope_digest: String,
    pub(crate) frame_identity: String,
    pub(crate) transform_chain_digest: String,
    pub(crate) movement_rotation_posture_identity: String,
    pub(crate) tolerance_policy_identity: String,
}

impl ProjectedEndpointSnapshot {
    fn from_receipt(receipt: &ProjectPointToCertifiedPlane2DReceipt) -> Self {
        Self {
            source_point_identity: receipt.source_point_identity().to_string(),
            point_2d: receipt.point_2d(),
            projection_declaration_digest: receipt.declaration_digest().to_string(),
            projection_envelope_digest: receipt.envelope_digest().to_string(),
            projection_fact_digest: receipt.fact_digest().to_string(),
            local_frame_fact_digest: receipt.local_frame_fact_digest().to_string(),
            local_frame_declaration_digest: receipt
                .basis()
                .local_frame_declaration_digest()
                .to_string(),
            local_frame_envelope_digest: receipt.basis().local_frame_envelope_digest().to_string(),
            frame_identity: receipt.basis().frame_identity().to_string(),
            transform_chain_digest: receipt.basis().transform_chain_digest().to_string(),
            movement_rotation_posture_identity: receipt
                .basis()
                .movement_rotation_posture_identity()
                .to_string(),
            tolerance_policy_identity: receipt.basis().tolerance_policy_identity().to_string(),
        }
    }

    fn missing() -> Self {
        Self {
            source_point_identity: String::new(),
            point_2d: [f64::NAN; 2],
            projection_declaration_digest: String::new(),
            projection_envelope_digest: String::new(),
            projection_fact_digest: String::new(),
            local_frame_fact_digest: String::new(),
            local_frame_declaration_digest: String::new(),
            local_frame_envelope_digest: String::new(),
            frame_identity: String::new(),
            transform_chain_digest: String::new(),
            movement_rotation_posture_identity: String::new(),
            tolerance_policy_identity: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OrientationReceiptSnapshot {
    pub(crate) predicate_kind: PlanarPredicateKind,
    pub(crate) projected_points: [[f64; 2]; 3],
    pub(crate) local_frame_identity: String,
    pub(crate) topology_basis_identity: String,
    pub(crate) movement_rotation_posture_identity: String,
    pub(crate) tolerance_policy_identity: String,
    pub(crate) sign: TriSign,
    pub(crate) precision_escalation: String,
    pub(crate) declaration_digest: String,
    pub(crate) envelope_digest: String,
    pub(crate) fact_digest: String,
}

impl OrientationReceiptSnapshot {
    pub(crate) fn from_receipt(receipt: &PlanarPredicateFactReceipt) -> Self {
        Self {
            predicate_kind: receipt.predicate_kind(),
            projected_points: receipt.input_basis().projected_points(),
            local_frame_identity: receipt.input_basis().local_frame_identity().to_string(),
            topology_basis_identity: receipt.input_basis().topology_basis_identity().to_string(),
            movement_rotation_posture_identity: receipt
                .input_basis()
                .movement_rotation_posture_identity()
                .to_string(),
            tolerance_policy_identity: receipt
                .input_basis()
                .tolerance_policy_identity()
                .to_string(),
            sign: receipt.certified_sign().sign(),
            precision_escalation: format!("{:?}", receipt.precision_escalation()),
            declaration_digest: receipt.declaration_digest().to_string(),
            envelope_digest: receipt.envelope_digest().to_string(),
            fact_digest: receipt.fact_digest().to_string(),
        }
    }
}
