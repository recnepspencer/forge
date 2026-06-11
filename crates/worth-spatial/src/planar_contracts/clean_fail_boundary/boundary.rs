use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::planar_contracts::planar_recovery::PlanarRecoveryPostureReceipt;

use super::validation::validate_planar_clean_fail_boundary_basis;
use super::{
    PlanarBoundedConversion, PlanarCleanFailBoundaryDenial, PlanarCleanFailBoundaryDenialKind,
    PlanarCleanFailInput, PlanarCleanFailTruthEffect, PlanarRepairAttempt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarCleanFailBoundaryBasis {
    input: PlanarCleanFailInput,
    repair_attempt: PlanarRepairAttempt,
    bounded_conversion: PlanarBoundedConversion,
    truth_effect: PlanarCleanFailTruthEffect,
    recovery: PlanarRecoveryPostureReceipt,
    diagnostics: PlanarDiagnosticBundleReceipt,
}

impl PlanarCleanFailBoundaryBasis {
    pub fn builder(input: PlanarCleanFailInput) -> PlanarCleanFailBoundaryBuilder {
        PlanarCleanFailBoundaryBuilder::new(input)
    }

    pub(crate) fn from_builder(
        builder: PlanarCleanFailBoundaryBuilder,
    ) -> Result<Self, PlanarCleanFailBoundaryDenial> {
        let basis = Self {
            input: builder.input,
            repair_attempt: builder.repair_attempt,
            bounded_conversion: builder.bounded_conversion,
            truth_effect: PlanarCleanFailTruthEffect::DoesNotChangePlanarTruth,
            recovery: builder.recovery.ok_or_else(missing_recovery_posture)?,
            diagnostics: builder.diagnostics.ok_or_else(|| missing("diagnostics"))?,
        };
        validate_planar_clean_fail_boundary_basis(&basis)?;
        Ok(basis)
    }

    pub fn input(&self) -> &PlanarCleanFailInput {
        &self.input
    }

    pub fn repair_attempt(&self) -> PlanarRepairAttempt {
        self.repair_attempt
    }

    pub fn bounded_conversion(&self) -> PlanarBoundedConversion {
        self.bounded_conversion
    }

    pub fn truth_effect(&self) -> PlanarCleanFailTruthEffect {
        self.truth_effect
    }

    pub fn recovery(&self) -> &PlanarRecoveryPostureReceipt {
        &self.recovery
    }

    pub fn diagnostics(&self) -> &PlanarDiagnosticBundleReceipt {
        &self.diagnostics
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarCleanFailBoundaryBuilder {
    input: PlanarCleanFailInput,
    repair_attempt: PlanarRepairAttempt,
    bounded_conversion: PlanarBoundedConversion,
    recovery: Option<PlanarRecoveryPostureReceipt>,
    diagnostics: Option<PlanarDiagnosticBundleReceipt>,
}

impl PlanarCleanFailBoundaryBuilder {
    fn new(input: PlanarCleanFailInput) -> Self {
        Self {
            input,
            repair_attempt: PlanarRepairAttempt::NotAttempted,
            bounded_conversion: PlanarBoundedConversion::NotAttempted,
            recovery: None,
            diagnostics: None,
        }
    }

    pub fn repair_was_attempted(mut self) -> Self {
        self.repair_attempt = PlanarRepairAttempt::Attempted;
        self
    }

    pub fn bounded_conversion_was_attempted(mut self) -> Self {
        self.bounded_conversion = PlanarBoundedConversion::Attempted;
        self
    }

    pub fn recovery_posture(mut self, receipt: PlanarRecoveryPostureReceipt) -> Self {
        self.recovery = Some(receipt);
        self
    }

    pub fn diagnostics(mut self, receipt: PlanarDiagnosticBundleReceipt) -> Self {
        self.diagnostics = Some(receipt);
        self
    }

    pub fn build(self) -> Result<PlanarCleanFailBoundaryBasis, PlanarCleanFailBoundaryDenial> {
        PlanarCleanFailBoundaryBasis::from_builder(self)
    }
}

fn missing_recovery_posture() -> PlanarCleanFailBoundaryDenial {
    PlanarCleanFailBoundaryDenial::new(
        PlanarCleanFailBoundaryDenialKind::MissingRecoveryPosture,
        "planar clean-fail boundary requires typed recovery posture",
    )
}

fn missing(label: &'static str) -> PlanarCleanFailBoundaryDenial {
    let kind = match label {
        "diagnostics" => PlanarCleanFailBoundaryDenialKind::MissingDiagnostics,
        _ => PlanarCleanFailBoundaryDenialKind::MissingRecoveryPosture,
    };
    PlanarCleanFailBoundaryDenial::new(
        kind,
        "planar clean-fail boundary requires recovery posture and diagnostics",
    )
}
