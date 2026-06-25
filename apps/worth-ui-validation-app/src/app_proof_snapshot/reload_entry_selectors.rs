use crate::reload::{
    ValidationAuthoredStructuralReloadEvidence, ValidationHeaderRebindEvidence,
    ValidationPageHostRebindEvidence, ValidationPhaseExecutionEvidence,
    ValidationReloadEvidenceEntry,
};

pub(super) fn latest_header_rebind(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<ValidationHeaderRebindEvidence> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::AuthoredBatchReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::ThemeReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::CommandReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::ComponentReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::CommandProjectionReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::AppearanceReload { header_rebind, .. }
        | ValidationReloadEvidenceEntry::DensityReload { header_rebind, .. } => {
            header_rebind.clone()
        }
        _ => None,
    }
}

pub(super) fn latest_page_host_rebind(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<ValidationPageHostRebindEvidence> {
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
        } => page_host_rebind.clone(),
        _ => None,
    }
}

pub(super) fn latest_phase_execution(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<ValidationPhaseExecutionEvidence> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload {
            phase_execution, ..
        }
        | ValidationReloadEvidenceEntry::AuthoredBatchReload {
            phase_execution, ..
        }
        | ValidationReloadEvidenceEntry::ThemeReload {
            phase_execution, ..
        }
        | ValidationReloadEvidenceEntry::CommandReload {
            phase_execution, ..
        }
        | ValidationReloadEvidenceEntry::ComponentReload {
            phase_execution, ..
        }
        | ValidationReloadEvidenceEntry::CommandProjectionReload {
            phase_execution, ..
        }
        | ValidationReloadEvidenceEntry::AppearanceReload {
            phase_execution, ..
        }
        | ValidationReloadEvidenceEntry::DensityReload {
            phase_execution, ..
        } => phase_execution.clone(),
        _ => None,
    }
}

pub(super) fn latest_authored_structural(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<ValidationAuthoredStructuralReloadEvidence> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload {
            authored_structural,
            ..
        }
        | ValidationReloadEvidenceEntry::AuthoredBatchReload {
            authored_structural,
            ..
        } => authored_structural.clone(),
        _ => None,
    }
}
