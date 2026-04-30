use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::declarative_live::{
    declare_runtime_live_query_session_with_grouped_baseline, declare_writeback_from_live_session,
    DeclarativeLiveQueryRequest, DeclarativeLiveQuerySession, DeclarativeLiveViewShape,
    DeclarativeWritebackChange, DeclarativeWritebackIntent, DeclarativeWritebackValue,
};
use crate::identity::hash_parts;
use crate::live::{BridgeChangeSummary, BridgeFieldDelta};
use crate::runtime::{aspect_values_to_payload, ForgeQueryAspectValue};
use crate::schema_view::QuerySchemaView;
use crate::view_shape_live::{execute_live_view_shape_change, ViewShapePatchEnvelope};
use forge_relational::facade::config::RelationalRuntimeProfile;
use forge_relational::facade::identity::{EntityId, KindId, PartitionId};
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::RelationalRuntime;
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
use forge_relational::facade::{
    bridge::bridge_snapshot_identity_for_commit, runtime::RelationalRuntimeApi,
};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeDiagnosticsTier,
    BridgeExecutionPolicyClass, BridgeMappingId, BridgeMappingRegistration,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgeRequestKind,
    BridgeRuntimePolicy, BridgeSignalInvalidationDelivery, BridgeWritebackCausalityBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackEffectIdentity,
    BridgeWritebackIdempotenceClass, BridgeWritebackIdempotenceIdentity,
    BridgeWritebackOutcomeClass, CoarseRoutingMode, InvalidationSink, MappingSelector,
    RawCommittedPatchEnvelope, RelationalBridgeSourceError, RelationalCommittedPatchRequest,
    RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader, TruthWritebackAuthority,
    TruthWritebackAuthorityError, TruthWritebackReceipt, TruthWritebackRequest,
};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryAspect {
    label: String,
    payload_path: String,
}

