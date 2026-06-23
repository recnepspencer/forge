use std::collections::BTreeMap;

use crate::capability::{
    CapabilitySnapshot, ComponentAcceptedRegistrationProof, ComponentAccessibilitySupport,
    ComponentChildPolicy, ComponentDescriptor, ComponentExecutionLane, ComponentFocusSupport,
    ComponentId, ComponentPropSchema, ComponentStateOwnership, FrozenComponentCapabilities,
};
use crate::runtime::{WorthUiRuntimeFactId, WorthUiRuntimeFactSet};

use super::component_compatibility::{
    classify_component_compatibility, merge_component_compatibility,
};
use super::{
    WorthUiCapabilityFamilyDelta, WorthUiCapabilityReloadFamilyCounters,
    WorthUiCapabilityReloadFamilyKind, WorthUiCapabilityReloadStage, WorthUiComponentCompatibility,
    WorthUiComponentReloadPackage, WorthUiComponentReloadReceipt, WorthUiComponentShapeDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiComponentDelta {
    snapshot: CapabilitySnapshot,
    receipt: WorthUiComponentReloadReceipt,
    touched_component_count: usize,
    changed_component_count: usize,
    descriptor_lookup_count: usize,
    component_family_entry_count: usize,
    changed_facts: WorthUiRuntimeFactSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiComponentDeltaDenial {
    stage: WorthUiCapabilityReloadStage,
    detail: String,
    counters: WorthUiCapabilityReloadFamilyCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ComponentSourceAssignment {
    descriptor: ComponentDescriptor,
}

impl WorthUiComponentDelta {
    pub(crate) fn derive(
        active_snapshot: &CapabilitySnapshot,
        package: &WorthUiComponentReloadPackage,
    ) -> Result<Self, WorthUiComponentDeltaDenial> {
        let assignments = parse_assignments(package.source_text())?;
        let mut descriptors = active_snapshot.components().descriptors().to_vec();
        let indices = descriptors
            .iter()
            .enumerate()
            .map(|(index, descriptor)| (descriptor.id().clone(), index))
            .collect::<BTreeMap<_, _>>();

        let touched_component_count = assignments.len();
        let touched_component_ids = assignments
            .iter()
            .map(|assignment| assignment.descriptor.id().clone())
            .collect::<Vec<_>>();
        let mut changed_component_count = 0;
        let mut descriptor_lookup_count = 0;
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        let mut final_compatibility = WorthUiComponentCompatibility::Equivalent;

        for assignment in assignments {
            descriptor_lookup_count += 1;
            let component_id = assignment.descriptor.id().clone();
            let Some(index) = indices.get(&component_id).copied() else {
                return Err(admission_denial(
                    WorthUiComponentShapeDenial::MissingComponent(component_id),
                    touched_component_count,
                    descriptor_lookup_count,
                    descriptors.len(),
                ));
            };
            let compatibility =
                classify_component_compatibility(&descriptors[index], &assignment.descriptor)
                    .map_err(|denial| {
                        admission_denial(
                            denial,
                            touched_component_count,
                            descriptor_lookup_count,
                            descriptors.len(),
                        )
                    })?;
            if descriptors[index] != assignment.descriptor {
                changed_component_count += 1;
                changed_facts.insert(WorthUiRuntimeFactId::component(&component_id));
            }
            final_compatibility = merge_component_compatibility(final_compatibility, compatibility);
            descriptors[index] = assignment.descriptor;
        }

        let accepted = ComponentAcceptedRegistrationProof::from_identity_texts(
            descriptors
                .iter()
                .map(|descriptor| descriptor.id().as_str().to_owned())
                .collect(),
        );
        let components =
            FrozenComponentCapabilities::from_accepted_descriptors(descriptors, &accepted);
        let component_family_entry_count = components.len();

        Ok(Self {
            snapshot: active_snapshot.with_components_replaced(components),
            receipt: WorthUiComponentReloadReceipt::new(touched_component_ids, final_compatibility),
            touched_component_count,
            changed_component_count,
            descriptor_lookup_count,
            component_family_entry_count,
            changed_facts,
        })
    }

    pub(crate) fn into_family_delta(self) -> WorthUiCapabilityFamilyDelta {
        WorthUiCapabilityFamilyDelta::with_component_reload_receipt(
            WorthUiCapabilityReloadFamilyKind::Components,
            self.snapshot,
            WorthUiCapabilityReloadFamilyCounters::new(
                1,
                self.touched_component_count,
                self.touched_component_count,
                self.changed_component_count,
                self.component_family_entry_count,
                self.descriptor_lookup_count,
            ),
            self.changed_facts,
            Some(self.receipt),
        )
    }
}

impl WorthUiComponentDeltaDenial {
    pub(crate) fn stage(&self) -> WorthUiCapabilityReloadStage {
        self.stage
    }

    pub(crate) fn detail(&self) -> String {
        self.detail.clone()
    }

    pub(crate) fn counters(&self) -> WorthUiCapabilityReloadFamilyCounters {
        self.counters
    }
}

fn parse_assignments(
    source_text: &str,
) -> Result<Vec<ComponentSourceAssignment>, WorthUiComponentDeltaDenial> {
    let mut assignments = Vec::new();
    let mut block_lines = Vec::new();
    for line in source_text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            if !block_lines.is_empty() {
                assignments.push(ComponentSourceAssignment {
                    descriptor: parse_descriptor_block(&block_lines)?,
                });
                block_lines.clear();
            }
            continue;
        }
        block_lines.push(trimmed.to_owned());
    }
    if !block_lines.is_empty() {
        assignments.push(ComponentSourceAssignment {
            descriptor: parse_descriptor_block(&block_lines)?,
        });
    }
    Ok(assignments)
}

fn parse_descriptor_block(
    lines: &[String],
) -> Result<ComponentDescriptor, WorthUiComponentDeltaDenial> {
    let mut values = BTreeMap::<String, String>::new();
    for line in lines {
        let Some((key, value)) = line.split_once('=') else {
            return Err(parse_denial(
                "component block lines must be `key = value`".to_owned(),
            ));
        };
        values.insert(key.trim().to_owned(), value.trim().to_owned());
    }

    let id = ComponentId::new(
        values
            .get("component_id")
            .ok_or_else(|| parse_denial("component block requires `component_id`".to_owned()))?
            .as_str(),
    )
    .map_err(|_| parse_denial("component block has invalid `component_id`".to_owned()))?;
    let prop_schema = parse_prop_schema(&id, values.get("prop_schema"))?;
    let child_policy = parse_child_policy(values.get("child_policy"))?;
    let state_ownership = parse_state_ownership(&id, values.get("state_ownership"))?;
    let accessibility = parse_accessibility(values.get("accessibility"))?;
    let focus = parse_focus(values.get("focus"))?;
    let execution_lane = parse_execution_lane(values.get("execution_lane"))?;

    let mut descriptor = ComponentDescriptor::new(id, prop_schema, child_policy, state_ownership)
        .with_accessibility(accessibility)
        .with_focus(focus)
        .with_execution_lane(execution_lane);
    for token_id in parse_csv(values.get("theme_token_dependencies")) {
        descriptor = descriptor.with_theme_token_dependency(
            crate::capability::ThemeTokenId::new(&token_id).map_err(|_| {
                parse_denial(format!("invalid theme token dependency `{token_id}`"))
            })?,
        );
    }
    for command_id in parse_csv(values.get("command_binding_slots")) {
        descriptor = descriptor.with_command_binding_slot(
            crate::capability::CommandId::new(&command_id).map_err(|_| {
                parse_denial(format!("invalid command binding slot `{command_id}`"))
            })?,
        );
    }
    Ok(descriptor)
}

fn parse_prop_schema(
    component_id: &ComponentId,
    value: Option<&String>,
) -> Result<ComponentPropSchema, WorthUiComponentDeltaDenial> {
    let raw = value.ok_or_else(|| {
        parse_denial(format!(
            "component `{}` requires `prop_schema`",
            component_id.as_str()
        ))
    })?;
    if let Some(schema_key) = raw.strip_prefix("typed:") {
        return Ok(ComponentPropSchema::named(schema_key.trim()));
    }
    if let Some(schema_key) = raw.strip_prefix("untyped:") {
        return Ok(ComponentPropSchema::untyped_for_diagnostics(
            schema_key.trim(),
        ));
    }
    Ok(ComponentPropSchema::named(raw))
}

fn parse_child_policy(
    value: Option<&String>,
) -> Result<ComponentChildPolicy, WorthUiComponentDeltaDenial> {
    match value.map(String::as_str).unwrap_or("no_children") {
        "no_children" => Ok(ComponentChildPolicy::no_children()),
        "text_children" => Ok(ComponentChildPolicy::text_children()),
        "component_children" => Ok(ComponentChildPolicy::component_children()),
        "shell_layout_authority" => {
            Ok(ComponentChildPolicy::shell_layout_authority_for_diagnostics())
        }
        _ => Err(parse_denial("invalid `child_policy`".to_owned())),
    }
}

fn parse_state_ownership(
    component_id: &ComponentId,
    value: Option<&String>,
) -> Result<ComponentStateOwnership, WorthUiComponentDeltaDenial> {
    match value.map(String::as_str) {
        Some("runtime_owned") => Ok(ComponentStateOwnership::runtime_owned()),
        Some("component_local") => Ok(ComponentStateOwnership::component_local()),
        Some("stateless") => Ok(ComponentStateOwnership::stateless()),
        Some(_) => Err(parse_denial("invalid `state_ownership`".to_owned())),
        None => Err(parse_denial(format!(
            "component `{}` requires `state_ownership`",
            component_id.as_str()
        ))),
    }
}

fn parse_accessibility(
    value: Option<&String>,
) -> Result<ComponentAccessibilitySupport, WorthUiComponentDeltaDenial> {
    match value.map(String::as_str).unwrap_or("semantic") {
        "semantic" => Ok(ComponentAccessibilitySupport::semantic()),
        "decorative_only" => Ok(ComponentAccessibilitySupport::decorative_only()),
        _ => Err(parse_denial("invalid `accessibility`".to_owned())),
    }
}

fn parse_focus(
    value: Option<&String>,
) -> Result<ComponentFocusSupport, WorthUiComponentDeltaDenial> {
    match value.map(String::as_str).unwrap_or("not_focusable") {
        "not_focusable" => Ok(ComponentFocusSupport::not_focusable()),
        "focusable" => Ok(ComponentFocusSupport::focusable()),
        "focus_container" => Ok(ComponentFocusSupport::focus_container()),
        _ => Err(parse_denial("invalid `focus`".to_owned())),
    }
}

fn parse_execution_lane(
    value: Option<&String>,
) -> Result<ComponentExecutionLane, WorthUiComponentDeltaDenial> {
    match value.map(String::as_str).unwrap_or("passive") {
        "passive" => Ok(ComponentExecutionLane::Passive),
        "interactive" => Ok(ComponentExecutionLane::Interactive),
        "virtualized" => Ok(ComponentExecutionLane::Virtualized),
        _ => Err(parse_denial("invalid `execution_lane`".to_owned())),
    }
}

fn parse_csv(value: Option<&String>) -> Vec<String> {
    value
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn parse_denial(detail: String) -> WorthUiComponentDeltaDenial {
    WorthUiComponentDeltaDenial {
        stage: WorthUiCapabilityReloadStage::ComponentSourceParse,
        detail,
        counters: WorthUiCapabilityReloadFamilyCounters::new(1, 0, 0, 0, 0, 0),
    }
}

fn admission_denial(
    denial: WorthUiComponentShapeDenial,
    touched_component_count: usize,
    descriptor_lookup_count: usize,
    component_family_entry_count: usize,
) -> WorthUiComponentDeltaDenial {
    WorthUiComponentDeltaDenial {
        stage: WorthUiCapabilityReloadStage::ComponentAdmission,
        detail: denial.detail(),
        counters: WorthUiCapabilityReloadFamilyCounters::new(
            1,
            touched_component_count,
            touched_component_count,
            0,
            component_family_entry_count,
            descriptor_lookup_count,
        ),
    }
}
