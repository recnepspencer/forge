use crate::planar_contracts::admission::{
    PlanarAdmissionClass, PlanarAdmissionFamily, PlanarAdmissionReceipt, PlanarRuntimeConcern,
};
use crate::planar_contracts::coplanar_overlap_contract::CoplanarOverlapContractReceipt;
use crate::planar_contracts::local_frame::PlanarLocalFrameCertificateReceipt;
use crate::planar_contracts::polygon_winding_2d::CertifiedPolygonWinding2DReceipt;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;
use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;
use crate::planar_contracts::predicate_consumption::PredicateCertificateConsumptionReceipt;
use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt;
use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;
use crate::planar_contracts::signed_area_2d::CertifiedSignedArea2DReceipt;
use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessReceipt;

use super::family_rows::{
    build_family_rows, PlanarContractBundleFamily, PlanarContractBundleFamilyRow,
};
use super::validation::validate_planar_contract_bundle_basis;
use super::PlanarContractBundleDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarContractBundlePolicy {
    CertifyBooleanReadinessOnly,
}

impl PlanarContractBundlePolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifyBooleanReadinessOnly => "certify-boolean-readiness-only",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarContractBundleValidationBasis {
    admission_receipt: PlanarAdmissionReceipt,
    topology_contract_receipt: PlanarTopologyContractCompletenessReceipt,
    precision_receipt: PlanarPrecisionCertificateReceipt,
    local_frame_receipt: PlanarLocalFrameCertificateReceipt,
    projection_receipts: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    predicate_receipts: Vec<PlanarPredicateFactReceipt>,
    segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    winding_receipt: CertifiedPolygonWinding2DReceipt,
    signed_area_receipt: CertifiedSignedArea2DReceipt,
    overlap_receipt: CoplanarOverlapContractReceipt,
    predicate_consumption_receipt: PredicateCertificateConsumptionReceipt,
    topology_basis_identity: String,
    movement_rotation_posture_identity: String,
    diagnostic_scope_identity: String,
    policy: PlanarContractBundlePolicy,
    family_rows: Vec<PlanarContractBundleFamilyRow>,
}

impl PlanarContractBundleValidationBasis {
    pub fn builder() -> PlanarBooleanReadinessBundleBuilder {
        PlanarBooleanReadinessBundleBuilder::default()
    }

    pub(crate) fn from_builder(
        builder: PlanarBooleanReadinessBundleBuilder,
    ) -> Result<Self, PlanarContractBundleDenial> {
        let mut basis = Self {
            admission_receipt: builder.admission_receipt.ok_or_else(|| {
                missing_family(PlanarContractBundleFamily::Admission, "admission receipt")
            })?,
            topology_contract_receipt: builder.topology_contract_receipt.ok_or_else(|| {
                missing_family(
                    PlanarContractBundleFamily::TopologyContractCompleteness,
                    "topology completeness receipt",
                )
            })?,
            precision_receipt: builder.precision_receipt.ok_or_else(|| {
                missing_family(PlanarContractBundleFamily::Precision, "precision receipt")
            })?,
            local_frame_receipt: builder.local_frame_receipt.ok_or_else(|| {
                missing_family(
                    PlanarContractBundleFamily::LocalFrame,
                    "local-frame receipt",
                )
            })?,
            projection_receipts: builder.projection_receipts,
            predicate_receipts: builder.predicate_receipts,
            segment_receipts: builder.segment_receipts,
            winding_receipt: builder.winding_receipt.ok_or_else(|| {
                missing_family(
                    PlanarContractBundleFamily::PolygonWinding,
                    "winding receipt",
                )
            })?,
            signed_area_receipt: builder.signed_area_receipt.ok_or_else(|| {
                missing_family(
                    PlanarContractBundleFamily::SignedArea,
                    "signed-area receipt",
                )
            })?,
            overlap_receipt: builder.overlap_receipt.ok_or_else(|| {
                missing_family(
                    PlanarContractBundleFamily::CoplanarOverlap,
                    "overlap receipt",
                )
            })?,
            predicate_consumption_receipt: builder.predicate_consumption_receipt.ok_or_else(
                || {
                    missing_family(
                        PlanarContractBundleFamily::PredicateCertificateConsumption,
                        "predicate-consumption receipt",
                    )
                },
            )?,
            topology_basis_identity: builder.topology_basis_identity.unwrap_or_default(),
            movement_rotation_posture_identity: builder
                .movement_rotation_posture_identity
                .unwrap_or_default(),
            diagnostic_scope_identity: builder.diagnostic_scope_identity.unwrap_or_default(),
            policy: PlanarContractBundlePolicy::CertifyBooleanReadinessOnly,
            family_rows: Vec::new(),
        };
        validate_planar_contract_bundle_basis(&basis)?;
        basis.family_rows = build_family_rows(&basis);
        Ok(basis)
    }

    pub fn admission_receipt(&self) -> &PlanarAdmissionReceipt {
        &self.admission_receipt
    }

    pub fn topology_contract_receipt(&self) -> &PlanarTopologyContractCompletenessReceipt {
        &self.topology_contract_receipt
    }

    pub fn precision_receipt(&self) -> &PlanarPrecisionCertificateReceipt {
        &self.precision_receipt
    }

    pub fn local_frame_receipt(&self) -> &PlanarLocalFrameCertificateReceipt {
        &self.local_frame_receipt
    }

    pub fn projection_receipts(&self) -> &[ProjectPointToCertifiedPlane2DReceipt] {
        &self.projection_receipts
    }

