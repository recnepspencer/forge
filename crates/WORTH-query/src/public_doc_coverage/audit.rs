use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::inventory::WorthQueryPublicDocCoverageInventory;
use super::row::WorthQueryPublicDocCoverageRow;
use super::WorthQueryPublicGoldenTranscriptKind;
use crate::orchestration_inventory::WorthQueryOrchestrationSurfaceInventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicDocCoverageAudit {
    coverage_digest: String,
    undocumented_public_surfaces: Vec<String>,
    surfaces_missing_goldens: Vec<String>,
    orphan_doc_rows: Vec<String>,
    orphan_golden_rows: Vec<String>,
    readme_discovery_gaps: Vec<String>,
    journey_coverage_gaps: Vec<String>,
}

impl WorthQueryPublicDocCoverageAudit {
    pub fn current() -> Self {
        Self::from_inventory(&WorthQueryPublicDocCoverageInventory::current())
    }

    pub fn from_inventory(inventory: &WorthQueryPublicDocCoverageInventory) -> Self {
        let live_surface_names = WorthQueryOrchestrationSurfaceInventory::current()
            .rows()
            .iter()
            .map(|row| row.public_name().to_string())
            .collect::<BTreeSet<_>>();
        let coverage_names = inventory
            .rows()
            .iter()
            .map(|row| row.public_name().to_string())
            .collect::<BTreeSet<_>>();
        let used_surface_goldens = inventory
            .rows()
            .iter()
            .filter_map(|row| row.golden_transcript())
            .filter(|golden| golden.kind() == WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage)
            .map(|golden| golden.label().to_string())
            .collect::<BTreeSet<_>>();
        let readme = read_workspace_text("crates/worth-query/docs/domain-capabilities/README.md")
            .unwrap_or_default();

        let mut undocumented_public_surfaces = live_surface_names
            .iter()
            .filter(|public_name| {
                inventory
                    .row_for_public_name(public_name)
                    .is_none_or(|row| !doc_row_exists(row))
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut surfaces_missing_goldens = live_surface_names
            .iter()
            .filter(|public_name| {
                inventory
                    .row_for_public_name(public_name)
                    .is_none_or(|row| !golden_exists(row))
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut orphan_doc_rows = coverage_names
            .difference(&live_surface_names)
            .cloned()
            .collect::<Vec<_>>();
        let mut orphan_golden_rows =
            super::goldens::worth_query_public_doc_coverage_golden_transcripts()
                .iter()
                .filter(|golden| {
                    golden.kind() == WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage
                        && !used_surface_goldens.contains(golden.label())
                })
                .map(|golden| golden.label().to_string())
                .collect::<Vec<_>>();
        let mut readme_discovery_gaps = inventory
            .rows()
            .iter()
            .filter(|row| {
                live_surface_names.contains(row.public_name())
                    && (!row.has_readme_discovery()
                        || !readme.contains(row.readme_discovery_label()))
            })
            .map(|row| row.public_name().to_string())
            .collect::<Vec<_>>();
        let mut journey_coverage_gaps = inventory
            .rows()
            .iter()
            .filter(|row| {
                live_surface_names.contains(row.public_name())
                    && (!row.has_journey_coverage()
                        || row
                            .golden_transcript()
                            .is_some_and(|golden| golden.journey() != row.journey()))
            })
            .map(|row| row.public_name().to_string())
            .collect::<Vec<_>>();

        undocumented_public_surfaces.sort();
        surfaces_missing_goldens.sort();
        orphan_doc_rows.sort();
        orphan_golden_rows.sort();
        readme_discovery_gaps.sort();
        journey_coverage_gaps.sort();

        Self {
            coverage_digest: inventory.coverage_digest().to_string(),
            undocumented_public_surfaces,
            surfaces_missing_goldens,
            orphan_doc_rows,
            orphan_golden_rows,
            readme_discovery_gaps,
            journey_coverage_gaps,
        }
    }

    pub fn coverage_digest(&self) -> &str {
        &self.coverage_digest
    }

    pub fn undocumented_public_surfaces(&self) -> &[String] {
        &self.undocumented_public_surfaces
    }

    pub fn surfaces_missing_goldens(&self) -> &[String] {
        &self.surfaces_missing_goldens
    }

    pub fn orphan_doc_rows(&self) -> &[String] {
        &self.orphan_doc_rows
    }

    pub fn orphan_golden_rows(&self) -> &[String] {
        &self.orphan_golden_rows
    }

    pub fn readme_discovery_gaps(&self) -> &[String] {
        &self.readme_discovery_gaps
    }

    pub fn journey_coverage_gaps(&self) -> &[String] {
        &self.journey_coverage_gaps
    }
}

fn doc_row_exists(row: &WorthQueryPublicDocCoverageRow) -> bool {
    if row.doc_reference().path().is_empty() || row.doc_reference().section().is_empty() {
        return false;
    }
    read_workspace_text(row.doc_reference().path()).is_some_and(|content| {
        (content.contains(row.public_name()) || content.contains(row.canonical_base_name()))
            && content.contains(row.doc_reference().section())
    })
}

fn golden_exists(row: &WorthQueryPublicDocCoverageRow) -> bool {
    row.golden_transcript().is_some_and(|golden| {
        !golden.label().is_empty()
            && !golden.dx_focus().is_empty()
            && golden.kind() == WorthQueryPublicGoldenTranscriptKind::SurfaceCoverage
            && super::goldens::worth_query_public_doc_coverage_golden_transcripts()
                .iter()
                .any(|manifest| manifest == &golden)
            && golden.journey() == row.journey()
            && crate_root().join(golden.path()).is_file()
    })
}

fn read_workspace_text(path: &str) -> Option<String> {
    std::fs::read_to_string(workspace_root().join(path)).ok()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-query manifest should live under crates/")
        .to_path_buf()
}

fn crate_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
