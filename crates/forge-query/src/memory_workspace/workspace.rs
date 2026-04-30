use super::*;
use crate::runtime::ForgeQueryAspectValue;
use forge_relational::facade::config::RelationalRuntimeProfile;
use forge_relational::facade::identity::PartitionId;
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::RelationalRuntimeApi;
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::symbols::InternedString;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, EntityMutationIntent, EntitySpec, MutationIntent,
    TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
};

impl ForgeQueryMemoryWorkspace {
    pub fn collection(
        kind_name: impl Into<String>,
        aspects: impl IntoIterator<Item = ForgeQueryAspect>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let kind_name = kind_name.into();
        let kind_id = KindId(1);
        let declared_aspects = aspects
            .into_iter()
            .map(|aspect| DeclaredAspect {
                key: AspectKey(InternedString::Raw(aspect.label)),
                binding: AspectBinding::EntityPayloadField {
                    field: InternedString::Raw(aspect.payload_path),
                },
                comparator: AspectComparator::JsonScalarEquality,
                precision: AspectPrecision::Structured,
            })
            .collect::<Vec<_>>();
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id,
                kind_name: kind_name.clone(),
                schema_id: SchemaId("forge-query-memory".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::new(declared_aspects),
            })
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let runtime = RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(registry)
            .build();
        Ok(Self {
            runtime,
            kind_id,
            kind_name,
            next_client_key: 0,
        })
    }

    pub fn kind_name(&self) -> &str {
        &self.kind_name
    }

    pub fn insert(
        &mut self,
        payload: Value,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        self.next_client_key += 1;
        let mut txn = self
            .runtime
            .begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("query-memory-insert").push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: self.kind_id,
                    client_key: InternedString::Raw(format!(
                        "{}:{}",
                        self.kind_name, self.next_client_key
                    )),
                    payload: RecordPayload::StructuredJson(payload),
                }),
            )),
        );
        let result = txn
            .commit()
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        Ok(self.receipt_from_commit(result, ForgeQueryMutationKind::Created, Vec::new()))
    }

    pub fn insert_aspects(
        &mut self,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let aspect_paths = aspects
            .iter()
            .map(|aspect| aspect.aspect_path().to_string())
            .collect::<Vec<_>>();
        let payload = crate::runtime::aspect_values_to_payload(&aspects)?;
        self.next_client_key += 1;
        let mut txn = self
            .runtime
            .begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("query-memory-insert").push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: self.kind_id,
                    client_key: InternedString::Raw(format!(
                        "{}:{}",
                        self.kind_name, self.next_client_key
                    )),
                    payload: RecordPayload::StructuredJson(payload),
                }),
            )),
        );
        let result = txn
            .commit()
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        Ok(self.receipt_from_commit(result, ForgeQueryMutationKind::Created, aspect_paths))
    }

    pub fn update_aspect(
        &mut self,
        entity_identity: &str,
        aspect_path: &str,
        value: Value,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::helpers::parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        super::helpers::set_json_path(&mut next, aspect_path, value)?;
        let mut txn = self
            .runtime
            .begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("query-memory-update").push(MutationIntent::Entity(
                EntityMutationIntent::Update(UpdateEntityIntent {
                    entity_id,
                    payload: RecordPayload::StructuredJson(next),
                }),
            )),
        );
        let result = txn
            .commit()
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        Ok(self.receipt_from_commit(
            result,
            ForgeQueryMutationKind::Updated,
            vec![aspect_path.to_string()],
        ))
    }

    pub fn update_aspects(
        &mut self,
        entity_identity: &str,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::helpers::parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        let mut aspect_paths = Vec::with_capacity(aspects.len());
        for aspect in aspects {
            aspect_paths.push(aspect.aspect_path().to_string());
            super::helpers::set_json_path(&mut next, aspect.aspect_path(), aspect.value().clone())?;
        }
        let mut txn = self
            .runtime
            .begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("query-memory-update").push(MutationIntent::Entity(
                EntityMutationIntent::Update(UpdateEntityIntent {
                    entity_id,
                    payload: RecordPayload::StructuredJson(next),
                }),
            )),
        );
        let result = txn
            .commit()
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        Ok(self.receipt_from_commit(result, ForgeQueryMutationKind::Updated, aspect_paths))
    }

    pub fn delete(
        &mut self,
        entity_identity: &str,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::helpers::parse_entity_identity(entity_identity)?;
        let mut txn = self
            .runtime
            .begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("query-memory-delete").push(MutationIntent::Entity(
                EntityMutationIntent::Delete(DeleteEntityIntent { entity_id }),
            )),
        );
        let result = txn
            .commit()
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let mut receipt =
            self.receipt_from_commit(result, ForgeQueryMutationKind::Deleted, Vec::new());
        if receipt.deltas.is_empty() {
            receipt.deltas.push(ForgeQueryMutationDelta {
                collection: self.kind_name.clone(),
                entity_identity: entity_identity.to_string(),
                kind: ForgeQueryMutationKind::Deleted,
                aspect_paths: Vec::new(),
            });
        }
        Ok(receipt)
    }

    pub fn entities(&self) -> Vec<ForgeQueryEntity> {
        let Some(version_id) = self
            .runtime
            .publication()
            .latest_bundle()
            .map(|bundle| bundle.commit.version_id)
        else {
            return Vec::new();
        };
        self.runtime
            .read_truth()
            .project_version(version_id)
            .entity_records(self.kind_id)
            .into_iter()
            .filter_map(|record| {
                Some(ForgeQueryEntity {
                    identity: super::helpers::entity_identity(record.entity_id),
                    payload: record.payload.as_json()?.clone(),
                })
            })
            .collect()
    }

    pub fn snapshot_token(&self) -> String {
        super::helpers::snapshot_token_from_runtime(&self.runtime)
    }

    fn latest_entity_payload(
        &self,
        entity_id: EntityId,
    ) -> Result<Option<Value>, ForgeQueryWorkspaceError> {
        let Some(version_id) = self
            .runtime
            .publication()
            .latest_bundle()
            .map(|bundle| bundle.commit.version_id)
        else {
            return Ok(None);
        };
        Ok(self
            .runtime
            .read_truth()
            .project_version(version_id)
            .entity_record(entity_id)
            .and_then(|record| record.payload.as_json().cloned()))
    }

    fn receipt_from_commit(
        &self,
        result: forge_relational::facade::transactions::CommitResult,
        kind: ForgeQueryMutationKind,
        aspect_paths: Vec<String>,
    ) -> ForgeQueryMutationReceipt {
        let snapshot_token = self.snapshot_token();
        let deltas = result
            .changed_records
            .iter()
            .filter_map(|record| match record {
                forge_relational::facade::transactions::RecordRef::Entity(entity) => {
                    Some(ForgeQueryMutationDelta {
                        collection: self.kind_name.clone(),
                        entity_identity: super::helpers::entity_identity(*entity),
                        kind: kind.clone(),
                        aspect_paths: aspect_paths.clone(),
                    })
                }
                forge_relational::facade::transactions::RecordRef::Relation(_) => None,
            })
            .collect::<Vec<_>>();
        ForgeQueryMutationReceipt {
            commit_identity: format!("commit-{}", result.commit.commit_id.0),
            snapshot_token,
            deltas,
            bridge_authority: None,
        }
    }
}
