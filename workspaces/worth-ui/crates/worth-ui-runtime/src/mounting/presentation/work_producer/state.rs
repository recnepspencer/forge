use worth_ui_host_contract::{
    UiMountedEffectFamily, UiMountedPaintCommand, UiMountedPaintCommandIdentity,
    UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity, UiMountedPresentationAuxiliaryState,
    UiMountedProjectionView,
};

use crate::runtime::persistent_index::{UiPersistentOrdMap, UiPersistentOrder};

#[derive(Clone)]
pub(crate) struct UiMountedPresentationState {
    pub(super) predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    pub(super) frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub(super) surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    pub(super) binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    pub(super) content: worth_ui_host_contract::UiMountedContentGeneration,
    pub(super) baseline: worth_ui_host_contract::UiHostSurfaceBaselineIdentity,
    commands_by_instance: UiPersistentOrdMap<
        worth_ui_host_contract::UiMountedInstanceIdentity,
        super::command_bundle::UiMountedPresentationCommandBundle,
    >,
    instance_order: UiPersistentOrder<worth_ui_host_contract::UiMountedInstanceIdentity>,
    command_order: UiPersistentOrdMap<PresentationOrderKey, UiMountedPaintCommandIdentity>,
    pub(super) order_integrity: UiMountedPaintOrderIntegrity,
    pub(super) auxiliary: UiMountedPresentationAuxiliaryState,
    pub(super) effects: Box<[UiMountedEffectFamily]>,
    pub(super) node_changes:
        std::sync::Arc<[worth_ui_host_contract::UiMountedPresentationNodeChange]>,
    pub(super) projection_rows_materialized: u64,
}

type PresentationOrderKey = (u32, u64, usize);

impl UiMountedPresentationState {
    pub(crate) fn from_projection(
        projection: &UiMountedProjectionView,
        requirement: worth_ui_host_contract::UiMountedSurfaceBindingRequirement,
        predecessor: Option<worth_ui_host_contract::UiMountedFrameIdentity>,
    ) -> Self {
        let mut instance_order = UiPersistentOrder::default();
        let mut command_order = UiPersistentOrdMap::default();
        let command_indices = projection.retained_command_indices();
        let retained_commands = projection.retained_paint_commands();
        let commands_by_instance = projection.retained_commands_by_instance();
        let mut persistent_commands = UiPersistentOrdMap::default();
        for node in projection.nodes() {
            let instance = node.mounted_instance();
            instance_order
                .append(instance)
                .expect("projection nodes are unique");
            let commands = commands_by_instance
                .get(&instance)
                .into_iter()
                .flat_map(|identities| identities.iter())
                .filter_map(|identity| {
                    command_indices
                        .get(identity)
                        .and_then(|index| retained_commands.get(*index))
                        .cloned()
                })
                .collect::<Vec<_>>();
            if !commands.is_empty() {
                let position = instance_order
                    .position(instance)
                    .expect("appended position");
                for (local, command) in commands.iter().enumerate() {
                    command_order.insert(
                        (command.layer_semantic_order(), position, local),
                        command.identity(),
                    );
                }
                persistent_commands.insert(
                    instance,
                    super::command_bundle::UiMountedPresentationCommandBundle::from_commands(
                        &commands,
                    ),
                );
            }
        }
        Self {
            predecessor,
            frame: projection.frame(),
            surface: projection.surface(),
            binding: projection.binding(),
            content: projection.content_generation(),
            baseline: requirement.baseline(),
            commands_by_instance: persistent_commands,
            instance_order,
            command_order,
            order_integrity: projection.retained_order_integrity(),
            auxiliary: UiMountedPresentationAuxiliaryState::from_runtime_mounting(projection),
            effects: super::super::effect_requirements::required_effects(
                requirement.presentation_mode(),
                projection,
            )
            .into_boxed_slice(),
            node_changes: std::sync::Arc::from([]),
            projection_rows_materialized: projection_row_count(projection),
        }
    }

