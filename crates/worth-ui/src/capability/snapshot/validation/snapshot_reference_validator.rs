use crate::capability::{
    CapabilitySnapshotFreezeInput, PluginSlotSupportPosture, COMMAND_FAMILY_NAME,
    COMMAND_PROJECTION_FAMILY_NAME, COMPONENT_FAMILY_NAME, ICON_FAMILY_NAME,
    PLUGIN_SLOT_FAMILY_NAME, THEME_TOKEN_FAMILY_NAME,
};

use super::{
    SnapshotReferenceValidationReport, SnapshotReferenceViolation, SnapshotReferenceViolationKind,
};

pub(crate) fn validate_snapshot_references(
    input: &CapabilitySnapshotFreezeInput,
) -> SnapshotReferenceValidationReport {
    let mut violations = Vec::new();
    collect_command_reference_violations(input, &mut violations);
    collect_component_reference_violations(input, &mut violations);
    collect_deferred_plugin_slot_reference_violations(input, &mut violations);
    SnapshotReferenceValidationReport::new(violations)
}

fn collect_command_reference_violations(
    input: &CapabilitySnapshotFreezeInput,
    violations: &mut Vec<SnapshotReferenceViolation>,
) {
    for command in input.commands.descriptors() {
        if let Some(icon_id) = command.icon() {
            collect_missing_reference_violation(
                violations,
                input.icons.get(icon_id).is_none(),
                COMMAND_FAMILY_NAME,
                command.id().as_str(),
                ICON_FAMILY_NAME,
                icon_id.as_str(),
            );
        }

        if let Some(projection_id) = command.projection_eligibility() {
            collect_missing_reference_violation(
                violations,
                input.command_projections.get(projection_id).is_none(),
                COMMAND_FAMILY_NAME,
                command.id().as_str(),
                COMMAND_PROJECTION_FAMILY_NAME,
                projection_id.as_str(),
            );
        }
    }
}

fn collect_component_reference_violations(
    input: &CapabilitySnapshotFreezeInput,
    violations: &mut Vec<SnapshotReferenceViolation>,
) {
    for component in input.components.descriptors() {
        for token_id in component.theme_token_dependencies() {
            collect_missing_reference_violation(
                violations,
                input.theme_tokens.get(token_id).is_none(),
                COMPONENT_FAMILY_NAME,
                component.id().as_str(),
                THEME_TOKEN_FAMILY_NAME,
                token_id.as_str(),
            );
        }

        for command_id in component.command_binding_slots() {
            collect_missing_reference_violation(
                violations,
                input.commands.get(command_id).is_none(),
                COMPONENT_FAMILY_NAME,
                component.id().as_str(),
                COMMAND_FAMILY_NAME,
                command_id.as_str(),
            );
        }
    }
}

fn collect_deferred_plugin_slot_reference_violations(
    input: &CapabilitySnapshotFreezeInput,
    violations: &mut Vec<SnapshotReferenceViolation>,
) {
    for entry in input.plugin_slots.entries() {
        let Some(reference) = entry.descriptor().contribution_reference() else {
            continue;
        };
        let Some(referenced_slot) = input.plugin_slots.get(reference.slot_id()) else {
            continue;
        };
        if referenced_slot.support() == Some(PluginSlotSupportPosture::deferred()) {
            violations.push(SnapshotReferenceViolation::new(
                SnapshotReferenceViolationKind::DeferredEntryUsedAsAdmitted,
                PLUGIN_SLOT_FAMILY_NAME,
                entry.descriptor().id().as_str(),
                PLUGIN_SLOT_FAMILY_NAME,
                reference.slot_id().as_str(),
            ));
        }
    }
}

fn collect_missing_reference_violation(
    violations: &mut Vec<SnapshotReferenceViolation>,
    is_missing: bool,
    source_family_name: &'static str,
    source_identity_text: &str,
    target_family_name: &'static str,
    target_identity_text: &str,
) {
    if is_missing {
        violations.push(SnapshotReferenceViolation::new(
            SnapshotReferenceViolationKind::MissingCrossFamilyReference,
            source_family_name,
            source_identity_text,
            target_family_name,
            target_identity_text,
        ));
    }
}
