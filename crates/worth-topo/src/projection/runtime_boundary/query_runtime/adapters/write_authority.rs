use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryBackendAdmissibleMutation, ForgeQueryCommitIdentity, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQuerySnapshotIdentity, ForgeQueryWorkspaceError,
    WriteAuthorityExecutionReceipt,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{EntityReference, MutationIntent};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;
use forge_runtime_bridge::facade::RuntimeBridge;

mod command_lowering;
mod patch_matching;
mod write_lowering;

use self::command_lowering::lower_write_command;
use super::schema_write_boundary::commit_topology_mutation_set_through_schema_runtime_boundary;
use super::write_support::{mutation_deltas_from_commit, mutation_deltas_from_patch_records};
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
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        let mutation_receipt = self.write_admitted_mutation(mutation.clone())?;
        let mutation_receipt = self.with_bridge_authority(bridge, &mutation, mutation_receipt)?;
        Ok(self.build_write_authority_execution_receipt(&mutation, mutation_receipt))
    }

    fn write_batch(
        &mut self,
        bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        mutations: Vec<ForgeQueryBackendAdmissibleMutation>,
    ) -> Result<Vec<WriteAuthorityExecutionReceipt>, ForgeQueryWorkspaceError> {
        let runtime = self.runtime()?;
        let mut lowered = Vec::with_capacity(mutations.len());
        let mut admitted_mutations = Vec::with_capacity(mutations.len());
        let mut created_entities = BTreeMap::<String, EntityReference>::new();
        for mutation in mutations {
            lowered.push(lower_write_command(
                &runtime,
                &mut created_entities,
                &mutation,
            )?);
            admitted_mutations.push(mutation);
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
            .zip(admitted_mutations)
            .map(|(command, mutation)| {
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
                    &command.declared_aspect_touches,
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
                let mutation_receipt =
                    self.with_bridge_authority(bridge, &mutation, mutation_receipt)?;
                Ok(self.build_write_authority_execution_receipt(&mutation, mutation_receipt))
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

    fn write_admitted_mutation(
        &mut self,
        mutation: ForgeQueryBackendAdmissibleMutation,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let runtime = self.runtime()?;
        let lowered = lower_write_command(&runtime, &mut BTreeMap::new(), &mutation)?;
        let commit = self.commit_mutation_intents(lowered.mutation_label, lowered.intents)?;
        let deltas = mutation_deltas_from_commit(
            &runtime,
            &commit,
            &lowered.declared_aspect_touches,
            lowered.declared_target_collection.as_deref(),
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
        mutation: &ForgeQueryBackendAdmissibleMutation,
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
            mutation,
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