    pub fn predicate_receipts(&self) -> &[PlanarPredicateFactReceipt] {
        &self.predicate_receipts
    }

    pub fn segment_receipts(&self) -> &[CertifiedSegmentSegment2DReceipt] {
        &self.segment_receipts
    }

    pub fn winding_receipt(&self) -> &CertifiedPolygonWinding2DReceipt {
        &self.winding_receipt
    }

    pub fn signed_area_receipt(&self) -> &CertifiedSignedArea2DReceipt {
        &self.signed_area_receipt
    }

    pub fn overlap_receipt(&self) -> &CoplanarOverlapContractReceipt {
        &self.overlap_receipt
    }

    pub fn predicate_consumption_receipt(&self) -> &PredicateCertificateConsumptionReceipt {
        &self.predicate_consumption_receipt
    }

    pub fn topology_basis_identity(&self) -> &str {
        &self.topology_basis_identity
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self.movement_rotation_posture_identity
    }

    pub fn diagnostic_scope_identity(&self) -> &str {
        &self.diagnostic_scope_identity
    }

    pub fn policy(&self) -> PlanarContractBundlePolicy {
        self.policy
    }

    pub fn family_rows(&self) -> &[PlanarContractBundleFamilyRow] {
        &self.family_rows
    }
}

pub type PlanarBooleanReadinessBundle = PlanarContractBundleValidationBasis;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlanarBooleanReadinessBundleBuilder {
    admission_receipt: Option<PlanarAdmissionReceipt>,
    topology_contract_receipt: Option<PlanarTopologyContractCompletenessReceipt>,
    precision_receipt: Option<PlanarPrecisionCertificateReceipt>,
    local_frame_receipt: Option<PlanarLocalFrameCertificateReceipt>,
    projection_receipts: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    predicate_receipts: Vec<PlanarPredicateFactReceipt>,
    segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    winding_receipt: Option<CertifiedPolygonWinding2DReceipt>,
    signed_area_receipt: Option<CertifiedSignedArea2DReceipt>,
    overlap_receipt: Option<CoplanarOverlapContractReceipt>,
    predicate_consumption_receipt: Option<PredicateCertificateConsumptionReceipt>,
    topology_basis_identity: Option<String>,
    movement_rotation_posture_identity: Option<String>,
    diagnostic_scope_identity: Option<String>,
}

impl PlanarBooleanReadinessBundleBuilder {
    pub fn admission(mut self, receipt: PlanarAdmissionReceipt) -> Self {
        self.admission_receipt = Some(receipt);
        self
    }

    pub fn topology_contract(mut self, receipt: PlanarTopologyContractCompletenessReceipt) -> Self {
        self.topology_contract_receipt = Some(receipt);
        self
    }

    pub fn precision(mut self, receipt: PlanarPrecisionCertificateReceipt) -> Self {
        self.precision_receipt = Some(receipt);
        self
    }

    pub fn local_frame(mut self, receipt: PlanarLocalFrameCertificateReceipt) -> Self {
        self.local_frame_receipt = Some(receipt);
        self
    }

    pub fn projection_consumption<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = ProjectPointToCertifiedPlane2DReceipt>,
    {
        self.projection_receipts = receipts.into_iter().collect();
        self
    }

    pub fn predicate_authority<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = PlanarPredicateFactReceipt>,
    {
        self.predicate_receipts = receipts.into_iter().collect();
        self
    }

    pub fn segment_contacts<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = CertifiedSegmentSegment2DReceipt>,
    {
        self.segment_receipts = receipts.into_iter().collect();
        self
    }

    pub fn winding(mut self, receipt: CertifiedPolygonWinding2DReceipt) -> Self {
        self.winding_receipt = Some(receipt);
        self
    }

    pub fn signed_area(mut self, receipt: CertifiedSignedArea2DReceipt) -> Self {
        self.signed_area_receipt = Some(receipt);
        self
    }

    pub fn coplanar_overlap(mut self, receipt: CoplanarOverlapContractReceipt) -> Self {
        self.overlap_receipt = Some(receipt);
        self
    }

    pub fn predicate_consumption(
        mut self,
        receipt: PredicateCertificateConsumptionReceipt,
    ) -> Self {
        self.predicate_consumption_receipt = Some(receipt);
        self
    }

    pub fn topology_basis(mut self, identity: impl Into<String>) -> Self {
        self.topology_basis_identity = Some(identity.into());
        self
    }

    pub fn movement_rotation_posture(mut self, identity: impl Into<String>) -> Self {
        self.movement_rotation_posture_identity = Some(identity.into());
        self
    }

    pub fn diagnostic_scope(mut self, identity: impl Into<String>) -> Self {
        self.diagnostic_scope_identity = Some(identity.into());
        self
    }

    pub fn build(self) -> Result<PlanarBooleanReadinessBundle, PlanarContractBundleDenial> {
        PlanarContractBundleValidationBasis::from_builder(self)
    }
}

fn missing_family(
    family: PlanarContractBundleFamily,
    label: &'static str,
) -> PlanarContractBundleDenial {
    PlanarContractBundleDenial::new(
        super::PlanarContractBundleDenialKind::MissingCertificateFamily,
        Some(family),
        format!("planar boolean-readiness bundle requires {label}"),
    )
}

pub(crate) fn admission_receipt_is_boolean_readiness(receipt: &PlanarAdmissionReceipt) -> bool {
    receipt.family() == PlanarAdmissionFamily::PlanarContractBundle
        && receipt.concern() == PlanarRuntimeConcern::BooleanReadinessBundle
        && receipt.class() == PlanarAdmissionClass::Admitted
}