    pub(in crate::mounting::presentation) fn successor_from_source(
        predecessor: &Self,
        source: crate::mounting::UiMountedPresentationDeltaSource<'_>,
        projection: Option<&UiMountedProjectionView>,
        requirement: worth_ui_host_contract::UiMountedSurfaceBindingRequirement,
    ) -> Self {
        let mut successor = predecessor.clone();
        successor.predecessor = source.predecessor();
        successor.frame = source.frame().frame_identity();
        successor.surface = requirement.semantic_surface();
        successor.binding = requirement.binding();
        successor.content = source.frame().content_generation();
        successor.baseline = requirement.baseline();
        successor.projection_rows_materialized = source.frame().materialized_projection_rows();
        for instance in source
            .changed_instances()
            .iter()
            .filter(|instance| !source.frame().has_precise_command_delta(**instance))
        {
            successor.remove_bundle(*instance);
        }
        successor.instance_order = source.frame().presentation_instance_order();
        let mut insertions = source
            .changed_instances()
            .iter()
            .filter(|instance| !source.frame().has_precise_command_delta(**instance))
            .filter_map(|instance| {
                source
                    .frame()
                    .presentation_order_position(*instance)
                    .map(|position| (position, *instance))
            })
            .collect::<Vec<_>>();
        insertions.sort_unstable_by_key(|(position, _)| *position);
        for (position, instance) in insertions {
            let commands = source.frame().presentation_commands_for_instance(
                instance,
                successor.surface,
                successor.binding,
            );
            successor.insert_bundle(position, instance, commands);
        }
        successor.apply_precise_replacements(source.frame().presentation_command_changes());
        successor.effects = source
            .frame()
            .presentation_effects(requirement.presentation_mode(), successor.binding);
        successor.node_changes = source
            .frame()
            .presentation_node_changes(successor.surface, successor.binding)
            .into();
        if let Some(projection) = projection {
            successor.auxiliary =
                UiMountedPresentationAuxiliaryState::from_runtime_mounting(projection);
        }
        successor
    }

    fn apply_precise_replacements(
        &mut self,
        changes: &[worth_ui_host_contract::UiMountedPaintCommandChange],
    ) {
        for change in changes {
            let worth_ui_host_contract::UiMountedPaintCommandChange::Replace(command) = change
            else {
                continue;
            };
            let instance = command.identity().mounted_instance();
            let Some(mut bundle) = self.commands_by_instance.get(&instance).cloned() else {
                continue;
            };
            assert!(
                bundle.replace(command.clone()),
                "precise replacement preserves key and order"
            );
            self.commands_by_instance.insert(instance, bundle);
        }
    }

    fn remove_bundle(&mut self, instance: worth_ui_host_contract::UiMountedInstanceIdentity) {
        let Some(commands) = self.commands_by_instance.get(&instance).cloned() else {
            return;
        };
        let position = self
            .instance_order
            .position(instance)
            .expect("retained command instance has an order position");
        let mut keys = commands
            .iter()
            .enumerate()
            .map(|(local, command)| {
                (
                    (command.layer_semantic_order(), position, local),
                    command.identity(),
                )
            })
            .collect::<Vec<_>>();
        keys.sort_unstable_by_key(|(key, _)| *key);
        for (key, command) in keys.into_iter().rev() {
            let identity = UiMountedPaintOrderIdentity::for_command(command);
            let predecessor = self
                .command_order
                .predecessor(&key)
                .map(|(_, identity)| UiMountedPaintOrderIdentity::for_command(*identity));
            let successor = self
                .command_order
                .successor(&key)
                .map(|(_, identity)| UiMountedPaintOrderIdentity::for_command(*identity));
            self.order_integrity = self
                .order_integrity
                .remove_edge(predecessor, identity, successor)
                .expect("retained order removal preserves bounded length");
            self.command_order.remove(&key);
        }
        self.commands_by_instance.remove(&instance);
    }

