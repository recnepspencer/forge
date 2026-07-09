use super::*;
use crate::aspect_field_authoring::{entity_string_field_aspect, planned_field_path_locator};
use crate::runtime::{WorthQueryAdmittedAspectValue, WorthQueryAspectTouch};
use worth_foundational::facade::AspectValue;
use worth_relational::facade::config::RelationalRuntimeProfile;
use worth_relational::facade::identity::PartitionId;
use worth_relational::facade::runtime::RelationalRuntimeApi;
use worth_relational::facade::runtime::{
    CustomInvariantRegistration, EntityProjectionRecord, InvariantCatalog, ProjectionAspectScope,
};
use worth_relational::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, EntityMutationIntent, EntitySpec, MutationIntent,
    TransactionOptions, UpdateEntityFieldsIntent, WorkerIntentBatch,
};

impl WorthQueryMemoryWorkspace {
    pub fn collection(
        kind_name: impl Into<String>,
        aspects: impl IntoIterator<Item = WorthQueryAspect>,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Self::collection_with_invariants(kind_name, aspects, InvariantCatalog::default(), [])
    }

    pub(crate) fn collection_with_invariants(
        kind_name: impl Into<String>,
        aspects: impl IntoIterator<Item = WorthQueryAspect>,
        invariant_catalog: InvariantCatalog,
        custom_invariants: impl IntoIterator<Item = CustomInvariantRegistration>,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        let kind_name = kind_name.into();
        let kind_id = KindId(1);
        let aspects = aspects.into_iter().collect::<Vec<_>>();
        let declared_aspects = aspects
            .iter()
            .map(|aspect| {
                let touch = aspect.aspect_touch();
                let target = touch.parsed_target();
                let field_label = match target.field_path().and_then(single_field_label).cloned() {
                    Some(field_label) => field_label,
                    None => final_field_key(aspect.native_field_path())?.clone(),
                };
                entity_string_field_aspect(target.aspect_key().as_str(), field_label.as_str())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(WorthQueryWorkspaceError::new)?;
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id,
                kind_name: kind_name.clone(),
                schema_id: SchemaId("worth-query-memory".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_contract_declarations: KindAspectContractDeclarations::new(declared_aspects),
            })
            .map_err(|error| WorthQueryWorkspaceError::new(format!("{error:?}")))?;
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
        aspects: Vec<WorthQueryAdmittedAspectValue>,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        let touched_aspects = touches_from_aspect_values(&aspects);
        let fields = self.field_patch_from_aspect_values(&aspects)?;
        self.commit_entity_create(fields, touched_aspects)
    }

    pub fn update_aspects(
        &mut self,
        entity_identity: WorthQueryEntityIdentity,
        aspects: Vec<WorthQueryAdmittedAspectValue>,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
        let entity_id = super::runtime_identity::entity_id_from_identity(entity_identity)?;
        self.ensure_entity_exists(entity_id)?;
        let touched_aspects = touches_from_aspect_values(&aspects);
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
            .map_err(|error| WorthQueryWorkspaceError::new(format!("{error:?}")))?;
        Ok(self.receipt_from_commit(result, WorthQueryMutationKind::Updated, touched_aspects))
    }

    pub fn delete(
        &mut self,
        entity_identity: WorthQueryEntityIdentity,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
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
            .map_err(|error| WorthQueryWorkspaceError::new(format!("{error:?}")))?;
        let mut receipt =
            self.receipt_from_commit(result, WorthQueryMutationKind::Deleted, Vec::new());
        if receipt.deltas.is_empty() {
            receipt
                .deltas
                .push(WorthQueryMutationDelta::from_collection_identity(
                    self.mutation_delta_collection_identity(),
                    entity_identity,
                    WorthQueryMutationKind::Deleted,
                    Vec::new(),
                ));
        }
        Ok(receipt)
    }

    pub fn entities(&self) -> Vec<WorthQueryEntity> {
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
                let (aspect_values, native_field_values) =
                    self.aspect_projection_from_projection_record(record);
                Some(WorthQueryEntity::from_aspect_projection(
                    super::runtime_identity::entity_identity(record.entity_id()),
                    aspect_values,
                    native_field_values,
                ))
            })
    }

    pub fn snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        super::runtime_identity::snapshot_identity_from_runtime(&self.runtime)
    }

    fn ensure_entity_exists(&self, entity_id: EntityId) -> Result<(), WorthQueryWorkspaceError> {
        let Some(version_id) = self
            .runtime
            .publication()
            .latest_bundle()
            .map(|bundle| bundle.commit.version_id)
        else {
            return Err(WorthQueryWorkspaceError::new("entity not found"));
        };
        self.runtime
            .read_truth()
            .project_version(version_id)
            .entity_record_with_projection_scope(entity_id, ProjectionAspectScope::empty(), |_| {
                Some(())
            })
            .ok_or_else(|| WorthQueryWorkspaceError::new("entity not found"))?;
        Ok(())
    }

    fn commit_entity_create(
        &mut self,
        fields: worth_relational::facade::transactions::AspectFieldPatch,
        touched_aspects: Vec<WorthQueryAspectTouch>,
    ) -> Result<WorthQueryMutationReceipt, WorthQueryWorkspaceError> {
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
            .map_err(|error| WorthQueryWorkspaceError::new(format!("{error:?}")))?;
        Ok(self.receipt_from_commit(result, WorthQueryMutationKind::Created, touched_aspects))
    }

    fn field_patch_from_aspect_values(
        &self,
        aspects: &[WorthQueryAdmittedAspectValue],
    ) -> Result<worth_relational::facade::transactions::AspectFieldPatch, WorthQueryWorkspaceError>
    {
        let mut targets = std::collections::BTreeMap::new();
        for aspect in aspects {
            let aspect_touch = aspect.aspect_touch();
            let declared = self.declared_aspect_for_touch(&aspect_touch)?;
            let Some(value) = aspect.foundational_value() else {
                return Err(WorthQueryWorkspaceError::new(format!(
                    "aspect `{}` cannot clear through the memory workspace field patch path yet",
                    aspect_touch.admitted_touch_digest_part()
                )));
            };
            let target = aspect.parsed_target();
            let field_path = match target.field_path() {
                Some(path) => path.clone(),
                None => worth_foundational::facade::CanonicalFieldPath::single(
                    final_field_key(declared.native_field_path())
                        .map_err(WorthQueryWorkspaceError::new)?
                        .clone(),
                ),
            };
            targets.insert(
                planned_field_path_locator(target.aspect_key().clone(), field_path),
                value.clone(),
            );
        }
        Ok(worth_relational::facade::transactions::AspectFieldPatch::from(targets))
    }

    fn workspace_projection_scope(&self) -> ProjectionAspectScope {
        ProjectionAspectScope::whole_aspects(
            self.aspects
                .iter()
                .map(|aspect| aspect.aspect_touch().parsed_target().aspect_key().clone()),
        )
    }

    fn aspect_projection_from_projection_record(
        &self,
        record: EntityProjectionRecord<'_>,
    ) -> (
        std::collections::BTreeMap<worth_foundational::facade::AspectKey, AspectValue>,
        std::collections::BTreeMap<worth_foundational::facade::CanonicalFieldPath, AspectValue>,
    ) {
        let mut aspect_values = std::collections::BTreeMap::new();
        let mut native_field_values = std::collections::BTreeMap::new();
        for aspect in &self.aspects {
            let target = aspect.aspect_touch().parsed_target();
            let Some(value) = record.aspect_value(target.aspect_key()) else {
                continue;
            };
            aspect_values.insert(target.aspect_key().clone(), value.clone());
            native_field_values.insert(aspect.native_field_path().clone(), value.clone());
        }
        (aspect_values, native_field_values)
    }

    fn declared_aspect_for_touch(
        &self,
        aspect_touch: &WorthQueryAspectTouch,
    ) -> Result<&WorthQueryAspect, WorthQueryWorkspaceError> {
        self.aspects
            .iter()
            .find(|aspect| declared_aspect_matches_touch(aspect, aspect_touch))
            .ok_or_else(|| {
                WorthQueryWorkspaceError::new(format!(
                    "undeclared aspect `{}`",
                    aspect_touch.admitted_touch_digest_part()
                ))
            })
    }

    fn receipt_from_commit(
        &self,
        result: worth_relational::facade::transactions::CommitResult,
        kind: WorthQueryMutationKind,
        touched_aspects: Vec<WorthQueryAspectTouch>,
    ) -> WorthQueryMutationReceipt {
        let snapshot_identity = self.snapshot_identity();
        let deltas = result
            .changed_records
            .iter()
            .filter_map(|record| match record {
                worth_relational::facade::transactions::RecordRef::Entity(entity) => {
                    Some(WorthQueryMutationDelta::from_collection_identity(
                        self.mutation_delta_collection_identity(),
                        super::runtime_identity::entity_identity(*entity),
                        kind.clone(),
                        touched_aspects.clone(),
                    ))
                }
                worth_relational::facade::transactions::RecordRef::Relation(_) => None,
            })
            .collect::<Vec<_>>();
        WorthQueryMutationReceipt {
            commit_identity: WorthQueryCommitIdentity::from_relational_commit_id(
                result.commit.commit_id.0,
            ),
            snapshot_identity,
            deltas,
            bridge_authority: None,
        }
    }

    fn mutation_delta_collection_identity(
        &self,
    ) -> crate::runtime::WorthQueryMutationTargetCollectionIdentity {
        crate::runtime::WorthQueryMutationTargetCollectionIdentity::new(
            "memory-workspace-mutation-delta-collection",
            &self.kind_name,
        )
    }
}

