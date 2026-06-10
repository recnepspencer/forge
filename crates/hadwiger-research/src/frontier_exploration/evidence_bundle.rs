use crate::frontier_seeds::FrontierGraphSeedArtifact;
use crate::mathematical_verification::{
    KColorabilityVerificationChecked, UnitDistanceVerificationChecked,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontierExplorationEvidencePosture {
    CandidateOnly,
    GeometryReady,
    ColorabilityReady,
    TerminalForcingReady,
}

impl FrontierExplorationEvidencePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CandidateOnly => "candidate_only",
            Self::GeometryReady => "geometry_ready",
            Self::ColorabilityReady => "colorability_ready",
            Self::TerminalForcingReady => "terminal_forcing_ready",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierExplorationEvidenceBundle {
    seed: FrontierGraphSeedArtifact,
    unit_checked: Option<UnitDistanceVerificationChecked>,
    color_checked: Option<KColorabilityVerificationChecked>,
}

impl FrontierExplorationEvidenceBundle {
    pub fn new(seed: &FrontierGraphSeedArtifact) -> Self {
        Self {
            seed: seed.clone(),
            unit_checked: None,
            color_checked: None,
        }
    }

    pub fn with_unit_distance_verification(
        mut self,
        checked: &UnitDistanceVerificationChecked,
    ) -> Self {
        self.unit_checked = Some(checked.clone());
        self
    }

    pub fn with_colorability_verification(
        mut self,
        checked: &KColorabilityVerificationChecked,
    ) -> Self {
        self.color_checked = Some(checked.clone());
        self
    }

    pub fn seed(&self) -> &FrontierGraphSeedArtifact {
        &self.seed
    }

    pub fn posture(&self) -> FrontierExplorationEvidencePosture {
        match (self.unit_checked.is_some(), self.color_checked.is_some()) {
            (true, true) => FrontierExplorationEvidencePosture::TerminalForcingReady,
            (true, false) => FrontierExplorationEvidencePosture::GeometryReady,
            (false, true) => FrontierExplorationEvidencePosture::ColorabilityReady,
            (false, false) => FrontierExplorationEvidencePosture::CandidateOnly,
        }
    }

    pub fn missing_evidence(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.unit_checked.is_none() {
            missing.push("algebraic_unit_distance_verification");
        }
        if self.color_checked.is_none() {
            missing.push("colorability_refutation_certificate");
        }
        missing
    }

    pub(crate) fn unit_checked(&self) -> Option<&UnitDistanceVerificationChecked> {
        self.unit_checked.as_ref()
    }

    pub(crate) fn color_checked(&self) -> Option<&KColorabilityVerificationChecked> {
        self.color_checked.as_ref()
    }
}
