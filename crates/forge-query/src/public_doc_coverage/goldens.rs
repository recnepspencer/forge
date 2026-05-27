use crate::identity::hash_parts;
use crate::public_doc_coverage::ForgeQueryPublicJourneyKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPublicGoldenTranscriptKind {
    SurfaceCoverage,
    CoverageBoundaryReadout,
}

impl ForgeQueryPublicGoldenTranscriptKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceCoverage => "surface_coverage",
            Self::CoverageBoundaryReadout => "coverage_boundary_readout",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublicGoldenTranscript {
    label: &'static str,
    path: &'static str,
    dx_focus: &'static str,
    kind: ForgeQueryPublicGoldenTranscriptKind,
    journey: Option<ForgeQueryPublicJourneyKind>,
}

impl ForgeQueryPublicGoldenTranscript {
    pub(crate) const fn new(
        label: &'static str,
        path: &'static str,
        dx_focus: &'static str,
        kind: ForgeQueryPublicGoldenTranscriptKind,
        journey: Option<ForgeQueryPublicJourneyKind>,
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

    pub fn kind(&self) -> ForgeQueryPublicGoldenTranscriptKind {
        self.kind
    }

    pub fn journey(self) -> Option<ForgeQueryPublicJourneyKind> {
        self.journey
    }
}

const GOLDEN_TRANSCRIPTS: [ForgeQueryPublicGoldenTranscript; 7] = [
    ForgeQueryPublicGoldenTranscript::new(
        "public_doc_coverage_surface_readout",
        "tests/ui/domain_handle/golden/public_doc_coverage_surface_readout_compiles.rs",
        "public doc coverage readout",
        ForgeQueryPublicGoldenTranscriptKind::CoverageBoundaryReadout,
        None,
    ),
    ForgeQueryPublicGoldenTranscript::new(
        "declaration_entry_orchestration_surface_readout",
        "tests/ui/domain_handle/golden/declaration_entry_orchestration_surface_readout_compiles.rs",
        "declaration-entry coverage surface readout",
        ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(ForgeQueryPublicJourneyKind::PlatformEntry),
    ),
    ForgeQueryPublicGoldenTranscript::new(
        "continuation_pipeline_surface_readout",
        "tests/ui/domain_handle/golden/continuation_pipeline_surface_readout_compiles.rs",
        "continuation coverage surface readout",
        ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(ForgeQueryPublicJourneyKind::Continuation),
    ),
    ForgeQueryPublicGoldenTranscript::new(
        "signal_compatibility_surface_readout",
        "tests/ui/domain_handle/golden/signal_compatibility_surface_readout_compiles.rs",
        "signal compatibility coverage surface readout",
        ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(ForgeQueryPublicJourneyKind::SignalFacing),
    ),
    ForgeQueryPublicGoldenTranscript::new(
        "contribution_composed_surface_readout",
        "tests/ui/domain_handle/golden/contribution_composed_surface_readout_compiles.rs",
        "contribution-composed coverage surface readout",
        ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(ForgeQueryPublicJourneyKind::ContributionComposed),
    ),
    ForgeQueryPublicGoldenTranscript::new(
        "family_helper_surface_readout",
        "tests/ui/domain_handle/golden/family_helper_surface_readout_compiles.rs",
        "family helper docs and coverage readout",
        ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(ForgeQueryPublicJourneyKind::HelperProjection),
    ),
    ForgeQueryPublicGoldenTranscript::new(
        "grouped_authoring_surface_readout",
        "tests/ui/domain_handle/golden/grouped_authoring_surface_readout_compiles.rs",
        "grouped authoring coverage surface readout",
        ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage,
        Some(ForgeQueryPublicJourneyKind::GroupedAuthoring),
    ),
];

pub fn forge_query_public_doc_coverage_golden_transcripts(
) -> &'static [ForgeQueryPublicGoldenTranscript] {
    &GOLDEN_TRANSCRIPTS
}

pub fn forge_query_public_doc_coverage_golden_transcript_digest() -> String {
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
                        .map(ForgeQueryPublicJourneyKind::as_str)
                        .unwrap_or("none"),
                )
            })
            .collect::<Vec<_>>(),
    )
}

pub(crate) fn golden_transcript_by_label(label: &str) -> ForgeQueryPublicGoldenTranscript {
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
        let rows = forge_query_public_doc_coverage_golden_transcripts();
        let labels = rows.iter().map(|row| row.label()).collect::<Vec<_>>();
        let paths = rows.iter().map(|row| row.path()).collect::<Vec<_>>();

        assert_eq!(rows.len(), 7);
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
                    ForgeQueryPublicGoldenTranscriptKind::CoverageBoundaryReadout,
                    None
                ) | (
                    ForgeQueryPublicGoldenTranscriptKind::SurfaceCoverage,
                    Some(_)
                )
            )
        }));
        assert!(!forge_query_public_doc_coverage_golden_transcript_digest().is_empty());
    }

    #[test]
    fn golden_transcript_manifest_paths_exist() {
        for row in forge_query_public_doc_coverage_golden_transcripts() {
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
