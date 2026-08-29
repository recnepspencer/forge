use super::RelationalRuntime;

impl RelationalRuntime {
    pub(crate) fn configure_diagnostics_for_test(
        &mut self,
        configure: impl FnOnce(&mut crate::diagnostics::data::RelationalDiagnosticsProfile),
    ) {
        configure(&mut self.config.diagnostics.profile);
        self.synchronize_preparation_configuration();
    }

    pub(crate) fn set_schema_registry_for_test(
        &mut self,
        registry: crate::schema::data::RelationalSchemaRegistry,
    ) {
        self.config.schema.registry = registry;
        self.synchronize_preparation_configuration();
    }

    pub(crate) fn set_entity_structural_identity_for_test(
        &mut self,
        entity_id: crate::identity::data::EntityId,
        structural_fingerprint: Option<crate::identity::data::StructuralFingerprint>,
        lineage_id: Option<crate::identity::data::LineageId>,
    ) -> bool {
        let mut partitions = self.partitions.write();
        let Some(partition) =
            crate::runtime::partition_entry_mut(&mut partitions, entity_id.partition_id)
        else {
            return false;
        };
        if partition.entity_arena.get(&entity_id).is_none() {
            return false;
        }
        partition.entity_arena.extra[entity_id.local_slot.0 as usize] =
            crate::storage::substrate::EntityExtra {
                structural_fingerprint,
                lineage_id,
                authoritative_aspect_state: None,
            };
        true
    }

    pub(crate) fn simulate_entity_slot_reuse_for_test(
        &mut self,
        entity_id: crate::identity::data::EntityId,
        structural_fingerprint: Option<crate::identity::data::StructuralFingerprint>,
        lineage_id: Option<crate::identity::data::LineageId>,
    ) -> Option<crate::identity::data::EntityId> {
        let replacement_version = crate::identity::data::VersionId(self.current_version_id().0 + 1);
        let mut partitions = self.partitions.write();
        let partition =
            crate::runtime::partition_entry_mut(&mut partitions, entity_id.partition_id)?;
        let arena = &mut partition.entity_arena;
        arena.get(&entity_id)?;
        let slot = entity_id.local_slot.0 as usize;
        let next_generation = arena.generations[slot] + 1;

        if let Some(current) = arena.metadata_history[slot].last_mut() {
            current.retired_at = Some(replacement_version);
        }

        arena.generations[slot] = next_generation;
        arena.lifecycle[slot] = crate::storage::data::RecordLifecycleState::Live;
        arena.metadata_history[slot].push(crate::storage::substrate::VersionedEntityMetadata {
            effective_at: replacement_version,
            retired_at: None,
            generation: next_generation,
            kind_id: arena.kind_ids[slot].expect("entity kind must exist for slot reuse test"),
            lineage_id,
            authoritative_aspect_state: None,
        });
        arena.created_at[slot] = replacement_version;
        arena.retired_at[slot] = None;
        arena.extra[slot] = crate::storage::substrate::EntityExtra {
            structural_fingerprint,
            lineage_id,
            authoritative_aspect_state: None,
        };
        arena.live_bitset.set(slot, true);
        arena.reclaimable_bitset.set(slot, false);
        self.history
            .with_ledger_mut(|ledger| ledger.next_version_id = replacement_version.0 + 1);

        Some(crate::identity::data::EntityId::new(
            entity_id.partition_id,
            entity_id.local_slot.0,
            next_generation,
        ))
    }

    pub(crate) fn entity_history_len_for_test(
        &self,
        entity_id: crate::identity::data::EntityId,
    ) -> usize {
        self.partitions
            .partition(entity_id.partition_id)
            .and_then(|partition| {
                partition
                    .entity_arena
                    .metadata_history_at(entity_id.local_slot.0 as usize)
                    .map(|history| history.len())
            })
            .unwrap_or(0)
    }

    pub(crate) fn relation_history_len_for_test(
        &self,
        relation_id: crate::identity::data::RelationId,
    ) -> usize {
        self.partitions
            .partition(relation_id.partition_id)
            .and_then(|partition| {
                partition
                    .relation_arena
                    .metadata_history_at(relation_id.local_slot.0 as usize)
                    .map(|history| history.len())
            })
            .unwrap_or(0)
    }
}
