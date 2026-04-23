use std::collections::BTreeMap;
use std::marker::PhantomData;

use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;
use serde_json::Value;

use crate::basis::ResolvedSnapshotBasis;
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::memory_workspace::{
    ForgeQueryCollection, ForgeQueryEntity, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryMemoryApp, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
    ForgeQueryWorkspaceError,
};
use crate::program::{
    validate_inputs, ForgeQueryAuthorityRequirement, ForgeQueryDerivedView,
    ForgeQueryOperationInput, ForgeQueryOperationOutput, ForgeQueryProgram,
    ForgeQueryProgramEffect, ForgeQueryProgramError, ForgeQueryProgramTrace,
};
use crate::schema_view::QuerySchemaView;
use crate::view_shape::ViewShapePlanArtifact;

pub trait ForgeQueryRuntimeBackend {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError>;

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError>;

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String>;

    fn snapshot_token(&self) -> String;

    fn grouped_baseline_members(
        &self,
        _request: &DeclarativeLiveQueryRequest,
        _plan: &ViewShapePlanArtifact,
        _basis: &ResolvedSnapshotBasis,
    ) -> Result<Option<Vec<(String, String)>>, ForgeQueryWorkspaceError> {
        Ok(None)
    }
}

pub trait ForgeQueryRuntimeSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError>;

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity>;

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch>;

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String>;

    fn snapshot_token(&self) -> String;
}

pub trait ForgeQueryRuntimeWriteAuthorityAdapter {
    fn write(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError>;
}

pub trait ForgeQueryRuntimeSignalSinkAdapter {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError>;
}

#[derive(Default)]
pub struct ForgeQueryRuntimeBackendParts {
    relational_runtime: Option<RelationalRuntime>,
    runtime_bridge: Option<RuntimeBridge>,
    schema_adapter: Option<Box<dyn ForgeQueryRuntimeSchemaAdapter>>,
    source_adapter: Option<Box<dyn ForgeQueryRuntimeSourceAdapter>>,
    write_authority: Option<Box<dyn ForgeQueryRuntimeWriteAuthorityAdapter>>,
    signal_sink: Option<Box<dyn ForgeQueryRuntimeSignalSinkAdapter>>,
}

impl ForgeQueryRuntimeBackendParts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn relational_runtime(mut self, runtime: RelationalRuntime) -> Self {
        self.relational_runtime = Some(runtime);
        self
    }

    pub fn runtime_bridge(mut self, bridge: RuntimeBridge) -> Self {
        self.runtime_bridge = Some(bridge);
        self
    }

    pub fn schema_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSchemaAdapter + 'static,
    ) -> Self {
        self.schema_adapter = Some(Box::new(adapter));
        self
    }

    pub fn source_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSourceAdapter + 'static,
    ) -> Self {
        self.source_adapter = Some(Box::new(adapter));
        self
    }

    pub fn write_authority(
        mut self,
        authority: impl ForgeQueryRuntimeWriteAuthorityAdapter + 'static,
    ) -> Self {
        self.write_authority = Some(Box::new(authority));
        self
    }

    pub fn signal_sink(mut self, sink: impl ForgeQueryRuntimeSignalSinkAdapter + 'static) -> Self {
        self.signal_sink = Some(Box::new(sink));
        self
    }
}

pub struct ForgeQueryBridgeBackedRuntimeBackend {
    relational_runtime: Option<RelationalRuntime>,
    runtime_bridge: RuntimeBridge,
    schema_adapter: Box<dyn ForgeQueryRuntimeSchemaAdapter>,
    source_adapter: Box<dyn ForgeQueryRuntimeSourceAdapter>,
    write_authority: Box<dyn ForgeQueryRuntimeWriteAuthorityAdapter>,
    signal_sink: Box<dyn ForgeQueryRuntimeSignalSinkAdapter>,
}

impl ForgeQueryBridgeBackedRuntimeBackend {
    pub fn from_parts(
        parts: ForgeQueryRuntimeBackendParts,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        Ok(Self {
            relational_runtime: parts.relational_runtime,
            runtime_bridge: parts
                .runtime_bridge
                .ok_or(ForgeQueryRuntimeError::MissingRuntimeBridge)?,
            schema_adapter: parts
                .schema_adapter
                .ok_or(ForgeQueryRuntimeError::MissingSchemaAdapter)?,
            source_adapter: parts
                .source_adapter
                .ok_or(ForgeQueryRuntimeError::MissingSourceAdapter)?,
            write_authority: parts
                .write_authority
                .ok_or(ForgeQueryRuntimeError::MissingWriteAuthority)?,
            signal_sink: parts
                .signal_sink
                .ok_or(ForgeQueryRuntimeError::MissingSignalSink)?,
        })
    }
}

impl ForgeQueryRuntimeBackend for ForgeQueryBridgeBackedRuntimeBackend {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.schema_adapter
            .admit_live_view(&name, &request, &schema_view)?;
        self.source_adapter
            .declare_live_view(name, request, schema_view)
    }

    fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.write_authority.write(
            &self.runtime_bridge,
            self.relational_runtime.as_mut(),
            command,
        )?;
        self.signal_sink.route_write_receipt(&receipt)?;
        Ok(receipt)
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        self.source_adapter.live_entities(view_name)
    }

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch> {
        self.source_adapter.drain_live_patches(view_name)
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        self.source_adapter.affected_live_view_ids(receipt)
    }

    fn snapshot_token(&self) -> String {
        self.source_adapter.snapshot_token()
    }
}

#[derive(Debug)]
pub enum ForgeQueryRuntimeError {
    MissingBackend,
    MissingRuntimeBridge,
    MissingSchemaAdapter,
    MissingSourceAdapter,
    MissingWriteAuthority,
    MissingSignalSink,
    Workspace(ForgeQueryWorkspaceError),
    Program(ForgeQueryProgramError),
    UnknownProgram(String),
    UnknownOperation {
        program_id: String,
        operation_id: String,
    },
    MissingLiveView(String),
    UnsupportedAuthority(String),
}

