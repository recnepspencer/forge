use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryAspectValue, ForgeQueryMutationReceipt, ForgeQueryRuntimeWriteAuthorityAdapter,
    ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
};
use forge_relational::facade::bridge::bridge_snapshot_identity_for_commit;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntityReference,
    MutationIntent, RelationMutationIntent, TransactionOptions, WorkerIntentBatch,
};
use forge_runtime_bridge::facade::RuntimeBridge;

mod write_lowering;

use self::write_lowering::{
    lower_topology_entity_insert, lower_topology_relation_insert, lower_write_command,
};
use super::write_support::{
    aspect_map, mutation_deltas_from_commit, mutation_deltas_from_patch_records,
    parse_entity_identity, parse_relation_identity, write_command_label,
};
use super::WorthTopologyRuntimeBinding;

pub(crate) struct WorthTopologyRuntimeWriteAuthority {
    binding: WorthTopologyRuntimeBinding,
}

impl WorthTopologyRuntimeWriteAuthority {
    pub(crate) fn new(binding: WorthTopologyRuntimeBinding) -> Self {
        Self { binding }
    }
}

impl ForgeQueryRuntimeWriteAuthorityAdapter for WorthTopologyRuntimeWriteAuthority {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        match command {
            ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
                ..
            } => self.write_insert(collection, aspects),
            ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths,
                ..
            } => self.write_delete_existing(binding, touched_aspect_paths),
            other => Err(ForgeQueryWorkspaceError::new(format!(
                "worth topology production runtime current-head slice does not admit `{}` write command yet",
                write_command_label(&other)
            ))),
        }
    }

    fn write_batch(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        let runtime = self.runtime()?;
        let mut lowered = Vec::with_capacity(commands.len());
        let mut created_entities = BTreeMap::<String, EntityReference>::new();
        for command in commands {
            lowered.push(lower_write_command(
                &runtime,
                &mut created_entities,
                command,
            )?);
        }

        let commit = {
            let runtime_handle = self.runtime()?;
            let mut runtime = runtime_handle
                .write()
                .expect("worth topology runtime write authority lock poisoned");
            let mut tx = runtime.begin_transaction(TransactionOptions::default());
            let batch = lowered
                .iter()
                .flat_map(|command| command.intents.iter().cloned())
                .fold(
                    WorkerIntentBatch::new("worth-query-runtime-atomic-batch"),
                    |batch, intent| batch.push(intent),
                );
            tx.push_batch(batch);
            tx.commit().map_err(|error| {
                ForgeQueryWorkspaceError::new(format!(
                    "worth topology production runtime write commit failed: {error:?}"
                ))
            })?
        };

        let snapshot_token = bridge_snapshot_identity_for_commit(
            commit.envelope().commit.commit_id,
            commit.envelope().commit.version_id,
        )
        .as_str()
        .to_string();
        let patch = commit.patch();
        let mut offset = 0usize;
        lowered
            .into_iter()
            .map(|command| {
                let end = offset + command.expected_patch_count;
                if end > patch.len() {
                    return Err(ForgeQueryWorkspaceError::new(format!(
                        "worth topology production runtime expected {} patch records for `{}`, observed only {} remaining",
                        command.expected_patch_count,
                        command.batch_label,
                        patch.len().saturating_sub(offset),
                    )));
                }
                let deltas = mutation_deltas_from_patch_records(
                    &runtime,
                    commit.envelope().commit.version_id,
                    &patch[offset..end],
                    &command.declared_aspect_paths,
                )?;
                offset = end;
                Ok(ForgeQueryMutationReceipt {
                    commit_identity: format!("commit-{}", commit.envelope().commit.commit_id.0),
                    snapshot_token: snapshot_token.clone(),
                    deltas,
                    bridge_authority: None,
                })
            })
            .collect()
    }
}

impl WorthTopologyRuntimeWriteAuthority {
    fn runtime(
        &self,
    ) -> Result<
        std::sync::Arc<std::sync::RwLock<forge_relational::facade::runtime::RelationalRuntime>>,
        ForgeQueryWorkspaceError,
    > {
        self.binding.runtime().ok_or_else(|| {
            ForgeQueryWorkspaceError::new(
                "worth topology snapshot certification runtime is read-only and does not admit authoritative writes",
            )
        })
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
            "WorthTopologyEntity" => lower_topology_entity_insert(&runtime, &aspect_map)?.0,
            "WorthTopologyRelation" => vec![MutationIntent::Create(CreateIntent::Relation(
                lower_topology_relation_insert(&runtime, &aspect_map, &[], &BTreeMap::new())?,
            ))],
            other => {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "worth topology production runtime does not admit insert collection `{other}`"
                )))
            }
        };

        let commit = {
            let runtime_handle = self.runtime()?;
            let mut runtime = runtime_handle
                .write()
                .expect("worth topology runtime write authority lock poisoned");
            let mut tx = runtime.begin_transaction(TransactionOptions::default());
            let batch = intents.into_iter().fold(
                WorkerIntentBatch::new("worth-query-runtime-insert"),
                |batch, intent| batch.push(intent),
            );
            tx.push_batch(batch);
            tx.commit().map_err(|error| {
                ForgeQueryWorkspaceError::new(format!(
                    "worth topology production runtime write commit failed: {error:?}"
                ))
            })?
        };

        let deltas = mutation_deltas_from_commit(&runtime, &commit, &declared_aspect_paths)?;
        Ok(ForgeQueryMutationReceipt {
            commit_identity: format!("commit-{}", commit.envelope().commit.commit_id.0),
            snapshot_token: bridge_snapshot_identity_for_commit(
                commit.envelope().commit.commit_id,
                commit.envelope().commit.version_id,
            )
            .as_str()
            .to_string(),
            deltas,
            bridge_authority: None,
        })
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
                    "worth topology production runtime delete requires a declared target collection",
                )
            })?
            .to_string();
        let intent = match collection.as_str() {
            "WorthTopologyEntity" => {
                MutationIntent::Entity(EntityMutationIntent::Delete(DeleteEntityIntent {
                    entity_id: parse_entity_identity(binding.resolved_target_identity())?,
                }))
            }
            "WorthTopologyRelation" => {
                MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
                    relation_id: parse_relation_identity(binding.resolved_target_identity())?,
                }))
            }
            other => {
                return Err(ForgeQueryWorkspaceError::new(format!(
                    "worth topology production runtime does not admit delete collection `{other}`"
                )))
            }
        };

        let commit = {
            let runtime_handle = self.runtime()?;
            let mut runtime = runtime_handle
                .write()
                .expect("worth topology runtime write authority lock poisoned");
            let mut tx = runtime.begin_transaction(TransactionOptions::default());
            tx.push_batch(WorkerIntentBatch::new("worth-query-runtime-delete").push(intent));
            tx.commit().map_err(|error| {
                ForgeQueryWorkspaceError::new(format!(
                    "worth topology production runtime delete commit failed: {error:?}"
                ))
            })?
        };

        let deltas = mutation_deltas_from_commit(&runtime, &commit, &touched_aspect_paths)?;
        Ok(ForgeQueryMutationReceipt {
            commit_identity: format!("commit-{}", commit.envelope().commit.commit_id.0),
            snapshot_token: bridge_snapshot_identity_for_commit(
                commit.envelope().commit.commit_id,
                commit.envelope().commit.version_id,
            )
            .as_str()
            .to_string(),
            deltas,
            bridge_authority: None,
        })
    }
}
