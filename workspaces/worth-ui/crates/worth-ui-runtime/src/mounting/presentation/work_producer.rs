use std::collections::{BTreeMap, HashSet};

use worth_ui_host_contract::{
    UiMountedLogicalDamage, UiMountedPaintCommand, UiMountedPresentationDeltaInput,
    UiMountedPresentationInitialInput, UiMountedPresentationProductionCost,
    UiMountedPresentationProductionCostInput, UiMountedPresentationReconstructionInput,
    UiMountedPresentationUnchangedInput, UiMountedProjectionView,
};

use super::{UiMountedPresentationLease, UiMountedPresentationWork};

#[path = "work_producer/command_bundle.rs"]
mod command_bundle;
#[path = "work_producer/delta_diff.rs"]
mod delta_diff;
#[path = "work_producer/effect_expectations.rs"]
mod effect_expectations;
#[path = "work_producer/overlay_attribution.rs"]
mod overlay_attribution;
#[path = "work_producer/state.rs"]
mod state;

pub(crate) use state::UiMountedPresentationState;

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
    pub(crate) fn issue_initial(
        &self,
        lease: &UiMountedPresentationLease,
        projection: &UiMountedProjectionView,
    ) -> UiMountedPresentationWork {
        assert!(self.predecessor.is_none());
        let commands = self.commands();
        let order = self.order();
        lease.issue_initial(UiMountedPresentationInitialInput {
            successor: self.frame,
            surface: self.surface,
            binding: self.binding,
            content: self.content,
            baseline: self.baseline,
            projection: projection.clone(),
            commands: commands.clone(),
            order: order.clone(),
            order_integrity: self.order_integrity,
            damage: commands
                .iter()
                .filter_map(command_visible_bounds)
                .map(UiMountedLogicalDamage::from_runtime_mounting)
                .collect(),
            production_cost: production_cost(
                self.source_instance_count(),
                commands.len(),
                order.len(),
                order.len(),
                0,
                self.projection_rows_materialized,
            ),
        })
    }

    pub(crate) fn issue_reconstruction(
        &self,
        lease: &UiMountedPresentationLease,
        projection: &UiMountedProjectionView,
        predecessor: worth_ui_host_contract::UiMountedFrameIdentity,
    ) -> UiMountedPresentationWork {
        assert_eq!(self.predecessor, Some(predecessor));
        let commands = self.commands();
        let order = self.order();
        lease.issue_reconstruction(UiMountedPresentationReconstructionInput {
            predecessor,
            successor: self.frame,
            surface: self.surface,
            binding: self.binding,
            content: self.content,
            baseline: self.baseline,
            projection: projection.clone(),
            commands: commands.clone(),
            order: order.clone(),
            order_integrity: self.order_integrity,
            damage: complete_damage(&commands),
            production_cost: production_cost(
                self.source_instance_count(),
                commands.len(),
                order.len(),
                order.len(),
                commands.len(),
                self.projection_rows_materialized,
            ),
        })
    }

    pub(super) fn issue_successor(
        &self,
        successor: &Self,
        changed_instances: &[worth_ui_host_contract::UiMountedInstanceIdentity],
        precise_changes: &[worth_ui_host_contract::UiMountedPaintCommandChange],
        surface_changed: bool,
        source_predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
        lease: &UiMountedPresentationLease,
    ) -> Result<UiMountedPresentationWork, UiMountedPresentationWorkProductionDenial> {
        if successor.predecessor != Some(self.frame) || source_predecessor != Some(self.frame) {
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
        let affected = if precise_changes.is_empty() {
            delta_diff::affected_commands(self, successor, changed_instances)
        } else {
            precise_changes
                .iter()
                .map(command_change_identity)
                .collect::<Vec<_>>()
        };
        let (mut changes, mut damage, order) = if precise_changes.is_empty() {
            let (changes, damage) = delta_diff::command_changes(self, successor, &affected);
            let order = delta_diff::order_edits(self, successor, &affected);
            (changes, damage, order)
        } else {
            let changes = precise_changes.to_vec();
            let damage = precise_damage(self, &changes);
            (changes, damage, Vec::new())
        };
        let changed_commands = changes
            .iter()
            .map(|change| match change {
                worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command)
                | worth_ui_host_contract::UiMountedPaintCommandChange::Replace {
                    successor: command,
                    ..
                } => command.identity(),
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
            || !self
                .auxiliary
                .same_lane_presentation_meaning(&successor.auxiliary);
        let auxiliary = auxiliary_changed.then(|| successor.auxiliary.clone());
        if changes.is_empty()
            && order.is_empty()
            && successor.node_changes.is_empty()
            && auxiliary.is_none()
        {
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
                    successor.projection_rows_materialized,
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
            nodes: successor.node_changes.to_vec(),
            order,
            order_integrity: successor.order_integrity,
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
                successor.projection_rows_materialized,
            ),
        }))
    }
}

fn command_change_identity(
    change: &worth_ui_host_contract::UiMountedPaintCommandChange,
) -> worth_ui_host_contract::UiMountedPaintCommandIdentity {
    match change {
        worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command)
        | worth_ui_host_contract::UiMountedPaintCommandChange::Replace {
            successor: command, ..
        } => command.identity(),
        worth_ui_host_contract::UiMountedPaintCommandChange::Remove(identity) => *identity,
    }
}

fn precise_damage(
    predecessor: &UiMountedPresentationState,
    changes: &[worth_ui_host_contract::UiMountedPaintCommandChange],
) -> Vec<UiMountedLogicalDamage> {
    changes
        .iter()
        .flat_map(|change| match change {
            worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command) => {
                [None, command_visible_bounds(command)]
            }
            worth_ui_host_contract::UiMountedPaintCommandChange::Replace {
                predecessor: predecessor_identity,
                successor: command,
            } => [
                predecessor
                    .command_option(*predecessor_identity)
                    .and_then(command_visible_bounds),
                command_visible_bounds(command),
            ],
            worth_ui_host_contract::UiMountedPaintCommandChange::Remove(identity) => [
                predecessor
                    .command_option(*identity)
                    .and_then(command_visible_bounds),
                None,
            ],
        })
        .flatten()
        .map(UiMountedLogicalDamage::from_runtime_mounting)
        .collect()
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

fn complete_damage(commands: &[UiMountedPaintCommand]) -> Vec<UiMountedLogicalDamage> {
    commands
        .iter()
        .filter_map(command_visible_bounds)
        .map(UiMountedLogicalDamage::from_runtime_mounting)
        .collect()
}

fn production_cost(
    source_instances: usize,
    commands_considered: usize,
    command_index_lookups: usize,
    order_lookups: usize,
    retained_command_scans: usize,
    projection_rows_materialized: u64,
) -> UiMountedPresentationProductionCost {
    UiMountedPresentationProductionCost::from_runtime_mounting(
        UiMountedPresentationProductionCostInput {
            source_instances: exact_count(source_instances),
            commands_considered: exact_count(commands_considered),
            command_index_lookups: exact_count(command_index_lookups),
            order_lookups: exact_count(order_lookups),
            retained_command_scans: exact_count(retained_command_scans),
            retained_command_clones: 0,
            projection_rows_materialized,
        },
    )
}

fn exact_count(value: usize) -> u64 {
    u64::try_from(value).expect("presentation work count fits the governed u64 counter")
}
