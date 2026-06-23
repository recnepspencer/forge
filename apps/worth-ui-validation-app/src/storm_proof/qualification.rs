use worth_ui::facade::{
    WorthUiCapabilityReloadStatus, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
};

use crate::reload::{ValidationReloadEvidenceEntry, ValidationReloadStatus};

use super::types::{
    ValidationMixedReloadStormBuildDenial, ValidationMixedReloadStormFamily,
    ValidationMixedReloadStormStatus, ValidationMixedReloadStormStep,
};

pub(super) fn latest_qualified_steps(
    entries: &[ValidationReloadEvidenceEntry],
) -> Result<Vec<ValidationMixedReloadStormStep>, ValidationMixedReloadStormBuildDenial> {
    for start in (0..entries.len()).rev() {
        let steps = entries[start..]
            .iter()
            .filter_map(ValidationMixedReloadStormStep::from_entry)
            .collect::<Vec<_>>();
        if qualifies_mixed_product_storm(&steps) {
            return Ok(steps);
        }
    }
    Err(ValidationMixedReloadStormBuildDenial::ScenarioNotQualified)
}

fn qualifies_mixed_product_storm(steps: &[ValidationMixedReloadStormStep]) -> bool {
    let mut activated_count = 0usize;
    let mut equivalent_count = 0usize;
    let mut denied_count = 0usize;
    let mut has_source_runtime_meaning_change = false;
    let mut has_command_change = false;
    let mut has_command_projection_change = false;
    let mut has_component_change = false;
    let mut has_appearance_change = false;
    let mut has_density_row = false;
    for step in steps {
        if matches!(
            step.family(),
            ValidationMixedReloadStormFamily::Theme | ValidationMixedReloadStormFamily::Input
        ) {
            return false;
        }
        match step.status() {
            ValidationMixedReloadStormStatus::Activated => activated_count += 1,
            ValidationMixedReloadStormStatus::EquivalentNoOp => equivalent_count += 1,
            ValidationMixedReloadStormStatus::Denied => denied_count += 1,
            ValidationMixedReloadStormStatus::ReadyForFrameBoundary => {}
        }
        match (step.family(), step.status()) {
            (
                ValidationMixedReloadStormFamily::Source,
                ValidationMixedReloadStormStatus::Activated,
            ) if step.changed_facts().iter().any(|fact| {
                matches!(
                    fact.family(),
                    WorthUiRuntimeFactFamily::SurfaceMount
                        | WorthUiRuntimeFactFamily::PrimitiveInteraction
                )
            }) =>
            {
                has_source_runtime_meaning_change = true;
            }
            (
                ValidationMixedReloadStormFamily::Command,
                ValidationMixedReloadStormStatus::Activated,
            ) if !step.changed_facts().is_empty() => {
                has_command_change = true;
            }
            (
                ValidationMixedReloadStormFamily::CommandProjection,
                ValidationMixedReloadStormStatus::Activated,
            ) if !step.changed_facts().is_empty() => {
                has_command_projection_change = true;
            }
            (
                ValidationMixedReloadStormFamily::Component,
                ValidationMixedReloadStormStatus::Activated,
            ) if !step.changed_facts().is_empty() => {
                has_component_change = true;
            }
            (
                ValidationMixedReloadStormFamily::Appearance,
                ValidationMixedReloadStormStatus::Activated,
            ) if !step.changed_facts().is_empty() => {
                has_appearance_change = true;
            }
            (
                ValidationMixedReloadStormFamily::Density,
                ValidationMixedReloadStormStatus::Activated
                | ValidationMixedReloadStormStatus::EquivalentNoOp,
            ) => {
                has_density_row = true;
            }
            _ => {}
        }
    }
    activated_count > 0
        && equivalent_count > 0
        && denied_count > 0
        && has_source_runtime_meaning_change
        && has_command_change
        && has_command_projection_change
        && has_component_change
        && has_appearance_change
        && has_density_row
}

