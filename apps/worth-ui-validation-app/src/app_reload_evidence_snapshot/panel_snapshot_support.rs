use crate::reload::ValidationReloadEvidenceEntry;

use super::structural_visible_evidence::ValidationVisibleStructuralEvidence;

pub(super) fn header_rebind(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<&crate::reload::ValidationHeaderRebindEvidence> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::AuthoredBatchReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::ThemeReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::CommandReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::ComponentReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::CommandProjectionReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::AppearanceReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::DensityReload { header_rebind, .. } => {
            header_rebind.as_ref()
        }
        ValidationReloadEvidenceEntry::ThemeDenied(_)
        | ValidationReloadEvidenceEntry::SourceActivationDenied(_)
        | ValidationReloadEvidenceEntry::ThemeActivationDenied(_)
        | ValidationReloadEvidenceEntry::ComponentActivationDenied(_)
        | ValidationReloadEvidenceEntry::CommandActivationDenied(_)
        | ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(_)
        | ValidationReloadEvidenceEntry::AppearanceActivationDenied(_)
        | ValidationReloadEvidenceEntry::DensityActivationDenied(_)
        | ValidationReloadEvidenceEntry::InputUnreadable(_) => None,
    }
}

pub(super) fn page_host_rebind(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<&crate::reload::ValidationPageHostRebindEvidence> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::AuthoredBatchReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::ThemeReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::CommandReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::ComponentReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::CommandProjectionReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::AppearanceReload {
            page_host_rebind, ..
        }
        | ValidationReloadEvidenceEntry::DensityReload {
            page_host_rebind, ..
        } => page_host_rebind.as_ref(),
        ValidationReloadEvidenceEntry::ThemeDenied(_)
        | ValidationReloadEvidenceEntry::SourceActivationDenied(_)
        | ValidationReloadEvidenceEntry::ThemeActivationDenied(_)
        | ValidationReloadEvidenceEntry::ComponentActivationDenied(_)
        | ValidationReloadEvidenceEntry::CommandActivationDenied(_)
        | ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(_)
        | ValidationReloadEvidenceEntry::AppearanceActivationDenied(_)
        | ValidationReloadEvidenceEntry::DensityActivationDenied(_)
        | ValidationReloadEvidenceEntry::InputUnreadable(_) => None,
    }
}

pub(super) fn structural_visible_evidence(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<ValidationVisibleStructuralEvidence> {
    let (authored_structural, header_rebind, page_host_rebind) = match entry {
        ValidationReloadEvidenceEntry::RuntimeReload {
            authored_structural,
            header_rebind,
            page_host_rebind,
            ..
        }
        | ValidationReloadEvidenceEntry::AuthoredBatchReload {
            authored_structural,
            header_rebind,
            page_host_rebind,
            ..
        } => (authored_structural, header_rebind, page_host_rebind),
        _ => return None,
    };
    Some(ValidationVisibleStructuralEvidence::new(
        authored_structural
            .as_ref()
            .map(|evidence| evidence.rows().to_vec())
            .unwrap_or_default(),
        header_rebind
            .as_ref()
            .map(|evidence| evidence.rows().to_vec())
            .unwrap_or_default(),
        page_host_rebind
            .as_ref()
            .map(|evidence| evidence.rows().to_vec())
            .unwrap_or_default(),
    ))
}

pub(super) fn phase_execution_summary_line(
    phase_execution: &crate::reload::ValidationPhaseExecutionEvidence,
) -> (&'static str, String) {
    (
        "Phase selection",
        format!(
            "rows={} skipped={} rebuilds={} digest={}",
            phase_execution.phase_row_count(),
            phase_execution.skipped_phase_count(),
            phase_execution.rebuild_attempt_count(),
            phase_execution.replay_digest()
        ),
    )
}

pub(super) fn authored_structural_summary_line(
    authored_structural: &crate::reload::ValidationAuthoredStructuralReloadEvidence,
) -> (&'static str, String) {
    (
        "Authored structural proof",
        format!(
            "rows={} digest={} previous_slots={} current_slots={}",
            authored_structural.rows().len(),
            authored_structural.authored_delta_digest(),
            authored_structural.previous_slots().len(),
            authored_structural.current_slots().len()
        ),
    )
}