impl ForgeQueryAspect {
    pub fn new(label: impl Into<String>, payload_path: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            payload_path: payload_path.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForgeQueryEntity {
    pub identity: String,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryMutationKind {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryMutationDelta {
    pub collection: String,
    pub entity_identity: String,
    pub kind: ForgeQueryMutationKind,
    pub aspect_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryMutationReceipt {
    pub commit_identity: String,
    pub snapshot_token: String,
    pub deltas: Vec<ForgeQueryMutationDelta>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryLiveViewHandle {
    name: String,
}

impl ForgeQueryLiveViewHandle {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryLivePatch {
    pub view_name: String,
    pub commit_identity: String,
    pub entity_identity: String,
    pub mutation_kind: ForgeQueryMutationKind,
    pub aspect_paths: Vec<String>,
    pub envelope: ViewShapePatchEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryWorkspaceError {
    message: String,
}

impl ForgeQueryWorkspaceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ForgeQueryWorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ForgeQueryWorkspaceError {}

pub struct ForgeQueryMemoryWorkspace {
    runtime: RelationalRuntime,
    kind_id: KindId,
    kind_name: String,
    next_client_key: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForgeQueryCollection {
    name: String,
    aspects: Vec<ForgeQueryAspect>,
}

impl ForgeQueryCollection {
    pub fn new(
        name: impl Into<String>,
        aspects: impl IntoIterator<Item = ForgeQueryAspect>,
    ) -> Self {
        Self {
            name: name.into(),
            aspects: aspects.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ForgeQueryCollectionRuntime {
    kind_id: KindId,
    next_client_key: u64,
}

#[derive(Debug, Clone)]
struct ForgeQueryLiveViewRuntime {
    session: DeclarativeLiveQuerySession,
    patches: Vec<ForgeQueryLivePatch>,
}

#[derive(Clone, Debug)]
struct ForgeQueryBridgeSource;

#[derive(Clone, Debug)]
struct ForgeQueryBridgeSnapshotReader {
    identity: TruthSnapshotIdentity,
}

#[derive(Clone, Debug)]
struct ForgeQueryBridgeSink;

#[derive(Clone)]
struct ForgeQueryWritebackAuthority {
    state: Arc<Mutex<ForgeQueryAuthorityState>>,
}

struct ForgeQueryAuthorityState {
    runtime: RelationalRuntime,
    pending: BTreeMap<String, ForgeQueryPendingWriteback>,
    completed: BTreeMap<String, ForgeQueryMutationReceipt>,
}

#[derive(Clone, Debug)]
struct ForgeQueryPendingWriteback {
    collection: String,
    kind: ForgeQueryMutationKind,
    aspect_paths: Vec<String>,
    operation: ForgeQueryPendingOperation,
}

#[derive(Clone, Debug)]
enum ForgeQueryPendingOperation {
    Insert {
        kind_id: KindId,
        client_key: InternedString,
        payload: Value,
    },
    Update {
        entity_id: EntityId,
        payload: Value,
    },
    Delete {
        entity_id: EntityId,
    },
}

pub struct ForgeQueryMemoryApp {
    authority_state: Arc<Mutex<ForgeQueryAuthorityState>>,
    bridge: RuntimeBridge,
    collections: BTreeMap<String, ForgeQueryCollectionRuntime>,
    entity_collections: BTreeMap<String, String>,
    live_views: BTreeMap<String, ForgeQueryLiveViewRuntime>,
}

impl ForgeQueryMemoryApp {
    pub fn compatibility_backend(
        collections: impl IntoIterator<Item = ForgeQueryCollection>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new(collections)
    }

    pub(crate) fn new(
        collections: impl IntoIterator<Item = ForgeQueryCollection>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let mut registry = RelationalSchemaRegistry::new();
        let mut collection_runtimes = BTreeMap::new();
        for (index, collection) in collections.into_iter().enumerate() {
            let kind_id = KindId(index as u32 + 1);
            let declared_aspects = collection
                .aspects
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
            registry = registry
                .register_entity_kind(EntityKindRegistration {
                    kind_id,
                    kind_name: collection.name.clone(),
                    schema_id: SchemaId("forge-query-memory-app".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    aspect_declarations: KindAspectDeclarations::new(declared_aspects),
                })
                .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
            collection_runtimes.insert(
                collection.name,
                ForgeQueryCollectionRuntime {
                    kind_id,
                    next_client_key: 0,
                },
            );
        }
        let runtime = RelationalRuntimeApi::builder()
            .profile(RelationalRuntimeProfile::CertificationCore)
            .schema_registry(registry)
            .build();
        let authority_state = Arc::new(Mutex::new(ForgeQueryAuthorityState {
            runtime,
            pending: BTreeMap::new(),
            completed: BTreeMap::new(),
        }));
        Ok(Self {
            bridge: build_query_memory_bridge(authority_state.clone())?,
            authority_state,
            collections: collection_runtimes,
            entity_collections: BTreeMap::new(),
            live_views: BTreeMap::new(),
        })
    }

    pub fn declare_live_view(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        let name = name.into();
        let grouped_baseline_members = self.grouped_baseline_members_for_request(&request);
        let session = declare_runtime_live_query_session_with_grouped_baseline(
            request,
            schema_view,
            self.snapshot_token(),
            grouped_baseline_members,
        )
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        self.live_views.insert(
            name.clone(),
            ForgeQueryLiveViewRuntime {
                session,
                patches: Vec::new(),
            },
        );
        Ok(ForgeQueryLiveViewHandle { name })
    }

    pub fn drain_live_patches(&mut self, name: &str) -> Vec<ForgeQueryLivePatch> {
        self.live_views
            .get_mut(name)
            .map(|view| std::mem::take(&mut view.patches))
            .unwrap_or_default()
    }

    pub fn live_entities(&self, name: &str) -> Vec<ForgeQueryEntity> {
        let Some(view) = self.live_views.get(name) else {
            return Vec::new();
        };
        self.entities(view.session.request().target())
    }

    pub fn insert(
        &mut self,
        collection: &str,
        payload: Value,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let collection_runtime = self.collection_mut(collection)?;
        collection_runtime.next_client_key += 1;
        let client_key = InternedString::Raw(format!(
            "{collection}:{}",
            collection_runtime.next_client_key
        ));
        let kind_id = collection_runtime.kind_id;
        let receipt = self.execute_query_writeback(
            collection,
            ForgeQueryMutationKind::Created,
            Vec::new(),
            &payload,
            ForgeQueryPendingOperation::Insert {
                kind_id,
                client_key,
                payload: payload.clone(),
            },
        );
        let receipt = receipt?;
        for entity in receipt.deltas.iter().map(|delta| &delta.entity_identity) {
            self.entity_collections
                .insert(entity.clone(), collection.to_string());
        }
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn insert_aspects(
        &mut self,
        collection: &str,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let aspect_paths = aspects
            .iter()
            .map(|aspect| aspect.aspect_path().to_string())
            .collect::<Vec<_>>();
        let payload = aspect_values_to_payload(&aspects)?;
        let collection_runtime = self.collection_mut(collection)?;
        collection_runtime.next_client_key += 1;
        let client_key = InternedString::Raw(format!(
            "{collection}:{}",
            collection_runtime.next_client_key
        ));
        let kind_id = collection_runtime.kind_id;
        let receipt = self.execute_query_writeback(
            collection,
            ForgeQueryMutationKind::Created,
            aspect_paths,
            &payload,
            ForgeQueryPendingOperation::Insert {
                kind_id,
                client_key,
                payload: payload.clone(),
            },
        )?;
        for entity in receipt.deltas.iter().map(|delta| &delta.entity_identity) {
            self.entity_collections
                .insert(entity.clone(), collection.to_string());
        }
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn update_aspect(
        &mut self,
        entity_identity: &str,
        aspect_path: &str,
        value: Value,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        set_json_path(&mut next, aspect_path, value)?;
        let collection = self
            .entity_collections
            .get(entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Updated,
            vec![aspect_path.to_string()],
            &next,
            ForgeQueryPendingOperation::Update {
                entity_id,
                payload: next.clone(),
            },
        )?;
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn update_aspects(
        &mut self,
        entity_identity: &str,
        aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        let mut aspect_paths = Vec::with_capacity(aspects.len());
        for aspect in aspects {
            aspect_paths.push(aspect.aspect_path().to_string());
            set_json_path(&mut next, aspect.aspect_path(), aspect.value().clone())?;
        }
        let collection = self
            .entity_collections
            .get(entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Updated,
            aspect_paths,
            &next,
            ForgeQueryPendingOperation::Update {
                entity_id,
                payload: next.clone(),
            },
        )?;
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn delete(
        &mut self,
        entity_identity: &str,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let entity_id = parse_entity_identity(entity_identity)?;
        let collection = self
            .entity_collections
            .get(entity_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity collection not tracked"))?;
        let receipt = self.execute_query_writeback(
            &collection,
            ForgeQueryMutationKind::Deleted,
            Vec::new(),
            &Value::String(entity_identity.to_string()),
            ForgeQueryPendingOperation::Delete { entity_id },
        )?;
        self.entity_collections.remove(entity_identity);
        self.deliver_live_patches(&receipt);
        Ok(receipt)
    }

    pub fn entities(&self, collection: &str) -> Vec<ForgeQueryEntity> {
        let Some(collection_runtime) = self.collections.get(collection) else {
            return Vec::new();
        };
        let Ok(state) = self.authority_state.lock() else {
            return Vec::new();
        };
        let Some(version_id) = state
            .runtime
            .publication()
            .latest_bundle()
            .map(|bundle| bundle.commit.version_id)
        else {
            return Vec::new();
        };
        state
            .runtime
            .read_truth()
            .project_version(version_id)
            .entity_records(collection_runtime.kind_id)
            .into_iter()
            .filter_map(|record| {
                Some(ForgeQueryEntity {
                    identity: entity_identity(record.entity_id),
                    payload: record.payload.as_json()?.clone(),
                })
            })
            .collect()
    }

    pub fn snapshot_token(&self) -> String {
        self.authority_state
            .lock()
            .map(|state| snapshot_token_from_runtime(&state.runtime))
            .unwrap_or_else(|_| "relational-snapshot:poisoned:version:0".to_string())
    }

    pub fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, view)| view.session.request().target() == delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }

    fn grouped_baseline_members_for_request(
        &self,
        request: &DeclarativeLiveQueryRequest,
    ) -> Option<Vec<(String, String)>> {
        let DeclarativeLiveViewShape::KanbanGrouped { grouping_aspect } = request.view_shape()
        else {
            return None;
        };
        let identity_path = request
            .projection()
            .iter()
            .find(|field| field.aspect() == "identity")
            .map(|field| format!("{}.{}", field.aspect(), field.field()))
            .unwrap_or_else(|| "identity.id".to_string());
        let grouping_path = request
            .projection()
            .iter()
            .find(|field| field.aspect() == grouping_aspect)
            .map(|field| format!("{}.{}", field.aspect(), field.field()))
            .unwrap_or_else(|| format!("{grouping_aspect}.value"));
        let members = self
            .entities(request.target())
            .into_iter()
            .filter_map(|entity| {
                let member = get_json_path(&entity.payload, &identity_path)
                    .and_then(json_scalar_text)
                    .unwrap_or(entity.identity);
                let lane =
                    get_json_path(&entity.payload, &grouping_path).and_then(json_scalar_text)?;
                Some((member, lane))
            })
            .collect::<Vec<_>>();
        Some(members)
    }

    fn collection_mut(
        &mut self,
        collection: &str,
    ) -> Result<&mut ForgeQueryCollectionRuntime, ForgeQueryWorkspaceError> {
        self.collections.get_mut(collection).ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!("unknown collection: {collection}"))
        })
    }

    fn latest_entity_payload(
        &self,
        entity_id: EntityId,
    ) -> Result<Option<Value>, ForgeQueryWorkspaceError> {
        let state = self
            .authority_state
            .lock()
            .map_err(|_| ForgeQueryWorkspaceError::new("query memory authority lock poisoned"))?;
        let Some(version_id) = state
            .runtime
            .publication()
            .latest_bundle()
            .map(|bundle| bundle.commit.version_id)
        else {
            return Ok(None);
        };
        Ok(state
            .runtime
            .read_truth()
            .project_version(version_id)
            .entity_record(entity_id)
            .and_then(|record| record.payload.as_json().cloned()))
    }

    fn execute_query_writeback(
        &self,
        collection: &str,
        kind: ForgeQueryMutationKind,
        aspect_paths: Vec<String>,
        payload: &Value,
        operation: ForgeQueryPendingOperation,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let session = self
            .live_views
            .values()
            .find(|view| view.session.request().target() == collection)
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "no live query session declared for writeback collection `{collection}`"
                ))
            })?;
        let changes = if aspect_paths.is_empty() {
            vec![DeclarativeWritebackChange::new(
                "mutation",
                kind.as_str(),
                DeclarativeWritebackValue::StructuredJson(payload.to_string()),
            )]
        } else {
            aspect_paths
                .iter()
                .map(|path| {
                    let (aspect, field) = split_aspect_path(path);
                    DeclarativeWritebackChange::new(
                        aspect,
                        field,
                        DeclarativeWritebackValue::StructuredJson(payload.to_string()),
                    )
                })
                .collect()
        };
        let artifact = declare_writeback_from_live_session(
            &session.session,
            DeclarativeWritebackIntent::new(changes),
        )
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let policy_contract = self
            .bridge
            .admit_policy_declaration(BridgePolicyDeclaration::new(
                BridgePolicyDeclarationIdentity::new(format!(
                    "policy:forge-query-memory:{}:{}",
                    collection,
                    artifact.artifact_digest()
                )),
                BridgeRequestKind::Authoritative,
                BridgeExecutionPolicyClass::DeterministicCanonical,
                BridgeDiagnosticsTier::Standard,
                true,
                true,
            ))
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let lowered_policy = self.bridge.lower_admitted_policy(&policy_contract);
        let contract = self
            .bridge
            .admit_writeback_declaration(
                artifact.declaration().bridge_declaration().clone(),
                &lowered_policy,
            )
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        let effect_digest = hash_parts(&[
            format!("collection:{collection}"),
            format!("kind:{}", kind.as_str()),
            format!("payload:{payload}"),
            format!("intent:{}", artifact.intent_digest()),
        ]);
        let causality = BridgeWritebackCausalityBasis::new(
            BridgeWritebackCausalityIdentity::new(format!("causality:{effect_digest}")),
            format!("truth-trigger:{effect_digest}"),
            "route:forge-query-memory",
            artifact.live_view_basis_digest(),
            self.snapshot_token(),
        );
        let effect = self.bridge.lower_writeback_effect(
            &contract,
            &causality,
            BridgeWritebackEffectIdentity::new(format!("effect:{effect_digest}")),
            format!("effect:{effect_digest}"),
        );
        let idempotence = self.bridge.classify_writeback_idempotence(
            &effect,
            &lowered_policy,
            self.snapshot_token(),
            BridgeWritebackIdempotenceIdentity::new(format!("idempotence:{effect_digest}")),
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        );
        let effect_key = format!("effect:{effect_digest}");
        {
            let mut state = self.authority_state.lock().map_err(|_| {
                ForgeQueryWorkspaceError::new("query memory authority lock poisoned")
            })?;
            state.pending.insert(
                effect_key,
                ForgeQueryPendingWriteback {
                    collection: collection.to_string(),
                    kind,
                    aspect_paths,
                    operation,
                },
            );
        }
        let (_, truth_receipt) = self
            .bridge
            .execute_writeback_authority(&contract, &effect, &idempotence)
            .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))?;
        self.authority_state
            .lock()
            .map_err(|_| ForgeQueryWorkspaceError::new("query memory authority lock poisoned"))?
            .completed
            .remove(truth_receipt.authoritative_artifact_digest())
            .ok_or_else(|| {
                ForgeQueryWorkspaceError::new(format!(
                    "writeback authority did not publish receipt `{}`",
                    truth_receipt.authoritative_artifact_digest()
                ))
            })
    }

    fn deliver_live_patches(&mut self, receipt: &ForgeQueryMutationReceipt) {
        for delta in &receipt.deltas {
            let change = bridge_change_from_delta(delta);
            for (view_name, view) in self.live_views.iter_mut() {
                if view.session.request().target() != delta.collection {
                    continue;
                }
                let Ok(execution) =
                    execute_live_view_shape_change(view.session.live_view(), &change)
                else {
                    continue;
                };
                view.session
                    .advance_live_view(execution.next_live_view().clone());
                view.patches.push(ForgeQueryLivePatch {
                    view_name: view_name.clone(),
                    commit_identity: receipt.commit_identity.clone(),
                    entity_identity: delta.entity_identity.clone(),
                    mutation_kind: delta.kind.clone(),
                    aspect_paths: delta.aspect_paths.clone(),
                    envelope: execution.patch_envelope().clone(),
                });
            }
        }
    }
}

impl crate::runtime::ForgeQueryRuntimeBackend for ForgeQueryMemoryApp {
    fn support_profile(&self) -> crate::runtime::ForgeQueryRuntimeSupportProfile {
        crate::runtime::ForgeQueryRuntimeSupportProfile::compatibility_backend()
    }

    fn admit_live_view_declaration(
        &self,
        _name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        if self.collections.contains_key(request.target()) {
            Ok(())
        } else {
            Err(ForgeQueryWorkspaceError::new(format!(
                "unknown live view collection `{}`",
                request.target()
            )))
        }
    }

    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Self::declare_live_view(self, name, request, schema_view)
    }

    #[allow(deprecated)]
    fn write(
        &mut self,
        command: crate::runtime::ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        match command {
            crate::runtime::ForgeQueryWriteCommand::Insert {
                collection,
                payload,
            } => self.insert(&collection, payload),
            crate::runtime::ForgeQueryWriteCommand::InsertAspects {
                collection,
                aspects,
            } => self.insert_aspects(&collection, aspects),
            crate::runtime::ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect_path,
                value,
            } => self.update_aspect(&entity_identity, &aspect_path, value),
            crate::runtime::ForgeQueryWriteCommand::UpdateAspects {
                entity_identity,
                aspects,
            } => self.update_aspects(&entity_identity, aspects),
            crate::runtime::ForgeQueryWriteCommand::Delete { entity_identity } => {
                self.delete(&entity_identity)
            }
        }
    }

    fn write_batch(
        &mut self,
        commands: Vec<crate::runtime::ForgeQueryWriteCommand>,
    ) -> Result<Vec<ForgeQueryMutationReceipt>, ForgeQueryWorkspaceError> {
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            receipts.push(self.write(command)?);
        }
        Ok(receipts)
    }

    fn execute_intent(
        &mut self,
        declaration: &crate::runtime::ForgeQueryIntentDeclaration,
    ) -> Result<crate::runtime::ForgeQueryIntentExecution, crate::runtime::ForgeQueryRuntimeError>
    {
        Err(crate::runtime::ForgeQueryRuntimeError::Workspace(
            ForgeQueryWorkspaceError::new(format!(
                "intent `{}` is not supported by the memory compatibility backend",
                declaration.name()
            )),
        ))
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        Self::live_entities(self, view_name)
    }

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Self::drain_live_patches(self, view_name)
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Self::affected_live_view_ids(self, receipt)
    }

    fn snapshot_token(&self) -> String {
        Self::snapshot_token(self)
    }

    fn install_live_subscription(
        &mut self,
        view_name: &str,
        activation: &crate::subscription::SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "memory-live-subscription:{}:{}",
            view_name,
            activation.activation_digest()
        ))
    }

    fn admit_preview_basis(
        &self,
        label: &str,
        effect_policy: crate::runtime::ForgeQueryEffectPolicy,
        authority: &crate::runtime::ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<crate::runtime::ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Ok(crate::runtime::ForgeQueryPreviewBasisAdmission::new(
            authority,
            label,
            effect_policy,
            ["memory-preview-basis"],
        ))
    }

    fn inspect_write_receipt(
        &self,
        receipt: &crate::runtime::ForgeQueryWriteReceipt,
        authority: &crate::runtime::ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<crate::runtime::ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(crate::runtime::ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "write-receipt",
            receipt.authority_lane(),
            ["memory-inspector-evidence"],
        ))
    }

