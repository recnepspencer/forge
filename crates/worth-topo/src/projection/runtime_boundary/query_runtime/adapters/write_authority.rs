use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryAspectValue, ForgeQueryCommitIdentity, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQuerySnapshotIdentity, ForgeQueryWorkspaceError,
    ForgeQueryWriteCommand, WriteAuthorityExecutionReceipt,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
    MutationIntent, RelationMutationIntent,
};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;
use forge_runtime_bridge::facade::RuntimeBridge;

mod command_lowering;
mod patch_matching;
mod write_lowering;

use self::command_lowering::lower_write_command;
use self::write_lowering::{lower_topology_entity_insert, lower_topology_relation_insert};
use super::schema_write_boundary::commit_topology_mutation_set_through_schema_runtime_boundary;
use super::write_support::{
    aspect_map, entity_id_from_query_identity, mutation_deltas_from_commit,
    mutation_deltas_from_patch_records, relation_id_from_query_identity, write_command_label,
};
use super::TopologyRuntimeBinding;

pub struct TopologyRuntimeWriteAuthority {
    binding: TopologyRuntimeBinding,
}

impl TopologyRuntimeWriteAuthority {
    pub fn new(binding: TopologyRuntimeBinding) -> Self {
        Self { binding }
    }
}

impl ForgeQueryRuntimeWriteAuthorityAdapter for TopologyRuntimeWriteAuthority {
    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let mutation_receipt = match &command {
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
                ..
            } => self.write_insert(collection.clone(), aspects.clone()),
            ForgeQueryWriteCommand::UpdateExistingAspects {
                binding, aspects, ..
            } => self.write_update_existing(binding.clone(), aspects.clone()),
            ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            } => self.write_delete_existing(binding.clone(), touched_aspect_paths.clone()),
            other => Err(ForgeQueryWorkspaceError::new(format!(
                "topology production runtime current-head slice does not admit `{}` write command yet",
                write_command_label(other)
            ))),
        }?;
        let mutation_receipt =
            self.with_bridge_authority(bridge, &command, mutation_receipt)?;
        Ok(self.build_write_authority_execution_receipt(&command, mutation_receipt))
    }

    fn write_batch(
        &mut self,
        bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<WriteAuthorityExecutionReceipt>, ForgeQueryWorkspaceError> {
        let runtime = self.runtime()?;
        let mut lowered = Vec::with_capacity(commands.len());
        let mut authored_commands = Vec::with_capacity(commands.len());
        let mut created_entities = BTreeMap::<String, EntityReference>::new();
        for command in commands {
            lowered.push(lower_write_command(
                &runtime,
                &mut created_entities,
                command.clone(),
            )?);
            authored_commands.push(command);
        }

        let commit = self.commit_mutation_intents(
            "query-runtime-mutation-group",
            lowered
                .iter()
                .flat_map(|command| command.intents.iter().cloned())
                .collect(),
        )?;

        let receipt_identity = mutation_receipt_identity_from_commit(&commit);
        let patch = commit.patch();
        let mut used_patch_indexes = std::collections::BTreeSet::new();
        lowered
            .into_iter()
            .zip(authored_commands)
            .map(|(command, authored_command)| {
                let matched_indexes = command.patch_match.matching_patch_indexes(
                    &runtime,
                    commit.envelope().commit.version_id,
                    patch,
                    &used_patch_indexes,
                );
                if matched_indexes.len() != command.expected_observable_patch_count {
                    return Err(ForgeQueryWorkspaceError::new(format!(
                        "topology production runtime expected {} observable patch records for `{}`, observed {}",
                        command.expected_observable_patch_count,
                        command.mutation_label,
                        matched_indexes.len(),
                    )));
                }
                for index in &matched_indexes {
                    used_patch_indexes.insert(*index);
                }
                let matched_patch = matched_indexes
                    .into_iter()
                    .map(|index| patch[index].clone())
                    .collect::<Vec<_>>();
                let deltas = mutation_deltas_from_patch_records(
                    &runtime,
                    commit.envelope().commit.version_id,
                    &matched_patch,
                    &command.declared_aspect_paths,
                    command.declared_target_collection.as_deref(),
                )
                .map_err(|error| {
                    ForgeQueryWorkspaceError::new(format!(
                        "topology production runtime could not derive observable query deltas for `{}`: {}",
                        command.mutation_label, error
                    ))
                })?;
                let mutation_receipt = ForgeQueryMutationReceipt::from_authoritative_parts(
                    receipt_identity.0.clone(),
                    receipt_identity.1.clone(),
                    deltas,
                );
                let mutation_receipt = self.with_bridge_authority(
                    bridge,
                    &authored_command,
                    mutation_receipt,
                )?;
                Ok(self.build_write_authority_execution_receipt(
                    &authored_command,
                    mutation_receipt,
                ))
            })
            .collect()
    }
}

impl TopologyRuntimeWriteAuthority {
    fn runtime(
        &self,
    ) -> Result<
        std::sync::Arc<std::sync::RwLock<forge_relational::facade::runtime::RelationalRuntime>>,
        ForgeQueryWorkspaceError,
    > {
        self.binding.runtime().ok_or_else(|| {
            ForgeQueryWorkspaceError::new(
                "topology snapshot certification runtime is read-only and does not admit authoritative writes",
            )
        })
    }