impl std::fmt::Display for ForgeQueryRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingBackend => {
                write!(
                    f,
                    "forge query runtime builder requires a backend, for example in_memory_collections(...)"
                )
            }
            Self::MissingRuntimeBridge => write!(
                f,
                "forge query runtime backend parts require a RuntimeBridge"
            ),
            Self::MissingSchemaAdapter => write!(
                f,
                "forge query runtime backend parts require a schema adapter"
            ),
            Self::MissingSourceAdapter => write!(
                f,
                "forge query runtime backend parts require a source adapter"
            ),
            Self::MissingWriteAuthority => write!(
                f,
                "forge query runtime backend parts require a write authority adapter"
            ),
            Self::MissingSignalSink => write!(
                f,
                "forge query runtime backend parts require a signal sink adapter"
            ),
            Self::Workspace(error) => write!(f, "{error}"),
            Self::Program(error) => write!(f, "{error}"),
            Self::UnknownProgram(program) => write!(f, "unknown query program `{program}`"),
            Self::UnknownOperation {
                program_id,
                operation_id,
            } => write!(
                f,
                "unknown query operation `{operation_id}` in program `{program_id}`"
            ),
            Self::MissingLiveView(view) => write!(f, "unknown live view `{view}`"),
            Self::UnsupportedAuthority(authority) => {
                write!(
                    f,
                    "authority requirement `{authority}` is not admitted by this runtime"
                )
            }
        }
    }
}

impl std::error::Error for ForgeQueryRuntimeError {}

impl From<ForgeQueryWorkspaceError> for ForgeQueryRuntimeError {
    fn from(value: ForgeQueryWorkspaceError) -> Self {
        Self::Workspace(value)
    }
}

impl From<ForgeQueryProgramError> for ForgeQueryRuntimeError {
    fn from(value: ForgeQueryProgramError) -> Self {
        Self::Program(value)
    }
}

#[derive(Default)]
pub struct ForgeQueryRuntimeBuilder {
    backend: Option<Result<Box<dyn ForgeQueryRuntimeBackend>, ForgeQueryRuntimeError>>,
    backend_parts: ForgeQueryRuntimeBackendParts,
}

impl ForgeQueryRuntimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_memory_collections(
        mut self,
        collections: impl IntoIterator<Item = ForgeQueryCollection>,
    ) -> Self {
        self.backend = Some(
            ForgeQueryMemoryApp::new(collections)
                .map(|backend| Box::new(backend) as Box<dyn ForgeQueryRuntimeBackend>)
                .map_err(ForgeQueryRuntimeError::Workspace),
        );
        self
    }

    pub fn backend(mut self, backend: impl ForgeQueryRuntimeBackend + 'static) -> Self {
        self.backend = Some(Ok(Box::new(backend)));
        self
    }

    pub fn relational_runtime(mut self, runtime: RelationalRuntime) -> Self {
        self.backend_parts = self.backend_parts.relational_runtime(runtime);
        self
    }

    pub fn runtime_bridge(mut self, bridge: RuntimeBridge) -> Self {
        self.backend_parts = self.backend_parts.runtime_bridge(bridge);
        self
    }

    pub fn schema_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSchemaAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.schema_adapter(adapter);
        self
    }

    pub fn source_adapter(
        mut self,
        adapter: impl ForgeQueryRuntimeSourceAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.source_adapter(adapter);
        self
    }

    pub fn write_authority(
        mut self,
        authority: impl ForgeQueryRuntimeWriteAuthorityAdapter + 'static,
    ) -> Self {
        self.backend_parts = self.backend_parts.write_authority(authority);
        self
    }

    pub fn signal_sink(mut self, sink: impl ForgeQueryRuntimeSignalSinkAdapter + 'static) -> Self {
        self.backend_parts = self.backend_parts.signal_sink(sink);
        self
    }

    pub fn build_backend_from_parts(mut self) -> Self {
        self.backend = Some(
            ForgeQueryBridgeBackedRuntimeBackend::from_parts(self.backend_parts)
                .map(|backend| Box::new(backend) as Box<dyn ForgeQueryRuntimeBackend>),
        );
        self.backend_parts = ForgeQueryRuntimeBackendParts::new();
        self
    }

    pub fn build(self) -> Result<ForgeQueryRuntime, ForgeQueryRuntimeError> {
        let backend = self
            .backend
            .ok_or(ForgeQueryRuntimeError::MissingBackend)??;
        Ok(ForgeQueryRuntime {
            backend,
            installed_programs: BTreeMap::new(),
            run_traces: BTreeMap::new(),
            derived_views: BTreeMap::new(),
            next_run_id: 0,
        })
    }
}

pub struct ForgeQueryRuntime {
    backend: Box<dyn ForgeQueryRuntimeBackend>,
    installed_programs: BTreeMap<String, ForgeQueryProgram>,
    run_traces: BTreeMap<String, ForgeQueryProgramTrace>,
    derived_views: BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    next_run_id: u64,
}

struct ForgeQueryDerivedViewRuntime {
    declaration: ForgeQueryDerivedView,
    patches: Vec<ForgeQueryDerivedPatch>,
    materialization: ForgeQueryDerivedViewMaterialization,
    maintainer: Option<Box<dyn ForgeQueryDerivedViewMaintainer>>,
}

impl ForgeQueryRuntime {
    pub fn builder() -> ForgeQueryRuntimeBuilder {
        ForgeQueryRuntimeBuilder::new()
    }

    pub fn declare_live_view<T>(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
        let handle = self
            .backend
            .declare_live_view(name.into(), request, schema_view)?;
        Ok(ForgeQueryLiveView::new(handle))
    }

