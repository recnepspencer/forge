#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiAdmittedPortalAnchorObservation {
    identity: super::UiPortalAnchorIdentity,
    rect: worth_ui_host_contract::UiPortalAnchorRectObservation,
    evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
    unit_posture: crate::evidence::UiMeasurementUnitPosture,
    rounding_posture: crate::evidence::UiMeasurementRoundingPosture,
    authority_witness: crate::evidence::UiHostMeasurementAuthorityWitness,
}

// Construction rejects NaN and infinity, so reflexive equality is guaranteed.
impl Eq for UiAdmittedPortalAnchorObservation {}

impl UiAdmittedPortalAnchorObservation {
    pub(crate) fn admit(result: &crate::evidence::UiMeasurementResult) -> Option<Self> {
        let crate::evidence::UiMeasurementValue::PortalAnchorRect(rect) = result.value() else {
            return None;
        };
        let values = [rect.x, rect.y, rect.width, rect.height];
        if values.iter().any(|value| !value.is_finite()) || rect.width < 0.0 || rect.height < 0.0 {
            return None;
        }
        Some(Self {
            identity: super::UiPortalAnchorIdentity::from_measurement_result(result)?,
            rect: *rect,
            evidence_generation: result.evidence_generation(),
            unit_posture: result.unit_posture(),
            rounding_posture: result.rounding_posture(),
            authority_witness: result.authority_witness(),
        })
    }

    pub const fn identity(self) -> super::UiPortalAnchorIdentity {
        self.identity
    }

    pub const fn rect(self) -> worth_ui_host_contract::UiPortalAnchorRectObservation {
        self.rect
    }

    pub const fn evidence_generation(self) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
        self.evidence_generation
    }

    pub const fn unit_posture(self) -> crate::evidence::UiMeasurementUnitPosture {
        self.unit_posture
    }

    pub const fn rounding_posture(self) -> crate::evidence::UiMeasurementRoundingPosture {
        self.rounding_posture
    }

    pub(crate) const fn authority_witness(
        self,
    ) -> crate::evidence::UiHostMeasurementAuthorityWitness {
        self.authority_witness
    }
}
