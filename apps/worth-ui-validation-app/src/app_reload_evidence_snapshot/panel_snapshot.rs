use crate::reload::{ValidationReloadEvidenceEntry, ValidationReloadEvidenceLog};

use super::panel_snapshot_support::{
    authored_structural_summary_line, header_rebind, page_host_rebind,
    phase_execution_summary_line, structural_visible_evidence,
};
use super::structural_visible_evidence::ValidationVisibleStructuralEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadEvidencePanelSnapshot {
    entries: Vec<ValidationReloadEvidenceVisibleEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadEvidenceVisibleEntry {
    heading: String,
    lines: Vec<ValidationReloadEvidenceVisibleLine>,
    structural_evidence: Option<ValidationVisibleStructuralEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationReloadEvidenceVisibleLine {
    label: String,
    value: String,
}

impl ValidationReloadEvidencePanelSnapshot {
    pub fn from_log(evidence_log: &ValidationReloadEvidenceLog) -> Self {
        Self {
            entries: evidence_log
                .entries()
                .iter()
                .rev()
                .map(ValidationReloadEvidenceVisibleEntry::from_entry)
                .collect(),
        }
    }

    pub fn entries(&self) -> &[ValidationReloadEvidenceVisibleEntry] {
        &self.entries
    }
}

impl ValidationReloadEvidenceVisibleEntry {
    fn from_entry(entry: &ValidationReloadEvidenceEntry) -> Self {
        let mut visible_entry = match entry {
            ValidationReloadEvidenceEntry::RuntimeReload {
                status,
                active_artifact_digest,
                active_plan_digest,
                phase_execution,
                authored_structural,
                query_bindings_compared,
                query_rebind_entries,
                changed_fact_count,
                ..
            } => Self::with_value("Reload status", format!("{status:?}"))
                .with_line("Active artifact", active_artifact_digest.to_string())
                .with_line("Active plan", active_plan_digest.to_string())
                .with_line(
                    "Query bindings compared",
                    query_bindings_compared.to_string(),
                )
                .with_line("Query rebind entries", query_rebind_entries.to_string())
                .with_line("Changed runtime facts", changed_fact_count.to_string())
                .with_optional_line(
                    authored_structural
                        .as_ref()
                        .map(authored_structural_summary_line),
                )
                .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::AuthoredBatchReload {
                source_evidence,
                runtime_change,
                phase_execution,
                authored_structural,
                ..
            } => Self::with_value(
                "Authoring-truth final boss",
                format!("{:?}", source_evidence.status()),
            )
            .with_line(
                "Authored delta digest",
                source_evidence
                    .authored_delta_digest()
                    .unwrap_or_default()
                    .to_string(),
            )
            .with_line("Runtime change digest", runtime_change.digest().to_string())
            .with_line(
                "Runtime change rows",
                runtime_change.counters().family_row_count().to_string(),
            )
            .with_line(
                "Changed runtime facts",
                runtime_change.counters().changed_fact_count().to_string(),
            )
            .with_optional_line(
                authored_structural
                    .as_ref()
                    .map(authored_structural_summary_line),
            )
            .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::ThemeDenied(denial) => {
                Self::with_value("Theme reload denied", format!("{:?}", denial.reason())).with_line(
                    "Theme source digest",
                    denial.theme_source_digest().to_string(),
                )
            }
            ValidationReloadEvidenceEntry::ThemeReload {
                status,
                active_snapshot_digest,
                phase_execution,
                touched_theme_token_count,
                changed_fact_count,
                ..
            } => Self::with_value("Theme reload status", format!("{status:?}"))
                .with_line("Active snapshot", active_snapshot_digest.to_string())
                .with_line("Touched entries", touched_theme_token_count.to_string())
                .with_line("Changed runtime facts", changed_fact_count.to_string())
                .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::CommandReload {
                status,
                active_snapshot_digest,
                phase_execution,
                touched_command_count,
                changed_fact_count,
                ..
            } => Self::with_value("Command reload status", format!("{status:?}"))
                .with_line("Active snapshot", active_snapshot_digest.to_string())
                .with_line("Touched entries", touched_command_count.to_string())
                .with_line("Changed runtime facts", changed_fact_count.to_string())
                .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::ComponentReload {
                status,
                active_snapshot_digest,
                phase_execution,
                touched_component_count,
                changed_component_count,
                family_rebuild_breadth,
                changed_fact_count,
                ..
            } => Self::with_value("Component reload status", format!("{status:?}"))
                .with_line("Active snapshot", active_snapshot_digest.to_string())
                .with_line("Touched entries", touched_component_count.to_string())
                .with_line("Changed components", changed_component_count.to_string())
                .with_line("Family breadth", family_rebuild_breadth.to_string())
                .with_line("Changed runtime facts", changed_fact_count.to_string())
                .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::CommandProjectionReload {
                status,
                active_snapshot_digest,
                phase_execution,
                touched_projection_count,
                changed_fact_count,
                ..
            } => Self::with_value("Command projection reload status", format!("{status:?}"))
                .with_line("Active snapshot", active_snapshot_digest.to_string())
                .with_line("Touched entries", touched_projection_count.to_string())
                .with_line("Changed runtime facts", changed_fact_count.to_string())
                .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::AppearanceReload {
                status,
                active_snapshot_digest,
                phase_execution,
                touched_appearance_count,
                changed_appearance_count,
                canonicalization_count,
                descriptor_lookup_count,
                family_rebuild_breadth,
                changed_fact_count,
                ..
            } => Self::with_value("Appearance reload status", format!("{status:?}"))
                .with_line("Active snapshot", active_snapshot_digest.to_string())
                .with_line("Touched entries", touched_appearance_count.to_string())
                .with_line(
                    "Changed appearance tokens",
                    changed_appearance_count.to_string(),
                )
                .with_line("Canonicalized values", canonicalization_count.to_string())
                .with_line("Descriptor lookups", descriptor_lookup_count.to_string())
                .with_line("Family breadth", family_rebuild_breadth.to_string())
                .with_line("Changed runtime facts", changed_fact_count.to_string())
                .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::DensityReload {
                status,
                active_snapshot_digest,
                phase_execution,
                touched_density_count,
                changed_density_count,
                canonicalization_count,
                descriptor_lookup_count,
                family_rebuild_breadth,
                changed_fact_count,
                ..
            } => Self::with_value("Density reload status", format!("{status:?}"))
                .with_line("Active snapshot", active_snapshot_digest.to_string())
                .with_line("Touched entries", touched_density_count.to_string())
                .with_line("Changed density tokens", changed_density_count.to_string())
                .with_line("Canonicalized values", canonicalization_count.to_string())
                .with_line("Descriptor lookups", descriptor_lookup_count.to_string())
                .with_line("Family breadth", family_rebuild_breadth.to_string())
                .with_line("Changed runtime facts", changed_fact_count.to_string())
                .with_optional_line(phase_execution.as_ref().map(phase_execution_summary_line)),
            ValidationReloadEvidenceEntry::SourceActivationDenied(stage) => {
                Self::with_value("Source activation denied", format!("{stage:?}"))
            }
            ValidationReloadEvidenceEntry::ThemeActivationDenied(stage) => {
                Self::with_value("Theme activation denied", format!("{stage:?}"))
            }
            ValidationReloadEvidenceEntry::ComponentActivationDenied(stage) => {
                Self::with_value("Component activation denied", format!("{stage:?}"))
            }
            ValidationReloadEvidenceEntry::CommandActivationDenied(stage) => {
                Self::with_value("Command activation denied", format!("{stage:?}"))
            }
            ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(stage) => {
                Self::with_value("Command projection activation denied", format!("{stage:?}"))
            }
            ValidationReloadEvidenceEntry::AppearanceActivationDenied(stage) => {
                Self::with_value("Appearance activation denied", format!("{stage:?}"))
            }
            ValidationReloadEvidenceEntry::DensityActivationDenied(stage) => {
                Self::with_value("Density activation denied", format!("{stage:?}"))
            }
            ValidationReloadEvidenceEntry::InputUnreadable(denial) => {
                Self::with_value("Reload input unreadable", denial.reason().to_owned())
                    .with_line("Path", denial.path().display().to_string())
            }
        };
        if let Some(header_rebind) = header_rebind(entry) {
            visible_entry = visible_entry
                .with_line("Header rebind", format!("{:?}", header_rebind.status()))
                .with_line(
                    "Header frame digest",
                    format!(
                        "{} -> {}",
                        header_rebind.previous_frame_digest(),
                        header_rebind.rebound_frame_digest()
                    ),
                )
                .with_line(
                    "Projection counters",
                    format!(
                        "inspected={} intersections={} rebuilds={} preserved={} denied={} rebuilt={}",
                        header_rebind.inspected_projection_count(),
                        header_rebind.dependency_intersection_count(),
                        header_rebind.rebuild_attempt_count(),
                        header_rebind.preserved_frame_count(),
                        header_rebind.denied_frame_count(),
                        header_rebind.rebuilt_frame_count()
                    ),
                );
            for row in header_rebind.rows() {
                visible_entry = visible_entry.with_line(
                    "Projection row",
                    format!(
                        "{} [{:?}] {:?} {} -> {}",
                        row.projection_identity(),
                        row.projection_family(),
                        row.status(),
                        row.previous_frame_digest(),
                        row.rebound_frame_digest()
                    ),
                );
            }
        }
        if let Some(page_host_rebind) = page_host_rebind(entry) {
            visible_entry = visible_entry
                .with_line(
                    "Page-host rebind",
                    format!("{:?}", page_host_rebind.status()),
                )
                .with_line(
                    "Page-host frame digest",
                    format!(
                        "{} -> {}",
                        page_host_rebind.previous_frame_digest(),
                        page_host_rebind.rebound_frame_digest()
                    ),
                )
                .with_line(
                    "Page-host projection counters",
                    format!(
                        "inspected={} intersections={} rebuilds={} preserved={} denied={} rebuilt={}",
                        page_host_rebind.inspected_projection_count(),
                        page_host_rebind.dependency_intersection_count(),
                        page_host_rebind.rebuild_attempt_count(),
                        page_host_rebind.preserved_frame_count(),
                        page_host_rebind.denied_frame_count(),
                        page_host_rebind.rebuilt_frame_count()
                    ),
                );
            for row in page_host_rebind.rows() {
                visible_entry = visible_entry.with_line(
                    "Page-host projection row",
                    format!(
                        "{} [{:?}] {:?} {} -> {}",
                        row.projection_identity(),
                        row.projection_family(),
                        row.status(),
                        row.previous_frame_digest(),
                        row.rebound_frame_digest()
                    ),
                );
            }
        }
        if let ValidationReloadEvidenceEntry::RuntimeReload {
            authored_structural: Some(authored_structural),
            ..
        }
        | ValidationReloadEvidenceEntry::AuthoredBatchReload {
            authored_structural: Some(authored_structural),
            ..
        } = entry
        {
            for row in authored_structural.rows() {
                visible_entry = visible_entry.with_line(
                    "Authored structural row",
                    format!(
                        "{:?} {} {:?} -> {}",
                        row.slice_id(),
                        row.subject_label(),
                        row.change_posture(),
                        row.changed_fact_labels().join(", ")
                    ),
                );
            }
        }
        visible_entry.structural_evidence = structural_visible_evidence(entry);
        visible_entry
    }

    fn with_value(heading: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            lines: vec![ValidationReloadEvidenceVisibleLine {
                label: "Value".to_owned(),
                value: value.into(),
            }],
            structural_evidence: None,
        }
    }

    fn with_line(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.lines.push(ValidationReloadEvidenceVisibleLine {
            label: label.into(),
            value: value.into(),
        });
        self
    }

    fn with_optional_line(self, line: Option<(&'static str, String)>) -> Self {
        match line {
            Some((label, value)) => self.with_line(label, value),
            None => self,
        }
    }

    pub fn heading(&self) -> &str {
        &self.heading
    }

    pub fn lines(&self) -> &[ValidationReloadEvidenceVisibleLine] {
        &self.lines
    }

    pub fn structural_evidence(&self) -> Option<&ValidationVisibleStructuralEvidence> {
        self.structural_evidence.as_ref()
    }
}

impl ValidationReloadEvidenceVisibleLine {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