    pub fn declare_derived_view(
        &mut self,
        view: ForgeQueryDerivedView,
    ) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
        self.insert_derived_runtime(view.clone(), None);
        Ok(view)
    }

    pub fn declare_maintained_derived_view<T>(
        &mut self,
        view: ForgeQueryDerivedView,
        maintainer: impl ForgeQueryDerivedViewMaintainer + 'static,
    ) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
        let name = view.name().to_string();
        self.insert_derived_runtime(view, Some(Box::new(maintainer)));
        Ok(ForgeQueryDerivedViewHandle::new(name))
    }

    fn insert_derived_runtime(
        &mut self,
        view: ForgeQueryDerivedView,
        maintainer: Option<Box<dyn ForgeQueryDerivedViewMaintainer>>,
    ) {
        self.derived_views.insert(
            view.name().to_string(),
            ForgeQueryDerivedViewRuntime {
                declaration: view.clone(),
                patches: Vec::new(),
                materialization: ForgeQueryDerivedViewMaterialization::default(),
                maintainer,
            },
        );
    }

    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let receipt = self.backend.write(command)?;
        let affected_live_view_ids = self.backend.affected_live_view_ids(&receipt);
        let (affected_derived_view_ids, refresh_fallback) =
            self.route_derived_view_patches(&receipt);
        Ok(ForgeQueryWriteReceipt::from_mutation_receipt(
            receipt,
            affected_live_view_ids,
            affected_derived_view_ids,
            refresh_fallback,
        ))
    }

    pub fn read_live<T>(&self, view: &ForgeQueryLiveView<T>) -> Vec<ForgeQueryEntity> {
        self.backend.live_entities(view.name())
    }

    pub fn drain_patches<T>(&mut self, view: &ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        ForgeQueryPatchBatch {
            view_name: view.name().to_string(),
            live_patches: self.backend.drain_live_patches(view.name()),
            derived_patch_notes: Vec::new(),
            derived_patches: Vec::new(),
        }
    }

    pub fn drain_derived_patches(&mut self, view_name: &str) -> ForgeQueryPatchBatch {
        let derived_patches = self
            .derived_views
            .get_mut(view_name)
            .map(|view| std::mem::take(&mut view.patches))
            .unwrap_or_default();
        ForgeQueryPatchBatch {
            view_name: view_name.to_string(),
            live_patches: Vec::new(),
            derived_patch_notes: derived_patches
                .iter()
                .map(ForgeQueryDerivedPatch::note)
                .collect(),
            derived_patches,
        }
    }

    pub fn read_derived<T>(&self, view: &ForgeQueryDerivedViewHandle<T>) -> Vec<Value> {
        self.derived_views
            .get(view.name())
            .map(|runtime| runtime.materialization.rows().to_vec())
            .unwrap_or_default()
    }

    pub fn snapshot_token(&self) -> String {
        self.backend.snapshot_token()
    }

    pub fn install_program(
        &mut self,
        program: ForgeQueryProgram,
    ) -> Result<ForgeQueryInstalledProgram, ForgeQueryRuntimeError> {
        let program_id = program.id().to_string();
        self.installed_programs.insert(program_id.clone(), program);
        Ok(ForgeQueryInstalledProgram { program_id })
    }

    pub fn run_operation(
        &mut self,
        operation: ForgeQueryInstalledOperation,
        inputs: Vec<ForgeQueryOperationInput>,
    ) -> Result<ForgeQueryRunReceipt, ForgeQueryRuntimeError> {
        let query_operation = self.installed_query_operation(&operation)?;
        admit_authority_requirements(query_operation.authority_requirements())?;
        let bound_inputs = validate_inputs(&query_operation, &inputs)?;
        let mut trace = ForgeQueryProgramTrace::new(
            operation.program_id.clone(),
            operation.operation_id.clone(),
            &bound_inputs,
            query_operation
                .authority_requirements()
                .iter()
                .cloned()
                .collect(),
        );
        let mut outputs = Vec::new();
        let mut write_receipts = Vec::new();
        let mut patch_batches = Vec::new();
        for effect in query_operation.effects() {
            match effect.clone() {
                ForgeQueryProgramEffect::DeclareLiveView {
                    name,
                    request,
                    schema_view,
                } => {
                    let _: ForgeQueryLiveView<Value> =
                        self.declare_live_view(name.clone(), request, schema_view)?;
                    trace.record_declaration(format!("live:{name}"));
                }
                ForgeQueryProgramEffect::DeclareDerivedView(view) => {
                    let name = view.name().to_string();
                    self.declare_derived_view(view)?;
                    trace.record_declaration(format!("derived:{name}"));
                }
                ForgeQueryProgramEffect::Write(command) => {
                    let receipt = self.write(command)?;
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::WriteTemplate(template) => {
                    let command = template.bind(&bound_inputs)?;
                    let receipt = self.write(command)?;
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::ReadLive { view_name } => {
                    let rows = self.backend.live_entities(&view_name);
                    outputs.push(ForgeQueryOperationOutput::new(
                        format!("live:{view_name}"),
                        Value::Array(rows.into_iter().map(|row| row.payload).collect()),
                    ));
                    trace.record_replay_or_parity(format!("read-live:{view_name}"));
                }
                ForgeQueryProgramEffect::DrainPatches { view_name } => {
                    let live_patches = self.backend.drain_live_patches(&view_name);
                    for patch in &live_patches {
                        trace.record_patch_artifact(format!(
                            "{}:{}",
                            patch.view_name, patch.commit_identity
                        ));
                    }
                    patch_batches.push(ForgeQueryPatchBatch {
                        view_name,
                        live_patches,
                        derived_patch_notes: Vec::new(),
                        derived_patches: Vec::new(),
                    });
                }
            }
        }
        let run_id = self.next_run_identity(&operation);
        self.run_traces.insert(run_id.clone(), trace);
        Ok(ForgeQueryRunReceipt {
            run_id,
            operation,
            outputs,
            write_receipts,
            patch_batches,
        })
    }

    fn installed_query_operation(
        &self,
        operation: &ForgeQueryInstalledOperation,
    ) -> Result<crate::program::ForgeQueryOperation, ForgeQueryRuntimeError> {
        let program = self
            .installed_programs
            .get(&operation.program_id)
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownProgram(operation.program_id.clone()))?;
        program
            .operation(&operation.operation_id)
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownOperation {
                program_id: operation.program_id.clone(),
                operation_id: operation.operation_id.clone(),
            })
            .cloned()
    }

    fn next_run_identity(&mut self, operation: &ForgeQueryInstalledOperation) -> String {
        self.next_run_id += 1;
        format!(
            "query-run:{}:{}:{}",
            operation.program_id, operation.operation_id, self.next_run_id
        )
    }

    pub fn inspect_run(
        &self,
        run: &ForgeQueryRunReceipt,
    ) -> Result<ForgeQueryProgramTrace, ForgeQueryRuntimeError> {
        self.run_traces
            .get(run.run_id())
            .cloned()
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownProgram(run.run_id().to_string()))
    }

    pub fn inspect_receipt<'a>(
        &'a self,
        receipt: &'a ForgeQueryWriteReceipt,
    ) -> ForgeQueryArtifactInspector<'a> {
        ForgeQueryArtifactInspector { receipt }
    }

    pub fn preview<'a>(&'a mut self, label: impl Into<String>) -> ForgeQueryPreviewSession<'a> {
        ForgeQueryPreviewSession {
            label: label.into(),
            runtime: self,
            pending_commands: Vec::new(),
            writes: Vec::new(),
            promoted: false,
            discarded: false,
        }
    }

    fn route_derived_view_patches(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> (Vec<String>, bool) {
        let mut affected = Vec::new();
        let mut refresh_fallback = false;
        for view in self.derived_views.values_mut() {
            for delta in &receipt.deltas {
                let relevant = delta.aspect_paths.is_empty()
                    || delta.aspect_paths.iter().any(|aspect_path| {
                        view.declaration
                            .dependency_aspects()
                            .iter()
                            .any(|dependency| aspect_path.starts_with(dependency))
                    });
                if relevant {
                    affected.push(view.declaration.name().to_string());
                    let patch = if let Some(maintainer) = view.maintainer.as_mut() {
                        maintainer.maintain(&view.declaration, delta, &mut view.materialization)
                    } else if view.declaration.incremental() {
                        ForgeQueryDerivedPatch::incremental(
                            view.declaration.name(),
                            receipt.commit_identity.clone(),
                            delta.entity_identity.clone(),
                            delta.aspect_paths.clone(),
                            Value::Null,
                        )
                    } else {
                        ForgeQueryDerivedPatch::whole_refresh_fallback(
                            view.declaration.name(),
                            receipt.commit_identity.clone(),
                            "derived view declared whole-refresh fallback",
                        )
                    };
                    if patch.is_refresh_fallback() {
                        refresh_fallback = true;
                    }
                    view.patches.push(patch);
                }
            }
        }
        affected.sort();
        affected.dedup();
        (affected, refresh_fallback)
    }
}

