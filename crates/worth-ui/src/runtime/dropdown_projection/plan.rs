use crate::capability::{
    CapabilitySnapshot, CommandId, CommandProjectionId, CommandProjectionSelectionMode, ComponentId,
};
use crate::runtime::{
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionDependencySet,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiProjectionPlanContract, WorthUiRuntimeFactId,
};

use super::{
    WorthUiDropdownAppearanceFrameReceipt, WorthUiDropdownAppearancePlanDenial,
    WorthUiDropdownCommand, WorthUiDropdownProjectionRequest, WorthUiDropdownSelectionState,
    WorthUiDropdownSelectionStateReconciliationReceipt, WorthUiDropdownSelectionStateStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownProjectionPlan {
    receipt: WorthUiDropdownFrameReceipt,
    projection_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownFrameReceipt {
    projection_id: String,
    component_id: String,
    selection_mode: CommandProjectionSelectionMode,
    commands: Vec<WorthUiDropdownCommand>,
    appearance: WorthUiDropdownAppearanceFrameReceipt,
    selection_state: WorthUiDropdownSelectionState,
    reconciliation: WorthUiDropdownSelectionStateReconciliationReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownProjectionPlanDenial {
    MissingProjection(String),
    EmptyProjection(String),
    ProjectionReferencesMissingCommand {
        projection_id: String,
        command_id: String,
    },
    MissingComponent(String),
    Appearance(WorthUiDropdownAppearancePlanDenial),
}

impl WorthUiDropdownProjectionPlan {
    pub fn from_snapshot(
        snapshot: &CapabilitySnapshot,
        request: WorthUiDropdownProjectionRequest,
    ) -> Result<Self, WorthUiDropdownProjectionPlanDenial> {
        Self::build(snapshot, request, None)
    }

    pub fn rebuild_from_snapshot(
        snapshot: &CapabilitySnapshot,
        request: WorthUiDropdownProjectionRequest,
        previous_selection_state: Option<&WorthUiDropdownSelectionState>,
    ) -> Result<Self, WorthUiDropdownProjectionPlanDenial> {
        Self::build(snapshot, request, previous_selection_state)
    }

    pub fn execute_frame(&self) -> &WorthUiDropdownFrameReceipt {
        &self.receipt
    }

    pub fn projection_digest(&self) -> u64 {
        self.projection_digest
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }

    fn build(
        snapshot: &CapabilitySnapshot,
        request: WorthUiDropdownProjectionRequest,
        previous_selection_state: Option<&WorthUiDropdownSelectionState>,
    ) -> Result<Self, WorthUiDropdownProjectionPlanDenial> {
        let projection = snapshot
            .command_projections()
            .get(request.projection_id())
            .ok_or_else(|| {
                WorthUiDropdownProjectionPlanDenial::MissingProjection(
                    request.projection_id().as_str().to_owned(),
                )
            })?;
        let commands = projection
            .command_references()
            .iter()
            .map(|reference| {
                command_from_snapshot(snapshot, request.projection_id(), reference.command_id())
            })
            .collect::<Result<Vec<_>, _>>()?;
        if commands.is_empty() {
            return Err(WorthUiDropdownProjectionPlanDenial::EmptyProjection(
                request.projection_id().as_str().to_owned(),
            ));
        }

        let selection_mode = projection.selection_mode();
        let component_id = active_component_id(&request, selection_mode);
        if snapshot.components().get(component_id).is_none() {
            return Err(WorthUiDropdownProjectionPlanDenial::MissingComponent(
                component_id.as_str().to_owned(),
            ));
        }
        let appearance = request
            .appearance_request()
            .resolve(snapshot)
            .map_err(WorthUiDropdownProjectionPlanDenial::Appearance)?;
        let command_ids = commands
            .iter()
            .map(|command| command.command_id().to_owned())
            .collect::<Vec<_>>();
        let (selection_state, reconciliation) = match previous_selection_state {
            Some(previous_state) => WorthUiDropdownSelectionState::reconcile(
                previous_state,
                selection_mode,
                &command_ids,
            ),
            None => {
                let selection_state = WorthUiDropdownSelectionState::empty_for_mode(selection_mode);
                let status = match selection_state {
                    WorthUiDropdownSelectionState::None => {
                        WorthUiDropdownSelectionStateStatus::Empty
                    }
                    WorthUiDropdownSelectionState::Single(_) => {
                        WorthUiDropdownSelectionStateStatus::PreservedSingle
                    }
                    WorthUiDropdownSelectionState::Multi(_) => {
                        WorthUiDropdownSelectionStateStatus::PreservedMulti
                    }
                };
                (
                    selection_state,
                    WorthUiDropdownSelectionStateReconciliationReceipt::new(status),
                )
            }
        };

        let dependencies = commands.iter().fold(
            request
                .appearance_request()
                .dependencies()
                .depends_on(WorthUiRuntimeFactId::command_projection(
                    request.projection_id(),
                ))
                .depends_on(WorthUiRuntimeFactId::dropdown_selection_state(
                    request.projection_id(),
                ))
                .depends_on(WorthUiRuntimeFactId::command_projection_interaction_policy(
                    request.projection_id(),
                ))
                .depends_on(WorthUiRuntimeFactId::component(component_id)),
            |dependencies, command| {
                dependencies.depends_on(WorthUiRuntimeFactId::command(
                    &CommandId::new(command.command_id()).expect("dropdown command ids stay valid"),
                ))
            },
        );

        let receipt = WorthUiDropdownFrameReceipt {
            projection_id: request.projection_id().as_str().to_owned(),
            component_id: component_id.as_str().to_owned(),
            selection_mode,
            commands,
            appearance,
            selection_state,
            reconciliation,
        };
        let projection_digest = receipt.digest();
        Ok(Self {
            receipt,
            projection_digest,
            dependencies,
        })
    }
}

impl WorthUiProjectionPlanContract for WorthUiDropdownProjectionPlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime(format!(
            "worth-ui.dropdown:{}",
            self.receipt.projection_id()
        ))
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::Dropdown
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(self.dependencies.clone())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        self.projection_digest
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    }
}

impl crate::runtime::projection_contract::plan_contract::private::Sealed
    for WorthUiDropdownProjectionPlan
{
}

impl WorthUiDropdownFrameReceipt {
    pub fn projection_id(&self) -> &str {
        &self.projection_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn selection_mode(&self) -> CommandProjectionSelectionMode {
        self.selection_mode
    }

    pub fn commands(&self) -> &[WorthUiDropdownCommand] {
        &self.commands
    }

    pub fn appearance(&self) -> &WorthUiDropdownAppearanceFrameReceipt {
        &self.appearance
    }

    pub fn selection_state(&self) -> &WorthUiDropdownSelectionState {
        &self.selection_state
    }

    pub fn reconciliation(&self) -> &WorthUiDropdownSelectionStateReconciliationReceipt {
        &self.reconciliation
    }

    fn digest(&self) -> u64 {
        let digest = fold_bytes(0xcbf2_9ce4_8422_2325, self.projection_id.as_bytes());
        let digest = fold_bytes(digest, self.component_id.as_bytes());
        let digest = fold_bytes(digest, self.selection_mode.token().as_bytes());
        let digest = fold_bytes(digest, &self.appearance.digest().to_le_bytes());
        let digest = self
            .selection_state
            .selected_command_ids()
            .iter()
            .fold(digest, |value, id| fold_bytes(value, id.as_bytes()));
        self.commands.iter().fold(digest, |value, command| {
            let value = fold_bytes(value, command.command_id().as_bytes());
            let value = fold_bytes(value, command.label().as_bytes());
            let value = match command.icon_id() {
                Some(icon_id) => fold_bytes(value, icon_id.as_bytes()),
                None => fold_bytes(value, b"no-icon"),
            };
            match command.shortcut() {
                Some(shortcut) => fold_bytes(value, shortcut.as_bytes()),
                None => fold_bytes(value, b"none"),
            }
        })
    }
}

fn active_component_id(
    request: &WorthUiDropdownProjectionRequest,
    selection_mode: CommandProjectionSelectionMode,
) -> &ComponentId {
    match selection_mode {
        CommandProjectionSelectionMode::SingleSelect => request.single_select_component_id(),
        CommandProjectionSelectionMode::MultiSelect => request.multi_select_component_id(),
    }
}

fn command_from_snapshot(
    snapshot: &CapabilitySnapshot,
    projection_id: &CommandProjectionId,
    command_id: &CommandId,
) -> Result<WorthUiDropdownCommand, WorthUiDropdownProjectionPlanDenial> {
    let command = snapshot.commands().get(command_id).ok_or_else(|| {
        WorthUiDropdownProjectionPlanDenial::ProjectionReferencesMissingCommand {
            projection_id: projection_id.as_str().to_owned(),
            command_id: command_id.as_str().to_owned(),
        }
    })?;
    Ok(WorthUiDropdownCommand::new(
        command.id().as_str(),
        command.label(),
        command.icon().map(|icon| icon.as_str().to_owned()),
        command
            .default_shortcut_reference()
            .map(std::borrow::ToOwned::to_owned),
    ))
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