    fn grouped_baseline_members(
        &self,
        request: &DeclarativeLiveQueryRequest,
    ) -> Result<Option<Vec<(String, String)>>, ForgeQueryWorkspaceError> {
        Ok(self.grouped_baseline_members_for_request(request))
    }
}

fn bridge_change_from_delta(delta: &ForgeQueryMutationDelta) -> BridgeChangeSummary {
    let mut change = match delta.kind {
        ForgeQueryMutationKind::Created => {
            BridgeChangeSummary::default().with_membership_transition(false, true)
        }
        ForgeQueryMutationKind::Updated => BridgeChangeSummary::default(),
        ForgeQueryMutationKind::Deleted => {
            BridgeChangeSummary::default().with_membership_transition(true, false)
        }
    };
    for path in &delta.aspect_paths {
        let (aspect, field) = path
            .split_once('.')
            .map(|(aspect, field)| (aspect.to_string(), field.to_string()))
            .unwrap_or_else(|| (path.clone(), "value".to_string()));
        change = change.with_field_delta(BridgeFieldDelta::new(
            aspect,
            field,
            None::<String>,
            None::<String>,
        ));
    }
    change
}

impl ForgeQueryMutationKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Deleted => "deleted",
        }
    }
}

fn split_aspect_path(path: &str) -> (&str, &str) {
    path.split_once('.').unwrap_or((path, "value"))
}

