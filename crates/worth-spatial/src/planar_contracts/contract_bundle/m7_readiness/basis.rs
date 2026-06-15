use crate::planar_contracts::clean_fail_boundary::PlanarCleanFailBoundaryReceipt;
use crate::planar_contracts::motion_posture::PlanarMotionPostureReceipt;
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::planar_contracts::planar_recovery::PlanarRecoveryPostureReceipt;
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;
use crate::planar_contracts::structural_identity::PlanarStructuralIdentityReceipt;

use super::validation::validate_m7_readiness_basis;
use super::PlanarM7ReadinessDenial;
use crate::planar_contracts::contract_bundle::PlanarContractBundleValidationReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarM7BooleanExecutionSupport {
    SupportGated,
}

impl PlanarM7BooleanExecutionSupport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportGated => "support-gated",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarM7ReadinessSupportPosture {
    boolean_execution: PlanarM7BooleanExecutionSupport,
    reason: String,
}

impl PlanarM7ReadinessSupportPosture {
    pub fn support_gated(reason: impl Into<String>) -> Self {
        Self {
            boolean_execution: PlanarM7BooleanExecutionSupport::SupportGated,
            reason: reason.into(),
        }
    }

    pub fn boolean_execution(&self) -> PlanarM7BooleanExecutionSupport {
        self.boolean_execution
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "m7_boolean_execution:{}:{}",
            self.boolean_execution.as_str(),
            self.reason
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PlanarM7ReadinessBasis {
    boolean_readiness: Box<PlanarContractBundleValidationReceipt>,
    structural_identity: Box<PlanarStructuralIdentityReceipt>,
    motion_posture: Box<PlanarMotionPostureReceipt>,
    retained_planar_facts: Box<RetainedPlanarFactsReceipt>,
    projection_consumed_facts: Box<ProjectionConsumedPlanarFactsReceipt>,
    recovery_posture: Box<PlanarRecoveryPostureReceipt>,
    diagnostics: Box<PlanarDiagnosticBundleReceipt>,
    clean_fail_boundary: Option<Box<PlanarCleanFailBoundaryReceipt>>,
    support_posture: PlanarM7ReadinessSupportPosture,
}

impl PlanarM7ReadinessBasis {
    pub(crate) fn from_builder(
        builder: PlanarM7ReadinessBundle,
    ) -> Result<Self, PlanarM7ReadinessDenial> {
        let basis = Self {
            boolean_readiness: builder.boolean_readiness,
            structural_identity: builder
                .structural_identity
                .ok_or_else(|| missing("structural identity receipt"))?,
            motion_posture: builder
                .motion_posture
                .ok_or_else(|| missing("movement and rotation posture receipt"))?,
            retained_planar_facts: builder
                .retained_planar_facts
                .ok_or_else(|| missing("retained planar facts receipt"))?,
            projection_consumed_facts: builder
                .projection_consumed_facts
                .ok_or_else(|| missing("projection-consumed planar facts receipt"))?,
            recovery_posture: builder
                .recovery_posture
                .ok_or_else(|| missing("recovery posture receipt"))?,
            diagnostics: builder
                .diagnostics
                .ok_or_else(|| missing("diagnostics receipt"))?,
            clean_fail_boundary: builder.clean_fail_boundary,
            support_posture: builder.support_posture.ok_or_else(|| {
                super::PlanarM7ReadinessDenial::new(
                    super::PlanarM7ReadinessDenialKind::MissingSupportPosture,
                    "M7 readiness requires explicit support posture for boolean execution lanes",
                )
            })?,
        };
        validate_m7_readiness_basis(&basis)?;
        Ok(basis)
    }

    pub fn boolean_readiness(&self) -> &PlanarContractBundleValidationReceipt {
        self.boolean_readiness.as_ref()
    }

    pub fn structural_identity(&self) -> &PlanarStructuralIdentityReceipt {
        self.structural_identity.as_ref()
    }

    pub fn motion_posture(&self) -> &PlanarMotionPostureReceipt {
        self.motion_posture.as_ref()
    }

    pub fn retained_planar_facts(&self) -> &RetainedPlanarFactsReceipt {
        self.retained_planar_facts.as_ref()
    }

    pub fn projection_consumed_facts(&self) -> &ProjectionConsumedPlanarFactsReceipt {
        self.projection_consumed_facts.as_ref()
    }

    pub fn recovery_posture(&self) -> &PlanarRecoveryPostureReceipt {
        self.recovery_posture.as_ref()
    }

    pub fn diagnostics(&self) -> &PlanarDiagnosticBundleReceipt {
        self.diagnostics.as_ref()
    }

    pub fn clean_fail_boundary(&self) -> Option<&PlanarCleanFailBoundaryReceipt> {
        self.clean_fail_boundary.as_deref()
    }

    pub fn support_posture(&self) -> &PlanarM7ReadinessSupportPosture {
        &self.support_posture
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarM7ReadinessBundle {
    boolean_readiness: Box<PlanarContractBundleValidationReceipt>,
    structural_identity: Option<Box<PlanarStructuralIdentityReceipt>>,
    motion_posture: Option<Box<PlanarMotionPostureReceipt>>,
    retained_planar_facts: Option<Box<RetainedPlanarFactsReceipt>>,
    projection_consumed_facts: Option<Box<ProjectionConsumedPlanarFactsReceipt>>,
    recovery_posture: Option<Box<PlanarRecoveryPostureReceipt>>,
    diagnostics: Option<Box<PlanarDiagnosticBundleReceipt>>,
    clean_fail_boundary: Option<Box<PlanarCleanFailBoundaryReceipt>>,
    support_posture: Option<PlanarM7ReadinessSupportPosture>,
}

impl PlanarM7ReadinessBundle {
    pub fn from_certified_planar_bundle(receipt: PlanarContractBundleValidationReceipt) -> Self {
        Self {
            boolean_readiness: Box::new(receipt),
            structural_identity: None,
            motion_posture: None,
            retained_planar_facts: None,
            projection_consumed_facts: None,
            recovery_posture: None,
            diagnostics: None,
            clean_fail_boundary: None,
            support_posture: None,
        }
    }

    pub fn with_structural_identity(mut self, receipt: PlanarStructuralIdentityReceipt) -> Self {
        self.structural_identity = Some(Box::new(receipt));
        self
    }

    pub fn with_motion_posture(mut self, receipt: PlanarMotionPostureReceipt) -> Self {
        self.motion_posture = Some(Box::new(receipt));
        self
    }

    pub fn with_retained_planar_facts(mut self, receipt: RetainedPlanarFactsReceipt) -> Self {
        self.retained_planar_facts = Some(Box::new(receipt));
        self
    }

    pub fn with_projection_consumed_facts(
        mut self,
        receipt: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.projection_consumed_facts = Some(Box::new(receipt));
        self
    }

    pub fn with_recovery_posture(mut self, receipt: PlanarRecoveryPostureReceipt) -> Self {
        self.recovery_posture = Some(Box::new(receipt));
        self
    }

    pub fn with_diagnostics(mut self, receipt: PlanarDiagnosticBundleReceipt) -> Self {
        self.diagnostics = Some(Box::new(receipt));
        self
    }

    pub fn with_clean_fail_boundary(mut self, receipt: PlanarCleanFailBoundaryReceipt) -> Self {
        self.clean_fail_boundary = Some(Box::new(receipt));
        self
    }

    pub fn with_support_posture(mut self, posture: PlanarM7ReadinessSupportPosture) -> Self {
        self.support_posture = Some(posture);
        self
    }

    pub(crate) fn build(self) -> Result<PlanarM7ReadinessBasis, PlanarM7ReadinessDenial> {
        PlanarM7ReadinessBasis::from_builder(self)
    }
}

fn missing(label: &'static str) -> PlanarM7ReadinessDenial {
    PlanarM7ReadinessDenial::new(
        super::PlanarM7ReadinessDenialKind::MissingCloseoutFamily,
        format!("M7 readiness requires {label}"),
    )
}
