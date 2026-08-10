use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use worth_ui_host_contract::{
    UiMountedEffectFamily, UiMountedLogicalDamage, UiMountedPaintCommand,
    UiMountedPaintCommandIdentity, UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity,
    UiMountedPresentationAuxiliaryState, UiMountedPresentationDeltaInput,
    UiMountedPresentationInitialInput, UiMountedPresentationProductionCost,
    UiMountedPresentationProductionCostInput, UiMountedPresentationUnchangedInput,
    UiMountedProjectionView,
};

use super::{UiMountedPresentationLease, UiMountedPresentationWork};

#[path = "work_producer/delta_diff.rs"]
mod delta_diff;
#[path = "work_producer/effect_expectations.rs"]
mod effect_expectations;
#[path = "work_producer/overlay_attribution.rs"]
mod overlay_attribution;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiMountedPresentationState {
    predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    content: worth_ui_host_contract::UiMountedContentGeneration,
    baseline: worth_ui_host_contract::UiHostSurfaceBaselineIdentity,
    commands: Arc<[UiMountedPaintCommand]>,
    command_indices: Arc<HashMap<UiMountedPaintCommandIdentity, usize>>,
    commands_by_instance: Arc<
        HashMap<
            worth_ui_host_contract::UiMountedInstanceIdentity,
            Arc<[UiMountedPaintCommandIdentity]>,
        >,
    >,
    order: Arc<[UiMountedPaintOrderIdentity]>,
    order_predecessors:
        Arc<HashMap<UiMountedPaintCommandIdentity, Option<UiMountedPaintOrderIdentity>>>,
    order_positions: Arc<HashMap<UiMountedPaintCommandIdentity, usize>>,
    auxiliary: UiMountedPresentationAuxiliaryState,
    effects: Box<[UiMountedEffectFamily]>,
}

pub(super) type UiMountedPresentationCandidates =
    BTreeMap<worth_ui_host_contract::UiSurfaceBindingGeneration, UiMountedPresentationState>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiMountedPresentationWorkProductionDenial {
    StalePredecessor,
    SurfaceChanged,
    BindingChanged,
    BaselineChanged,
}

impl UiMountedPresentationState {
    pub(super) const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }

    pub(crate) fn from_projection(
        projection: &UiMountedProjectionView,
        requirement: worth_ui_host_contract::UiMountedSurfaceBindingRequirement,
        predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    ) -> Self {
        Self {
            predecessor,
            frame: projection.frame(),
            surface: projection.surface(),
            binding: projection.binding(),
            content: projection.content_generation(),
            baseline: requirement.baseline(),
            commands: projection.retained_paint_commands(),
            command_indices: projection.retained_command_indices(),
            commands_by_instance: projection.retained_commands_by_instance(),
            order: projection.retained_paint_order(),
            order_predecessors: projection.retained_order_predecessors(),
            order_positions: projection.retained_order_positions(),
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
        assert!(self.predecessor.is_none());
        lease.issue_initial(UiMountedPresentationInitialInput {
            successor: self.frame,
            surface: self.surface,
            binding: self.binding,
            content: self.content,
            baseline: self.baseline,
            projection: projection.clone(),
            commands: self.commands.to_vec(),
            order: self.order.to_vec(),
            order_integrity: UiMountedPaintOrderIntegrity::for_order(&self.order),
            damage: self
                .commands
                .iter()
                .filter_map(command_visible_bounds)
                .map(UiMountedLogicalDamage::from_runtime_mounting)
                .collect(),
            production_cost: production_cost(
                self.commands_by_instance.len(),
                self.commands.len(),
                self.order.len(),
                self.order.len(),
                0,
            ),
        })
    }

    pub(super) fn issue_successor(
        &self,
        successor: &Self,
        changed_instances: &[worth_ui_host_contract::UiMountedInstanceIdentity],
        surface_changed: bool,
        lease: &UiMountedPresentationLease,
    ) -> Result<UiMountedPresentationWork, UiMountedPresentationWorkProductionDenial> {
        if successor.predecessor != Some(self.frame) {
            return Err(UiMountedPresentationWorkProductionDenial::StalePredecessor);
        }
        if self.surface != successor.surface {
            return Err(UiMountedPresentationWorkProductionDenial::SurfaceChanged);
        }
        if self.binding != successor.binding {
            return Err(UiMountedPresentationWorkProductionDenial::BindingChanged);
        }
        if self.baseline != successor.baseline {
            return Err(UiMountedPresentationWorkProductionDenial::BaselineChanged);
        }
        let affected = delta_diff::affected_commands(self, successor, changed_instances);
        let (mut changes, mut damage) = delta_diff::command_changes(self, successor, &affected);
        let order = delta_diff::order_edits(self, successor, &affected);
        let changed_commands = changes
            .iter()
            .map(|change| match change {
                worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command)
                | worth_ui_host_contract::UiMountedPaintCommandChange::Replace(command) => {
                    command.identity()
                }
                worth_ui_host_contract::UiMountedPaintCommandChange::Remove(identity) => *identity,
            })
            .collect::<HashSet<_>>();
        damage.extend(
            order
                .iter()
                .filter(|edit| !edit.is_removal())
                .filter(|edit| !changed_commands.contains(&edit.identity().command()))
                .filter_map(|edit| {
                    successor
                        .command_option(edit.identity().command())
                        .and_then(command_visible_bounds)
                        .map(UiMountedLogicalDamage::from_runtime_mounting)
                }),
        );
        let auxiliary_changed = surface_changed
            || self.effects != successor.effects
            || !self
                .auxiliary
                .same_lane_presentation_meaning(&successor.auxiliary)
            || changed_instances.iter().any(|instance| {
                self.commands_by_instance.contains_key(instance)
                    || successor.commands_by_instance.contains_key(instance)
            });
        let auxiliary = auxiliary_changed.then(|| successor.auxiliary.clone());
        if changes.is_empty() && order.is_empty() && auxiliary.is_none() {
            return Ok(lease.issue_unchanged(UiMountedPresentationUnchangedInput {
                predecessor: self.frame,
                successor: successor.frame,
                surface: successor.surface,
                binding: successor.binding,
                content: successor.content,
                baseline: successor.baseline,
                production_cost: production_cost(
                    changed_instances.len(),
                    affected.len(),
                    affected.len().saturating_mul(2),
                    affected.len().saturating_mul(2),
                    0,
                ),
            }));
        }
        let overlay_cost = overlay_attribution::refresh_commands(
            self,
            successor,
            auxiliary.is_some(),
            &mut changes,
        );
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
            production_cost: production_cost(
                changed_instances.len(),
                affected
                    .len()
                    .saturating_add(overlay_cost.commands_considered),
                affected
                    .len()
                    .saturating_mul(2)
                    .saturating_add(overlay_cost.command_lookups),
                affected
                    .len()
                    .saturating_mul(2)
                    .saturating_add(overlay_cost.order_items_scanned),
                overlay_cost.order_items_scanned,
            ),
        }))
    }
}
impl UiMountedPresentationState {
    fn command(&self, identity: UiMountedPaintCommandIdentity) -> &UiMountedPaintCommand {
        self.command_option(identity)
            .expect("paint order names a retained command")
    }

