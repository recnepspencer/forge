use crate::identity::hash_parts;
use crate::orchestration_inventory::ForgeQueryOrchestrationSurfaceInventory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPlatformEntryHostileDivergenceClass {
    DistinctSemanticStop,
    DistinctBoundary,
    DistinctCoverageKind,
}

impl ForgeQueryPlatformEntryHostileDivergenceClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DistinctSemanticStop => "distinct_semantic_stop",
            Self::DistinctBoundary => "distinct_boundary",
            Self::DistinctCoverageKind => "distinct_coverage_kind",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryHostileRow {
    label: &'static str,
    anchor_surface: Option<&'static str>,
    divergence_class: ForgeQueryPlatformEntryHostileDivergenceClass,
    proof_path: &'static str,
    proof_name: &'static str,
}

impl ForgeQueryPlatformEntryHostileRow {
    const fn new(
        label: &'static str,
        anchor_surface: Option<&'static str>,
        divergence_class: ForgeQueryPlatformEntryHostileDivergenceClass,
        proof_path: &'static str,
        proof_name: &'static str,
    ) -> Self {
        Self {
            label,
            anchor_surface,
            divergence_class,
            proof_path,
            proof_name,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn anchor_surface(&self) -> Option<&'static str> {
        self.anchor_surface
    }

    pub fn divergence_class(&self) -> ForgeQueryPlatformEntryHostileDivergenceClass {
        self.divergence_class
    }

    pub fn proof_path(&self) -> &'static str {
        self.proof_path
    }

    pub fn proof_name(&self) -> &'static str {
        self.proof_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryHostileManifest {
    rows: Vec<ForgeQueryPlatformEntryHostileRow>,
    hostile_digest: String,
}

impl ForgeQueryPlatformEntryHostileManifest {
    fn new(rows: Vec<ForgeQueryPlatformEntryHostileRow>) -> Self {
        let hostile_digest = hash_parts(
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}|{}|{}",
                        row.label(),
                        row.anchor_surface().unwrap_or("none"),
                        row.proof_path()
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            hostile_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryPlatformEntryHostileRow] {
        &self.rows
    }

    pub fn hostile_digest(&self) -> &str {
        &self.hostile_digest
    }

