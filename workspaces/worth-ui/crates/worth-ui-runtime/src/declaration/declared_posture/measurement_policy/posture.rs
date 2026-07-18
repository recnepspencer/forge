use super::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementMode,
    UiDeclaredMeasurementOwnershipPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclaredMeasurementPolicyPosture {
    mode: Option<UiDeclaredMeasurementMode>,
    constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
    evidence_requirements: Box<[UiDeclaredMeasurementEvidenceRequirement]>,
}

impl UiDeclaredMeasurementPolicyPosture {
    pub(crate) fn new(
        mode: Option<UiDeclaredMeasurementMode>,
        constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
        basis_source: Option<UiDeclaredMeasurementBasisSource>,
        ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
        mut evidence_requirements: Vec<UiDeclaredMeasurementEvidenceRequirement>,
    ) -> Option<Self> {
        evidence_requirements.sort_unstable();
        evidence_requirements.dedup();

        (mode.is_some()
            || constraint_modifier.is_some()
            || basis_source.is_some()
            || ownership_posture.is_some()
            || !evidence_requirements.is_empty())
        .then_some(Self {
            mode,
            constraint_modifier,
            basis_source,
            ownership_posture,
            evidence_requirements: evidence_requirements.into_boxed_slice(),
        })
    }

    pub const fn mode(&self) -> Option<UiDeclaredMeasurementMode> {
        self.mode
    }

    pub const fn constraint_modifier(&self) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.constraint_modifier
    }

    pub(crate) fn with_constraint_modifier(
        mut self,
        modifier: UiDeclaredMeasurementConstraintModifier,
    ) -> Self {
        self.constraint_modifier = Some(modifier);
        self
    }

    pub const fn basis_source(&self) -> Option<UiDeclaredMeasurementBasisSource> {
        self.basis_source
    }

    pub const fn ownership_posture(&self) -> Option<UiDeclaredMeasurementOwnershipPosture> {
        self.ownership_posture
    }

    pub fn evidence_requirements(&self) -> &[UiDeclaredMeasurementEvidenceRequirement] {
        &self.evidence_requirements
    }

    pub fn requires_host_font_metrics(&self) -> bool {
        self.evidence_requirements
            .contains(&UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics)
    }
}
