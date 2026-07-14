use super::{
    UiMeasurementCoordinateSpace, UiMeasurementEvidenceCategory, UiMeasurementResult,
    UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};

/// Value-independent authority required to route a host measurement into an
/// admitted allocation neighborhood.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiHostMeasurementAuthorityWitness {
    request_identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    request_shape_digest: u64,
    evidence_category: UiMeasurementEvidenceCategory,
    evidence_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
    capability_observation_generation: u64,
    capability_profile_digest: u64,
    viewport_assumption_digest: u64,
    dpi_assumption_digest: u64,
    font_assumption_digest: u64,
    adapter_profile_digest: u64,
    unit_posture: UiMeasurementUnitPosture,
    coordinate_space: UiMeasurementCoordinateSpace,
    rounding_posture: UiMeasurementRoundingPosture,
}

impl UiHostMeasurementAuthorityWitness {
    pub(crate) fn identity_digest(self) -> u64 {
        let mut digest =
            crate::declaration::stable_text_digest("worth-ui.host-measurement-witness");
        digest ^= self.request_identity.as_u64().rotate_left(5);
        digest ^= self.request_shape_digest.rotate_left(11);
        digest ^= self.evidence_generation.as_u64().rotate_left(17);
        digest ^= self.capability_observation_generation.rotate_left(23);
        digest ^= self.capability_profile_digest.rotate_left(29);
        digest ^= self.viewport_assumption_digest.rotate_left(31);
        digest ^= self.dpi_assumption_digest.rotate_left(37);
        digest ^= self.font_assumption_digest.rotate_left(41);
        digest ^ self.adapter_profile_digest.rotate_left(47)
    }
    pub(super) fn seal(result: &UiMeasurementResult) -> Self {
        let profile = result.assumption_profile();
        Self {
            request_identity: result.request_identity(),
            request_shape_digest: result.request_shape_digest(),
            evidence_category: result.evidence_category(),
            evidence_generation: result.evidence_generation(),
            capability_observation_generation: profile.capability_observation_generation().as_u64(),
            capability_profile_digest: profile.capability_profile_digest(),
            viewport_assumption_digest: profile.viewport_assumption_digest(),
            dpi_assumption_digest: profile.dpi_assumption_digest(),
            font_assumption_digest: profile.font_assumption_digest(),
            adapter_profile_digest: profile.adapter_profile_digest(),
            unit_posture: result.unit_posture(),
            coordinate_space: result.coordinate_space(),
            rounding_posture: result.rounding_posture(),
        }
    }

    pub(crate) fn request_identity(self) -> worth_ui_host_contract::UiMeasurementRequestIdentity {
        self.request_identity
    }

    pub(crate) fn evidence_category(self) -> UiMeasurementEvidenceCategory {
        self.evidence_category
    }

    pub(crate) fn evidence_generation(self) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
        self.evidence_generation
    }

    /// Compares the stable normalization authority while deliberately excluding
    /// the sampled generation, request payload, and coordinate space. Portal
    /// anchor target and coordinate-space transitions are runtime semantics,
    /// not normalization-authority mismatches.
    pub(crate) fn same_normalization_authority(self, other: Self) -> bool {
        self.request_identity == other.request_identity
            && self.evidence_category == other.evidence_category
            && self.capability_observation_generation == other.capability_observation_generation
            && self.capability_profile_digest == other.capability_profile_digest
            && self.viewport_assumption_digest == other.viewport_assumption_digest
            && self.dpi_assumption_digest == other.dpi_assumption_digest
            && self.font_assumption_digest == other.font_assumption_digest
            && self.adapter_profile_digest == other.adapter_profile_digest
            && self.unit_posture == other.unit_posture
            && self.rounding_posture == other.rounding_posture
    }
}
