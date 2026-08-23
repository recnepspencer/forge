use std::collections::HashSet;

use worth_ui_host_contract::{
    UiMountedInstanceIdentity, UiMountedLogicalDamage, UiMountedPaintCommandChange,
    UiMountedPaintCommandIdentity, UiMountedPaintOrderEdit, UiMountedPresentationAuxiliaryState,
    UiMountedPresentationDeltaInput, UiMountedPresentationUnchangedInput,
};

use super::{
    command_visible_bounds, delta_diff, overlay_attribution, production_cost, LocalWorkCost,
    RetainedTraversalCost, UiMountedPresentationLease, UiMountedPresentationState,
    UiMountedPresentationWork, UiMountedPresentationWorkProductionDenial,
};

pub(in crate::mounting::presentation) struct SuccessorIssueRequest<'a> {
    pub(super) successor: &'a UiMountedPresentationState,
    pub(super) changed_instances: &'a [UiMountedInstanceIdentity],
    pub(super) precise_changes: &'a [UiMountedPaintCommandChange],
    pub(super) surface_changed: bool,
    pub(super) source_predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    pub(super) lease: &'a UiMountedPresentationLease,
}

impl<'a> SuccessorIssueRequest<'a> {
    pub(in crate::mounting::presentation) fn new(
        successor: &'a UiMountedPresentationState,
        changed_instances: &'a [UiMountedInstanceIdentity],
        precise_changes: &'a [UiMountedPaintCommandChange],
        lease: &'a UiMountedPresentationLease,
    ) -> Self {
        Self {
            successor,
            changed_instances,
            precise_changes,
            surface_changed: false,
            source_predecessor: successor.predecessor,
            lease,
        }
    }

    pub(in crate::mounting::presentation) const fn with_surface_changed(
        mut self,
        changed: bool,
    ) -> Self {
        self.surface_changed = changed;
        self
    }

    pub(in crate::mounting::presentation) const fn with_source_predecessor(
        mut self,
        predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    ) -> Self {
        self.source_predecessor = predecessor;
        self
    }
}

pub(super) struct SuccessorIssue<'a> {
    pub(super) predecessor: &'a UiMountedPresentationState,
    pub(super) request: SuccessorIssueRequest<'a>,
    pub(super) retained_traversal: RetainedTraversalCost,
}

impl<'a> std::ops::Deref for SuccessorIssue<'a> {
    type Target = SuccessorIssueRequest<'a>;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl SuccessorIssue<'_> {
    pub(super) fn issue(
        self,
    ) -> Result<UiMountedPresentationWork, UiMountedPresentationWorkProductionDenial> {
        self.validate_lineage()?;
        let mut delta = PreparedDelta::new(&self);
        if delta.is_unchanged(self.successor) {
            return Ok(self.issue_unchanged(delta.affected_count));
        }
        let overlay = overlay_attribution::refresh_commands(
            self.predecessor,
            self.successor,
            delta.auxiliary.is_some(),
            &mut delta.changes,
        );
        let cost = LocalWorkCost {
            source_instances: self.changed_instances.len(),
            commands_considered: delta
                .affected_count
                .saturating_add(overlay.commands_considered),
            command_index_lookups: delta
                .affected_count
                .saturating_mul(2)
                .saturating_add(overlay.command_lookups),
            order_lookups: delta
                .affected_count
                .saturating_mul(2)
                .saturating_add(overlay.order_items_scanned),
        };
        Ok(self.lease.issue_delta(UiMountedPresentationDeltaInput {
            predecessor: self.predecessor.frame,
            successor: self.successor.frame,
            surface: self.successor.surface,
            binding: self.successor.binding,
            content: self.successor.content,
            baseline: self.successor.baseline,
            changes: delta.changes,
            nodes: self.successor.node_changes.to_vec(),
            order: delta.order,
            order_integrity: self.successor.order_integrity,
            damage: delta.damage,
            auxiliary: delta.auxiliary,
            production_cost: production_cost(
                cost,
                self.retained_traversal
                    .add_scans(overlay.order_items_scanned),
                self.successor.projection_rows_materialized,
            ),
        }))
    }

    fn validate_lineage(&self) -> Result<(), UiMountedPresentationWorkProductionDenial> {
        if self.successor.predecessor != Some(self.predecessor.frame)
            || self.source_predecessor != Some(self.predecessor.frame)
        {
            return Err(UiMountedPresentationWorkProductionDenial::StalePredecessor);
        }
        if self.predecessor.surface != self.successor.surface {
            return Err(UiMountedPresentationWorkProductionDenial::SurfaceChanged);
        }
        if self.predecessor.binding != self.successor.binding {
            return Err(UiMountedPresentationWorkProductionDenial::BindingChanged);
        }
        if self.predecessor.baseline != self.successor.baseline {
            return Err(UiMountedPresentationWorkProductionDenial::BaselineChanged);
        }
        Ok(())
    }

    fn issue_unchanged(&self, affected_count: usize) -> UiMountedPresentationWork {
        let local = LocalWorkCost {
            source_instances: self.changed_instances.len(),
            commands_considered: affected_count,
            command_index_lookups: affected_count.saturating_mul(2),
            order_lookups: affected_count.saturating_mul(2),
        };
        self.lease
            .issue_unchanged(UiMountedPresentationUnchangedInput {
                predecessor: self.predecessor.frame,
                successor: self.successor.frame,
                surface: self.successor.surface,
                binding: self.successor.binding,
                content: self.successor.content,
                baseline: self.successor.baseline,
                production_cost: production_cost(
                    local,
                    self.retained_traversal,
                    self.successor.projection_rows_materialized,
                ),
            })
    }
}

struct PreparedDelta {
    affected_count: usize,
    changes: Vec<UiMountedPaintCommandChange>,
    damage: Vec<UiMountedLogicalDamage>,
    order: Vec<UiMountedPaintOrderEdit>,
    auxiliary: Option<UiMountedPresentationAuxiliaryState>,
}

impl PreparedDelta {
    fn new(issue: &SuccessorIssue<'_>) -> Self {
        let affected = affected_commands(issue);
        let (changes, mut damage, order) = changed_content(issue, &affected);
        append_order_damage(issue.successor, &changes, &order, &mut damage);
        let auxiliary_changed = issue.surface_changed
            || !issue
                .predecessor
                .auxiliary
                .same_lane_presentation_meaning(&issue.successor.auxiliary);
        Self {
            affected_count: affected.len(),
            changes,
            damage,
            order,
            auxiliary: auxiliary_changed.then(|| issue.successor.auxiliary.clone()),
        }
    }