impl ValidationMixedReloadStormStep {
    fn from_entry(entry: &ValidationReloadEvidenceEntry) -> Option<Self> {
        match entry {
            ValidationReloadEvidenceEntry::RuntimeReload {
                status,
                changed_facts,
                header_rebind,
                page_host_rebind,
                ..
            } => Some(Self::new(
                ValidationMixedReloadStormFamily::Source,
                source_status(*status),
                changed_facts.clone(),
                None,
                header_rebind.clone(),
                page_host_rebind.clone(),
            )),
            ValidationReloadEvidenceEntry::AuthoredBatchReload { .. } => None,
            ValidationReloadEvidenceEntry::ThemeReload {
                status,
                changed_facts,
                header_rebind,
                page_host_rebind,
                ..
            } => Some(Self::capability_step(
                ValidationMixedReloadStormFamily::Theme,
                *status,
                changed_facts,
                header_rebind.clone(),
                page_host_rebind.clone(),
            )),
            ValidationReloadEvidenceEntry::CommandReload {
                status,
                changed_facts,
                header_rebind,
                page_host_rebind,
                ..
            } => Some(Self::capability_step(
                ValidationMixedReloadStormFamily::Command,
                *status,
                changed_facts,
                header_rebind.clone(),
                page_host_rebind.clone(),
            )),
            ValidationReloadEvidenceEntry::ComponentReload {
                status,
                changed_facts,
                header_rebind,
                page_host_rebind,
                ..
            } => Some(Self::capability_step(
                ValidationMixedReloadStormFamily::Component,
                *status,
                changed_facts,
                header_rebind.clone(),
                page_host_rebind.clone(),
            )),
            ValidationReloadEvidenceEntry::CommandProjectionReload {
                status,
                changed_facts,
                header_rebind,
                page_host_rebind,
                ..
            } => Some(Self::capability_step(
                ValidationMixedReloadStormFamily::CommandProjection,
                *status,
                changed_facts,
                header_rebind.clone(),
                page_host_rebind.clone(),
            )),
            ValidationReloadEvidenceEntry::AppearanceReload {
                status,
                changed_facts,
                header_rebind,
                page_host_rebind,
                ..
            } => Some(Self::capability_step(
                ValidationMixedReloadStormFamily::Appearance,
                *status,
                changed_facts,
                header_rebind.clone(),
                page_host_rebind.clone(),
            )),
            ValidationReloadEvidenceEntry::DensityReload {
                status,
                changed_facts,
                header_rebind,
                page_host_rebind,
                ..
            } => Some(Self::capability_step(
                ValidationMixedReloadStormFamily::Density,
                *status,
                changed_facts,
                header_rebind.clone(),
                page_host_rebind.clone(),
            )),
            ValidationReloadEvidenceEntry::ThemeDenied(denial) => Some(Self::denied_input(
                ValidationMixedReloadStormFamily::Theme,
                format!("{:?}", denial.reason()),
            )),
            ValidationReloadEvidenceEntry::SourceActivationDenied(stage) => {
                Some(Self::denied_input(
                    ValidationMixedReloadStormFamily::Source,
                    format!("{stage:?}"),
                ))
            }
            ValidationReloadEvidenceEntry::ThemeActivationDenied(stage) => {
                Some(Self::denied_input(
                    ValidationMixedReloadStormFamily::Theme,
                    format!("{stage:?}"),
                ))
            }
            ValidationReloadEvidenceEntry::ComponentActivationDenied(stage) => {
                Some(Self::denied_input(
                    ValidationMixedReloadStormFamily::Component,
                    format!("{stage:?}"),
                ))
            }
            ValidationReloadEvidenceEntry::CommandActivationDenied(stage) => {
                Some(Self::denied_input(
                    ValidationMixedReloadStormFamily::Command,
                    format!("{stage:?}"),
                ))
            }
            ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(stage) => {
                Some(Self::denied_input(
                    ValidationMixedReloadStormFamily::CommandProjection,
                    format!("{stage:?}"),
                ))
            }
            ValidationReloadEvidenceEntry::AppearanceActivationDenied(stage) => {
                Some(Self::denied_input(
                    ValidationMixedReloadStormFamily::Appearance,
                    format!("{stage:?}"),
                ))
            }
            ValidationReloadEvidenceEntry::DensityActivationDenied(stage) => {
                Some(Self::denied_input(
                    ValidationMixedReloadStormFamily::Density,
                    format!("{stage:?}"),
                ))
            }
            ValidationReloadEvidenceEntry::InputUnreadable(denial) => Some(Self::new(
                ValidationMixedReloadStormFamily::Input,
                ValidationMixedReloadStormStatus::Denied,
                Vec::new(),
                Some(format!("{}: {}", denial.path().display(), denial.reason())),
                None,
                None,
            )),
        }
    }

    fn capability_step(
        family: ValidationMixedReloadStormFamily,
        status: WorthUiCapabilityReloadStatus,
        changed_facts: &[WorthUiRuntimeFactId],
        header_rebind: Option<crate::reload::ValidationHeaderRebindEvidence>,
        page_host_rebind: Option<crate::reload::ValidationPageHostRebindEvidence>,
    ) -> Self {
        Self::new(
            family,
            capability_status(status),
            changed_facts.to_vec(),
            None,
            header_rebind,
            page_host_rebind,
        )
    }

    fn denied_input(family: ValidationMixedReloadStormFamily, detail: String) -> Self {
        Self::new(
            family,
            ValidationMixedReloadStormStatus::Denied,
            Vec::new(),
            Some(detail),
            None,
            None,
        )
    }
}

fn source_status(status: ValidationReloadStatus) -> ValidationMixedReloadStormStatus {
    match status {
        ValidationReloadStatus::Activated => ValidationMixedReloadStormStatus::Activated,
        ValidationReloadStatus::EquivalentNoOp => ValidationMixedReloadStormStatus::EquivalentNoOp,
        ValidationReloadStatus::ReadyForFrameBoundary => {
            ValidationMixedReloadStormStatus::ReadyForFrameBoundary
        }
        ValidationReloadStatus::Denied(_) => ValidationMixedReloadStormStatus::Denied,
    }
}

fn capability_status(status: WorthUiCapabilityReloadStatus) -> ValidationMixedReloadStormStatus {
    match status {
        WorthUiCapabilityReloadStatus::Activated => ValidationMixedReloadStormStatus::Activated,
        WorthUiCapabilityReloadStatus::EquivalentNoOp => {
            ValidationMixedReloadStormStatus::EquivalentNoOp
        }
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => {
            ValidationMixedReloadStormStatus::ReadyForFrameBoundary
        }
        WorthUiCapabilityReloadStatus::Denied(_) => ValidationMixedReloadStormStatus::Denied,
    }
}
