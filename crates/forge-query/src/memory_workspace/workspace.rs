use super::*;
use crate::aspect_field_authoring::{
    aspect_field_patch_from_json_values, aspect_key, entity_string_field_aspect, field_key,
    lower_json_scalar_to_aspect_value, planned_single_field_locator,
    project_aspect_value_to_workspace_json, terminal_field_label,
};
use crate::runtime::ForgeQueryAspectValue;
use forge_foundational::facade::ContractValidatedAspectValueView;
use forge_relational::facade::config::RelationalRuntimeProfile;
use forge_relational::facade::identity::PartitionId;
use forge_relational::facade::runtime::RelationalRuntimeApi;
use forge_relational::facade::schema::{
    EntityKindRegistration, KindAspectDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::symbols::ClientKey;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, EntityMutationIntent, EntitySpec, MutationIntent,
    TransactionOptions, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

impl ForgeQueryMemoryWorkspace {
    pub fn collection(
        kind_name: impl Into<String>,
        aspects: impl IntoIterator<Item = ForgeQueryAspect>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let kind_name = kind_name.into();
        let kind_id = KindId(1);
        let aspects = aspects.into_iter().collect::<Vec<_>>();
        let declared_aspects = aspects
            .iter()
            .map(|aspect| {
                entity_string_field_aspect(
                    aspect.label(),
                    terminal_field_label(aspect.payload_path())?,
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(ForgeQueryWorkspaceError::new)?;
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
            aspects,
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
        let fields = self.field_patch_from_payload(&payload)?;
        self.commit_entity_create(fields, Vec::new())
    }

    pub fn insert_aspects(
        &mut self,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let aspect_paths = aspects
            .iter()
            .map(|aspect| aspect.aspect_path().to_string())
            .collect::<Vec<_>>();
        let fields = self.field_patch_from_aspect_values(&aspects)?;
        self.commit_entity_create(fields, aspect_paths)
    }

    pub fn update_aspect(
        &mut self,
        entity_identity: &str,
        aspect_path: &str,
        value: Value,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::helpers::parse_entity_identity(entity_identity)?;
        self.ensure_entity_exists(entity_id)?;
        let field_label = self.field_label_for_aspect_path(aspect_path)?;
        let mut txn = self
            .runtime
            .begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("query-memory-update").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                    entity_id,
                    fields: aspect_field_patch_from_json_values([(
                        aspect_path,
                        field_label.as_str(),
                        value,
                    )])
                    .map_err(ForgeQueryWorkspaceError::new)?,
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
        self.ensure_entity_exists(entity_id)?;
        let mut aspect_paths = Vec::with_capacity(aspects.len());
        for aspect in &aspects {
            aspect_paths.push(aspect.aspect_path().to_string());
        }
        let fields = self.field_patch_from_aspect_values(&aspects)?;
        let mut txn = self
            .runtime
            .begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("query-memory-update").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent { entity_id, fields }),
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
                    payload: self.payload_from_authoritative_aspect_state(
                        record.authoritative_aspect_state.as_ref()?,
                    ),
                })
            })
            .collect()
    }

    pub fn snapshot_token(&self) -> String {
        super::helpers::snapshot_token_from_runtime(&self.runtime)
    }

    fn ensure_entity_exists(&self, entity_id: EntityId) -> Result<(), ForgeQueryWorkspaceError> {
        let Some(version_id) = self
            .runtime
            .publication()
            .latest_bundle()
            .map(|bundle| bundle.commit.version_id)
        else {
            return Err(ForgeQueryWorkspaceError::new("entity not found"));
        };
        self.runtime
            .read_truth()
            .project_version(version_id)
            .entity_record(entity_id)
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        Ok(())
    }

    fn commit_entity_create(
        &mut self,
        fields: forge_relational::facade::transactions::AspectFieldPatch,
        aspect_paths: Vec<String>,
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
                    client_key: ClientKey::raw(format!(
                        "{}:{}",
                        self.kind_name, self.next_client_key
                    )),
                    fields,
                }),
            )),
        );
        let result = txn
            .commit()
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        Ok(self.receipt_from_commit(result, ForgeQueryMutationKind::Created, aspect_paths))
    }

    fn field_patch_from_payload(
        &self,
        payload: &Value,
    ) -> Result<forge_relational::facade::transactions::AspectFieldPatch, ForgeQueryWorkspaceError>
    {
        let mut values = Vec::new();
        for aspect in &self.aspects {
            if let Some(value) = json_path_value(payload, aspect.payload_path()) {
                values.push((
                    aspect.label(),
                    terminal_field_label(aspect.payload_path())
                        .map_err(ForgeQueryWorkspaceError::new)?,
                    value.clone(),
                ));
            }
        }
        aspect_field_patch_from_json_values(values).map_err(ForgeQueryWorkspaceError::new)
    }

    fn field_patch_from_aspect_values(
        &self,
        aspects: &[ForgeQueryAspectValue],
    ) -> Result<forge_relational::facade::transactions::AspectFieldPatch, ForgeQueryWorkspaceError>
    {
        let mut targets = std::collections::BTreeMap::new();
        for aspect in aspects {
            let declared = self.declared_aspect_for_path(aspect.aspect_path())?;
            targets.insert(
                planned_single_field_locator(
                    aspect_key(declared.label()).map_err(ForgeQueryWorkspaceError::new)?,
                    field_key(
                        terminal_field_label(declared.payload_path())
                            .map_err(ForgeQueryWorkspaceError::new)?,
                    )
                    .map_err(ForgeQueryWorkspaceError::new)?,
                ),
                lower_json_scalar_to_aspect_value(aspect.value().clone())
                    .map_err(ForgeQueryWorkspaceError::new)?,
            );
        }
        Ok(forge_relational::facade::transactions::AspectFieldPatch::from(targets))
    }

    fn payload_from_authoritative_aspect_state(
        &self,
        state: &forge_foundational::facade::AuthoritativeRecordAspectState,
    ) -> Value {
        let mut payload = Value::Object(serde_json::Map::new());
        for aspect in &self.aspects {
            let Ok(key) = aspect_key(aspect.label()) else {
                continue;
            };
            let Some(validated) = state.get(&key) else {
                continue;
            };
            let ContractValidatedAspectValueView::Scalar(value) = validated.view() else {
                continue;
            };
            let _ = super::helpers::set_json_path(
                &mut payload,
                aspect.payload_path(),
                project_aspect_value_to_workspace_json(value),
            );
        }
        payload
    }

    fn field_label_for_aspect_path(
        &self,
        aspect_path: &str,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        let field_label =
            terminal_field_label(self.declared_aspect_for_path(aspect_path)?.payload_path())
                .map_err(ForgeQueryWorkspaceError::new)
                .map(str::to_owned)?;
        Ok(field_label)
    }

    fn declared_aspect_for_path(
        &self,
        aspect_path: &str,
    ) -> Result<&ForgeQueryAspect, ForgeQueryWorkspaceError> {
        self.aspects
            .iter()
            .find(|aspect| aspect.label() == aspect_path || aspect.payload_path() == aspect_path)
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!("undeclared aspect `{aspect_path}`"))
            })
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

fn json_path_value<'a>(payload: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = payload;
    for part in path.split('.') {
        current = current.as_object()?.get(part)?;
    }
    Some(current)
}
