use super::*;

impl<'runtime> VisibilityReadContext<'runtime> {
    pub fn entity_aspect_versions(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self.runtime.partitions.partition(entity_id.partition_id)?;
        let slot = entity_id.slot_index();
        let slot_view = partition.entity_arena.get_slot(slot)?;
        if slot_view.generation() != entity_id.generation_value()
            || slot_view.partition_id() != entity_id.partition_id
        {
            return None;
        }
        let versions = partition
            .entity_arena
            .aspect_versions_at(slot)?
            .iter()
            .filter_map(|(symbol, version)| {
                self.runtime
                    .services
                    .symbols
                    .resolve(*symbol)
                    .and_then(AspectKey::new)
                    .map(|aspect_key| (aspect_key, *version))
            })
            .collect::<Vec<_>>();
        debug_assert!(aspect_versions_are_canonical(&versions));
        Some(versions)
    }

    pub fn relation_aspect_versions(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> Option<Vec<(AspectKey, u64)>> {
        let partition = self
            .runtime
            .partitions
            .partition(relation_id.partition_id)?;
        let slot = relation_id.slot_index();
        let slot_view = partition.relation_arena.get_slot(slot)?;
        if slot_view.generation() != relation_id.generation_value()
            || slot_view.partition_id() != relation_id.partition_id
        {
            return None;
        }
        let mut resolved = partition
            .relation_arena
            .aspect_versions_at(slot)?
            .iter()
            .filter_map(|(symbol, version)| {
                self.runtime
                    .services
                    .symbols
                    .resolve(*symbol)
                    .and_then(AspectKey::new)
                    .map(|aspect_key| (aspect_key, *version))
            })
            .collect::<Vec<_>>();
        resolved.sort();
        debug_assert!(aspect_versions_are_canonical(&resolved));
        Some(resolved)
    }
}

fn aspect_versions_are_canonical(versions: &[(AspectKey, u64)]) -> bool {
    versions.windows(2).all(|window| window[0] <= window[1])
}

#[cfg(test)]
mod tests {
    use crate::facade::config::CascadeDeletePolicy;
    use crate::facade::identity::PartitionId;
    use crate::tests::support::{create_entity, runtime_with_declared_aspect_schema};

    #[test]
    fn unresolved_symbolic_aspect_versions_are_not_exposed_as_foundational_keys() {
        let mut runtime =
            runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
        let entity = create_entity(&mut runtime, "alpha");
        {
            let mut writer = runtime.edit_partitions();
            let partition = writer
                .partition_mut(PartitionId::main())
                .expect("main partition");
            let versions = partition
                .entity_arena
                .aspect_versions_at_mut(entity.slot_index())
                .expect("test entity aspect version slot is materialized");
            versions.clear();
            versions.insert(crate::symbols::data::Symbol(41), 7);
        }

        let observed = runtime
            .read_truth()
            .entity_aspect_versions(entity)
            .expect("entity versions");

        assert!(observed.is_empty());
    }
}
