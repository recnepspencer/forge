use std::collections::{BTreeMap, HashMap};

use worth_ui_host_contract::{
    UiMountedEffectFamily, UiMountedLogicalDamage, UiMountedPaintCommand,
    UiMountedPaintCommandIdentity, UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity,
    UiMountedPresentationAuxiliaryState, UiMountedPresentationDeltaInput,
    UiMountedPresentationInitialInput, UiMountedPresentationUnchangedInput,
    UiMountedProjectionView,
};

use super::{UiMountedPresentationLease, UiMountedPresentationWork};

#[path = "work_producer/delta_diff.rs"]
mod delta_diff;
#[path = "work_producer/effect_expectations.rs"]
mod effect_expectations;
#[path = "work_producer/order_source.rs"]
mod order_source;
#[path = "work_producer/overlay_attribution.rs"]
mod overlay_attribution;

#[derive(Clone, Debug, PartialEq)]
enum CommandSnapshot {
    FilledRect {
        table_index: u16,
        mechanic: worth_ui_host_contract::UiMountedFilledRectMechanic,
        previous_order: Option<UiMountedPaintOrderIdentity>,
    },
    SemanticText {
        table_index: u16,
        mechanic: worth_ui_host_contract::UiMountedSemanticTextMechanic,
        previous_order: Option<UiMountedPaintOrderIdentity>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiMountedPresentationState {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    content: worth_ui_host_contract::UiMountedContentGeneration,
    baseline: worth_ui_host_contract::UiHostSurfaceBaselineIdentity,
    commands: HashMap<UiMountedPaintCommandIdentity, CommandSnapshot>,
    order: Box<[UiMountedPaintOrderIdentity]>,
    auxiliary: UiMountedPresentationAuxiliaryState,
    effects: Box<[UiMountedEffectFamily]>,
}

pub(super) type UiMountedPresentationCandidates =
    BTreeMap<worth_ui_host_contract::UiSurfaceBindingGeneration, UiMountedPresentationState>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiMountedPresentationWorkProductionDenial {
    SurfaceChanged,
    BindingChanged,
    BaselineChanged,
}

impl UiMountedPresentationState {
    pub(crate) fn from_projection(
        projection: &UiMountedProjectionView,
        requirement: worth_ui_host_contract::UiMountedSurfaceBindingRequirement,
    ) -> Self {
        let (mut commands, order) = order_source::commands_and_total_order(projection);
        let mut previous_order = None;
        for identity in &order {
            commands
                .get_mut(&identity.command())
                .expect("paint order identity comes from the command map")
                .set_previous_order(previous_order);
            previous_order = Some(*identity);
        }
        Self {
            frame: projection.frame(),
            surface: projection.surface(),
            binding: projection.binding(),
            content: projection.content_generation(),
            baseline: requirement.baseline(),
            commands,
            order,
            auxiliary: UiMountedPresentationAuxiliaryState::from_runtime_mounting(projection),
            effects: super::effect_requirements::required_effects(
                requirement.presentation_mode(),
                projection,
            )
            .into_boxed_slice(),
        }
    }

    pub(crate) fn issue_initial(
        &self,
        lease: &UiMountedPresentationLease,
        projection: &UiMountedProjectionView,
    ) -> UiMountedPresentationWork {
        lease.issue_initial(UiMountedPresentationInitialInput {
            successor: self.frame,
            surface: self.surface,
            binding: self.binding,
            content: self.content,
            baseline: self.baseline,
            projection: projection.clone(),
            commands: self
                .order
                .iter()
                .map(|identity| {
                    self.commands
                        .get(&identity.command())
                        .expect("paint order names a retained command")
                        .to_command(identity.command())
                })
                .collect(),
            order: self.order.to_vec(),
            order_integrity: UiMountedPaintOrderIntegrity::for_order(&self.order),
            damage: self
                .commands
                .values()
                .filter_map(CommandSnapshot::visible_bounds)
                .map(UiMountedLogicalDamage::from_runtime_mounting)
                .collect(),
        })
    }

    pub(super) fn issue_successor(
        &self,
        successor: &Self,
        lease: &UiMountedPresentationLease,
    ) -> Result<UiMountedPresentationWork, UiMountedPresentationWorkProductionDenial> {
        if self.surface != successor.surface {
            return Err(UiMountedPresentationWorkProductionDenial::SurfaceChanged);
        }
        if self.binding != successor.binding {
            return Err(UiMountedPresentationWorkProductionDenial::BindingChanged);
        }
        if self.baseline != successor.baseline {
            return Err(UiMountedPresentationWorkProductionDenial::BaselineChanged);
        }
        let (mut changes, mut damage) = delta_diff::command_changes(self, successor);
        let order = delta_diff::order_edits(self, successor);
        damage.extend(
            order
                .iter()
                .filter(|edit| !edit.is_removal())
                .filter_map(|edit| {
                    successor
                        .commands
                        .get(&edit.identity().command())
                        .and_then(CommandSnapshot::visible_bounds)
                        .map(UiMountedLogicalDamage::from_runtime_mounting)
                }),
        );
        let auxiliary = (!self
            .auxiliary
            .same_presentation_meaning(&successor.auxiliary))
        .then(|| successor.auxiliary.clone());
        if changes.is_empty() && order.is_empty() && auxiliary.is_none() {
            return Ok(lease.issue_unchanged(UiMountedPresentationUnchangedInput {
                predecessor: self.frame,
                successor: successor.frame,
                surface: successor.surface,
                binding: successor.binding,
                content: successor.content,
                baseline: successor.baseline,
            }));
        }
        overlay_attribution::refresh_commands(self, successor, auxiliary.is_some(), &mut changes);
        Ok(lease.issue_delta(UiMountedPresentationDeltaInput {
            predecessor: self.frame,
            successor: successor.frame,
            surface: successor.surface,
            binding: successor.binding,
            content: successor.content,
            baseline: successor.baseline,
            changes,
            order,
            order_integrity: UiMountedPaintOrderIntegrity::for_order(&successor.order),
            damage,
            auxiliary,
        }))
    }
}

impl CommandSnapshot {
    fn set_previous_order(&mut self, previous: Option<UiMountedPaintOrderIdentity>) {
        match self {
            Self::FilledRect { previous_order, .. } | Self::SemanticText { previous_order, .. } => {
                *previous_order = previous
            }
        }
    }

    fn previous_order(&self) -> Option<UiMountedPaintOrderIdentity> {
        match self {
            Self::FilledRect { previous_order, .. } | Self::SemanticText { previous_order, .. } => {
                *previous_order
            }
        }
    }

    fn same_presentation_meaning(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::FilledRect { mechanic: left, .. },
                Self::FilledRect {
                    mechanic: right, ..
                },
            ) => {
                left.mounted_instance() == right.mounted_instance()
                    && left.allocation_basis() == right.allocation_basis()
                    && left.bounds() == right.bounds()
                    && left.clip_bounds() == right.clip_bounds()
                    && left.color() == right.color()
                    && left.layer_semantic_order() == right.layer_semantic_order()
            }
            (
                Self::SemanticText { mechanic: left, .. },
                Self::SemanticText {
                    mechanic: right, ..
                },
            ) => {
                left.mounted_instance() == right.mounted_instance()
                    && left.allocation_basis() == right.allocation_basis()
                    && left.bounds() == right.bounds()
                    && left.clip_bounds() == right.clip_bounds()
                    && left.origin_x() == right.origin_x()
                    && left.origin_y() == right.origin_y()
                    && left.text() == right.text()
                    && left.slot() == right.slot()
                    && left.collection_row() == right.collection_row()
                    && left.color() == right.color()
                    && left.profile() == right.profile()
                    && left.layer_semantic_order() == right.layer_semantic_order()
                    && left.capability_generation() == right.capability_generation()
                    && left.capability_profile_digest() == right.capability_profile_digest()
            }
            _ => false,
        }
    }

