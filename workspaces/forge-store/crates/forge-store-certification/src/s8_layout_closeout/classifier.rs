use super::S8LayoutCloseoutSources;
use forge_store_physical_certification::layout_harness::scenario::layout_scenario;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutCloseoutClassification {
    ScenarioDefinition,
    PerformanceEvidence,
    CertificationCloseout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutCloseoutClassifier {
    classification: S8LayoutCloseoutClassification,
}

pub fn classify_s8_layout_closeout_sources(
    sources: &S8LayoutCloseoutSources,
) -> S8LayoutCloseoutClassifier {
    let canonical = layout_scenario(sources.scenario().scenario().kind());
    let classification = match canonical.closeout() {
            forge_store_physical_certification::layout_harness::closeout::S8LayoutCloseoutEvidenceLane::ScenarioDefinition => {
                S8LayoutCloseoutClassification::ScenarioDefinition
            }
            forge_store_physical_certification::layout_harness::closeout::S8LayoutCloseoutEvidenceLane::PerformanceEvidence => {
                S8LayoutCloseoutClassification::PerformanceEvidence
            }
            forge_store_physical_certification::layout_harness::closeout::S8LayoutCloseoutEvidenceLane::CertificationCloseout => {
                S8LayoutCloseoutClassification::CertificationCloseout
            }
    };
    S8LayoutCloseoutClassifier { classification }
}

impl S8LayoutCloseoutClassifier {
    pub const fn classification(&self) -> S8LayoutCloseoutClassification {
        self.classification
    }
}