    fn commit_mutation_intents(
        &self,
        transaction_label: &'static str,
        intents: Vec<MutationIntent>,
    ) -> Result<forge_relational::facade::transactions::CommitResult, ForgeQueryWorkspaceError>
    {
        let runtime_handle = self.runtime()?;
        let mut runtime = runtime_handle
            .write()
            .expect("topology runtime write authority lock poisoned");
        commit_topology_mutation_set_through_schema_runtime_boundary(
            &mut runtime,
            transaction_label,
            intents,
        )
    }

    fn write_insert(
        &mut self,
        collection: String,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let runtime = self.runtime()?;
        let aspect_map = aspect_map(&aspects);
        let declared_aspect_paths = aspects
            .iter()
            .map(|aspect| aspect.aspect_path().to_string())
            .collect::<Vec<_>>();
        let intents = match collection.as_str() {
            "TopologyEntity" => lower_topology_entity_insert(&runtime, &aspect_map)?.0,
            "TopologyRelation" => vec![MutationIntent::Create(CreateIntent::Relation(
                lower_topology_relation_insert(&runtime, &aspect_map, &[], &BTreeMap::new())?,
            ))],
            other => {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "topology production runtime does not admit insert collection `{other}`"
                )))
            }
        };

        let commit = self.commit_mutation_intents("query-runtime-mutation-insert", intents)?;

        let deltas = mutation_deltas_from_commit(&runtime, &commit, &declared_aspect_paths, None)?;
        let (commit_identity, snapshot_identity) = mutation_receipt_identity_from_commit(&commit);
        Ok(ForgeQueryMutationReceipt::from_authoritative_parts(
            commit_identity,
            snapshot_identity,
            deltas,
        ))
    }

    fn write_delete_existing(
        &mut self,
        binding: forge_query::facade::ForgeQueryExistingTruthTargetBinding,
        touched_aspect_paths: Vec<String>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let runtime = self.runtime()?;
        let collection = binding
            .target_collection()
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(
                    "topology production runtime delete requires a declared target collection",
                )
            })?
            .to_string();
        let intent = match collection.as_str() {
            "TopologyEntity" => {
                MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
                    entity_id: entity_id_from_query_identity(binding.resolved_target_identity())?,
                }))
            }
            "TopologyRelation" => {
                MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                    relation_id: relation_id_from_query_identity(
                        binding.resolved_target_identity(),
                    )?,
                }))
            }
            other => {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "topology production runtime does not admit delete collection `{other}`"
                )))
            }
        };

        let commit = self.commit_mutation_intents("query-runtime-mutation-delete", vec![intent])?;

        let deltas = mutation_deltas_from_commit(
            &runtime,
            &commit,
            &touched_aspect_paths,
            Some(collection.as_str()),
        )?;
        let (commit_identity, snapshot_identity) = mutation_receipt_identity_from_commit(&commit);
        Ok(ForgeQueryMutationReceipt::from_authoritative_parts(
            commit_identity,
            snapshot_identity,
            deltas,
        ))
    }

    fn write_update_existing(
        &mut self,
        binding: forge_query::facade::ForgeQueryExistingTruthTargetBinding,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let runtime = self.runtime()?;
        let lowered = lower_write_command(
            &runtime,
            &mut BTreeMap::new(),
            ForgeQueryWriteCommand::UpdateExistingAspects {
                binding,
                aspects: aspects.clone(),
                metadata: forge_query::facade::ForgeQueryMutationMetadata::default(),
                naming_intent: None,
                continuity_intent: None,
            },
        )?;
        let declared_aspect_paths = lowered.declared_aspect_paths.clone();
        let commit =
            self.commit_mutation_intents("query-runtime-mutation-update", lowered.intents)?;

        let deltas = mutation_deltas_from_commit(
            &runtime,
            &commit,
            &declared_aspect_paths,
            Some("TopologyRelation"),
        )?;
        let (commit_identity, snapshot_identity) = mutation_receipt_identity_from_commit(&commit);
        Ok(ForgeQueryMutationReceipt::from_authoritative_parts(
            commit_identity,
            snapshot_identity,
            deltas,
        ))
    }

    fn with_bridge_authority(
        &self,
        bridge: &RuntimeBridge,
        command: &ForgeQueryWriteCommand,
        receipt: ForgeQueryMutationReceipt,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let delta = receipt.deltas().first().ok_or_else(|| {
            ForgeQueryWorkspaceError::new(
                "topology production runtime cannot attach bridge authority without mutation deltas",
            )
        })?;
        let bridge_authority = self.build_bridge_mutation_authority_bundle(
            bridge,
            receipt.snapshot_identity(),
            command,
            delta.collection(),
            delta.entity_identity(),
            delta.kind().clone(),
        )?;
        Ok(ForgeQueryMutationReceipt::from_bridge_authoritative_parts(
            receipt.commit_identity().clone(),
            receipt.snapshot_identity().clone(),
            receipt.deltas().to_vec(),
            bridge_authority,
        ))
    }
}

fn mutation_receipt_identity_from_commit(
    commit: &forge_relational::facade::transactions::CommitResult,
) -> (ForgeQueryCommitIdentity, ForgeQuerySnapshotIdentity) {
    (
        ForgeQueryCommitIdentity::from_relational_commit_id(commit.envelope().commit.commit_id.0),
        ForgeQuerySnapshotIdentity::from_relational_snapshot(
            RelationalBridgeSnapshotIdentityParts::new(
                commit.envelope().commit.commit_id.0,
                commit.envelope().commit.version_id.0,
            ),
        ),
    )
}
