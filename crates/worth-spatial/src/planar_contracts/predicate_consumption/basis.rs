use crate::planar_contracts::coplanar_overlap_contract::CoplanarOverlapContractReceipt;
use crate::planar_contracts::polygon_winding_2d::CertifiedPolygonWinding2DReceipt;
use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;
use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;
use crate::planar_contracts::signed_area_2d::CertifiedSignedArea2DReceipt;

use super::validation::validate_predicate_certificate_consumption_basis;
use super::PredicateCertificateConsumptionDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PredicateCertificateConsumerKind {
    SegmentContact,
    PolygonWinding,
    SignedArea,
    CoplanarOverlap,
}

impl PredicateCertificateConsumerKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SegmentContact => "segment-contact",
            Self::PolygonWinding => "polygon-winding",
            Self::SignedArea => "signed-area",
            Self::CoplanarOverlap => "coplanar-overlap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionRow {
    consumer_kind: PredicateCertificateConsumerKind,
    consumer_fact_digest: String,
    predicate_fact_digest: String,
    certified_sign_identity: String,
    precision_escalation_identity: String,
    local_frame_identity: String,
    topology_basis_identity: String,
    movement_rotation_posture_identity: String,
    tolerance_policy_identity: String,
    predicate_declaration_digest: String,
    predicate_envelope_digest: String,
}

impl PredicateCertificateConsumptionRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        consumer_kind: PredicateCertificateConsumerKind,
        consumer_fact_digest: impl Into<String>,
        predicate: &PlanarPredicateFactReceipt,
    ) -> Self {
        let precision = predicate.precision_escalation();
        Self {
            consumer_kind,
            consumer_fact_digest: consumer_fact_digest.into(),
            predicate_fact_digest: predicate.fact_digest().to_string(),
            certified_sign_identity: format!("{:?}", predicate.certified_sign()),
            precision_escalation_identity: format!(
                "resolved:{:?};float:{};expansion:{:?};target:{}",
                precision.get_resolved_at(),
                precision.get_float_agreed(),
                precision.get_expansion_length(),
                precision.get_target_triple()
            ),
            local_frame_identity: predicate.input_basis().local_frame_identity().to_string(),
            topology_basis_identity: predicate
                .input_basis()
                .topology_basis_identity()
                .to_string(),
            movement_rotation_posture_identity: predicate
                .input_basis()
                .movement_rotation_posture_identity()
                .to_string(),
            tolerance_policy_identity: predicate
                .input_basis()
                .tolerance_policy_identity()
                .to_string(),
            predicate_declaration_digest: predicate.declaration_digest().to_string(),
            predicate_envelope_digest: predicate.envelope_digest().to_string(),
        }
    }

    pub fn consumer_kind(&self) -> PredicateCertificateConsumerKind {
        self.consumer_kind
    }

    pub fn consumer_fact_digest(&self) -> &str {
        &self.consumer_fact_digest
    }

    pub fn predicate_fact_digest(&self) -> &str {
        &self.predicate_fact_digest
    }

    pub fn certified_sign_identity(&self) -> &str {
        &self.certified_sign_identity
    }

    pub fn precision_escalation_identity(&self) -> &str {
        &self.precision_escalation_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        &self.tolerance_policy_identity
    }

    pub fn predicate_declaration_digest(&self) -> &str {
        &self.predicate_declaration_digest
    }

    pub fn predicate_envelope_digest(&self) -> &str {
        &self.predicate_envelope_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PredicateCertificateConsumptionBasis {
    topology_basis_identity: String,
    movement_rotation_posture_identity: String,
    local_frame_identity: String,
    predicate_receipts: Vec<PlanarPredicateFactReceipt>,
    segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    winding_receipt: Option<CertifiedPolygonWinding2DReceipt>,
    signed_area_receipt: Option<CertifiedSignedArea2DReceipt>,
    overlap_receipt: Option<CoplanarOverlapContractReceipt>,
    consumption_rows: Vec<PredicateCertificateConsumptionRow>,
}

impl PredicateCertificateConsumptionBasis {
    pub fn builder() -> PredicateCertificateConsumptionBuilder {
        PredicateCertificateConsumptionBuilder::default()
    }

    pub(crate) fn from_builder(
        builder: PredicateCertificateConsumptionBuilder,
    ) -> Result<Self, PredicateCertificateConsumptionDenial> {
        let mut basis = Self {
            topology_basis_identity: builder.topology_basis_identity.unwrap_or_default(),
            movement_rotation_posture_identity: builder
                .movement_rotation_posture_identity
                .unwrap_or_default(),
            local_frame_identity: builder.local_frame_identity.unwrap_or_default(),
            predicate_receipts: builder.predicate_receipts,
            segment_receipts: builder.segment_receipts,
            winding_receipt: builder.winding_receipt,
            signed_area_receipt: builder.signed_area_receipt,
            overlap_receipt: builder.overlap_receipt,
            consumption_rows: Vec::new(),
        };
        validate_predicate_certificate_consumption_basis(&mut basis)?;
        Ok(basis)
    }

    pub(crate) fn set_consumption_rows(&mut self, rows: Vec<PredicateCertificateConsumptionRow>) {
        self.consumption_rows = rows;
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn predicate_receipts(&self) -> &[PlanarPredicateFactReceipt] {
        &self.predicate_receipts
    }

    pub fn segment_receipts(&self) -> &[CertifiedSegmentSegment2DReceipt] {
        &self.segment_receipts
    }

    pub fn winding_receipt(&self) -> Option<&CertifiedPolygonWinding2DReceipt> {
        self.winding_receipt.as_ref()
    }

    pub fn signed_area_receipt(&self) -> Option<&CertifiedSignedArea2DReceipt> {
        self.signed_area_receipt.as_ref()
    }

    pub fn overlap_receipt(&self) -> Option<&CoplanarOverlapContractReceipt> {
        self.overlap_receipt.as_ref()
    }

    pub fn consumption_rows(&self) -> &[PredicateCertificateConsumptionRow] {
        &self.consumption_rows
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PredicateCertificateConsumptionBuilder {
    topology_basis_identity: Option<String>,
    movement_rotation_posture_identity: Option<String>,
    local_frame_identity: Option<String>,
    predicate_receipts: Vec<PlanarPredicateFactReceipt>,
    segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    winding_receipt: Option<CertifiedPolygonWinding2DReceipt>,
    signed_area_receipt: Option<CertifiedSignedArea2DReceipt>,
    overlap_receipt: Option<CoplanarOverlapContractReceipt>,
}

impl PredicateCertificateConsumptionBuilder {
    pub fn expecting_topology_basis(mut self, identity: impl Into<String>) -> Self {
        self.topology_basis_identity = Some(identity.into());
        self
    }

    pub fn expecting_movement_rotation_posture(mut self, identity: impl Into<String>) -> Self {
        self.movement_rotation_posture_identity = Some(identity.into());
        self
    }

    pub fn expecting_local_frame(mut self, identity: impl Into<String>) -> Self {
        self.local_frame_identity = Some(identity.into());
        self
    }

    pub fn with_predicate_authority<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = PlanarPredicateFactReceipt>,
    {
        self.predicate_receipts = receipts.into_iter().collect();
        self
    }

    pub fn with_segment_contacts<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = CertifiedSegmentSegment2DReceipt>,
    {
        self.segment_receipts = receipts.into_iter().collect();
        self
    }

    pub fn with_polygon_winding(mut self, receipt: CertifiedPolygonWinding2DReceipt) -> Self {
        self.winding_receipt = Some(receipt);
        self
    }

    pub fn with_signed_area(mut self, receipt: CertifiedSignedArea2DReceipt) -> Self {
        self.signed_area_receipt = Some(receipt);
        self
    }

    pub fn with_coplanar_overlap(mut self, receipt: CoplanarOverlapContractReceipt) -> Self {
        self.overlap_receipt = Some(receipt);
        self
    }

    pub fn build(
        self,
    ) -> Result<PredicateCertificateConsumptionBasis, PredicateCertificateConsumptionDenial> {
        PredicateCertificateConsumptionBasis::from_builder(self)
    }
}
