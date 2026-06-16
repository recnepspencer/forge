use std::collections::{BTreeMap, BTreeSet};

use crate::capability::{
    CapabilitySnapshot, CommandAcceptedRegistrationProof, CommandDescriptor, CommandId,
    FrozenCommandCapabilities,
};
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::{WorthUiCapabilityReloadStage, WorthUiCommandReloadPackage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCommandDelta {
    snapshot: CapabilitySnapshot,
    touched_command_count: usize,
    registry_lookup_count: usize,
    command_family_entry_count: usize,
    changed_facts: WorthUiRuntimeFactSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCommandDeltaDenial {
    stage: WorthUiCapabilityReloadStage,
    detail: String,
}

impl WorthUiCommandDelta {
    pub(crate) fn derive(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiCommandReloadPackage,
    ) -> Result<Self, WorthUiCommandDeltaDenial> {
        let parsed_commands = parse_command_label_assignments(package.source_text())?;
        let descriptors = replacement_descriptors(active_snapshot, &parsed_commands)?;
        let command_family_entry_count = descriptors.len();
        let accepted = CommandAcceptedRegistrationProof::from_identity_texts(
            descriptors
                .iter()
                .map(|descriptor| descriptor.id().as_str().to_owned())
                .collect(),
        );
        let commands = FrozenCommandCapabilities::from_accepted_descriptors(descriptors, &accepted);
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        changed_facts.extend(
            parsed_commands
                .iter()
                .map(|(command_id, _)| WorthUiRuntimeFactId::command(command_id)),
        );
        Ok(Self {
            snapshot: active_snapshot.with_commands_replaced(commands),
            touched_command_count: parsed_commands.len(),
            registry_lookup_count: parsed_commands.len(),
            command_family_entry_count,
            changed_facts,
        })
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        CapabilitySnapshot,
        usize,
        usize,
        usize,
        WorthUiRuntimeFactSet,
    ) {
        (
            self.snapshot,
            self.touched_command_count,
            self.command_family_entry_count,
            self.registry_lookup_count,
            self.changed_facts,
        )
    }
}

impl WorthUiCommandDeltaDenial {
    pub(crate) fn stage(&self) -> WorthUiCapabilityReloadStage {
        self.stage
    }

    pub(crate) fn detail(self) -> String {
        self.detail
    }
}

fn parse_command_label_assignments(
    source_text: &str,
) -> Result<Vec<(CommandId, String)>, WorthUiCommandDeltaDenial> {
    let mut seen = BTreeSet::new();
    let mut assignments = Vec::new();
    for (line_index, line) in source_text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((raw_id, raw_label)) = trimmed.split_once('=') else {
            return Err(parse_denial(format!(
                "line {} is not `command.id = label`",
                line_index + 1
            )));
        };
        let command_id = CommandId::new(raw_id.trim())
            .map_err(|_| parse_denial(format!("line {} has invalid command id", line_index + 1)))?;
        if !seen.insert(command_id.clone()) {
            return Err(parse_denial(format!("duplicate command `{command_id}`")));
        }
        let label = raw_label.trim().trim_matches('"').to_owned();
        if label.is_empty() {
            return Err(parse_denial(format!(
                "command `{command_id}` cannot have an empty label"
            )));
        }
        assignments.push((command_id, label));
    }
    Ok(assignments)
}

fn replacement_descriptors(
    active_snapshot: &CapabilitySnapshot,
    parsed_commands: &[(CommandId, String)],
) -> Result<Vec<CommandDescriptor>, WorthUiCommandDeltaDenial> {
    let mut descriptors = active_snapshot.commands().descriptors().to_vec();
    let command_indices = descriptors
        .iter()
        .enumerate()
        .map(|(index, descriptor)| (descriptor.id().clone(), index))
        .collect::<BTreeMap<_, _>>();
    for (command_id, label) in parsed_commands {
        let Some(index) = command_indices.get(command_id).copied() else {
            return Err(admission_denial(format!("unknown command `{command_id}`")));
        };
        descriptors[index] = descriptors[index].with_label_replaced(label.clone());
    }
    Ok(descriptors)
}

fn parse_denial(detail: String) -> WorthUiCommandDeltaDenial {
    WorthUiCommandDeltaDenial {
        stage: WorthUiCapabilityReloadStage::CommandSourceParse,
        detail,
    }
}

fn admission_denial(detail: String) -> WorthUiCommandDeltaDenial {
    WorthUiCommandDeltaDenial {
        stage: WorthUiCapabilityReloadStage::CommandAdmission,
        detail,
    }
}