fn touches_from_aspect_values(
    aspects: &[WorthQueryAdmittedAspectValue],
) -> Vec<WorthQueryAspectTouch> {
    aspects
        .iter()
        .map(|aspect| WorthQueryAspectTouch::from_parsed_target(aspect.parsed_target().clone()))
        .collect()
}

fn declared_aspect_matches_touch(
    aspect: &WorthQueryAspect,
    requested: &WorthQueryAspectTouch,
) -> bool {
    let field_path_alias = WorthQueryAspectTouch::aspect_field_path(
        aspect.aspect_touch().native_aspect_key().clone(),
        aspect.native_field_path().clone(),
    );
    [aspect.aspect_touch().clone(), field_path_alias]
        .into_iter()
        .any(|declared| declared.matches_or_contains(requested))
}

fn single_field_label(
    path: &worth_foundational::facade::CanonicalFieldPath,
) -> Option<&worth_foundational::facade::FieldKey> {
    let fields = path.fields();
    if fields.len() != 1 {
        return None;
    }
    fields.first()
}

fn final_field_key(
    path: &worth_foundational::facade::CanonicalFieldPath,
) -> Result<&worth_foundational::facade::FieldKey, String> {
    path.fields()
        .last()
        .ok_or_else(|| "native field path must contain a field segment".to_string())
}
