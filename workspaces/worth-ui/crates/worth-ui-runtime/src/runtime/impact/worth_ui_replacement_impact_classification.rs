use crate::runtime::{
    WorthUiAccessibilityImpact, WorthUiCommandImpact, WorthUiRendererResourceImpact,
    WorthUiReplacementImpact, WorthUiReplacementImpactCounters, WorthUiTokenThemeImpact,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReplacementImpactClassification {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    impact: WorthUiReplacementImpact,
    command_impact: WorthUiCommandImpact,
    token_theme_impact: WorthUiTokenThemeImpact,
    accessibility_impact: WorthUiAccessibilityImpact,
    renderer_resource_impact: WorthUiRendererResourceImpact,
    counters: WorthUiReplacementImpactCounters,
}

impl WorthUiReplacementImpactClassification {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        impact: WorthUiReplacementImpact,
        command_impact: WorthUiCommandImpact,
        token_theme_impact: WorthUiTokenThemeImpact,
        accessibility_impact: WorthUiAccessibilityImpact,
        renderer_resource_impact: WorthUiRendererResourceImpact,
        counters: WorthUiReplacementImpactCounters,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            impact,
            command_impact,
            token_theme_impact,
            accessibility_impact,
            renderer_resource_impact,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn impact(&self) -> &WorthUiReplacementImpact {
        &self.impact
    }

    pub fn command_impact(&self) -> WorthUiCommandImpact {
        self.command_impact
    }

    pub fn token_theme_impact(&self) -> WorthUiTokenThemeImpact {
        self.token_theme_impact
    }

    pub fn accessibility_impact(&self) -> WorthUiAccessibilityImpact {
        self.accessibility_impact
    }

    pub fn renderer_resource_impact(&self) -> WorthUiRendererResourceImpact {
        self.renderer_resource_impact
    }

    pub fn counters(&self) -> WorthUiReplacementImpactCounters {
        self.counters
    }
}