fn build_query_memory_bridge(
    authority_state: Arc<Mutex<ForgeQueryAuthorityState>>,
) -> Result<RuntimeBridge, ForgeQueryWorkspaceError> {
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::development())
        .with_relational_source(ForgeQueryBridgeSource)
        .with_signal_sink(ForgeQueryBridgeSink)
        .with_writeback_authority(ForgeQueryWritebackAuthority {
            state: authority_state,
        })
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("forge-query-memory"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("forge-query-memory"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))
}

impl forge_runtime_bridge::facade::CommittedPatchSource for ForgeQueryBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
            TruthSnapshotIdentity::new("snapshot:forge-query-memory"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new(
                "entity:forge-query-memory",
                "mutation",
                "value",
            )],
        ))
    }
}

impl SnapshotReadSource for ForgeQueryBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(ForgeQueryBridgeSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

impl TruthSnapshotReader for ForgeQueryBridgeSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        Ok(SnapshotReadPacketResult::new(
            self.identity.clone(),
            request
                .reads()
                .iter()
                .map(|read| SnapshotReadRecord::new(read.request_key(), Vec::new()))
                .collect(),
        ))
    }
}

impl InvalidationSink for ForgeQueryBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

impl TruthWritebackAuthority for ForgeQueryWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| TruthWritebackAuthorityError::new("query memory authority poisoned"))?;
        let pending = state
            .pending
            .remove(request.proposed_effect_digest())
            .ok_or_else(|| {
                TruthWritebackAuthorityError::new(format!(
                    "no pending query writeback for `{}`",
                    request.proposed_effect_digest()
                ))
            })?;
        let mut txn = state
            .runtime
            .begin_transaction(TransactionOptions::default());
        let batch = match pending.operation {
            ForgeQueryPendingOperation::Insert {
                kind_id,
                client_key,
                payload,
            } => WorkerIntentBatch::new("query-memory-authority-insert").push(
                MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id,
                    client_key,
                    payload: RecordPayload::StructuredJson(payload),
                })),
            ),
            ForgeQueryPendingOperation::Update { entity_id, payload } => {
                WorkerIntentBatch::new("query-memory-authority-update").push(
                    MutationIntent::Entity(EntityMutationIntent::Update(UpdateEntityIntent {
                        entity_id,
                        payload: RecordPayload::StructuredJson(payload),
                    })),
                )
            }
            ForgeQueryPendingOperation::Delete { entity_id } => WorkerIntentBatch::new(
                "query-memory-authority-delete",
            )
            .push(MutationIntent::Entity(EntityMutationIntent::Delete(
                DeleteEntityIntent { entity_id },
            ))),
        };
        txn.push_batch(batch);
        let result = txn
            .commit()
            .map_err(|error| TruthWritebackAuthorityError::new(format!("{error:?}")))?;
        let receipt = receipt_from_runtime_commit(
            &state.runtime,
            result,
            pending.collection,
            pending.kind,
            pending.aspect_paths,
        );
        let artifact_digest = format!("forge-query-authoritative:{}", receipt.commit_identity);
        state.completed.insert(artifact_digest.clone(), receipt);
        Ok(TruthWritebackReceipt::new(
            BridgeWritebackOutcomeClass::AuthoritativeCommit,
            artifact_digest,
            &request,
        ))
    }
}