    pub fn row_for_label(&self, label: &str) -> Option<&ForgeQueryPlatformEntryHostileRow> {
        self.rows.iter().find(|row| row.label() == label)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryHostileAudit {
    hostile_digest: String,
    missing_divergence_rows: Vec<String>,
    unknown_surfaces: Vec<String>,
    missing_proof_paths: Vec<String>,
    missing_proof_anchors: Vec<String>,
}

impl ForgeQueryPlatformEntryHostileAudit {
    pub fn current() -> Self {
        Self::from_manifest(&forge_query_platform_entry_hostile_manifest())
    }

    pub fn from_manifest(manifest: &ForgeQueryPlatformEntryHostileManifest) -> Self {
        let inventory = ForgeQueryOrchestrationSurfaceInventory::current();
        let mut missing_divergence_rows = REQUIRED_HOSTILE_LABELS
            .iter()
            .filter(|label| manifest.row_for_label(label).is_none())
            .map(|label| (*label).to_string())
            .collect::<Vec<_>>();
        let mut unknown_surfaces = manifest
            .rows()
            .iter()
            .filter_map(|row| row.anchor_surface())
            .filter(|surface| inventory.row_for_public_name(surface).is_none())
            .map(str::to_string)
            .collect::<Vec<_>>();
        let mut missing_proof_paths = manifest
            .rows()
            .iter()
            .filter(|row| {
                !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(row.proof_path())
                    .is_file()
            })
            .map(|row| row.proof_path().to_string())
            .collect::<Vec<_>>();
        let mut missing_proof_anchors = manifest
            .rows()
            .iter()
            .filter(|row| !proof_anchor_exists(row.proof_path(), row.proof_name()))
            .map(|row| format!("{}::{}", row.proof_path(), row.proof_name()))
            .collect::<Vec<_>>();
        missing_divergence_rows.sort();
        unknown_surfaces.sort();
        missing_proof_paths.sort();
        missing_proof_anchors.sort();
        Self {
            hostile_digest: manifest.hostile_digest().to_string(),
            missing_divergence_rows,
            unknown_surfaces,
            missing_proof_paths,
            missing_proof_anchors,
        }
    }

    pub fn hostile_digest(&self) -> &str {
        &self.hostile_digest
    }

    pub fn missing_divergence_rows(&self) -> &[String] {
        &self.missing_divergence_rows
    }

    pub fn unknown_surfaces(&self) -> &[String] {
        &self.unknown_surfaces
    }

    pub fn missing_proof_paths(&self) -> &[String] {
        &self.missing_proof_paths
    }

    pub fn missing_proof_anchors(&self) -> &[String] {
        &self.missing_proof_anchors
    }
}

const REQUIRED_HOSTILE_LABELS: &[&str] = &[
    "signal_compatible_remains_distinct_from_prepared",
    "prepared_continuation_remains_distinct_from_execution",
    "declaration_denial_remains_distinct_from_contribution_denial",
    "helper_family_gating_remains_distinct_from_generic_admitted_path",
    "grouped_member_stop_remains_distinct_from_group_alignment_stop",
    "coverage_boundary_readout_remains_distinct_from_surface_coverage_readout",
    "docs_coverage_gap_remains_distinct_from_closed_alignment",
];

pub fn forge_query_platform_entry_hostile_manifest() -> ForgeQueryPlatformEntryHostileManifest {
    use ForgeQueryPlatformEntryHostileDivergenceClass::{
        DistinctBoundary as B, DistinctCoverageKind as C, DistinctSemanticStop as S,
    };

    ForgeQueryPlatformEntryHostileManifest::new(vec![
        ForgeQueryPlatformEntryHostileRow::new(
            "signal_compatible_remains_distinct_from_prepared",
            Some("orchestrate_signal_compatibility"),
            S,
            "src/signal_compatibility_orchestration/tests/support.rs",
            "orchestration_outcome_token",
        ),
        ForgeQueryPlatformEntryHostileRow::new(
            "prepared_continuation_remains_distinct_from_execution",
            Some("execute_prepared_continuation"),
            S,
            "src/continuation_pipeline/tests/execution.rs",
            "execution_stays_separate_from_preparation_and_produces_runtime_artifact",
        ),
        ForgeQueryPlatformEntryHostileRow::new(
            "declaration_denial_remains_distinct_from_contribution_denial",
            Some("orchestrate_declaration_with_contributions_outcome"),
            S,
            "src/recovery_boundary/tests/ordinary.rs",
            "ordinary_contribution_denial_uses_contribution_recovery_surface",
        ),
        ForgeQueryPlatformEntryHostileRow::new(
            "helper_family_gating_remains_distinct_from_generic_admitted_path",
            Some("prepare_preview_for_active_face_selection_checked"),
            B,
            "tests/ui/domain_handle/boundaries/family_helper_rejects_non_geometry_family.rs",
            "main",
        ),
        ForgeQueryPlatformEntryHostileRow::new(
            "grouped_member_stop_remains_distinct_from_group_alignment_stop",
            Some("orchestrate_local_neighborhood_for_active_face_selection_checked"),
            S,
            "src/platform_entry_closeout/tests/hostile.rs",
            "grouped_member_stop_remains_distinct_from_group_alignment_stop",
        ),
        ForgeQueryPlatformEntryHostileRow::new(
            "coverage_boundary_readout_remains_distinct_from_surface_coverage_readout",
            None,
            C,
            "src/platform_entry_closeout/tests/hostile.rs",
            "coverage_boundary_readout_remains_distinct_from_surface_coverage_readout",
        ),
        ForgeQueryPlatformEntryHostileRow::new(
            "docs_coverage_gap_remains_distinct_from_closed_alignment",
            None,
            C,
            "src/platform_entry_closeout/tests/hostile.rs",
            "docs_coverage_gap_remains_distinct_from_closed_alignment",
        ),
    ])
}

fn proof_anchor_exists(path: &str, proof_name: &str) -> bool {
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .ok()
        .is_some_and(|content| content.contains(&format!("fn {proof_name}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hostile_manifest_rows_are_present_and_backed_by_real_proof_files() {
        let manifest = forge_query_platform_entry_hostile_manifest();

        assert_eq!(manifest.rows().len(), 7);
        assert!(!manifest.hostile_digest().is_empty());
        assert!(ForgeQueryPlatformEntryHostileAudit::current()
            .missing_divergence_rows()
            .is_empty());
        assert!(ForgeQueryPlatformEntryHostileAudit::current()
            .unknown_surfaces()
            .is_empty());
        assert!(ForgeQueryPlatformEntryHostileAudit::current()
            .missing_proof_paths()
            .is_empty());
        assert!(ForgeQueryPlatformEntryHostileAudit::current()
            .missing_proof_anchors()
            .is_empty());
    }
}