    fn command_option(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<&UiMountedPaintCommand> {
        self.command_indices
            .get(&identity)
            .and_then(|index| self.commands.get(*index))
    }

    fn command_identities_for_instance(
        &self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> &[UiMountedPaintCommandIdentity] {
        self.commands_by_instance
            .get(&instance)
            .map_or(&[], AsRef::as_ref)
    }

    fn previous_order(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<UiMountedPaintOrderIdentity> {
        self.order_predecessors.get(&identity).copied().flatten()
    }

    fn order_position(&self, identity: UiMountedPaintCommandIdentity) -> Option<usize> {
        self.order_positions.get(&identity).copied()
    }
}

fn command_same_presentation_meaning(
    left: &UiMountedPaintCommand,
    right: &UiMountedPaintCommand,
) -> bool {
    match (left, right) {
        (
            UiMountedPaintCommand::FilledRect { mechanic: left, .. },
            UiMountedPaintCommand::FilledRect {
                mechanic: right, ..
            },
        ) => left.semantic_digest() == right.semantic_digest(),
        (
            UiMountedPaintCommand::SemanticText { mechanic: left, .. },
            UiMountedPaintCommand::SemanticText {
                mechanic: right, ..
            },
        ) => left.semantic_digest() == right.semantic_digest(),
        _ => false,
    }
}

fn command_visible_bounds(
    command: &UiMountedPaintCommand,
) -> Option<worth_ui_host_contract::UiMountedCanonicalBox> {
    let (bounds, clip) = match command {
        UiMountedPaintCommand::FilledRect { mechanic, .. } => {
            (mechanic.bounds(), mechanic.clip_bounds())
        }
        UiMountedPaintCommand::SemanticText { mechanic, .. } => {
            (mechanic.bounds(), mechanic.clip_bounds())
        }
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

fn production_cost(
    source_instances: usize,
    commands_considered: usize,
    command_index_lookups: usize,
    order_lookups: usize,
    retained_command_scans: usize,
) -> UiMountedPresentationProductionCost {
    UiMountedPresentationProductionCost::from_runtime_mounting(
        UiMountedPresentationProductionCostInput {
            source_instances: exact_count(source_instances),
            commands_considered: exact_count(commands_considered),
            command_index_lookups: exact_count(command_index_lookups),
            order_lookups: exact_count(order_lookups),
            retained_command_scans: exact_count(retained_command_scans),
            retained_command_clones: 0,
        },
    )
}

fn exact_count(value: usize) -> u64 {
    u64::try_from(value).expect("presentation work count fits the governed u64 counter")
}