fn snapshot_token_from_runtime(runtime: &RelationalRuntime) -> String {
    runtime
        .publication()
        .latest_bundle()
        .map(|bundle| {
            bridge_snapshot_identity_for_commit(bundle.commit.commit_id, bundle.commit.version_id)
                .as_str()
                .to_string()
        })
        .unwrap_or_else(|| "relational-snapshot:empty:version:0".to_string())
}

fn receipt_from_runtime_commit(
    runtime: &RelationalRuntime,
    result: forge_relational::facade::transactions::CommitResult,
    collection: String,
    kind: ForgeQueryMutationKind,
    aspect_paths: Vec<String>,
) -> ForgeQueryMutationReceipt {
    let deltas = result
        .changed_records
        .iter()
        .filter_map(|record| match record {
            forge_relational::facade::transactions::RecordRef::Entity(entity) => {
                Some(ForgeQueryMutationDelta {
                    collection: collection.clone(),
                    entity_identity: entity_identity(*entity),
                    kind: kind.clone(),
                    aspect_paths: aspect_paths.clone(),
                })
            }
            forge_relational::facade::transactions::RecordRef::Relation(_) => None,
        })
        .collect::<Vec<_>>();
    ForgeQueryMutationReceipt {
        commit_identity: format!("commit-{}", result.commit.commit_id.0),
        snapshot_token: snapshot_token_from_runtime(runtime),
        deltas,
    }
}

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
        let payload = aspect_values_to_payload(&aspects)?;
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
        let entity_id = parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        set_json_path(&mut next, aspect_path, value)?;
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
        let entity_id = parse_entity_identity(entity_identity)?;
        let current = self
            .latest_entity_payload(entity_id)?
            .ok_or_else(|| ForgeQueryWorkspaceError::new("entity not found"))?;
        let mut next = current;
        let mut aspect_paths = Vec::with_capacity(aspects.len());
        for aspect in aspects {
            aspect_paths.push(aspect.aspect_path().to_string());
            set_json_path(&mut next, aspect.aspect_path(), aspect.value().clone())?;
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
        let entity_id = parse_entity_identity(entity_identity)?;
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
        Ok(self.receipt_from_commit(result, ForgeQueryMutationKind::Deleted, Vec::new()))
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
                    identity: entity_identity(record.entity_id),
                    payload: record.payload.as_json()?.clone(),
                })
            })
            .collect()
    }

    pub fn snapshot_token(&self) -> String {
        self.runtime
            .publication()
            .latest_bundle()
            .map(|bundle| {
                bridge_snapshot_identity_for_commit(
                    bundle.commit.commit_id,
                    bundle.commit.version_id,
                )
                .as_str()
                .to_string()
            })
            .unwrap_or_else(|| "relational-snapshot:empty:version:0".to_string())
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
                        entity_identity: entity_identity(*entity),
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
        }
    }
}

