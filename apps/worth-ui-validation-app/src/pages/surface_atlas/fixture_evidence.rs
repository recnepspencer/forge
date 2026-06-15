use worth_ui_harness::facade::{
    HarnessEvidenceBundle, HarnessEvidenceFamily, HarnessExpectedObservation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasFixtureEvidence {
    label: &'static str,
    evidence: HarnessEvidenceBundle,
    expected_observations: Vec<HarnessExpectedObservation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureEvidenceCompletionDenial {
    SampleOnlyEvidenceCannotCompleteScenario,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureEvidenceLabelDenial {
    MissingSampleOnlyLabel,
}

impl SurfaceAtlasFixtureEvidence {
    pub fn sample_only() -> Self {
        Self {
            label: "SampleOnly fixture evidence",
            evidence: HarnessEvidenceBundle::empty(),
            expected_observations: vec![HarnessExpectedObservation::visual_observation()],
        }
    }

    pub fn with_label_for_diagnostics(label: &'static str) -> Self {
        Self {
            label,
            evidence: HarnessEvidenceBundle::empty(),
            expected_observations: vec![HarnessExpectedObservation::visual_observation()],
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn evidence(&self) -> &HarnessEvidenceBundle {
        &self.evidence
    }

    pub fn expected_observations(&self) -> &[HarnessExpectedObservation] {
        &self.expected_observations
    }

    pub fn display_families(&self) -> impl Iterator<Item = HarnessEvidenceFamily> + '_ {
        self.expected_observations
            .iter()
            .map(|observation| observation.evidence_family())
    }

    pub fn validate_label(&self) -> Result<(), FixtureEvidenceLabelDenial> {
        if self.label.contains("SampleOnly") {
            Ok(())
        } else {
            Err(FixtureEvidenceLabelDenial::MissingSampleOnlyLabel)
        }
    }

    pub fn mark_success(&self) -> Result<(), FixtureEvidenceCompletionDenial> {
        Err(FixtureEvidenceCompletionDenial::SampleOnlyEvidenceCannotCompleteScenario)
    }
}
