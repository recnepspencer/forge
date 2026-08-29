use worth_foundational::facade::{AspectValue, InternedString};
use worth_relational::facade::{
    identity::PartitionId,
    symbols::ClientKey,
    transactions::{CreateIntent, EntityAspectCreateIntent, MutationIntent, WorkerIntentBatch},
};

use super::{WorthQueryCommitIdentity, WorthQueryMemoryWorkspace, WorthQueryWorkspaceError};

impl WorthQueryMemoryWorkspace {
    pub(crate) fn insert_seed_rows_atomically(
        &mut self,
        rows: Vec<Vec<crate::runtime::WorthQueryAuthoredAspectMutation>>,
    ) -> Result<WorthQueryCommitIdentity, WorthQueryWorkspaceError> {
        if rows.is_empty() {
            return Err(WorthQueryWorkspaceError::new(
                "an initial seed batch must contain at least one row",
            ));
        }
        let patches = rows
            .iter()
            .map(|aspects| self.portable_creation_patch_from_authored(aspects))
            .collect::<Result<Vec<_>, _>>()?;
        let next_key = self
            .next_client_key
            .checked_add(rows.len() as u64)
            .ok_or_else(|| WorthQueryWorkspaceError::new("seed client-key space exhausted"))?;
        let batch = patches.into_iter().enumerate().fold(
            WorkerIntentBatch::new("query-memory-initial-seed"),
            |batch, (index, patch)| {
                batch.push(MutationIntent::Create(CreateIntent::EntityAspects(
                    EntityAspectCreateIntent {
                        partition_id: PartitionId::main(),
                        kind_id: self.kind_id,
                        client_key: ClientKey::raw(format!(
                            "{}:{}",
                            self.kind_name,
                            self.next_client_key + index as u64 + 1
                        )),
                        aspect_patch: patch,
                    },
                )))
            },
        );
        let main_identity = self.runtime.main_branch_identity();
        let options = self
            .runtime
            .admit_branch_basis(&main_identity)
            .map_err(super::transaction_denial::basis)?;
        let mut transaction = self
            .runtime
            .begin_branch_transaction(
                &options,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .map_err(super::transaction_denial::admission)?;
        transaction
            .push_batch(batch)
            .map_err(super::transaction_denial::staging)?;
        let result = transaction
            .commit(&self.runtime)
            .map_err(super::transaction_denial::commit)?;
        let published_snapshot = result.snapshot.clone();
        self.next_client_key = next_key;
        let identity =
            WorthQueryCommitIdentity::from_runtime_receipt_commit(result.commit.commit_id.0);
        super::commit_snapshot_closeout::release_commit_snapshot(
            &mut self.runtime,
            &published_snapshot,
        );
        Ok(identity)
    }

    pub(crate) fn seeded_identity_entities(
        &self,
        identity_touch: &crate::runtime::WorthQueryAspectTouch,
    ) -> Result<
        std::collections::BTreeMap<String, super::WorthQueryEntityIdentity>,
        WorthQueryWorkspaceError,
    > {
        let field_path = self
            .aspects
            .iter()
            .find(|aspect| aspect.aspect_touch() == identity_touch)
            .map(|aspect| aspect.native_field_path())
            .ok_or_else(|| {
                WorthQueryWorkspaceError::new(
                    "seed identity touch is not declared by the test backend schema",
                )
            })?;
        self.entities()?
            .into_iter()
            .map(|entity| {
                let identity = match entity.scalar_value_at(field_path) {
                    Some(AspectValue::String(InternedString::Raw(value))) => value.clone(),
                    _ => {
                        return Err(WorthQueryWorkspaceError::new(
                            "seed identity field must resolve to one raw string value per row",
                        ));
                    }
                };
                Ok((identity, entity.identity().clone()))
            })
            .collect()
    }
}