    fn layer(&self) -> u32 {
        match self {
            Self::FilledRect { mechanic, .. } => mechanic.layer_semantic_order(),
            Self::SemanticText { mechanic, .. } => mechanic.layer_semantic_order(),
        }
    }

    fn visible_bounds(&self) -> Option<worth_ui_host_contract::UiMountedCanonicalBox> {
        let (bounds, clip) = match self {
            Self::FilledRect { mechanic, .. } => (mechanic.bounds(), mechanic.clip_bounds()),
            Self::SemanticText { mechanic, .. } => (mechanic.bounds(), mechanic.clip_bounds()),
        };
        if bounds.coordinate_space() != clip.coordinate_space() {
            return None;
        }
        let x = bounds.x().max(clip.x());
        let y = bounds.y().max(clip.y());
        let right = (bounds.x() + bounds.width()).min(clip.x() + clip.width());
        let bottom = (bounds.y() + bounds.height()).min(clip.y() + clip.height());
        if right <= x || bottom <= y {
            return None;
        }
        worth_ui_host_contract::UiMountedCanonicalBox::canonicalize(
            worth_ui_host_contract::UiMountedCanonicalBoxInput {
                x,
                y,
                width: right - x,
                height: bottom - y,
                coordinate_space: bounds.coordinate_space(),
            },
        )
        .ok()
    }

    fn to_command(&self, identity: UiMountedPaintCommandIdentity) -> UiMountedPaintCommand {
        match self {
            Self::FilledRect {
                table_index,
                mechanic,
                ..
            } => UiMountedPaintCommand::FilledRect {
                identity,
                table_index: *table_index,
                mechanic: *mechanic,
            },
            Self::SemanticText {
                table_index,
                mechanic,
                ..
            } => UiMountedPaintCommand::SemanticText {
                identity,
                table_index: *table_index,
                mechanic: mechanic.clone(),
            },
        }
    }
}