    fn insert_bundle(
        &mut self,
        position: u64,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        commands: std::sync::Arc<[UiMountedPaintCommand]>,
    ) {
        if commands.is_empty() {
            return;
        }
        for (local, command) in commands.iter().enumerate() {
            let key = (command.layer_semantic_order(), position, local);
            let identity = UiMountedPaintOrderIdentity::for_command(command.identity());
            let predecessor = self
                .command_order
                .predecessor(&key)
                .map(|(_, identity)| UiMountedPaintOrderIdentity::for_command(*identity));
            let successor = self
                .command_order
                .successor(&key)
                .map(|(_, identity)| UiMountedPaintOrderIdentity::for_command(*identity));
            self.order_integrity = self
                .order_integrity
                .insert_edge(predecessor, identity, successor)
                .expect("retained order insertion preserves bounded length");
            self.command_order.insert(key, command.identity());
        }
        self.commands_by_instance.insert(
            instance,
            super::command_bundle::UiMountedPresentationCommandBundle::from_commands(&commands),
        );
    }

    pub(in crate::mounting::presentation) fn frame(
        &self,
    ) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }

    pub(super) fn command_option(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<&UiMountedPaintCommand> {
        self.commands_by_instance
            .get(&identity.mounted_instance())?
            .get(identity)
    }

    #[cfg(test)]
    pub(crate) fn install_command_lookup_probe(&mut self, commands: &[UiMountedPaintCommand]) {
        self.commands_by_instance = UiPersistentOrdMap::default();
        let instance = commands[0].identity().mounted_instance();
        self.commands_by_instance.insert(
            instance,
            super::command_bundle::UiMountedPresentationCommandBundle::from_commands(commands),
        );
    }

    #[cfg(test)]
    pub(crate) fn command_option_probe(&self, identity: UiMountedPaintCommandIdentity) -> usize {
        let _ = super::command_bundle::take_last_lookup_probes();
        assert!(self.command_option(identity).is_some());
        super::command_bundle::take_last_lookup_probes()
    }

    pub(super) fn command(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> &UiMountedPaintCommand {
        self.command_option(identity)
            .expect("paint order names a retained command")
    }

    pub(super) fn command_identities_for_instance(
        &self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> impl Iterator<Item = UiMountedPaintCommandIdentity> + '_ {
        self.commands_by_instance
            .get(&instance)
            .into_iter()
            .flat_map(super::command_bundle::UiMountedPresentationCommandBundle::iter)
            .map(UiMountedPaintCommand::identity)
    }

    pub(super) fn order_key(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<PresentationOrderKey> {
        let instance = identity.mounted_instance();
        let position = self.instance_order.position(instance)?;
        let local = self
            .commands_by_instance
            .get(&instance)?
            .position(identity)?;
        let command = self.commands_by_instance.get(&instance)?.get(identity)?;
        Some((command.layer_semantic_order(), position, local))
    }

    pub(super) fn previous_order(
        &self,
        identity: UiMountedPaintCommandIdentity,
    ) -> Option<UiMountedPaintOrderIdentity> {
        let key = self.order_key(identity)?;
        self.command_order
            .predecessor(&key)
            .map(|(_, command)| UiMountedPaintOrderIdentity::for_command(*command))
    }

    pub(super) fn order(&self) -> Vec<UiMountedPaintOrderIdentity> {
        self.command_order
            .iter()
            .map(|(_, command)| UiMountedPaintOrderIdentity::for_command(*command))
            .collect()
    }

    pub(super) fn commands(&self) -> Vec<UiMountedPaintCommand> {
        self.command_order
            .iter()
            .filter_map(|(_, identity)| self.command_option(*identity).cloned())
            .collect()
    }

    pub(super) fn source_instance_count(&self) -> usize {
        self.commands_by_instance.len()
    }
}

fn projection_row_count(projection: &UiMountedProjectionView) -> u64 {
    let rows = [
        projection.nodes().len(),
        projection.clips().rows().len(),
        projection.layers().rows().len(),
        projection.filled_rects().rows().len(),
        projection.semantic_text().rows().len(),
        projection.hit_tests().rows().len(),
        projection.paint_batches().rows().len(),
        projection.spatial_batches().rows().len(),
        projection.realtime_batches().rows().len(),
        projection.resources().entries().len(),
    ]
    .into_iter()
    .try_fold(0usize, usize::checked_add)
    .and_then(|rows| u64::try_from(rows).ok());
    rows.expect("an admitted projection row count fits u64")
}