fn entity_identity(entity: EntityId) -> String {
    format!(
        "entity:{}:{}:{}",
        entity.partition_id.0, entity.local_slot.0, entity.generation.0
    )
}

fn parse_entity_identity(identity: &str) -> Result<EntityId, ForgeQueryWorkspaceError> {
    let mut parts = identity.split(':');
    if parts.next() != Some("entity") {
        return Err(ForgeQueryWorkspaceError::new("expected entity identity"));
    }
    let partition = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing partition"))?
        .parse::<u32>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid partition"))?;
    let slot = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing slot"))?
        .parse::<u64>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid slot"))?;
    let generation = parts
        .next()
        .ok_or_else(|| ForgeQueryWorkspaceError::new("missing generation"))?
        .parse::<u32>()
        .map_err(|_| ForgeQueryWorkspaceError::new("invalid generation"))?;
    Ok(EntityId::new(PartitionId(partition), slot, generation))
}

fn set_json_path(
    target: &mut Value,
    path: &str,
    value: Value,
) -> Result<(), ForgeQueryWorkspaceError> {
    let mut parts = path.split('.').peekable();
    let mut current = target;
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            let object = current
                .as_object_mut()
                .ok_or_else(|| ForgeQueryWorkspaceError::new("target payload is not an object"))?;
            object.insert(part.to_string(), value);
            return Ok(());
        }
        let object = current
            .as_object_mut()
            .ok_or_else(|| ForgeQueryWorkspaceError::new("target payload is not an object"))?;
        current = object
            .entry(part.to_string())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    Err(ForgeQueryWorkspaceError::new("empty aspect path"))
}

