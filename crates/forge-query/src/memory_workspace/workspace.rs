use super::*;
use crate::aspect_field_authoring::{
    aspect_key, entity_string_field_aspect, field_key,
    lower_external_json_through_scalar_string_contract, planned_single_field_locator,
    project_aspect_value_to_workspace_json, terminal_field_label,
};
use crate::runtime::ForgeQueryAspectValue;
use forge_foundational::facade::AspectValue;
use forge_relational::facade::config::RelationalRuntimeProfile;
use forge_relational::facade::identity::PartitionId;
use forge_relational::facade::runtime::RelationalRuntimeApi;
use forge_relational::facade::runtime::{
    CustomInvariantRegistration, EntityProjectionRecord, InvariantCatalog, ProjectionAspectScope,
};
use forge_relational::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationalSchemaRegistry, SchemaId,
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
        Self::collection_with_invariants(kind_name, aspects, InvariantCatalog::default(), [])
    }

    pub(crate) fn collection_with_invariants(
        kind_name: impl Into<String>,
        aspects: impl IntoIterator<Item = ForgeQueryAspect>,
        invariant_catalog: InvariantCatalog,
        custom_invariants: impl IntoIterator<Item = CustomInvariantRegistration>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let kind_name = kind_name.into();
        let kind_id = KindId(1);
        let aspects = aspects.into_iter().collect::<Vec<_>>();
        let declared_aspects = aspects
            .iter()
            .map(|aspect| {
                entity_string_field_aspect(
                    aspect.label(),
                    terminal_field_label(aspect.external_projection_path())?,
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
                aspect_contract_declarations: KindAspectContractDeclarations::new(declared_aspects),
            })
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let mut runtime_builder = RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(registry)
            .invariant_catalog(invariant_catalog.canonicalized());
        for custom_invariant in custom_invariants {
            runtime_builder = runtime_builder.custom_invariant(custom_invariant);
        }
        let runtime = runtime_builder.build();
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

    pub fn update_aspects(
        &mut self,
        entity_identity: ForgeQueryEntityIdentity,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::runtime_identity::entity_id_from_identity(entity_identity)?;
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
        entity_identity: ForgeQueryEntityIdentity,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = super::runtime_identity::entity_id_from_identity(entity_identity.clone())?;
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
                entity_identity,
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
        let projection_scope = self.workspace_projection_scope();
        self.runtime
            .read_truth()
            .project_version(version_id)
            .entity_records_with_projection_scope(self.kind_id, projection_scope, |record| {
                let (aspect_values, external_projection) =
                    self.aspect_projection_from_projection_record(record);
                Some(ForgeQueryEntity::from_aspect_projection(
                    super::runtime_identity::entity_identity(record.entity_id()),
                    aspect_values,
                    external_projection,
                ))
            })
    }

    pub fn snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        super::runtime_identity::snapshot_identity_from_runtime(&self.runtime)
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
            .entity_record_with_projection_scope(entity_id, ProjectionAspectScope::empty(), |_| {
                Some(())
            })
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
                        terminal_field_label(declared.external_projection_path())
                            .map_err(ForgeQueryWorkspaceError::new)?,
                    )
                    .map_err(ForgeQueryWorkspaceError::new)?,
                ),
                lower_external_json_through_scalar_string_contract(
                    declared.label(),
                    aspect.value(),
                )
                .map_err(ForgeQueryWorkspaceError::new)?,
            );
        }
        Ok(forge_relational::facade::transactions::AspectFieldPatch::from(targets))
    }

    fn workspace_projection_scope(&self) -> ProjectionAspectScope {
        ProjectionAspectScope::whole_aspects(
            self.aspects
                .iter()
                .filter_map(|aspect| aspect_key(aspect.label()).ok()),
        )
    }

    fn aspect_projection_from_projection_record(
        &self,
        record: EntityProjectionRecord<'_>,
    ) -> (std::collections::BTreeMap<String, AspectValue>, Value) {
        let mut aspect_values = std::collections::BTreeMap::new();
        let mut external_projection = Value::Object(serde_json::Map::new());
        for aspect in &self.aspects {
            let Ok(key) = aspect_key(aspect.label()) else {
                continue;
            };
            let Some(value) = record.aspect_value(&key) else {
                continue;
            };
            aspect_values.insert(aspect.label().to_string(), value.clone());
            let _ = super::external_projection::set_json_path(
                &mut external_projection,
                aspect.external_projection_path(),
                project_aspect_value_to_workspace_json(value),
            );
        }
        (aspect_values, external_projection)
    }

    fn declared_aspect_for_path(
        &self,
        aspect_path: &str,
    ) -> Result<&ForgeQueryAspect, ForgeQueryWorkspaceError> {
        self.aspects
            .iter()
            .find(|aspect| {
                aspect.label() == aspect_path || aspect.external_projection_path() == aspect_path
            })
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
        let snapshot_identity = self.snapshot_identity();
        let deltas = result
            .changed_records
            .iter()
            .filter_map(|record| match record {
                forge_relational::facade::transactions::RecordRef::Entity(entity) => {
                    Some(ForgeQueryMutationDelta {
                        collection: self.kind_name.clone(),
                        entity_identity: super::runtime_identity::entity_identity(*entity),
                        kind: kind.clone(),
                        aspect_paths: aspect_paths.clone(),
                    })
                }
                forge_relational::facade::transactions::RecordRef::Relation(_) => None,
            })
            .collect::<Vec<_>>();
        ForgeQueryMutationReceipt {
            commit_identity: ForgeQueryCommitIdentity::from_relational_commit_id(
                result.commit.commit_id.0,
            ),
            snapshot_identity,
            deltas,
            bridge_authority: None,
        }
    }
}
