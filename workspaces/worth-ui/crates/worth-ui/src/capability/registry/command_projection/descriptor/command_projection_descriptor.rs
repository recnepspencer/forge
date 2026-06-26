use std::collections::BTreeSet;

use crate::capability::{CommandCategory, CommandProjectionId};

use super::{
    CommandProjectionCommandReference, CommandProjectionGrouping, CommandProjectionIconLabelPolicy,
    CommandProjectionMeaningOverride, CommandProjectionMosaicScope, CommandProjectionOrdering,
    CommandProjectionOverflowBehavior, CommandProjectionReadinessDisplayPolicy,
    CommandProjectionShortcutVisibility, CommandProjectionSurface,
};

/// Declarative command-spine projection supplied by an application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandProjectionDescriptor {
    id: CommandProjectionId,
    surface: CommandProjectionSurface,
    command_references: Vec<CommandProjectionCommandReference>,
    eligible_categories: Vec<CommandCategory>,
    groupings: Vec<CommandProjectionGrouping>,
    ordering: CommandProjectionOrdering,
    shortcut_visibility: CommandProjectionShortcutVisibility,
    readiness_display_policy: CommandProjectionReadinessDisplayPolicy,
    icon_label_policy: CommandProjectionIconLabelPolicy,
    overflow_behavior: CommandProjectionOverflowBehavior,
    mosaic_scope: Option<CommandProjectionMosaicScope>,
    meaning_overrides: Vec<CommandProjectionMeaningOverride>,
}

impl CommandProjectionDescriptor {
    pub fn new(id: CommandProjectionId, surface: CommandProjectionSurface) -> Self {
        Self {
            id,
            surface,
            command_references: Vec::new(),
            eligible_categories: Vec::new(),
            groupings: Vec::new(),
            ordering: CommandProjectionOrdering::Declaration,
            shortcut_visibility: CommandProjectionShortcutVisibility::Hidden,
            readiness_display_policy: CommandProjectionReadinessDisplayPolicy::HideReadiness,
            icon_label_policy: CommandProjectionIconLabelPolicy::PreferCommandIconAndLabel,
            overflow_behavior: CommandProjectionOverflowBehavior::NoOverflow,
            mosaic_scope: None,
            meaning_overrides: Vec::new(),
        }
    }

    pub fn with_command_reference(mut self, reference: CommandProjectionCommandReference) -> Self {
        self.command_references.push(reference);
        self
    }

    pub fn with_eligible_category(mut self, category: CommandCategory) -> Self {
        self.eligible_categories.push(category);
        self
    }

    pub fn with_grouping(mut self, grouping: CommandProjectionGrouping) -> Self {
        self.groupings.push(grouping);
        self
    }

    pub fn with_ordering(mut self, ordering: CommandProjectionOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    pub fn show_shortcuts(mut self) -> Self {
        self.shortcut_visibility =
            CommandProjectionShortcutVisibility::VisibleWhenCommandHasShortcut;
        self
    }

    pub fn show_readiness(mut self) -> Self {
        self.readiness_display_policy = CommandProjectionReadinessDisplayPolicy::ShowReadiness;
        self
    }

    pub fn disable_unavailable_commands(mut self) -> Self {
        self.readiness_display_policy =
            CommandProjectionReadinessDisplayPolicy::DisableUnavailableCommands;
        self
    }

    pub fn prefer_command_icon_and_label(mut self) -> Self {
        self.icon_label_policy = CommandProjectionIconLabelPolicy::PreferCommandIconAndLabel;
        self
    }

    pub fn with_icon_label_policy(mut self, policy: CommandProjectionIconLabelPolicy) -> Self {
        self.icon_label_policy = policy;
        self
    }

    pub fn with_overflow_behavior(mut self, behavior: CommandProjectionOverflowBehavior) -> Self {
        self.overflow_behavior = behavior;
        self
    }

    pub fn with_mosaic_scope(mut self, scope: CommandProjectionMosaicScope) -> Self {
        self.mosaic_scope = Some(scope);
        self
    }

    pub fn with_command_meaning_override_for_diagnostics(
        mut self,
        override_kind: CommandProjectionMeaningOverride,
    ) -> Self {
        self.meaning_overrides.push(override_kind);
        self
    }

    pub fn id(&self) -> &CommandProjectionId {
        &self.id
    }

    pub fn surface(&self) -> &CommandProjectionSurface {
        &self.surface
    }

    pub fn command_references(&self) -> &[CommandProjectionCommandReference] {
        &self.command_references
    }

    pub fn eligible_categories(&self) -> &[CommandCategory] {
        &self.eligible_categories
    }

    pub fn groupings(&self) -> &[CommandProjectionGrouping] {
        &self.groupings
    }

    pub fn ordering(&self) -> CommandProjectionOrdering {
        self.ordering
    }

    pub fn shortcut_visibility(&self) -> CommandProjectionShortcutVisibility {
        self.shortcut_visibility
    }

    pub fn readiness_display_policy(&self) -> CommandProjectionReadinessDisplayPolicy {
        self.readiness_display_policy
    }

    pub fn icon_label_policy(&self) -> CommandProjectionIconLabelPolicy {
        self.icon_label_policy
    }

    pub fn overflow_behavior(&self) -> CommandProjectionOverflowBehavior {
        self.overflow_behavior
    }

    pub fn mosaic_scope(&self) -> Option<&CommandProjectionMosaicScope> {
        self.mosaic_scope.as_ref()
    }

    pub(crate) fn meaning_overrides(&self) -> &[CommandProjectionMeaningOverride] {
        &self.meaning_overrides
    }

    pub(crate) fn canonicalized_for_freeze(mut self) -> Self {
        self.command_references = canonical_command_references(
            self.ordering,
            std::mem::take(&mut self.command_references),
        );
        self.eligible_categories.sort();
        self.eligible_categories.dedup();
        self.groupings =
            deduplicate_groupings_preserving_declaration_order(std::mem::take(&mut self.groupings));
        self
    }
}

fn canonical_command_references(
    ordering: CommandProjectionOrdering,
    mut command_references: Vec<CommandProjectionCommandReference>,
) -> Vec<CommandProjectionCommandReference> {
    match ordering {
        CommandProjectionOrdering::Declaration => {
            deduplicate_command_references_preserving_declaration_order(command_references)
        }
        CommandProjectionOrdering::ByCommandId
        | CommandProjectionOrdering::ByCategoryThenCommandId => {
            command_references.sort();
            command_references.dedup();
            command_references
        }
    }
}

fn deduplicate_command_references_preserving_declaration_order(
    command_references: Vec<CommandProjectionCommandReference>,
) -> Vec<CommandProjectionCommandReference> {
    let mut seen_command_ids = BTreeSet::new();
    command_references
        .into_iter()
        .filter(|reference| seen_command_ids.insert(reference.command_id().as_str().to_owned()))
        .collect()
}

fn deduplicate_groupings_preserving_declaration_order(
    groupings: Vec<CommandProjectionGrouping>,
) -> Vec<CommandProjectionGrouping> {
    let mut seen_grouping_keys = BTreeSet::new();
    groupings
        .into_iter()
        .filter(|grouping| seen_grouping_keys.insert(grouping.digest_basis()))
        .collect()
}