fn get_json_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for part in path.split('.') {
        current = current.as_object()?.get(part)?;
    }
    Some(current)
}

fn json_scalar_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        Value::Null | Value::Array(_) | Value::Object(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::declarative_live::{DeclarativeLiveViewShape, DeclarativeProjectionField};
    use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
    use crate::view_shape_live::ViewShapePatchFamily;
    use serde_json::json;

    #[test]
    fn memory_app_routes_mutations_through_declared_live_views() {
        let mut app = ForgeQueryMemoryApp::new([ForgeQueryCollection::new(
            "Task",
            [
                ForgeQueryAspect::new("identity.id", "identity.id"),
                ForgeQueryAspect::new("title.value", "title.value"),
            ],
        )])
        .expect("memory app should build");
        app.declare_live_view(
            "tasks.table",
            DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
                .project(
                    DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"),
                )
                .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
                .order_by(DeclarativeProjectionField::new("title", "value")),
            QuerySchemaView::new(
                "todo-task",
                [
                    SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                    SchemaFieldView::new("title", "value", SchemaFieldKind::String),
                ],
                [],
            ),
        )
        .expect("live view should declare");

        let insert = app
            .insert(
                "Task",
                json!({
                    "identity": { "id": "" },
                    "title": { "value": "Buy milk" },
                }),
            )
            .expect("insert should commit");
        let task_id = insert.deltas[0].entity_identity.clone();
        let insert_patches = app.drain_live_patches("tasks.table");

        assert_eq!(insert_patches.len(), 1);
        assert_eq!(
            insert_patches[0].envelope.patch_family(),
            Some(ViewShapePatchFamily::TableRowPatch)
        );

        app.update_aspect(
            &task_id,
            "title.value",
            Value::String("Buy oat milk".to_string()),
        )
        .expect("narrow aspect update should commit");
        let update_patches = app.drain_live_patches("tasks.table");

        assert_eq!(update_patches.len(), 1);
        assert_eq!(update_patches[0].aspect_paths, vec!["title.value"]);
        assert_eq!(
            update_patches[0].envelope.patch_family(),
            Some(ViewShapePatchFamily::TableRowPatch)
        );
    }

    #[test]
    fn memory_app_declares_grouped_live_view_with_internal_baseline() {
        let mut app = ForgeQueryMemoryApp::new([ForgeQueryCollection::new(
            "Task",
            [
                ForgeQueryAspect::new("identity.id", "identity.id"),
                ForgeQueryAspect::new("status.lane", "status.lane"),
            ],
        )])
        .expect("memory app should build");
        app.declare_live_view(
            "tasks.table",
            DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
                .project(
                    DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"),
                )
                .project(DeclarativeProjectionField::new("status", "lane").delivered_as("status")),
            grouped_task_schema(),
        )
        .expect("table live view should declare");
        app.insert(
            "Task",
            json!({
                "identity": { "id": "task-1" },
                "status": { "lane": "todo" },
            }),
        )
        .expect("first insert should write through bridge");
        app.insert(
            "Task",
            json!({
                "identity": { "id": "task-2" },
                "status": { "lane": "doing" },
            }),
        )
        .expect("second insert should write through bridge");

        app.declare_live_view(
            "tasks.grouped",
            DeclarativeLiveQueryRequest::new(
                "Task",
                DeclarativeLiveViewShape::kanban_grouped("status"),
            )
            .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
            .project(DeclarativeProjectionField::new("status", "lane").delivered_as("status")),
            grouped_task_schema(),
        )
        .expect("grouped live view should auto-materialize baseline");

        assert_eq!(app.live_entities("tasks.grouped").len(), 2);
    }

    fn grouped_task_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "memory-grouped-task",
            [
                SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                SchemaFieldView::new("status", "lane", SchemaFieldKind::String),
            ],
            [],
        )
    }
}