    fn is_unchanged(&self, successor: &UiMountedPresentationState) -> bool {
        self.changes.is_empty()
            && self.order.is_empty()
            && successor.node_changes.is_empty()
            && self.auxiliary.is_none()
    }
}

fn affected_commands(issue: &SuccessorIssue<'_>) -> Vec<UiMountedPaintCommandIdentity> {
    if issue.precise_changes.is_empty() {
        delta_diff::affected_commands(issue.predecessor, issue.successor, issue.changed_instances)
    } else {
        issue
            .precise_changes
            .iter()
            .map(command_change_identity)
            .collect()
    }
}

fn changed_content(
    issue: &SuccessorIssue<'_>,
    affected: &[UiMountedPaintCommandIdentity],
) -> (
    Vec<UiMountedPaintCommandChange>,
    Vec<UiMountedLogicalDamage>,
    Vec<UiMountedPaintOrderEdit>,
) {
    if issue.precise_changes.is_empty() {
        let (changes, damage) =
            delta_diff::command_changes(issue.predecessor, issue.successor, affected);
        let order = delta_diff::order_edits(issue.predecessor, issue.successor, affected);
        (changes, damage, order)
    } else {
        let changes = issue.precise_changes.to_vec();
        let damage = precise_damage(issue.predecessor, &changes);
        (changes, damage, Vec::new())
    }
}

fn append_order_damage(
    successor: &UiMountedPresentationState,
    changes: &[UiMountedPaintCommandChange],
    order: &[UiMountedPaintOrderEdit],
    damage: &mut Vec<UiMountedLogicalDamage>,
) {
    let changed = changes
        .iter()
        .map(command_change_identity)
        .collect::<HashSet<_>>();
    damage.extend(
        order
            .iter()
            .filter(|edit| !edit.is_removal())
            .filter(|edit| !changed.contains(&edit.identity().command()))
            .filter_map(|edit| {
                successor
                    .command_option(edit.identity().command())
                    .and_then(command_visible_bounds)
                    .map(UiMountedLogicalDamage::from_runtime_mounting)
            }),
    );
}

fn command_change_identity(change: &UiMountedPaintCommandChange) -> UiMountedPaintCommandIdentity {
    match change {
        UiMountedPaintCommandChange::Insert(command)
        | UiMountedPaintCommandChange::Replace {
            successor: command, ..
        } => command.identity(),
        UiMountedPaintCommandChange::Remove(identity) => *identity,
    }
}

fn precise_damage(
    predecessor: &UiMountedPresentationState,
    changes: &[UiMountedPaintCommandChange],
) -> Vec<UiMountedLogicalDamage> {
    changes
        .iter()
        .flat_map(|change| match change {
            UiMountedPaintCommandChange::Insert(command) => [None, command_visible_bounds(command)],
            UiMountedPaintCommandChange::Replace {
                predecessor: identity,
                successor: command,
            } => [
                predecessor
                    .command_option(*identity)
                    .and_then(command_visible_bounds),
                command_visible_bounds(command),
            ],
            UiMountedPaintCommandChange::Remove(identity) => [
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