fn admit_authority_requirements(
    requirements: &std::collections::BTreeSet<ForgeQueryAuthorityRequirement>,
) -> Result<(), ForgeQueryRuntimeError> {
    for requirement in requirements {
        match requirement {
            ForgeQueryAuthorityRequirement::ReadOnly
            | ForgeQueryAuthorityRequirement::Live
            | ForgeQueryAuthorityRequirement::BranchLocal
            | ForgeQueryAuthorityRequirement::Previewable
            | ForgeQueryAuthorityRequirement::Writeback
            | ForgeQueryAuthorityRequirement::ReplayRequired => {}
            ForgeQueryAuthorityRequirement::Merge | ForgeQueryAuthorityRequirement::Destructive => {
                return Err(ForgeQueryRuntimeError::UnsupportedAuthority(
                    requirement.as_str().to_string(),
                ));
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommand {
    Insert {
        collection: String,
        payload: Value,
    },
    UpdateAspect {
        entity_identity: String,
        aspect_path: String,
        value: Value,
    },
    Delete {
        entity_identity: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceipt {
    inner: ForgeQueryMutationReceipt,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    refresh_fallback: bool,
}

impl ForgeQueryWriteReceipt {
    fn from_mutation_receipt(
        inner: ForgeQueryMutationReceipt,
        affected_live_view_ids: Vec<String>,
        affected_derived_view_ids: Vec<String>,
        refresh_fallback: bool,
    ) -> Self {
        Self {
            inner,
            affected_live_view_ids,
            affected_derived_view_ids,
            refresh_fallback,
        }
    }

    fn preview(
        label: &str,
        sequence: usize,
        command: &ForgeQueryWriteCommand,
        snapshot_token: String,
    ) -> Self {
        let delta = match command {
            ForgeQueryWriteCommand::Insert {
                collection,
                payload: _,
            } => crate::memory_workspace::ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: format!("preview:{label}:{sequence}"),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths: Vec::new(),
            },
            ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect_path,
                value: _,
            } => crate::memory_workspace::ForgeQueryMutationDelta {
                collection: "preview".to_string(),
                entity_identity: entity_identity.clone(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec![aspect_path.clone()],
            },
            ForgeQueryWriteCommand::Delete { entity_identity } => {
                crate::memory_workspace::ForgeQueryMutationDelta {
                    collection: "preview".to_string(),
                    entity_identity: entity_identity.clone(),
                    kind: ForgeQueryMutationKind::Deleted,
                    aspect_paths: Vec::new(),
                }
            }
        };
        Self {
            inner: ForgeQueryMutationReceipt {
                commit_identity: format!("preview:{label}:{sequence}"),
                snapshot_token,
                deltas: vec![delta],
            },
            affected_live_view_ids: Vec::new(),
            affected_derived_view_ids: Vec::new(),
            refresh_fallback: false,
        }
    }

    pub fn commit_identity(&self) -> &str {
        &self.inner.commit_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.inner.snapshot_token
    }

    pub fn deltas(&self) -> &[crate::memory_workspace::ForgeQueryMutationDelta] {
        &self.inner.deltas
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub fn into_inner(self) -> ForgeQueryMutationReceipt {
        self.inner
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryPatchBatch {
    pub view_name: String,
    pub live_patches: Vec<ForgeQueryLivePatch>,
    pub derived_patch_notes: Vec<String>,
    pub derived_patches: Vec<ForgeQueryDerivedPatch>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedViewMaterialization {
    rows: Vec<Value>,
}

impl Default for ForgeQueryDerivedViewMaterialization {
    fn default() -> Self {
        Self { rows: Vec::new() }
    }
}

impl ForgeQueryDerivedViewMaterialization {
    pub fn rows(&self) -> &[Value] {
        &self.rows
    }

    pub fn replace_rows(&mut self, rows: impl IntoIterator<Item = Value>) {
        self.rows = rows.into_iter().collect();
    }

    pub fn push_row(&mut self, row: Value) {
        self.rows.push(row);
    }

    pub fn retain_rows(&mut self, mut predicate: impl FnMut(&Value) -> bool) {
        self.rows.retain(|row| predicate(row));
    }
}

pub trait ForgeQueryDerivedViewMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryDerivedPatchFamily {
    Incremental,
    RefreshFallback,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedPatch {
    view_name: String,
    commit_identity: String,
    entity_identity: Option<String>,
    aspect_paths: Vec<String>,
    family: ForgeQueryDerivedPatchFamily,
    payload: Value,
    reason: Option<String>,
}

impl ForgeQueryDerivedPatch {
    pub fn incremental(
        view_name: impl Into<String>,
        commit_identity: impl Into<String>,
        entity_identity: impl Into<String>,
        aspect_paths: impl IntoIterator<Item = String>,
        payload: Value,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity: commit_identity.into(),
            entity_identity: Some(entity_identity.into()),
            aspect_paths: aspect_paths.into_iter().collect(),
            family: ForgeQueryDerivedPatchFamily::Incremental,
            payload,
            reason: None,
        }
    }

    pub fn whole_refresh_fallback(
        view_name: impl Into<String>,
        commit_identity: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            commit_identity: commit_identity.into(),
            entity_identity: None,
            aspect_paths: Vec::new(),
            family: ForgeQueryDerivedPatchFamily::RefreshFallback,
            payload: Value::Null,
            reason: Some(reason.into()),
        }
    }

    pub fn note(&self) -> String {
        match self.family {
            ForgeQueryDerivedPatchFamily::Incremental => format!(
                "incremental:{}:{}",
                self.commit_identity,
                self.entity_identity.as_deref().unwrap_or("unknown")
            ),
            ForgeQueryDerivedPatchFamily::RefreshFallback => format!(
                "whole-refresh-fallback:{}:{}",
                self.commit_identity,
                self.reason.as_deref().unwrap_or("unspecified")
            ),
        }
    }

    pub fn is_refresh_fallback(&self) -> bool {
        self.family == ForgeQueryDerivedPatchFamily::RefreshFallback
    }

    pub fn payload(&self) -> &Value {
        &self.payload
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDerivedViewHandle<T = Value> {
    name: String,
    marker: PhantomData<T>,
}

impl<T> ForgeQueryDerivedViewHandle<T> {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveView<T = Value> {
    handle: ForgeQueryLiveViewHandle,
    marker: PhantomData<T>,
}

impl<T> ForgeQueryLiveView<T> {
    fn new(handle: ForgeQueryLiveViewHandle) -> Self {
        Self {
            handle,
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.handle.name()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledProgram {
    program_id: String,
}

impl ForgeQueryInstalledProgram {
    pub fn program_id(&self) -> &str {
        &self.program_id
    }

    pub fn operation(
        &self,
        operation_id: impl Into<String>,
    ) -> Result<ForgeQueryInstalledOperation, ForgeQueryRuntimeError> {
        Ok(ForgeQueryInstalledOperation {
            program_id: self.program_id.clone(),
            operation_id: operation_id.into(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInstalledOperation {
    program_id: String,
    operation_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRunReceipt {
    run_id: String,
    operation: ForgeQueryInstalledOperation,
    outputs: Vec<ForgeQueryOperationOutput>,
    write_receipts: Vec<ForgeQueryWriteReceipt>,
    patch_batches: Vec<ForgeQueryPatchBatch>,
}

impl ForgeQueryRunReceipt {
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn outputs(&self) -> &[ForgeQueryOperationOutput] {
        &self.outputs
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn patch_batches(&self) -> &[ForgeQueryPatchBatch] {
        &self.patch_batches
    }
}

pub struct ForgeQueryPreviewSession<'a> {
    label: String,
    runtime: &'a mut ForgeQueryRuntime,
    pending_commands: Vec<ForgeQueryWriteCommand>,
    writes: Vec<ForgeQueryWriteReceipt>,
    promoted: bool,
    discarded: bool,
}

impl<'a> ForgeQueryPreviewSession<'a> {
    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let receipt = ForgeQueryWriteReceipt::preview(
            &self.label,
            self.pending_commands.len() + 1,
            &command,
            self.runtime.snapshot_token(),
        );
        self.pending_commands.push(command);
        self.writes.push(receipt.clone());
        Ok(receipt)
    }

    pub fn run_operation(
        &mut self,
        operation: ForgeQueryInstalledOperation,
        inputs: Vec<ForgeQueryOperationInput>,
    ) -> Result<ForgeQueryRunReceipt, ForgeQueryRuntimeError> {
        let query_operation = self.runtime.installed_query_operation(&operation)?;
        admit_authority_requirements(query_operation.authority_requirements())?;
        let bound_inputs = validate_inputs(&query_operation, &inputs)?;
        let mut trace = ForgeQueryProgramTrace::new(
            operation.program_id.clone(),
            operation.operation_id.clone(),
            &bound_inputs,
            query_operation
                .authority_requirements()
                .iter()
                .cloned()
                .collect(),
        );
        trace.record_replay_or_parity(format!("preview-session:{}", self.label));
        let mut outputs = Vec::new();
        let mut write_receipts = Vec::new();
        let mut patch_batches = Vec::new();

        for effect in query_operation.effects() {
            match effect.clone() {
                ForgeQueryProgramEffect::DeclareLiveView {
                    name,
                    request,
                    schema_view,
                } => {
                    let _: ForgeQueryLiveView<Value> =
                        self.runtime
                            .declare_live_view(name.clone(), request, schema_view)?;
                    trace.record_declaration(format!("preview-live:{name}"));
                }
                ForgeQueryProgramEffect::DeclareDerivedView(view) => {
                    let name = view.name().to_string();
                    self.runtime.declare_derived_view(view)?;
                    trace.record_declaration(format!("preview-derived:{name}"));
                }
                ForgeQueryProgramEffect::Write(command) => {
                    let receipt = self.stage_command(command);
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::WriteTemplate(template) => {
                    let command = template.bind(&bound_inputs)?;
                    let receipt = self.stage_command(command);
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::ReadLive { view_name } => {
                    let rows = self.runtime.backend.live_entities(&view_name);
                    outputs.push(ForgeQueryOperationOutput::new(
                        format!("preview-live:{view_name}"),
                        Value::Array(rows.into_iter().map(|row| row.payload).collect()),
                    ));
                    trace.record_replay_or_parity(format!("preview-read-live:{view_name}"));
                }
                ForgeQueryProgramEffect::DrainPatches { view_name } => {
                    patch_batches.push(ForgeQueryPatchBatch {
                        view_name,
                        live_patches: Vec::new(),
                        derived_patch_notes: vec![format!(
                            "preview:{}:patch-drain-deferred",
                            self.label
                        )],
                        derived_patches: Vec::new(),
                    });
                }
            }
        }

        let run_id = self.runtime.next_run_identity(&operation);
        self.runtime.run_traces.insert(run_id.clone(), trace);
        self.writes.extend(write_receipts.iter().cloned());
        Ok(ForgeQueryRunReceipt {
            run_id,
            operation,
            outputs,
            write_receipts,
            patch_batches,
        })
    }

    pub fn compare_to_authoritative(&self) -> ForgeQueryPreviewDiff {
        ForgeQueryPreviewDiff {
            label: self.label.clone(),
            write_count: self.writes.len(),
            changed_entity_count: self
                .writes
                .iter()
                .flat_map(|receipt| receipt.deltas())
                .filter(|delta| {
                    matches!(
                        delta.kind,
                        ForgeQueryMutationKind::Created
                            | ForgeQueryMutationKind::Updated
                            | ForgeQueryMutationKind::Deleted
                    )
                })
                .count(),
        }
    }

    pub fn promote(mut self) -> ForgeQueryPreviewOutcome {
        let mut promoted_writes = 0;
        for command in std::mem::take(&mut self.pending_commands) {
            if let Ok(receipt) = self.runtime.write(command) {
                self.writes.push(receipt);
                promoted_writes += 1;
            }
        }
        self.promoted = true;
        ForgeQueryPreviewOutcome {
            label: self.label,
            promoted: self.promoted,
            discarded: self.discarded,
            write_count: promoted_writes,
        }
    }

    pub fn discard(mut self) -> ForgeQueryPreviewOutcome {
        self.discarded = true;
        ForgeQueryPreviewOutcome {
            label: self.label,
            promoted: self.promoted,
            discarded: self.discarded,
            write_count: self.writes.len(),
        }
    }
}

impl<'a> ForgeQueryPreviewSession<'a> {
    fn stage_command(&mut self, command: ForgeQueryWriteCommand) -> ForgeQueryWriteReceipt {
        let receipt = ForgeQueryWriteReceipt::preview(
            &self.label,
            self.pending_commands.len() + 1,
            &command,
            self.runtime.snapshot_token(),
        );
        self.pending_commands.push(command);
        receipt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewDiff {
    pub label: String,
    pub write_count: usize,
    pub changed_entity_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewOutcome {
    pub label: String,
    pub promoted: bool,
    pub discarded: bool,
    pub write_count: usize,
}

pub struct ForgeQueryArtifactInspector<'a> {
    receipt: &'a ForgeQueryWriteReceipt,
}

impl<'a> ForgeQueryArtifactInspector<'a> {
    pub fn canonical(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "canonical",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn workflow(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "workflow",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn bridge_authority(&self) -> ForgeQueryInspectedArtifact {
        ForgeQueryInspectedArtifact::new(
            "bridge-authority",
            self.receipt.commit_identity(),
            self.receipt.snapshot_token(),
        )
    }

    pub fn live_patch_artifacts(&self) -> Vec<String> {
        self.receipt
            .deltas()
            .iter()
            .map(|delta| format!("{}:{}", delta.collection, delta.entity_identity))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInspectedArtifact {
    family: String,
    identity: String,
    basis: String,
}

impl ForgeQueryInspectedArtifact {
    fn new(
        family: impl Into<String>,
        identity: impl Into<String>,
        basis: impl Into<String>,
    ) -> Self {
        Self {
            family: family.into(),
            identity: identity.into(),
            basis: basis.into(),
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn basis(&self) -> &str {
        &self.basis
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::declarative_live::{DeclarativeLiveViewShape, DeclarativeProjectionField};
    use crate::program::{
        ForgeQueryOperation, ForgeQueryPortType, ForgeQueryProgramSource, ForgeQuerySchemaAdapter,
        ForgeQueryTypedPort, ForgeQueryValueExpr, ForgeQueryWriteCommandTemplate,
    };
    use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
    use forge_runtime_bridge::facade::{
        BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId,
        BridgeMappingRegistration, CoarseRoutingMode, InvalidationSink, MappingSelector,
        RawCommittedPatchEnvelope, RelationalBridgeSourceError, RelationalCommittedPatchRequest,
        RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket,
        SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity,
        TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
        TruthSnapshotReader,
    };

    #[test]
    fn runtime_builder_rejects_missing_backend_inputs() {
        let error = match ForgeQueryRuntime::builder().build() {
            Ok(_) => panic!("builder should reject missing v1 backend"),
            Err(error) => error,
        };

        assert!(matches!(error, ForgeQueryRuntimeError::MissingBackend));
    }

    #[test]
    fn runtime_builder_rejects_incomplete_backend_parts() {
        let error = ForgeQueryRuntime::builder()
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing bridge should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingRuntimeBridge
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing schema adapter should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingSchemaAdapter
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing source adapter should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingSourceAdapter
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing write authority should reject"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            ForgeQueryRuntimeError::MissingWriteAuthority
        ));

        let error = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .build_backend_from_parts()
            .build();
        let error = match error {
            Ok(_) => panic!("missing signal sink should reject"),
            Err(error) => error,
        };
        assert!(matches!(error, ForgeQueryRuntimeError::MissingSignalSink));
    }

    #[test]
    fn runtime_builder_accepts_bridge_backed_backend_parts() {
        let mut runtime = ForgeQueryRuntime::builder()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .signal_sink(TestSignalSink)
            .build_backend_from_parts()
            .build()
            .expect("complete backend parts should build");
        let view: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("external.tasks", task_live_request(), task_schema())
            .expect("external backend should declare live view");
        let receipt = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "external-1" },
                    "title": { "value": "External task" },
                }),
            })
            .expect("external write authority should execute");

        assert_eq!(view.name(), "external.tasks");
        assert_eq!(receipt.commit_identity(), "external-commit-1");
        assert_eq!(
            receipt.affected_live_view_ids(),
            &["external.tasks".to_string()]
        );
    }

    #[test]
    fn runtime_declares_live_view_and_routes_minimal_write_patches() {
        let mut runtime = task_runtime();
        let view: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("tasks.table", task_live_request(), task_schema())
            .expect("live view should declare");

        let insert = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Buy milk" },
                }),
            })
            .expect("insert should execute through runtime facade");
        let task_id = insert.deltas()[0].entity_identity.clone();
        let insert_patches = runtime.drain_patches(&view);

        assert_eq!(insert.deltas().len(), 1);
        assert!(insert.deltas()[0].aspect_paths.is_empty());
        assert_eq!(
            insert.affected_live_view_ids(),
            &["tasks.table".to_string()]
        );
        assert_eq!(insert_patches.live_patches.len(), 1);

        let update = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: task_id,
                aspect_path: "title.value".to_string(),
                value: Value::String("Buy oat milk".to_string()),
            })
            .expect("update should execute through runtime facade");
        let update_patches = runtime.drain_patches(&view);

        assert_eq!(update.deltas()[0].aspect_paths, vec!["title.value"]);
        assert_eq!(update_patches.live_patches.len(), 1);
    }

    #[test]
    fn compiled_typed_program_installs_runs_and_emits_trace() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        let run = runtime
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new(
                    "title",
                    Value::String("Typed task".to_string()),
                )],
            )
            .expect("program should run");
        let trace = runtime.inspect_run(&run).expect("trace should be retained");

        assert_eq!(trace.operation_id(), "create_task");
        assert_eq!(run.outputs()[0].name(), "live:tasks.table");
        assert_eq!(run.outputs()[0].value()[0]["title"]["value"], "Typed task");
        assert!(trace
            .generated_declarations()
            .iter()
            .any(|declaration| declaration == "live:tasks.table"));
        assert_eq!(trace.write_receipts().len(), 1);
        assert_eq!(trace.patch_artifacts().len(), 1);
    }

    #[test]
    fn compiled_typed_program_rejects_type_mismatch_before_execution() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        let error = runtime
            .run_operation(
                operation,
                vec![ForgeQueryOperationInput::new("title", Value::Bool(true))],
            )
            .expect_err("type mismatch should reject before effects execute");

        assert!(matches!(error, ForgeQueryRuntimeError::Program(_)));
    }

    #[test]
    fn preview_run_operation_stages_compiled_writes_until_promote() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        let preview_run = {
            let mut preview = runtime.preview("draft create");
            let run = preview
                .run_operation(
                    operation.clone(),
                    vec![ForgeQueryOperationInput::new(
                        "title",
                        Value::String("Preview-only task".to_string()),
                    )],
                )
                .expect("preview operation should run");

            assert_eq!(run.write_receipts().len(), 1);
            assert!(run.write_receipts()[0]
                .commit_identity()
                .starts_with("preview:draft create"));
            run
        };

        assert_eq!(
            preview_run.outputs()[0].value().as_array().unwrap().len(),
            0
        );

        {
            let mut preview = runtime.preview("promote create");
            preview
                .run_operation(
                    operation,
                    vec![ForgeQueryOperationInput::new(
                        "title",
                        Value::String("Promoted preview task".to_string()),
                    )],
                )
                .expect("preview operation should stage");
            let outcome = preview.promote();
            assert!(outcome.promoted);
            assert_eq!(outcome.write_count, 1);
        }

        let view = runtime
            .declare_live_view::<Value>("tasks.after-preview", task_live_request(), task_schema())
            .expect("live view should declare");
        let rows = runtime.read_live(&view);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].payload["title"]["value"], "Promoted preview task");
    }

    #[test]
    fn preview_run_operation_discard_keeps_authoritative_state_unchanged() {
        let mut runtime = task_runtime();
        let program = ForgeQueryProgram::compile(FakeDsl, &FakeSchemaAdapter)
            .expect("fake DSL should compile");
        let installed = runtime
            .install_program(program)
            .expect("program should install");
        let operation = installed
            .operation("create_task")
            .expect("operation ref should build");

        {
            let mut preview = runtime.preview("discard create");
            preview
                .run_operation(
                    operation,
                    vec![ForgeQueryOperationInput::new(
                        "title",
                        Value::String("Discarded preview task".to_string()),
                    )],
                )
                .expect("preview operation should stage");
            let outcome = preview.discard();
            assert!(outcome.discarded);
        }

        let view = runtime
            .declare_live_view::<Value>("tasks.after-discard", task_live_request(), task_schema())
            .expect("live view should declare");
        assert!(runtime.read_live(&view).is_empty());
    }

    #[test]
    fn derived_view_receives_narrow_or_fallback_patch_notes() {
        let mut runtime = task_runtime();
        let _: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("tasks.table", task_live_request(), task_schema())
            .expect("live view should declare");
        runtime
            .declare_derived_view(
                ForgeQueryDerivedView::new("task_titles", ["title".to_string()])
                    .whole_refresh_fallback(),
            )
            .expect("derived view should declare");
        let insert = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "Derived task" },
                }),
            })
            .expect("insert should route to derived view");
        let update = runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: insert.deltas()[0].entity_identity.clone(),
                aspect_path: "title.value".to_string(),
                value: Value::String("Derived task renamed".to_string()),
            })
            .expect("title update should route to derived view");

        let patches = runtime.drain_derived_patches("task_titles");

        assert_eq!(
            update.affected_derived_view_ids(),
            &["task_titles".to_string()]
        );
        assert!(update.refresh_fallback());
        assert!(patches
            .derived_patch_notes
            .iter()
            .any(|note| note.starts_with("whole-refresh-fallback")));
    }

    #[test]
    fn maintained_derived_view_materializes_incremental_patches() {
        let mut runtime = task_runtime();
        let _: ForgeQueryLiveView<Value> = runtime
            .declare_live_view("tasks.table", task_live_request(), task_schema())
            .expect("live view should declare");
        let titles = runtime
            .declare_maintained_derived_view::<Value>(
                ForgeQueryDerivedView::new("task_titles", ["title".to_string()]),
                TitleListMaintainer,
            )
            .expect("maintained derived view should declare");

        let insert = runtime
            .write(ForgeQueryWriteCommand::Insert {
                collection: "Task".to_string(),
                payload: json!({
                    "identity": { "id": "" },
                    "title": { "value": "First title" },
                }),
            })
            .expect("insert should route derived patch");
        let patches = runtime.drain_derived_patches(titles.name());

        assert_eq!(
            insert.affected_derived_view_ids(),
            &["task_titles".to_string()]
        );
        let expected_row = Value::String(insert.deltas()[0].entity_identity.clone());
        assert_eq!(runtime.read_derived(&titles), vec![expected_row.clone()]);
        assert_eq!(patches.derived_patches.len(), 1);
        assert_eq!(patches.derived_patches[0].payload(), &expected_row);

        runtime
            .write(ForgeQueryWriteCommand::UpdateAspect {
                entity_identity: insert.deltas()[0].entity_identity.clone(),
                aspect_path: "identity.id".to_string(),
                value: Value::String("ignored".to_string()),
            })
            .expect("irrelevant update should not route derived patch");
        let irrelevant = runtime.drain_derived_patches(titles.name());

        assert!(irrelevant.derived_patches.is_empty());
    }

    struct FakeDsl;

    struct FakeSchemaAdapter;

    struct TitleListMaintainer;

    impl ForgeQueryDerivedViewMaintainer for TitleListMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = Value::String(delta.entity_identity.clone());
            materialization.push_row(row.clone());
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                "derived-test-commit",
                delta.entity_identity.clone(),
                delta.aspect_paths.clone(),
                row,
            )
        }
    }

    impl ForgeQuerySchemaAdapter for FakeSchemaAdapter {
        fn schema_view(&self, operation_id: &str) -> Option<QuerySchemaView> {
            (operation_id == "create_task").then(task_schema)
        }
    }

    impl ForgeQueryProgramSource for FakeDsl {
        fn compile_program<A>(
            self,
            schema_adapter: &A,
        ) -> Result<ForgeQueryProgram, ForgeQueryProgramError>
        where
            A: ForgeQuerySchemaAdapter + ?Sized,
        {
            let schema_view = schema_adapter
                .schema_view("create_task")
                .ok_or_else(|| ForgeQueryProgramError::new("missing schema for create_task"))?;
            ForgeQueryProgram::new(
                "fake.strict.dsl",
                [ForgeQueryOperation::new("create_task")
                    .with_input(ForgeQueryTypedPort::new(
                        "title",
                        ForgeQueryPortType::String,
                    ))
                    .requires(ForgeQueryAuthorityRequirement::Live)
                    .requires(ForgeQueryAuthorityRequirement::Writeback)
                    .with_effect(ForgeQueryProgramEffect::DeclareLiveView {
                        name: "tasks.table".to_string(),
                        request: task_live_request(),
                        schema_view,
                    })
                    .with_effect(ForgeQueryProgramEffect::WriteTemplate(
                        ForgeQueryWriteCommandTemplate::Insert {
                            collection: "Task".to_string(),
                            payload: ForgeQueryValueExpr::object([
                                (
                                    "identity".to_string(),
                                    ForgeQueryValueExpr::object([(
                                        "id".to_string(),
                                        ForgeQueryValueExpr::literal(Value::String(String::new())),
                                    )]),
                                ),
                                (
                                    "title".to_string(),
                                    ForgeQueryValueExpr::object([(
                                        "value".to_string(),
                                        ForgeQueryValueExpr::input("title"),
                                    )]),
                                ),
                            ]),
                        },
                    ))
                    .with_effect(ForgeQueryProgramEffect::ReadLive {
                        view_name: "tasks.table".to_string(),
                    })
                    .with_effect(ForgeQueryProgramEffect::DrainPatches {
                        view_name: "tasks.table".to_string(),
                    })],
            )
        }
    }

    #[derive(Default)]
    struct TestSourceAdapter {
        live_views: BTreeMap<String, String>,
    }

    impl ForgeQueryRuntimeSchemaAdapter for TestSchemaAdapter {
        fn admit_live_view(
            &self,
            _name: &str,
            _request: &DeclarativeLiveQueryRequest,
            _schema_view: &QuerySchemaView,
        ) -> Result<(), ForgeQueryWorkspaceError> {
            Ok(())
        }
    }

    struct TestSchemaAdapter;

    impl ForgeQueryRuntimeSourceAdapter for TestSourceAdapter {
        fn declare_live_view(
            &mut self,
            name: String,
            request: DeclarativeLiveQueryRequest,
            _schema_view: QuerySchemaView,
        ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
            self.live_views
                .insert(name.clone(), request.target().to_string());
            Ok(ForgeQueryLiveViewHandle::new(name))
        }

        fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
            Vec::new()
        }

        fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
            Vec::new()
        }

        fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
            let mut affected = receipt
                .deltas
                .iter()
                .flat_map(|delta| {
                    self.live_views
                        .iter()
                        .filter(move |(_, collection)| *collection == &delta.collection)
                        .map(|(name, _)| name.clone())
                })
                .collect::<Vec<_>>();
            affected.sort();
            affected.dedup();
            affected
        }

        fn snapshot_token(&self) -> String {
            "external-snapshot".to_string()
        }
    }

    struct TestWriteAuthority;

    impl ForgeQueryRuntimeWriteAuthorityAdapter for TestWriteAuthority {
        fn write(
            &mut self,
            _bridge: &RuntimeBridge,
            _relational_runtime: Option<&mut RelationalRuntime>,
            command: ForgeQueryWriteCommand,
        ) -> Result<ForgeQueryMutationReceipt, ForgeQueryWorkspaceError> {
            let collection = match command {
                ForgeQueryWriteCommand::Insert { collection, .. } => collection,
                ForgeQueryWriteCommand::UpdateAspect { .. } => "Task".to_string(),
                ForgeQueryWriteCommand::Delete { .. } => "Task".to_string(),
            };
            Ok(ForgeQueryMutationReceipt {
                commit_identity: "external-commit-1".to_string(),
                snapshot_token: "external-snapshot-1".to_string(),
                deltas: vec![crate::memory_workspace::ForgeQueryMutationDelta {
                    collection,
                    entity_identity: "external-entity-1".to_string(),
                    kind: ForgeQueryMutationKind::Created,
                    aspect_paths: Vec::new(),
                }],
            })
        }
    }

    struct TestSignalSink;

    impl ForgeQueryRuntimeSignalSinkAdapter for TestSignalSink {
        fn route_write_receipt(
            &mut self,
            _receipt: &ForgeQueryMutationReceipt,
        ) -> Result<(), ForgeQueryWorkspaceError> {
            Ok(())
        }
    }

    #[derive(Clone, Debug)]
    struct TestBridgeSource;

    impl forge_runtime_bridge::facade::CommittedPatchSource for TestBridgeSource {
        fn load_committed_patch(
            &self,
            request: RelationalCommittedPatchRequest,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            Ok(RawCommittedPatchEnvelope::new(
                TruthCommitIdentity::new(request.commit_identity()),
                TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
                TruthSnapshotIdentity::new("external-snapshot"),
                TruthBranchIdentity::new("main"),
                vec![BridgeCommittedPatchItem::new("entity", "aspect", "field")],
            ))
        }
    }

    impl SnapshotReadSource for TestBridgeSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
            Ok(Box::new(TestSnapshotReader {
                identity: identity.clone(),
            }))
        }
    }

    struct TestSnapshotReader {
        identity: TruthSnapshotIdentity,
    }

    impl TruthSnapshotReader for TestSnapshotReader {
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

    struct TestBridgeSink;

    impl InvalidationSink for TestBridgeSink {
        fn deliver_invalidation(
            &self,
            delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
        ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
            Ok(BridgeDeliveryReceipt::new(
                delivery.invalidation_targets().len(),
                delivery.source_snapshot().clone(),
            ))
        }
    }

    fn test_bridge() -> RuntimeBridge {
        RuntimeBridgeBuilder::new()
            .with_relational_source(TestBridgeSource)
            .with_signal_sink(TestBridgeSink)
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::new("external-test"),
                TruthPatchScope::new(
                    MappingSelector::any(),
                    MappingSelector::any(),
                    MappingSelector::any(),
                ),
                SignalInvalidationScope::new("external-test"),
                CoarseRoutingMode::Direct,
            ))
            .build()
            .expect("test bridge should build")
    }

    fn task_runtime() -> ForgeQueryRuntime {
        ForgeQueryRuntime::builder()
            .in_memory_collections([ForgeQueryCollection::new(
                "Task",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                ],
            )])
            .build()
            .expect("runtime should build")
    }

    fn task_live_request() -> DeclarativeLiveQueryRequest {
        DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
            .project(DeclarativeProjectionField::new("identity", "id").delivered_as("identity.id"))
            .project(DeclarativeProjectionField::new("title", "value").delivered_as("title"))
            .order_by(DeclarativeProjectionField::new("title", "value"))
    }

    fn task_schema() -> QuerySchemaView {
        QuerySchemaView::new(
            "runtime-task",
            [
                SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
                SchemaFieldView::new("title", "value", SchemaFieldKind::String),
            ],
            [],
        )
    }
}
