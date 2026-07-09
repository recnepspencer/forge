use super::docs::WorthQueryPublicDocReference;
use super::goldens::WorthQueryPublicGoldenTranscript;
use super::journeys::WorthQueryPublicJourneyKind;
use crate::identity::hash_parts;
use crate::orchestration_inventory::{
    WorthQueryOrchestrationSurfaceFamily, WorthQueryOrchestrationSurfaceVisibility,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicDocCoverageRow {
    public_name: &'static str,
    canonical_base_name: &'static str,
    orchestration_family: WorthQueryOrchestrationSurfaceFamily,
    visibility: WorthQueryOrchestrationSurfaceVisibility,
    surface_row_digest: String,
    doc_reference: WorthQueryPublicDocReference,
    readme_discovery_label: &'static str,
    golden_transcript: Option<WorthQueryPublicGoldenTranscript>,
    journey: Option<WorthQueryPublicJourneyKind>,
    coverage_digest: String,
}

impl WorthQueryPublicDocCoverageRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        public_name: &'static str,
        canonical_base_name: &'static str,
        orchestration_family: WorthQueryOrchestrationSurfaceFamily,
        visibility: WorthQueryOrchestrationSurfaceVisibility,
        surface_row_digest: String,
        doc_reference: WorthQueryPublicDocReference,
        readme_discovery_label: &'static str,
        golden_transcript: Option<WorthQueryPublicGoldenTranscript>,
        journey: Option<WorthQueryPublicJourneyKind>,
    ) -> Self {
        let coverage_digest = hash_parts(&[
            public_name.to_string(),
            canonical_base_name.to_string(),
            orchestration_family.as_str().to_string(),
            visibility.as_str().to_string(),
            surface_row_digest.clone(),
            doc_reference.path().to_string(),
            doc_reference.section().to_string(),
            readme_discovery_label.to_string(),
            golden_transcript
                .map(|golden| format!("{}|{}", golden.label(), golden.path()))
                .unwrap_or_else(|| "none".to_string()),
            journey
                .map(|kind| kind.as_str().to_string())
                .unwrap_or_else(|| "none".to_string()),
        ]);
        Self {
            public_name,
            canonical_base_name,
            orchestration_family,
            visibility,
            surface_row_digest,
            doc_reference,
            readme_discovery_label,
            golden_transcript,
            journey,
            coverage_digest,
        }
    }

    pub fn public_name(&self) -> &'static str {
        self.public_name
    }

    pub fn canonical_base_name(&self) -> &'static str {
        self.canonical_base_name
    }

    pub fn orchestration_family(&self) -> WorthQueryOrchestrationSurfaceFamily {
        self.orchestration_family
    }

    pub fn visibility(&self) -> WorthQueryOrchestrationSurfaceVisibility {
        self.visibility
    }

    pub fn surface_row_digest(&self) -> &str {
        &self.surface_row_digest
    }

    pub fn doc_reference(&self) -> WorthQueryPublicDocReference {
        self.doc_reference
    }

    pub fn readme_discovery_label(&self) -> &'static str {
        self.readme_discovery_label
    }

    pub fn golden_transcript(&self) -> Option<WorthQueryPublicGoldenTranscript> {
        self.golden_transcript
    }

    pub fn journey(&self) -> Option<WorthQueryPublicJourneyKind> {
        self.journey
    }

    pub fn has_golden_transcript(&self) -> bool {
        self.golden_transcript.is_some()
    }

    pub fn has_readme_discovery(&self) -> bool {
        !self.readme_discovery_label.is_empty()
    }

    pub fn has_journey_coverage(&self) -> bool {
        self.journey.is_some()
    }

    pub fn coverage_digest(&self) -> &str {
        &self.coverage_digest
    }
}
