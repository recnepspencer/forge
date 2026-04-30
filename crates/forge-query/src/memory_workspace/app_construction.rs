use super::*;
use crate::declarative_live::declare_runtime_live_query_session_with_grouped_baseline;
use forge_relational::facade::config::RelationalRuntimeProfile;
use forge_relational::facade::runtime::RelationalRuntimeApi;
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::symbols::InternedString;

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
            bridge: super::bridge::build_query_memory_bridge(authority_state.clone())?,
            authority_state,
            collections: collection_runtimes,
            entity_collections: BTreeMap::new(),
            live_views: BTreeMap::new(),
        })
    }

    pub fn declare_live_view(
        &mut self,
        name: impl Into<String>,
        request: crate::declarative_live::DeclarativeLiveQueryRequest,
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
                    identity: super::helpers::entity_identity(record.entity_id),
                    payload: record.payload.as_json()?.clone(),
                })
            })
            .collect()
    }

    pub fn snapshot_token(&self) -> String {
        self.authority_state
            .lock()
            .map(|state| super::helpers::snapshot_token_from_runtime(&state.runtime))
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

    pub(super) fn grouped_baseline_members_for_request(
        &self,
        request: &crate::declarative_live::DeclarativeLiveQueryRequest,
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
                let member = super::helpers::get_json_path(&entity.payload, &identity_path)
                    .and_then(super::helpers::json_scalar_text)
                    .unwrap_or(entity.identity);
                let lane = super::helpers::get_json_path(&entity.payload, &grouping_path)
                    .and_then(super::helpers::json_scalar_text)?;
                Some((member, lane))
            })
            .collect::<Vec<_>>();
        Some(members)
    }

    pub(super) fn collection_mut(
        &mut self,
        collection: &str,
    ) -> Result<&mut ForgeQueryCollectionRuntime, ForgeQueryWorkspaceError> {
        self.collections.get_mut(collection).ok_or_else(|| {
            ForgeQueryWorkspaceError::new(format!("unknown collection: {collection}"))
        })
    }

    pub(super) fn latest_entity_payload(
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
}
