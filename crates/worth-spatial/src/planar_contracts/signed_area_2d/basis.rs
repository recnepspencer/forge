use crate::planar_contracts::polygon_winding_2d::CertifiedPolygonWinding2DReceipt;
use crate::planar_contracts::precision_basis::PlanarPrecisionCertificateReceipt;

use super::validation::validate_signed_area_basis;
use super::{
    AreaDegeneracyClass, AreaDegeneracyPolicy, CertifiedSignedArea2DDenial,
    SignedAreaDegeneracyCause, SignedAreaOrientation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSignedArea2DBasis {
    winding_receipt: CertifiedPolygonWinding2DReceipt,
    precision_receipt: PlanarPrecisionCertificateReceipt,
    degeneracy_policy: AreaDegeneracyPolicy,
    orientation: SignedAreaOrientation,
    degeneracy: AreaDegeneracyClass,
    signed_area_twice_decimal: String,
    localized_cause: Option<SignedAreaDegeneracyCause>,
}

impl CertifiedSignedArea2DBasis {
    pub(crate) fn new(
        winding_receipt: CertifiedPolygonWinding2DReceipt,
        precision_receipt: PlanarPrecisionCertificateReceipt,
        degeneracy_policy: AreaDegeneracyPolicy,
    ) -> Result<Self, CertifiedSignedArea2DDenial> {
        let basis = Self {
            winding_receipt,
            precision_receipt,
            degeneracy_policy,
            orientation: SignedAreaOrientation::Zero,
            degeneracy: AreaDegeneracyClass::PolicyRequired,
            signed_area_twice_decimal: "0".to_string(),
            localized_cause: None,
        };
        validate_signed_area_basis(&basis)?;
        Ok(basis)
    }

    pub(crate) fn with_measurement(
        mut self,
        measurement: super::CertifiedSignedAreaMeasurement,
    ) -> Self {
        self.orientation = measurement.orientation;
        self.degeneracy = measurement.degeneracy;
        self.signed_area_twice_decimal = measurement.signed_area_twice_decimal;
        self.localized_cause = measurement.localized_cause;
        self
    }

    pub fn winding_receipt(&self) -> &CertifiedPolygonWinding2DReceipt {
        &self.winding_receipt
    }

    pub fn precision_receipt(&self) -> &PlanarPrecisionCertificateReceipt {
        &self.precision_receipt
    }

    pub fn degeneracy_policy(&self) -> AreaDegeneracyPolicy {
        self.degeneracy_policy
    }

    pub fn orientation(&self) -> SignedAreaOrientation {
        self.orientation
    }

    pub fn degeneracy(&self) -> AreaDegeneracyClass {
        self.degeneracy
    }

    pub fn signed_area_twice_decimal(&self) -> &str {
        &self.signed_area_twice_decimal
    }

    pub fn localized_cause(&self) -> Option<&SignedAreaDegeneracyCause> {
        self.localized_cause.as_ref()
    }

    pub(crate) fn loops(
        &self,
    ) -> &[crate::planar_contracts::polygon_winding_2d::CertifiedLoopWindingSummary] {
        self.winding_receipt.basis().loop_summaries()
    }

    pub fn primary_loop_identity(&self) -> &str {
        self.winding_receipt.basis().primary_loop_identity()
    }

    pub fn planar_neighborhood_identity(&self) -> &str {
        self.winding_receipt.basis().planar_neighborhood_identity()
    }

    pub fn frame_identity(&self) -> &str {
        self.winding_receipt.basis().frame_identity()
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        self.winding_receipt
            .basis()
            .movement_rotation_posture_identity()
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        self.winding_receipt.basis().tolerance_policy_identity()
    }
}
