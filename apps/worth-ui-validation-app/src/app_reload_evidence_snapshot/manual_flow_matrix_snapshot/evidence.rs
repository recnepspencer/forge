use worth_ui::facade::{WorthUiProjectionRebindStatus, WorthUiRuntimeFactId};

use crate::reload::{
    ValidationHeaderRebindEvidence, ValidationPageHostRebindEvidence, ValidationReloadEvidenceEntry,
};
use crate::ValidationAppProofSnapshot;

pub(super) fn observed_changed_facts(entry: &ValidationReloadEvidenceEntry) -> Vec<String> {
    match entry {
        ValidationReloadEvidenceEntry::RuntimeReload { changed_facts, .. } => {
            changed_facts.iter().map(runtime_fact_label).collect()
        }
        ValidationReloadEvidenceEntry::AuthoredBatchReload { runtime_change, .. } => runtime_change
            .rows()
            .iter()
            .flat_map(|row| row.changed_facts().iter())
            .map(runtime_fact_label)
            .collect(),
        ValidationReloadEvidenceEntry::ThemeReload { changed_facts, .. }
        | ValidationReloadEvidenceEntry::CommandReload { changed_facts, .. }
        | ValidationReloadEvidenceEntry::ComponentReload { changed_facts, .. }
        | ValidationReloadEvidenceEntry::CommandProjectionReload { changed_facts, .. }
        | ValidationReloadEvidenceEntry::AppearanceReload { changed_facts, .. }
        | ValidationReloadEvidenceEntry::DensityReload { changed_facts, .. } => {
            changed_facts.iter().map(runtime_fact_label).collect()
        }
        ValidationReloadEvidenceEntry::ThemeDenied(_)
        | ValidationReloadEvidenceEntry::SourceActivationDenied(_)
        | ValidationReloadEvidenceEntry::ThemeActivationDenied(_)
        | ValidationReloadEvidenceEntry::ComponentActivationDenied(_)
        | ValidationReloadEvidenceEntry::CommandActivationDenied(_)
        | ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(_)
        | ValidationReloadEvidenceEntry::AppearanceActivationDenied(_)
        | ValidationReloadEvidenceEntry::DensityActivationDenied(_)
        | ValidationReloadEvidenceEntry::InputUnreadable(_) => Vec::new(),
    }
}

pub(super) fn observed_projection_sets(
    entry: &ValidationReloadEvidenceEntry,
) -> (Vec<String>, Vec<String>) {
    let mut rebuilt = Vec::new();
    let mut preserved = Vec::new();
    if let Some(header) = header_rebind(entry) {
        collect_projection_rows(
            header
                .rows()
                .iter()
                .map(|row| (row.projection_identity(), row.status())),
            &mut rebuilt,
            &mut preserved,
        );
    }
    if let Some(page_host) = page_host_rebind(entry) {
        collect_projection_rows(
            page_host
                .rows()
                .iter()
                .map(|row| (row.projection_identity(), row.status())),
            &mut rebuilt,
            &mut preserved,
        );
    }
    rebuilt.sort();
    rebuilt.dedup();
    preserved.sort();
    preserved.dedup();
    (rebuilt, preserved)
}

pub(super) fn header_rebind(
    entry: &ValidationReloadEvidenceEntry,
) -> Option<&ValidationHeaderRebindEvidence> {
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
) -> Option<&ValidationPageHostRebindEvidence> {
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

pub(super) fn runtime_fact_label(fact: &WorthUiRuntimeFactId) -> String {
    format!("{:?}({})", fact.family(), fact.identity())
}

pub(super) fn command_label(proof: &ValidationAppProofSnapshot, command_id: &str) -> String {
    file_menu(proof)
        .and_then(|menu| {
            menu.commands()
                .iter()
                .find(|command| command.command_id() == command_id)
        })
        .map(|command| command.label().to_owned())
        .unwrap_or_else(|| format!("Missing command `{command_id}`"))
}

pub(super) fn file_menu(
    proof: &ValidationAppProofSnapshot,
) -> Option<&crate::ValidationHeaderMenuProofSnapshot> {
    proof
        .header()
        .menus()
        .iter()
        .find(|menu| menu.title() == "File")
}

fn collect_projection_rows<'a>(
    rows: impl Iterator<Item = (&'a str, WorthUiProjectionRebindStatus)>,
    rebuilt: &mut Vec<String>,
    preserved: &mut Vec<String>,
) {
    for (projection_identity, status) in rows {
        match status {
            WorthUiProjectionRebindStatus::ReboundAfterActivation => {
                rebuilt.push(projection_identity.to_owned());
            }
            WorthUiProjectionRebindStatus::PreservedEquivalentReload
            | WorthUiProjectionRebindStatus::PreservedDeniedReload
            | WorthUiProjectionRebindStatus::DeniedReloadNotActivated
            | WorthUiProjectionRebindStatus::EquivalentAfterActivation => {
                preserved.push(projection_identity.to_owned());
            }
        }
    }
}
