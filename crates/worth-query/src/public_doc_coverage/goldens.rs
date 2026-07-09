use crate::identity::hash_parts;
use crate::public_doc_coverage::WorthQueryPublicJourneyKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublicGoldenTranscriptKind {
    SurfaceCoverage,
    CoverageBoundaryReadout,
}

impl WorthQueryPublicGoldenTranscriptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceCoverage => "surface_coverage",
            Self::CoverageBoundaryReadout => "coverage_boundary_readout",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicGoldenTranscript {
    label: &'static str,
    path: &'static str,
    dx_focus: &'static str,
    kind: WorthQueryPublicGoldenTranscriptKind,
    journey: Option<WorthQueryPublicJourneyKind>,
}

impl WorthQueryPublicGoldenTranscript {
    pub(crate) const fn new(
        label: &'static str,
        path: &'static str,
        dx_focus: &'static str,
        kind: WorthQueryPublicGoldenTranscriptKind,
        journey: Option<WorthQueryPublicJourneyKind>,
    ) -> Self {
        Self {
            label,
            path,
            dx_focus,
            kind,
            journey,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn dx_focus(&self) -> &'static str {
        self.dx_focus
    }

    pub fn kind(&self) -> WorthQueryPublicGoldenTranscriptKind {
        self.kind
    }

    pub fn journey(self) -> Option<WorthQueryPublicJourneyKind> {
        self.journey
    }
}

const GOLDEN_TRANSCRIPTS: [WorthQueryPublicGoldenTranscript; 8] = [
    WorthQueryPublicGoldenTranscript::new(
        "public_doc_coverage_surface_readout",
        "tests/ui/domain_handle/golden/public_doc_coverage_surface_readout_compiles.rs",
        "public doc coverage readout",
        WorthQueryPublicGoldenTranscriptKind::CoverageBoundaryReadout,
        None,
    ),
    WorthQueryPublicGoldenTranscript::new(
        "declaration_entry_orchestration_surface_readout",
        "tests/ui/domain_handle/golden/declaration_entry_orchestration_surface_readout_compiles.rs",
        "declaration-entry coverage surface readout",
        WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(WorthQueryPublicJourneyKind::PlatformEntry),
    ),
    WorthQueryPublicGoldenTranscript::new(
        "continuation_pipeline_surface_readout",
        "tests/ui/domain_handle/golden/continuation_pipeline_surface_readout_compiles.rs",
        "continuation coverage surface readout",
        WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(WorthQueryPublicJourneyKind::Continuation),
    ),
    WorthQueryPublicGoldenTranscript::new(
        "signal_compatibility_surface_readout",
        "tests/ui/domain_handle/golden/signal_compatibility_surface_readout_compiles.rs",
        "signal compatibility coverage surface readout",
        WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(WorthQueryPublicJourneyKind::SignalFacing),
    ),
    WorthQueryPublicGoldenTranscript::new(
        "contribution_composed_surface_readout",
        "tests/ui/domain_handle/golden/contribution_composed_surface_readout_compiles.rs",
        "contribution-composed coverage surface readout",
        WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(WorthQueryPublicJourneyKind::ContributionComposed),
    ),
    WorthQueryPublicGoldenTranscript::new(
        "family_helper_surface_readout",
        "tests/ui/domain_handle/golden/family_helper_surface_readout_compiles.rs",
        "family helper docs and coverage readout",
        WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(WorthQueryPublicJourneyKind::HelperProjection),
    ),
    WorthQueryPublicGoldenTranscript::new(
        "grouped_authoring_surface_readout",
        "tests/ui/domain_handle/golden/grouped_authoring_surface_readout_compiles.rs",
        "grouped authoring coverage surface readout",
        WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(WorthQueryPublicJourneyKind::GroupedAuthoring),
    ),
    WorthQueryPublicGoldenTranscript::new(
        "recovery_boundary_surface_readout",
        "tests/ui/domain_handle/golden/recovery_boundary_surface_readout_compiles.rs",
        "recovery boundary coverage surface readout",
        WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(WorthQueryPublicJourneyKind::Recovery),
    ),
];

pub fn worth_query_public_doc_coverage_golden_transcripts(
) -> &'static [WorthQueryPublicGoldenTranscript] {
    &GOLDEN_TRANSCRIPTS
}

pub fn worth_query_public_doc_coverage_golden_transcript_digest() -> String {
    hash_parts(
        &GOLDEN_TRANSCRIPTS
            .iter()
            .map(|row| {
                format!(
                    "{}|{}|{}|{}|{}",
                    row.label(),
                    row.path(),
                    row.dx_focus(),
                    row.kind().as_str(),
                    row.journey()
                        .map(WorthQueryPublicJourneyKind::as_str)
                        .unwrap_or("none"),
                )
            })
            .collect::<Vec<_>>(),
    )
}

pub(crate) fn golden_transcript_by_label(label: &str) -> WorthQueryPublicGoldenTranscript {
    *GOLDEN_TRANSCRIPTS
        .iter()
        .find(|row| row.label() == label)
        .unwrap_or_else(|| panic!("expected golden transcript label {label}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use std::path::Path;

    #[test]
    fn golden_transcript_manifest_is_duplicate_free_and_nonempty() {
        let rows = worth_query_public_doc_coverage_golden_transcripts();
        let labels = rows.iter().map(|row| row.label()).collect::<Vec<_>>();
        let paths = rows.iter().map(|row| row.path()).collect::<Vec<_>>();

        assert_eq!(rows.len(), 8);
        assert_eq!(
            labels.len(),
            labels.iter().copied().collect::<BTreeSet<_>>().len()
        );
        assert_eq!(
            paths.len(),
            paths.iter().copied().collect::<BTreeSet<_>>().len()
        );
        assert!(rows.iter().all(|row| !row.dx_focus().is_empty()));
        assert!(rows.iter().all(|row| {
            matches!(
                (row.kind(), row.journey()),
                (
                    WorthQueryPublicGoldenTranscriptKind::CoverageBoundaryReadout,
                    None
                ) | (
                    WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage,
                    Some(_)
                )
            )
        }));
        assert!(!worth_query_public_doc_coverage_golden_transcript_digest().is_empty());
    }

    #[test]
    fn golden_transcript_manifest_paths_exist() {
        for row in worth_query_public_doc_coverage_golden_transcripts() {
            assert!(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(row.path())
                    .is_file(),
                "missing golden transcript {}",
                row.path()
            );
        }
    }
}
