use crate::identity::hash_parts;
use crate::orchestration_inventory::WorthQueryOrchestrationSurfaceInventory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPlatformEntryParityLane {
    kind: &'static str,
    surface: Option<&'static str>,
}

impl WorthQueryPlatformEntryParityLane {
    const fn new(kind: &'static str, surface: Option<&'static str>) -> Self {
        Self { kind, surface }
    }

    pub fn kind(&self) -> &'static str {
        self.kind
    }

    pub fn surface(&self) -> Option<&'static str> {
        self.surface
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPlatformEntryParityAssertionClass {
    Equality,
}

impl WorthQueryPlatformEntryParityAssertionClass {
    pub fn as_str(self) -> &'static str {
        "equality"
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryPlatformEntryParityRow {
    label: &'static str,
    control_lane: WorthQueryPlatformEntryParityLane,
    parity_lane: WorthQueryPlatformEntryParityLane,
    assertion_class: WorthQueryPlatformEntryParityAssertionClass,
    proof_path: &'static str,
    proof_name: &'static str,
}

impl WorthQueryPlatformEntryParityRow {
    const fn new(
        label: &'static str,
        control_lane: WorthQueryPlatformEntryParityLane,
        parity_lane: WorthQueryPlatformEntryParityLane,
        proof_path: &'static str,
        proof_name: &'static str,
    ) -> Self {
        Self {
            label,
            control_lane,
            parity_lane,
            assertion_class: WorthQueryPlatformEntryParityAssertionClass::Equality,
            proof_path,
            proof_name,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn control_lane(&self) -> WorthQueryPlatformEntryParityLane {
        self.control_lane
    }

    pub fn parity_lane(&self) -> WorthQueryPlatformEntryParityLane {
        self.parity_lane
    }

    pub fn assertion_class(&self) -> WorthQueryPlatformEntryParityAssertionClass {
        self.assertion_class
    }

    pub fn proof_path(&self) -> &'static str {
        self.proof_path
    }

    pub fn proof_name(&self) -> &'static str {
        self.proof_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPlatformEntryParityManifest {
    rows: Vec<WorthQueryPlatformEntryParityRow>,
    parity_digest: String,
}

impl WorthQueryPlatformEntryParityManifest {
    fn new(rows: Vec<WorthQueryPlatformEntryParityRow>) -> Self {
        let parity_digest = hash_parts(
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}|{}|{}|{}",
                        row.label(),
                        row.control_lane().kind(),
                        row.parity_lane().kind(),
                        row.proof_path()
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            parity_digest,
        }
    }

    pub fn rows(&self) -> &[WorthQueryPlatformEntryParityRow] {
        &self.rows
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }

    pub fn row_for_label(&self, label: &str) -> Option<&WorthQueryPlatformEntryParityRow> {
        self.rows.iter().find(|row| row.label() == label)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPlatformEntryParityAudit {
    parity_digest: String,
    missing_equivalence_rows: Vec<String>,
    unknown_surfaces: Vec<String>,
    missing_proof_paths: Vec<String>,
    missing_proof_anchors: Vec<String>,
}

impl WorthQueryPlatformEntryParityAudit {
    pub fn current() -> Self {
        Self::from_manifest(&worth_query_platform_entry_parity_manifest())
    }

    pub fn from_manifest(manifest: &WorthQueryPlatformEntryParityManifest) -> Self {
        let inventory = WorthQueryOrchestrationSurfaceInventory::current();
        let mut missing_equivalence_rows = REQUIRED_PARITY_LABELS
            .iter()
            .filter(|label| manifest.row_for_label(label).is_none())
            .map(|label| (*label).to_string())
            .collect::<Vec<_>>();
        let mut unknown_surfaces = manifest
            .rows()
            .iter()
            .flat_map(|row| [row.control_lane().surface(), row.parity_lane().surface()])
            .flatten()
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
        missing_equivalence_rows.sort();
        unknown_surfaces.sort();
        missing_proof_paths.sort();
        missing_proof_anchors.sort();
        Self {
            parity_digest: manifest.parity_digest().to_string(),
            missing_equivalence_rows,
            unknown_surfaces,
            missing_proof_paths,
            missing_proof_anchors,
        }
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }

    pub fn missing_equivalence_rows(&self) -> &[String] {
        &self.missing_equivalence_rows
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

const REQUIRED_PARITY_LABELS: &[&str] = &[
    "geometry_preview_helper_matches_generic_signal_path",
    "geometry_material_attachment_helper_matches_generic_contribution_path",
    "grouped_helper_matches_generic_grouped_path",
    "signal_bridge_request_matches_explicit_prepared_continuation",
    "recovery_brief_matches_between_checked_and_proof_grouped_wrong_world",
];

pub fn worth_query_platform_entry_parity_manifest() -> WorthQueryPlatformEntryParityManifest {
    WorthQueryPlatformEntryParityManifest::new(vec![
        WorthQueryPlatformEntryParityRow::new(
            "geometry_preview_helper_matches_generic_signal_path",
            WorthQueryPlatformEntryParityLane::new(
                "helper",
                Some("prepare_preview_for_active_face_selection_checked"),
            ),
            WorthQueryPlatformEntryParityLane::new(
                "generic",
                Some("orchestrate_signal_compatibility_checked"),
            ),
            "src/family_helpers/tests.rs",
            "preview_helper_matches_generic_signal_orchestration_path",
        ),
        WorthQueryPlatformEntryParityRow::new(
            "geometry_material_attachment_helper_matches_generic_contribution_path",
            WorthQueryPlatformEntryParityLane::new(
                "helper",
                Some("orchestrate_material_attachment_for_active_face_selection_proof"),
            ),
            WorthQueryPlatformEntryParityLane::new(
                "generic",
                Some("orchestrate_declaration_with_contributions_proof"),
            ),
            "src/family_helpers/tests.rs",
            "material_attachment_helper_matches_generic_composed_path",
        ),
        WorthQueryPlatformEntryParityRow::new(
            "grouped_helper_matches_generic_grouped_path",
            WorthQueryPlatformEntryParityLane::new(
                "helper",
                Some("orchestrate_local_neighborhood_for_active_face_selection_checked"),
            ),
            WorthQueryPlatformEntryParityLane::new("generic_grouped_authoring", None),
            "src/grouped_authoring/tests/parity.rs",
            "grouped_orchestration_matches_generic_checked_lowering",
        ),
        WorthQueryPlatformEntryParityRow::new(
            "signal_bridge_request_matches_explicit_prepared_continuation",
            WorthQueryPlatformEntryParityLane::new(
                "signal_orchestration",
                Some("orchestrate_signal_compatibility"),
            ),
            WorthQueryPlatformEntryParityLane::new(
                "prepared_continuation",
                Some("prepare_continuation_from_target_checked"),
            ),
            "src/signal_compatibility_orchestration/tests/parity.rs",
            "bridge_request_path_matches_explicit_continuation_preparation_posture",
        ),
        WorthQueryPlatformEntryParityRow::new(
            "recovery_brief_matches_between_checked_and_proof_grouped_wrong_world",
            WorthQueryPlatformEntryParityLane::new("checked_recovery", None),
            WorthQueryPlatformEntryParityLane::new("proof_recovery", None),
            "src/recovery_boundary/tests/ordinary.rs",
            "grouped_wrong_world_recovery_brief_matches_between_checked_and_proof_lanes",
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
    fn parity_manifest_rows_are_present_and_backed_by_real_tests() {
        let manifest = worth_query_platform_entry_parity_manifest();

        assert_eq!(manifest.rows().len(), 5);
        assert!(!manifest.parity_digest().is_empty());
        assert!(WorthQueryPlatformEntryParityAudit::current()
            .missing_equivalence_rows()
            .is_empty());
        assert!(WorthQueryPlatformEntryParityAudit::current()
            .unknown_surfaces()
            .is_empty());
        assert!(WorthQueryPlatformEntryParityAudit::current()
            .missing_proof_paths()
            .is_empty());
        assert!(WorthQueryPlatformEntryParityAudit::current()
            .missing_proof_anchors()
            .is_empty());
    }
}
