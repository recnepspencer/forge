use std::collections::{BTreeMap, BTreeSet};

use crate::capability::{
    CapabilitySnapshot, CommandProjectionAcceptedRegistrationProof, CommandProjectionDescriptor,
    CommandProjectionId, CommandProjectionSelectionMode, FrozenCommandProjectionCapabilities,
};
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::{WorthUiCapabilityReloadStage, WorthUiCommandProjectionReloadPackage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCommandProjectionDelta {
    snapshot: CapabilitySnapshot,
    touched_projection_count: usize,
    registry_lookup_count: usize,
    projection_family_entry_count: usize,
    changed_facts: WorthUiRuntimeFactSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiCommandProjectionDeltaDenial {
    stage: WorthUiCapabilityReloadStage,
    detail: String,
}

impl WorthUiCommandProjectionDelta {
    pub(crate) fn derive(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiCommandProjectionReloadPackage,
    ) -> Result<Self, WorthUiCommandProjectionDeltaDenial> {
        let parsed_projections = parse_projection_selection_assignments(package.source_text())?;
        let descriptors = replacement_descriptors(active_snapshot, &parsed_projections)?;
        let projection_family_entry_count = descriptors.len();
        let accepted = CommandProjectionAcceptedRegistrationProof::from_identity_texts(
            descriptors
                .iter()
                .map(|descriptor| descriptor.id().as_str().to_owned())
                .collect(),
        );
        let projections =
            FrozenCommandProjectionCapabilities::from_accepted_descriptors(descriptors, &accepted);
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        for (projection_id, _) in &parsed_projections {
            changed_facts.insert(WorthUiRuntimeFactId::command_projection(projection_id));
            changed_facts.insert(WorthUiRuntimeFactId::command_projection_interaction_policy(
                projection_id,
            ));
        }
        Ok(Self {
            snapshot: active_snapshot.with_command_projections_replaced(projections),
            touched_projection_count: parsed_projections.len(),
            registry_lookup_count: parsed_projections.len(),
            projection_family_entry_count,
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
            self.touched_projection_count,
            self.projection_family_entry_count,
            self.registry_lookup_count,
            self.changed_facts,
        )
    }
}

impl WorthUiCommandProjectionDeltaDenial {
    pub(crate) fn stage(&self) -> WorthUiCapabilityReloadStage {
        self.stage
    }

    pub(crate) fn detail(self) -> String {
        self.detail
    }
}

fn parse_projection_selection_assignments(
    source_text: &str,
) -> Result<
    Vec<(CommandProjectionId, CommandProjectionSelectionMode)>,
    WorthUiCommandProjectionDeltaDenial,
> {
    let mut seen = BTreeSet::new();
    let mut assignments = Vec::new();
    for (line_index, line) in source_text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((raw_id, raw_mode)) = trimmed.split_once('=') else {
            return Err(parse_denial(format!(
                "line {} is not `projection.id = single|multi`",
                line_index + 1
            )));
        };
        let projection_id = CommandProjectionId::new(raw_id.trim()).map_err(|_| {
            parse_denial(format!("line {} has invalid projection id", line_index + 1))
        })?;
        if !seen.insert(projection_id.clone()) {
            return Err(parse_denial(format!(
                "duplicate command projection `{projection_id}`"
            )));
        }
        assignments.push((projection_id, selection_mode(raw_mode.trim())?));
    }
    Ok(assignments)
}

fn replacement_descriptors(
    active_snapshot: &CapabilitySnapshot,
    parsed_projections: &[(CommandProjectionId, CommandProjectionSelectionMode)],
) -> Result<Vec<CommandProjectionDescriptor>, WorthUiCommandProjectionDeltaDenial> {
    let mut descriptors = active_snapshot
        .command_projections()
        .entries()
        .iter()
        .map(|entry| entry.descriptor().clone())
        .collect::<Vec<_>>();
    let projection_indices = descriptors
        .iter()
        .enumerate()
        .map(|(index, descriptor)| (descriptor.id().clone(), index))
        .collect::<BTreeMap<_, _>>();
    for (projection_id, selection_mode) in parsed_projections {
        let Some(index) = projection_indices.get(projection_id).copied() else {
            return Err(admission_denial(format!(
                "unknown command projection `{projection_id}`"
            )));
        };
        descriptors[index] = descriptors[index]
            .clone()
            .with_selection_mode(*selection_mode);
    }
    Ok(descriptors)
}

fn selection_mode(
    raw_mode: &str,
) -> Result<CommandProjectionSelectionMode, WorthUiCommandProjectionDeltaDenial> {
    match raw_mode {
        "single" | "single_select" => Ok(CommandProjectionSelectionMode::SingleSelect),
        "multi" | "multi_select" => Ok(CommandProjectionSelectionMode::MultiSelect),
        _ => Err(parse_denial(format!(
            "unknown command projection selection mode `{raw_mode}`"
        ))),
    }
}

fn parse_denial(detail: String) -> WorthUiCommandProjectionDeltaDenial {
    WorthUiCommandProjectionDeltaDenial {
        stage: WorthUiCapabilityReloadStage::CommandProjectionSourceParse,
        detail,
    }
}

fn admission_denial(detail: String) -> WorthUiCommandProjectionDeltaDenial {
    WorthUiCommandProjectionDeltaDenial {
        stage: WorthUiCapabilityReloadStage::CommandProjectionAdmission,
        detail,
    }
}
