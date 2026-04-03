use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use forge_signal::facade::adapters::{
    branch_state_proof_report, merge_plan_proof_report, merge_result_proof_report,
    replay_artifact_proof_report, replay_parity_proof_report, runtime_proof_report,
    BranchStateDenseGridProofBasis, BranchStateProofBasis, BranchStateProofReport,
    MergePlanProofReport, MergeResultProofReport, ReplayArtifactProofInput,
    ReplayArtifactProofReport, ReplayParityProofReport, RuntimeProofReport,
    BRANCH_STATE_PROOF_BASIS_VERSION,
};
use forge_signal::facade::adapters::{ArtifactMergeAction, BranchMergePlan, BranchMergeResult};
use forge_signal::facade::history::RuntimeSnapshot;
use forge_signal::facade::history::{RuntimeBranch, RuntimeBranchId};
use forge_signal::facade::specialist::EvaluationOutput;
use forge_signal::facade::specialist::EvaluationVerdict;
use forge_signal::facade::{
    Aspect, AspectVersion, DependencyEdge, EvaluationContext, NodeEvaluationResult, NodeId,
    OutputChange, SignalError, SignalGraph, SignalRuntime as NativeRuntime,
};

use crate::boundary::errors::ForgeSignalJsError;
use crate::expression::evaluation::ExprEnvironment;
use crate::expression::model::{IdentitySpec, SignalValue};
use crate::recipe::model::{
    KeyedRecipeFamilySpec, KeyedSetValue, KeyedSourceFamilySpec, RecipeFamilyReadSpec, RecipeSpec,
    SourceSpec, TransactionOp,
};
use crate::runtime::adapters::{
    MergePlanProofEnvelope, MergeResultProofEnvelope, RuntimeDefinitionEnvelope, RuntimeEnvelope,
};
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::specialist::VersionSummary;
use crate::runtime::summaries::{
    HealthSummary, LineageSummary, ReplaySummary, RunSummary, RuntimeSnapshotEnvelope,
    RuntimeStoreSnapshot, StoredRecipeSnapshot, StoredSourceSnapshot, WhySummary,
};

const DEFAULT_ASPECT: Aspect = Aspect::new(0);
#[cfg(target_arch = "wasm32")]
const WASM_DEBUG_LOGS: bool = false;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(message: &str);
}

fn perf_now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        return js_sys::Date::now();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::sync::OnceLock;
        use std::time::Instant;

        static START: OnceLock<Instant> = OnceLock::new();
        let start = START.get_or_init(Instant::now);
        return start.elapsed().as_secs_f64() * 1000.0;
    }
}

#[cfg(target_arch = "wasm32")]
fn wasm_debug(message: impl AsRef<str>) {
    if WASM_DEBUG_LOGS {
        console_log(message.as_ref());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn wasm_debug(_message: impl AsRef<str>) {}

type SharedStore = Arc<Mutex<RuntimeStore>>;
pub type SharedCore = Rc<RefCell<RuntimeCore>>;
type WasmRuntime = NativeRuntime<(), (), (), SharedStore, ()>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MergePolicyPreviewRequest {
    pub source_branch_id: u64,
    pub target_branch_id: u64,
    #[serde(default)]
    pub conflict_policy_name: Option<String>,
    #[serde(default)]
    pub conflict_isolation_policy_name: Option<String>,
    #[serde(default)]
    pub identity_matcher_name: Option<String>,
    #[serde(default)]
    pub deletion_policy_name: Option<String>,
}

#[derive(Debug, Clone)]
struct CatalogEntry {
    node: NodeId,
}

#[derive(Debug, Clone)]
struct StoredSource {
    value: SignalValue,
    version: u64,
}

#[derive(Debug, Clone)]
struct StoredRecipe {
    spec: RecipeSpec,
    value: SignalValue,
    version: u64,
    initialized: bool,
    output_identity: Option<String>,
}

#[derive(Debug, Clone)]
struct StoredSourceFamily {
    spec: KeyedSourceFamilySpec,
}

#[derive(Debug, Clone)]
struct StoredRecipeFamily {
    spec: KeyedRecipeFamilySpec,
}

#[derive(Debug, Clone)]
struct DenseGridFamily {
    width: u32,
    height: u32,
    ids: Vec<String>,
    nodes: Vec<NodeId>,
    key_to_index: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Default)]
struct BranchRuntimeMetadata {
    catalog: BTreeMap<String, CatalogEntry>,
    nodes_by_id: BTreeMap<NodeId, String>,
    dense_grids: BTreeMap<String, Arc<DenseGridFamily>>,
}

#[derive(Debug, Clone, Default)]
struct BranchRuntimeState {
    metadata: BranchRuntimeMetadata,
    store: RuntimeStoreSnapshot,
}

#[derive(Debug, Clone, Default)]
struct RuntimeStore {
    sources: BTreeMap<String, StoredSource>,
    recipes: BTreeMap<String, StoredRecipe>,
    source_families: BTreeMap<String, StoredSourceFamily>,
    recipe_families: BTreeMap<String, StoredRecipeFamily>,
}

impl RuntimeStore {
    fn read_value(&self, id: &str) -> Option<SignalValue> {
        self.sources
            .get(id)
            .map(|source| source.value.clone())
            .or_else(|| self.recipes.get(id).map(|recipe| recipe.value.clone()))
    }

    fn snapshot(&self) -> RuntimeStoreSnapshot {
        RuntimeStoreSnapshot {
            sources: self
                .sources
                .iter()
                .map(|(id, source)| StoredSourceSnapshot {
                    id: id.clone(),
                    value: source.value.clone(),
                    version: source.version,
                })
                .collect(),
            recipes: self
                .recipes
                .iter()
                .map(|(id, recipe)| StoredRecipeSnapshot {
                    id: id.clone(),
                    value: recipe.value.clone(),
                    version: recipe.version,
                    initialized: recipe.initialized,
                    output_identity: recipe.output_identity.clone(),
                })
                .collect(),
        }
    }

    fn restore_snapshot(&mut self, snapshot: RuntimeStoreSnapshot) {
        self.sources = snapshot
            .sources
            .into_iter()
            .map(|source| {
                (
                    source.id,
                    StoredSource {
                        value: source.value,
                        version: source.version,
                    },
                )
            })
            .collect();
        for recipe in snapshot.recipes {
            if let Some(existing) = self.recipes.get_mut(&recipe.id) {
                existing.value = recipe.value;
                existing.version = recipe.version;
                existing.initialized = recipe.initialized;
                existing.output_identity = recipe.output_identity;
            }
        }
    }
}

pub struct RuntimeCore {
    runtime: WasmRuntime,
    store: SharedStore,
    catalog: BTreeMap<String, CatalogEntry>,
    nodes_by_id: BTreeMap<NodeId, String>,
    dense_grids: BTreeMap<String, Arc<DenseGridFamily>>,
    branch_states: BTreeMap<u64, BranchRuntimeState>,
    snapshot_states: BTreeMap<u64, BranchRuntimeState>,
    runtime_snapshots: BTreeMap<u64, RuntimeSnapshot>,
    policy: RuntimePolicySpec,
}

impl RuntimeCore {
    pub fn new(policy: RuntimePolicySpec) -> Result<Self, ForgeSignalJsError> {
        let graph = SignalGraph::new();
        let mut runtime = NativeRuntime::build_for::<SharedStore>(graph);
        runtime.set_runtime_policy(policy.clone().into_native()?);
        let current_branch_id = runtime.current_branch().id.0;
        let mut branch_metadata = BTreeMap::new();
        branch_metadata.insert(current_branch_id, BranchRuntimeState::default());
        Ok(Self {
            runtime,
            store: Arc::new(Mutex::new(RuntimeStore::default())),
            catalog: BTreeMap::new(),
            nodes_by_id: BTreeMap::new(),
            dense_grids: BTreeMap::new(),
            branch_states: branch_metadata,
            snapshot_states: BTreeMap::new(),
            runtime_snapshots: BTreeMap::new(),
            policy,
        })
    }

    pub fn set_runtime_policy(
        &mut self,
        policy: RuntimePolicySpec,
    ) -> Result<(), ForgeSignalJsError> {
        self.runtime
            .set_runtime_policy(policy.clone().into_native()?);
        self.policy = policy;
        Ok(())
    }

    pub fn define_source_family(
        &mut self,
        spec: KeyedSourceFamilySpec,
    ) -> Result<(), ForgeSignalJsError> {
        let mut store = self.lock_store()?;
        if store.source_families.contains_key(&spec.family_id)
            || store.recipe_families.contains_key(&spec.family_id)
        {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "family `{}` already exists",
                spec.family_id
            )));
        }
        store
            .source_families
            .insert(spec.family_id.clone(), StoredSourceFamily { spec });
        Ok(())
    }

    pub fn define_keyed_recipe_family(
        &mut self,
        spec: KeyedRecipeFamilySpec,
    ) -> Result<(), ForgeSignalJsError> {
        let mut store = self.lock_store()?;
        if store.recipe_families.contains_key(&spec.family_id)
            || store.source_families.contains_key(&spec.family_id)
        {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "family `{}` already exists",
                spec.family_id
            )));
        }
        for read in &spec.reads {
            match read {
                RecipeFamilyReadSpec::Signal { id } => {
                    if !self.catalog.contains_key(id) {
                        return Err(ForgeSignalJsError::invalid_input(format!(
                            "keyed family `{}` reads unknown signal `{id}`",
                            spec.family_id
                        )));
                    }
                }
                RecipeFamilyReadSpec::Keyed { family_id } => {
                    if !store.source_families.contains_key(family_id)
                        && !store.recipe_families.contains_key(family_id)
                    {
                        return Err(ForgeSignalJsError::invalid_input(format!(
                            "keyed family `{}` reads unknown family `{family_id}`",
                            spec.family_id
                        )));
                    }
                }
            }
        }
        store
            .recipe_families
            .insert(spec.family_id.clone(), StoredRecipeFamily { spec });
        Ok(())
    }

    pub fn define_source(&mut self, spec: SourceSpec) -> Result<(), ForgeSignalJsError> {
        self.ensure_unique_id(&spec.id)?;
        let source_id = spec.id.clone();
        let node = self.runtime.graph_mut().node().build();
        self.catalog
            .insert(source_id.clone(), CatalogEntry { node });
        self.nodes_by_id.insert(node, source_id.clone());
        let mut store = self.lock_store()?;
        store.sources.insert(
            source_id.clone(),
            StoredSource {
                value: spec.initial,
                version: 1,
            },
        );
        drop(store);

        let evaluator = self.evaluator();
        self.runtime
            .read(node, &self.store, &evaluator)
            .map_err(ForgeSignalJsError::from)?;
        self.runtime.clear_live_branch_mutation_residue();
        Ok(())
    }

    pub fn define_recipe(&mut self, spec: RecipeSpec) -> Result<(), ForgeSignalJsError> {
        self.ensure_unique_id(&spec.id)?;
        self.ensure_known_reads(&spec.reads)?;
        let dependencies = spec
            .reads
            .iter()
            .map(|read| {
                self.catalog
                    .get(read)
                    .map(|entry| DependencyEdge::new(entry.node, DEFAULT_ASPECT))
                    .ok_or_else(|| {
                        ForgeSignalJsError::invalid_input(format!("unknown read `{read}`"))
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut graph = self.runtime.graph_mut();
        let node = graph.node().on_demand().build();
        graph
            .set_dependencies(node, dependencies)
            .map_err(ForgeSignalJsError::from)?;
        drop(graph);
        self.catalog.insert(spec.id.clone(), CatalogEntry { node });
        self.nodes_by_id.insert(node, spec.id.clone());
        let mut store = self.lock_store()?;
        let recipe_id = spec.id.clone();
        store.recipes.insert(
            recipe_id,
            StoredRecipe {
                spec,
                value: SignalValue::Null,
                version: 0,
                initialized: false,
                output_identity: None,
            },
        );
        Ok(())
    }

    pub fn read_value(&mut self, id: &str) -> Result<SignalValue, ForgeSignalJsError> {
        let node = self.node_for_id(id)?;
        let should_recompute_recipe = self
            .lock_store()?
            .recipes
            .get(id)
            .map(|recipe| !recipe.initialized)
            .unwrap_or(false);
        if should_recompute_recipe {
            forge_signal::facade::core::mark_dirty(self.runtime.graph_mut(), node, DEFAULT_ASPECT)
                .map_err(ForgeSignalJsError::from)?;
        }
        let evaluator = self.evaluator();
        self.runtime
            .read(node, &self.store, &evaluator)
            .map_err(ForgeSignalJsError::from)?;
        self.runtime.clear_live_branch_mutation_residue();
        let store = self.lock_store()?;
        store
            .read_value(id)
            .ok_or_else(|| ForgeSignalJsError::invalid_input(format!("unknown signal id `{id}`")))
    }

    pub fn ensure_source_key(
        &mut self,
        family_id: &str,
        key: &str,
        initial: Option<SignalValue>,
    ) -> Result<String, ForgeSignalJsError> {
        if let Some(grid) = self.dense_grids.get(family_id) {
            if let Some(index) = grid.key_to_index.get(key) {
                return Ok(grid.ids[*index].clone());
            }
            return Err(ForgeSignalJsError::invalid_input(format!(
                "key `{key}` is outside dense grid family `{family_id}`"
            )));
        }

        let composite_id = composite_keyed_id(family_id, key);
        if self.catalog.contains_key(&composite_id) {
            return Ok(composite_id);
        }
        let spec = {
            let store = self.lock_store()?;
            let family = store.source_families.get(family_id).ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown source family `{family_id}`"))
            })?;
            SourceSpec {
                id: composite_id.clone(),
                initial: initial.unwrap_or_else(|| family.spec.initial.clone()),
            }
        };
        self.define_source(spec)?;
        Ok(composite_id)
    }

    fn ensure_dense_rgba_grid(
        &mut self,
        family_id: &str,
        width: u32,
        height: u32,
    ) -> Result<Arc<DenseGridFamily>, ForgeSignalJsError> {
        if let Some(existing) = self.dense_grids.get(family_id) {
            if existing.width != width || existing.height != height {
                return Err(ForgeSignalJsError::invalid_input(format!(
                    "dense grid family `{family_id}` was initialized as {}x{} and cannot become {width}x{height}",
                    existing.width, existing.height
                )));
            }
            return Ok(existing.clone());
        }

        let started_at = perf_now_ms();
        wasm_debug(format!(
            "[forge-signal-wasm] dense-grid:init family={family_id} size={}x{} cells={}",
            width,
            height,
            (width as usize) * (height as usize)
        ));

        let initial = {
            let store = self.lock_store()?;
            let family = store.source_families.get(family_id).ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown source family `{family_id}`"))
            })?;
            family.spec.initial.clone()
        };

        let mut ids = Vec::with_capacity((width as usize) * (height as usize));
        let mut nodes = Vec::with_capacity((width as usize) * (height as usize));
        let mut key_to_index = BTreeMap::new();
        let mut pending_sources = Vec::with_capacity((width as usize) * (height as usize));

        for index in 0..((width as usize) * (height as usize)) {
            let x = index % (width as usize);
            let y = index / (width as usize);
            let key = format!("{x},{y}");
            let id = composite_keyed_id(family_id, &key);
            if let Some(existing) = self.catalog.get(&id) {
                ids.push(id.clone());
                nodes.push(existing.node);
                key_to_index.insert(key, index);
                continue;
            }

            let node = self.runtime.graph_mut().node().build();
            self.catalog.insert(id.clone(), CatalogEntry { node });
            self.nodes_by_id.insert(node, id.clone());
            ids.push(id.clone());
            nodes.push(node);
            key_to_index.insert(key, index);
            pending_sources.push((id, initial.clone()));

            if index > 0 && index % 10_000 == 0 {
                wasm_debug(format!(
                    "[forge-signal-wasm] dense-grid:init progress family={family_id} built={index}"
                ));
            }
        }

        if !pending_sources.is_empty() {
            let mut store = self.lock_store()?;
            for (id, value) in pending_sources {
                store.sources.insert(id, StoredSource { value, version: 1 });
            }
        }

        let family = Arc::new(DenseGridFamily {
            width,
            height,
            ids,
            nodes,
            key_to_index,
        });
        self.dense_grids
            .insert(family_id.to_owned(), family.clone());
        wasm_debug(format!(
            "[forge-signal-wasm] dense-grid:ready family={family_id} elapsed_ms={:.1}",
            perf_now_ms() - started_at
        ));
        Ok(family)
    }

    pub fn ensure_recipe_key(
        &mut self,
        family_id: &str,
        key: &str,
    ) -> Result<String, ForgeSignalJsError> {
        let composite_id = composite_keyed_id(family_id, key);
        if self.catalog.contains_key(&composite_id) {
            return Ok(composite_id);
        }
        let family = {
            let store = self.lock_store()?;
            store
                .recipe_families
                .get(family_id)
                .cloned()
                .ok_or_else(|| {
                    ForgeSignalJsError::invalid_input(format!(
                        "unknown recipe family `{family_id}`"
                    ))
                })?
        };
        for read in &family.spec.reads {
            if let RecipeFamilyReadSpec::Keyed { family_id } = read {
                let source_id = composite_keyed_id(family_id, key);
                if !self.catalog.contains_key(&source_id) {
                    if self.lock_store()?.source_families.contains_key(family_id) {
                        self.ensure_source_key(family_id, key, None)?;
                    } else {
                        self.ensure_recipe_key(family_id, key)?;
                    }
                }
            }
        }
        let recipe = RecipeSpec {
            id: composite_id.clone(),
            reads: family
                .spec
                .reads
                .iter()
                .map(|read| match read {
                    RecipeFamilyReadSpec::Signal { id } => id.clone(),
                    RecipeFamilyReadSpec::Keyed { family_id } => composite_keyed_id(family_id, key),
                })
                .collect(),
            expr: rewrite_keyed_expr(&family.spec.expr, &family.spec.reads, key),
            when: family.spec.when.as_ref().map(|condition| {
                crate::expression::model::ConditionSpec {
                    expr: rewrite_keyed_expr(&condition.expr, &family.spec.reads, key),
                }
            }),
            identity: family
                .spec
                .identity
                .as_ref()
                .map(|identity| match identity {
                    IdentitySpec::Exact => IdentitySpec::Exact,
                    IdentitySpec::Expr { expr } => IdentitySpec::Expr {
                        expr: rewrite_keyed_expr(expr, &family.spec.reads, key),
                    },
                }),
        };
        self.define_recipe(recipe)?;
        Ok(composite_id)
    }

    pub fn read_keyed_value(
        &mut self,
        family_id: &str,
        key: &str,
    ) -> Result<SignalValue, ForgeSignalJsError> {
        let id = if self.lock_store()?.recipe_families.contains_key(family_id) {
            self.ensure_recipe_key(family_id, key)?
        } else {
            self.ensure_source_key(family_id, key, None)?
        };
        self.read_value(&id)
    }

    pub fn set_keyed_value(
        &mut self,
        family_id: &str,
        key: &str,
        value: SignalValue,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let id = self.ensure_source_key(family_id, key, Some(value.clone()))?;
        self.apply_transaction(vec![TransactionOp::Set { id, value }])
    }

    pub fn read_keyed_values(
        &mut self,
        family_id: &str,
        keys: Vec<String>,
    ) -> Result<Vec<SignalValue>, ForgeSignalJsError> {
        let mut values = Vec::with_capacity(keys.len());
        for key in keys {
            values.push(self.read_keyed_value(family_id, &key)?);
        }
        Ok(values)
    }

    pub fn set_keyed_values(
        &mut self,
        family_id: &str,
        values: Vec<KeyedSetValue>,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let mut normalized = Vec::with_capacity(values.len());
        for entry in values {
            let id = self.ensure_source_key(family_id, &entry.key, Some(entry.value.clone()))?;
            normalized.push(crate::recipe::model::SetValue {
                id,
                value: entry.value,
            });
        }

        self.apply_transaction(vec![TransactionOp::SetMany { values: normalized }])
    }

    pub fn clear_keyed_family_cache(&mut self, family_id: &str) -> Result<(), ForgeSignalJsError> {
        let prefix = format!("{family_id}::");

        if let Some(grid) = self.dense_grids.remove(family_id) {
            for node in &grid.nodes {
                self.nodes_by_id.remove(node);
            }
            for id in &grid.ids {
                self.catalog.remove(id);
            }
        }

        let stale_ids: Vec<String> = self
            .catalog
            .keys()
            .filter(|id| id.starts_with(&prefix))
            .cloned()
            .collect();

        for id in stale_ids {
            if let Some(entry) = self.catalog.remove(&id) {
                self.nodes_by_id.remove(&entry.node);
            }
        }

        let mut store = self.lock_store()?;
        store.sources.retain(|id, _| !id.starts_with(&prefix));
        store.recipes.retain(|id, _| !id.starts_with(&prefix));
        Ok(())
    }

    pub fn apply_transaction(
        &mut self,
        ops: Vec<TransactionOp>,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let started_at = perf_now_ms();
        wasm_debug(format!("[forge-signal-wasm] tx:start ops={}", ops.len()));
        let previous = self.lock_store()?.clone();
        let changes = self.collect_changes(&ops)?;
        wasm_debug(format!(
            "[forge-signal-wasm] tx:collect-done changes={} elapsed_ms={:.1}",
            changes.len(),
            perf_now_ms() - started_at
        ));
        let store = self.store.clone();
        let dense_grids = self.dense_grids.clone();
        let evaluator = self.evaluator();

        let result = self.runtime.transaction(&mut self.store, move |tx| {
            wasm_debug("[forge-signal-wasm] tx:apply-start");
            {
                let mut locked = store
                    .lock()
                    .map_err(|_| SignalError::internal("runtime store mutex poisoned"))?;
                for change in &changes {
                    match change {
                        SetChange::Source { id, value, node } => {
                            let source = locked.sources.get_mut(id).ok_or_else(|| {
                                SignalError::invalid_input(format!("unknown source `{id}`"))
                            })?;
                            source.value = value.clone();
                            source.version = source.version.saturating_add(1);
                            tx.mark_changed(*node, DEFAULT_ASPECT)?;
                        }
                        SetChange::DenseGridRgba { family_id, rgba } => {
                            let family = dense_grids.get(family_id).ok_or_else(|| {
                                SignalError::invalid_input(format!(
                                    "unknown dense grid family `{family_id}`"
                                ))
                            })?;
                            wasm_debug(format!(
                                "[forge-signal-wasm] tx:dense-apply-start family={family_id} cells={}",
                                family.ids.len()
                            ));
                            for index in 0..family.ids.len() {
                                let offset = index * 4;
                                let source = locked.sources.get_mut(&family.ids[index]).ok_or_else(|| {
                                    SignalError::invalid_input(format!(
                                        "unknown dense source `{}`",
                                        family.ids[index]
                                    ))
                                })?;
                                set_rgba_signal_value(
                                    &mut source.value,
                                    rgba[offset],
                                    rgba[offset + 1],
                                    rgba[offset + 2],
                                    rgba[offset + 3],
                                );
                                source.version = source.version.saturating_add(1);
                                tx.mark_changed(family.nodes[index], DEFAULT_ASPECT)?;
                                if index > 0 && index % 10_000 == 0 {
                                    wasm_debug(format!(
                                        "[forge-signal-wasm] tx:dense-apply progress family={family_id} applied={index}"
                                    ));
                                }
                            }
                            wasm_debug(format!(
                                "[forge-signal-wasm] tx:dense-apply-done family={family_id}"
                            ));
                        }
                    }
                }
            }

            wasm_debug("[forge-signal-wasm] tx:evaluate-dirty-start");
            tx.evaluate_dirty(&evaluator)?;
            wasm_debug("[forge-signal-wasm] tx:evaluate-dirty-done");
            Ok(())
        });

        match result {
            Ok(result) => {
                let active_branch_id = self.runtime.current_branch().id.0;
                self.branch_states
                    .insert(active_branch_id, self.snapshot_branch_state());
                wasm_debug(format!(
                    "[forge-signal-wasm] tx:done touched={} evaluated={} elapsed_ms={:.1}",
                    result.touched_nodes,
                    result.evaluation_summary.nodes_evaluated,
                    perf_now_ms() - started_at
                ));
                Ok(RunSummary {
                    touched_nodes: result.touched_nodes,
                    nodes_evaluated: result.evaluation_summary.nodes_evaluated,
                    nodes_recomputed: result.evaluation_summary.nodes_recomputed,
                    nodes_suppressed: result.evaluation_summary.nodes_suppressed,
                    plans_built: result.evaluation_summary.plans_built,
                    stages_executed: result.evaluation_summary.stages_executed,
                    total_nanos: result.timing.total_nanos.to_string(),
                    evaluation_nanos: result.timing.evaluation_nanos.to_string(),
                    commit_nanos: result.timing.commit_nanos.to_string(),
                })
            }
            Err(err) => {
                wasm_debug(format!(
                    "[forge-signal-wasm] tx:error elapsed_ms={:.1} message={}",
                    perf_now_ms() - started_at,
                    err
                ));
                self.restore_store(previous)?;
                Err(ForgeSignalJsError::from(err))
            }
        }
    }

    pub fn why(&self, id: &str) -> Result<WhySummary, ForgeSignalJsError> {
        let node = self.node_for_id(id)?;
        let explanation = self
            .runtime
            .diagnostics()
            .why(node)
            .map_err(ForgeSignalJsError::from)?;
        Ok(WhySummary {
            id: id.to_owned(),
            node: explanation.node.to_string(),
            state: format!("{:?}", explanation.state),
            upstream: explanation
                .upstream
                .iter()
                .map(|cause| format!("{cause:?}"))
                .collect(),
            changed_regions: explanation
                .changed_regions
                .iter()
                .map(|region| match &region.detail {
                    Some(detail) => format!("{}:{}", region.partition.0, detail),
                    None => region.partition.0.clone(),
                })
                .collect(),
            propagation_suppressed: explanation.propagation_suppressed,
            output_change: explanation
                .output_change
                .map(|change| format!("{change:?}")),
            output_identity: explanation.output_identity.map(|value| String::from(value)),
        })
    }

    pub fn health(&self) -> Result<HealthSummary, ForgeSignalJsError> {
        Ok(self
            .runtime
            .diagnostics()
            .health_view()
            .summary_now()
            .into())
    }

    pub fn diagnostics_summary_now(
        &self,
    ) -> Result<forge_signal::facade::diagnostics::GraphSummary, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().summary_now())
    }

    pub fn execution_history_now(
        &self,
    ) -> Result<forge_signal::facade::diagnostics::ExecutionHistorySummary, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().history_now())
    }

    pub fn latest_flow(
        &self,
    ) -> Result<Option<forge_signal::diagnostics::FlowSummary>, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().latest_flow().cloned())
    }

    pub fn latest_failure(
        &self,
    ) -> Result<Option<forge_signal::diagnostics::FailureSummary>, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().latest_failure().cloned())
    }

    pub fn latest_rollback(
        &self,
    ) -> Result<Option<forge_signal::diagnostics::RollbackDiagnostic>, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().latest_rollback().cloned())
    }

    pub fn latest_frontier_execution(
        &self,
    ) -> Result<Option<forge_signal::facade::adapters::FrontierExecutionSummary>, ForgeSignalJsError>
    {
        Ok(self.runtime.diagnostics().latest_frontier_execution().cloned())
    }

    pub fn latest_invalidation_trace_records(
        &self,
    ) -> Result<Vec<forge_signal::facade::adapters::InvalidationTraceRecord>, ForgeSignalJsError>
    {
        Ok(self
            .runtime
            .diagnostics()
            .latest_invalidation_trace_records()
            .to_vec())
    }

    pub fn recent_history(
        &self,
    ) -> Result<Vec<forge_signal::facade::diagnostics::ExecutionHistorySummary>, ForgeSignalJsError>
    {
        Ok(self.runtime.diagnostics().recent_history().iter().cloned().collect())
    }

    pub fn replay_for_id(&mut self, id: &str) -> Result<ReplaySummary, ForgeSignalJsError> {
        let node = self.node_for_id(id)?;
        let replay = {
            let history = self.runtime.history();
            history.replay_for_node(node)
        };
        Ok(replay.into())
    }

    pub fn lineage_for_id(&mut self, id: &str) -> Result<LineageSummary, ForgeSignalJsError> {
        let node = self.node_for_id(id)?;
        let chain = {
            let history = self.runtime.history();
            history.lineage_for_node(node)
        };
        Ok(chain.to_owned_records().into())
    }

    pub fn snapshot(&mut self) -> Result<RuntimeSnapshotEnvelope, ForgeSignalJsError> {
        let snapshot: RuntimeSnapshot = {
            let mut history = self.runtime.history();
            history.snapshot()
        };
        self.runtime_snapshots
            .insert(snapshot.meta.snapshot_id.0, snapshot.clone());
        self.snapshot_states
            .insert(snapshot.meta.snapshot_id.0, self.snapshot_branch_state());
        Ok(RuntimeSnapshotEnvelope {
            snapshot,
            state: self.lock_store()?.snapshot(),
        })
    }

    pub fn restore_snapshot(
        &mut self,
        envelope: RuntimeSnapshotEnvelope,
    ) -> Result<(), ForgeSignalJsError> {
        self.runtime
            .restore_snapshot(&envelope.snapshot)
            .map_err(ForgeSignalJsError::from)?;
        let mut store = self.lock_store()?;
        store.restore_snapshot(envelope.state);
        Ok(())
    }

    pub fn current_branch(&self) -> RuntimeBranch {
        self.runtime.current_branch()
    }

    pub fn branches(&self) -> Vec<RuntimeBranch> {
        self.runtime.known_branches()
    }

    pub fn create_branch(&mut self, name: String) -> Result<RuntimeBranch, ForgeSignalJsError> {
        let state = self.snapshot_branch_state();
        let branch = self
            .runtime
            .create_branch(name)
            .map_err(ForgeSignalJsError::from)?;
        self.branch_states.insert(branch.id.0, state);
        Ok(branch)
    }

    pub fn switch_branch(&mut self, branch_id: u64) -> Result<(), ForgeSignalJsError> {
        let current_branch_id = self.runtime.current_branch().id.0;
        let current_state = self.snapshot_branch_state();
        self.branch_states
            .insert(current_branch_id, current_state.clone());
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        self.runtime
            .switch_branch(branch)
            .map_err(ForgeSignalJsError::from)?;
        let target_state = self
            .branch_states
            .get(&branch_id)
            .cloned()
            .unwrap_or(current_state);
        self.restore_branch_state(target_state)?;
        Ok(())
    }

    pub fn replay_for_branch(
        &mut self,
        branch_id: u64,
    ) -> Result<ReplaySummary, ForgeSignalJsError> {
        Ok(self
            .runtime
            .replay_for_branch(RuntimeBranchId(branch_id))
            .into())
    }

    pub fn branch_snapshot(
        &mut self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshot, ForgeSignalJsError> {
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        let mut history = self.runtime.history();
        let snapshot = history
            .branch_snapshot(branch)
            .map_err(ForgeSignalJsError::from)?;
        self.runtime_snapshots
            .insert(snapshot.meta.snapshot_id.0, snapshot.clone());
        self.snapshot_states.insert(
            snapshot.meta.snapshot_id.0,
            self.state_for_branch(branch_id),
        );
        Ok(snapshot)
    }

    pub fn branch_snapshot_id(&mut self, branch_id: u64) -> Result<u64, ForgeSignalJsError> {
        Ok(self.branch_snapshot(branch_id)?.meta.snapshot_id.0)
    }

    pub fn branch_snapshot_envelope(
        &mut self,
        branch_id: u64,
    ) -> Result<RuntimeSnapshotEnvelope, ForgeSignalJsError> {
        let snapshot = self.branch_snapshot(branch_id)?;
        let state = self
            .snapshot_states
            .get(&snapshot.meta.snapshot_id.0)
            .map(|state| state.store.clone())
            .ok_or_else(|| {
                ForgeSignalJsError::internal(format!(
                    "snapshot `{}` missing runtime-local branch state",
                    snapshot.meta.snapshot_id.0
                ))
            })?;
        Ok(RuntimeSnapshotEnvelope { snapshot, state })
    }

    pub fn restore_branch_snapshot(
        &mut self,
        branch_id: u64,
        snapshot: RuntimeSnapshot,
    ) -> Result<(), ForgeSignalJsError> {
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        self.runtime
            .restore_branch_snapshot(branch, &snapshot)
            .map_err(ForgeSignalJsError::from)?;
        let state = self
            .snapshot_states
            .get(&snapshot.meta.snapshot_id.0)
            .cloned()
            .ok_or_else(|| {
                ForgeSignalJsError::internal(format!(
                    "snapshot `{}` is missing runtime-local branch semantic state",
                    snapshot.meta.snapshot_id.0
                ))
            })?;
        self.branch_states.insert(branch_id, state.clone());
        if self.runtime.current_branch().id.0 == branch_id {
            self.restore_branch_state(state)?;
        }
        Ok(())
    }

    pub fn restore_branch_snapshot_by_id(
        &mut self,
        branch_id: u64,
        snapshot_id: u64,
    ) -> Result<(), ForgeSignalJsError> {
        let snapshot = self
            .runtime_snapshots
            .get(&snapshot_id)
            .cloned()
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!(
                    "unknown runtime snapshot `{snapshot_id}`"
                ))
            })?;
        self.restore_branch_snapshot(branch_id, snapshot)
    }

    pub fn merge_branches(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<BranchMergeResult, ForgeSignalJsError> {
        let current_branch_id = self.runtime.current_branch().id.0;
        let current_state = self.snapshot_branch_state();
        self.branch_states
            .insert(current_branch_id, current_state.clone());

        if current_branch_id != source_branch_id {
            self.switch_branch(source_branch_id)?;
        }
        let source_state = self.snapshot_branch_state();
        self.branch_states
            .insert(source_branch_id, source_state.clone());

        if self.runtime.current_branch().id.0 != target_branch_id {
            self.switch_branch(target_branch_id)?;
        }
        let target_state = self.snapshot_branch_state();
        self.branch_states
            .insert(target_branch_id, target_state.clone());

        if self.runtime.current_branch().id.0 != source_branch_id {
            self.switch_branch(source_branch_id)?;
        }

        let merged_metadata = merge_branch_metadata(&target_state.metadata, &source_state.metadata);
        self.restore_branch_metadata(merged_metadata.clone());
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(source_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{source_branch_id}`"))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(target_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{target_branch_id}`"))
            })?;
        self.runtime
            .merge_branch(source, target)
            .map_err(ForgeSignalJsError::from)
            .map(|result| {
                let merged_store = merge_branch_store(
                    &target_state.store,
                    &source_state.store,
                    &source_state.metadata,
                    &merged_metadata,
                    &result,
                );
                let merged_state = BranchRuntimeState {
                    metadata: merged_metadata,
                    store: merged_store,
                };
                self.branch_states
                    .insert(target_branch_id, merged_state.clone());
                let active_branch_id = self.runtime.current_branch().id.0;
                let restored = if active_branch_id == target_branch_id {
                    merged_state
                } else if active_branch_id == source_branch_id {
                    source_state
                } else {
                    current_state
                };
                let _ = self.restore_branch_state(restored);
                result
            })
    }

    pub fn merge_branches_with_proof(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergeResultProofEnvelope, ForgeSignalJsError> {
        let result = self.merge_branches(source_branch_id, target_branch_id)?;
        let proof = self.merge_result_proof_report(&result)?;
        Ok(MergeResultProofEnvelope { result, proof })
    }

    pub fn plan_merge_branches(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<BranchMergePlan, ForgeSignalJsError> {
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(source_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{source_branch_id}`"))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(target_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{target_branch_id}`"))
            })?;
        self.runtime
            .merge()
            .from(source)
            .into_branch(target)
            .plan()
            .map(|planned| planned.plan().clone())
            .map_err(ForgeSignalJsError::from)
    }

    pub fn plan_merge_branches_with_proof(
        &mut self,
        source_branch_id: u64,
        target_branch_id: u64,
    ) -> Result<MergePlanProofEnvelope, ForgeSignalJsError> {
        let plan = self.plan_merge_branches(source_branch_id, target_branch_id)?;
        let proof = self.merge_plan_proof_report(&plan)?;
        Ok(MergePlanProofEnvelope { plan, proof })
    }

    pub fn plan_merge_policy_preview(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<BranchMergePlan, ForgeSignalJsError> {
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(request.source_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.source_branch_id
                ))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(request.target_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.target_branch_id
                ))
            })?;

        let mut merge = self.runtime.merge().from(source).into_branch(target);
        if let Some(policy_name) = request.conflict_policy_name {
            merge = merge.conflict_policy_named(policy_name);
        }
        if let Some(policy_name) = request.conflict_isolation_policy_name {
            merge = merge.conflict_isolation_policy_named(policy_name);
        }
        if let Some(matcher_name) = request.identity_matcher_name {
            merge = merge.identity_matcher_named(matcher_name);
        }
        if let Some(policy_name) = request.deletion_policy_name {
            merge = merge.deletion_policy_named(policy_name);
        }

        merge
            .plan()
            .map(|planned| planned.plan().clone())
            .map_err(ForgeSignalJsError::from)
    }

    pub fn plan_merge_policy_preview_with_proof(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergePlanProofEnvelope, ForgeSignalJsError> {
        let plan = self.plan_merge_policy_preview(request)?;
        let proof = self.merge_plan_proof_report(&plan)?;
        Ok(MergePlanProofEnvelope { plan, proof })
    }

    pub fn merge_branches_policy_preview(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<BranchMergeResult, ForgeSignalJsError> {
        let source = self
            .runtime
            .branch_handle(RuntimeBranchId(request.source_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.source_branch_id
                ))
            })?;
        let target = self
            .runtime
            .branch_handle(RuntimeBranchId(request.target_branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!(
                    "unknown branch `{}`",
                    request.target_branch_id
                ))
            })?;

        let mut merge = self.runtime.merge().from(source).into_branch(target);
        if let Some(policy_name) = request.conflict_policy_name {
            merge = merge.conflict_policy_named(policy_name);
        }
        if let Some(policy_name) = request.conflict_isolation_policy_name {
            merge = merge.conflict_isolation_policy_named(policy_name);
        }
        if let Some(matcher_name) = request.identity_matcher_name {
            merge = merge.identity_matcher_named(matcher_name);
        }
        if let Some(policy_name) = request.deletion_policy_name {
            merge = merge.deletion_policy_named(policy_name);
        }

        merge.run().map_err(ForgeSignalJsError::from)
    }

    pub fn merge_branches_policy_preview_with_proof(
        &mut self,
        request: MergePolicyPreviewRequest,
    ) -> Result<MergeResultProofEnvelope, ForgeSignalJsError> {
        let result = self.merge_branches_policy_preview(request)?;
        let proof = self.merge_result_proof_report(&result)?;
        Ok(MergeResultProofEnvelope { result, proof })
    }

    pub fn graph_summary(
        &self,
    ) -> Result<forge_signal::facade::diagnostics::GraphSummary, ForgeSignalJsError> {
        Ok(self.runtime.diagnostics().summary_now())
    }

    pub fn evaluate_dirty(&mut self) -> Result<RunSummary, ForgeSignalJsError> {
        let evaluator = self.evaluator();
        let report = self
            .runtime
            .evaluate_dirty(&self.store, &evaluator)
            .map_err(ForgeSignalJsError::from)?;
        Ok(RunSummary {
            touched_nodes: report.task_count,
            nodes_evaluated: report.tasks_executed,
            nodes_recomputed: report
                .stages
                .iter()
                .flat_map(|stage| stage.task_records.iter())
                .filter(|record| matches!(record.verdict, Some(EvaluationVerdict::Recomputed)))
                .count() as u32,
            nodes_suppressed: report.tasks_with_suppressed_propagation,
            plans_built: 1,
            stages_executed: report.stage_count,
            total_nanos: (report.execution_snapshot_nanos
                + report.stage_precompute_nanos
                + report.stage_apply_nanos
                + report.semantic_finalize_nanos)
                .to_string(),
            evaluation_nanos: (report.stage_precompute_nanos + report.stage_apply_nanos)
                .to_string(),
            commit_nanos: report.semantic_finalize_nanos.to_string(),
        })
    }

    pub fn read_versions(
        &mut self,
        ids: Vec<String>,
    ) -> Result<Vec<VersionSummary>, ForgeSignalJsError> {
        let mut versions = Vec::with_capacity(ids.len());
        let evaluator = self.evaluator();
        for id in ids {
            let node = self.node_for_id(&id)?;
            let version = self
                .runtime
                .read(node, &self.store, &evaluator)
                .map_err(ForgeSignalJsError::from)?;
            self.runtime.clear_live_branch_mutation_residue();
            versions.push(VersionSummary {
                id,
                version: version.get(DEFAULT_ASPECT),
            });
        }
        Ok(versions)
    }

    pub fn export_definitions(&self) -> Result<RuntimeDefinitionEnvelope, ForgeSignalJsError> {
        let store = self.lock_store()?;
        Ok(RuntimeDefinitionEnvelope {
            policy: self.policy.clone(),
            sources: store
                .sources
                .iter()
                .map(|(id, source)| SourceSpec {
                    id: id.clone(),
                    initial: source.value.clone(),
                })
                .collect(),
            recipes: store
                .recipes
                .values()
                .map(|recipe| recipe.spec.clone())
                .collect(),
            source_families: store
                .source_families
                .values()
                .map(|family| family.spec.clone())
                .collect(),
            recipe_families: store
                .recipe_families
                .values()
                .map(|family| family.spec.clone())
                .collect(),
        })
    }

    pub fn export_runtime_envelope(&mut self) -> Result<RuntimeEnvelope, ForgeSignalJsError> {
        Ok(RuntimeEnvelope {
            definitions: self.export_definitions()?,
            snapshot: self.snapshot()?,
        })
    }

    pub fn runtime_proof_report(&self) -> RuntimeProofReport {
        runtime_proof_report(
            self.runtime.schema_registry().registry_digest(),
            self.runtime.merge_strategy_registry().registry_digest(),
            self.runtime
                .merge_base_strategy_registry()
                .registry_digest(),
            self.runtime
                .aspect_merge_policy_registry()
                .registry_digest(),
            self.runtime.conflict_isolation_registry().registry_digest(),
            self.runtime.conflict_policy_registry().registry_digest(),
            self.runtime.identity_matcher_registry().registry_digest(),
            self.runtime.source_only_policy_registry().registry_digest(),
            self.runtime.deletion_policy_registry().registry_digest(),
        )
    }

    pub fn branch_state_proof(
        &self,
        branch_id: u64,
    ) -> Result<BranchStateProofReport, ForgeSignalJsError> {
        let branch = self
            .runtime
            .branch_handle(RuntimeBranchId(branch_id))
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!("unknown branch `{branch_id}`"))
            })?;
        let state = self.state_for_branch(branch_id);
        Ok(branch_state_proof_report(
            branch_id,
            branch.name,
            branch.head_snapshot_id.map(|id| id.0),
            BRANCH_STATE_PROOF_BASIS_VERSION,
            &build_branch_state_proof_basis(&state),
        ))
    }

    pub fn replay_parity_proof(
        &self,
        expected_branch_id: u64,
        replayed_branch_id: u64,
    ) -> Result<ReplayParityProofReport, ForgeSignalJsError> {
        let expected = self.branch_state_proof(expected_branch_id)?;
        let replayed = self.branch_state_proof(replayed_branch_id)?;
        Ok(replay_parity_proof_report(
            expected.branch_id,
            expected.branch_name,
            expected.snapshot_id,
            expected.state_digest,
            replayed.branch_id,
            replayed.branch_name,
            replayed.snapshot_id,
            replayed.state_digest,
        ))
    }

    pub fn replay_artifact_proof(
        &self,
        expected: ReplayArtifactProofInput,
        replayed_branch_id: u64,
    ) -> Result<ReplayArtifactProofReport, ForgeSignalJsError> {
        let replayed_state = self.branch_state_proof(replayed_branch_id)?;
        let runtime_proof = self.runtime_proof_report();
        Ok(replay_artifact_proof_report(
            expected,
            ReplayArtifactProofInput {
                proof_schema_version: runtime_proof.proof_schema_version.clone(),
                registry_bundle_digest: Some(runtime_proof.registry_bundle_digest),
                lowered_strategy_bundle_digest: None,
                merge_plan_digest: None,
                merge_result_digest: None,
                lineage_digest: None,
                branch_state_digest: replayed_state.state_digest,
            },
        ))
    }

    fn merge_plan_proof_report(
        &self,
        plan: &BranchMergePlan,
    ) -> Result<MergePlanProofReport, ForgeSignalJsError> {
        Ok(merge_plan_proof_report(
            plan,
            &self.runtime_proof_report().registry_bundle_digest,
        ))
    }

    fn merge_result_proof_report(
        &self,
        result: &BranchMergeResult,
    ) -> Result<MergeResultProofReport, ForgeSignalJsError> {
        Ok(merge_result_proof_report(result))
    }

    pub fn replace_runtime_envelope(
        &mut self,
        envelope: RuntimeEnvelope,
    ) -> Result<(), ForgeSignalJsError> {
        let mut rebuilt = RuntimeCore::new(envelope.definitions.policy.clone())?;
        for family in envelope.definitions.source_families {
            rebuilt.define_source_family(family)?;
        }
        for family in envelope.definitions.recipe_families {
            rebuilt.define_keyed_recipe_family(family)?;
        }
        for source in envelope.definitions.sources {
            rebuilt.define_source(source)?;
        }
        for recipe in envelope.definitions.recipes {
            rebuilt.define_recipe(recipe)?;
        }
        rebuilt.restore_snapshot(envelope.snapshot)?;
        *self = rebuilt;
        Ok(())
    }

    fn evaluator(
        &self,
    ) -> impl for<'ctx> Fn(
        &mut EvaluationContext<'ctx, SharedStore>,
    ) -> Result<EvaluationOutput, SignalError>
           + Sync {
        let store = self.store.clone();
        let nodes_by_id = self.nodes_by_id.clone();
        move |view| evaluate_node(view, &store, &nodes_by_id)
    }

    fn ensure_unique_id(&self, id: &str) -> Result<(), ForgeSignalJsError> {
        if self.catalog.contains_key(id) {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "signal id `{id}` already exists"
            )));
        }
        Ok(())
    }

    fn ensure_known_reads(&self, reads: &[String]) -> Result<(), ForgeSignalJsError> {
        for read in reads {
            if !self.catalog.contains_key(read) {
                return Err(ForgeSignalJsError::invalid_input(format!(
                    "unknown read `{read}`"
                )));
            }
        }
        Ok(())
    }

    fn collect_changes(
        &mut self,
        ops: &[TransactionOp],
    ) -> Result<Vec<SetChange>, ForgeSignalJsError> {
        let mut deduped = BTreeMap::<String, SignalValue>::new();
        let mut packed = Vec::new();
        for op in ops {
            match op {
                TransactionOp::Set { id, value } => {
                    deduped.insert(id.clone(), value.clone());
                }
                TransactionOp::SetMany { values } => {
                    for value in values {
                        deduped.insert(value.id.clone(), value.value.clone());
                    }
                }
                TransactionOp::SetManyKeyed { family_id, values } => {
                    for value in values {
                        let id = self.ensure_source_key(
                            family_id,
                            &value.key,
                            Some(value.value.clone()),
                        )?;
                        deduped.insert(id, value.value.clone());
                    }
                }
                TransactionOp::SetPackedGridRgba {
                    family_id,
                    width,
                    height,
                    rgba,
                } => {
                    let expected_len = (*width as usize) * (*height as usize) * 4;
                    if rgba.len() != expected_len {
                        return Err(ForgeSignalJsError::invalid_input(format!(
                            "packed rgba length {} does not match {width}x{height} grid",
                            rgba.len()
                        )));
                    }
                    self.ensure_dense_rgba_grid(family_id, *width, *height)?;
                    packed.push(SetChange::DenseGridRgba {
                        family_id: family_id.clone(),
                        rgba: rgba.clone(),
                    });
                }
            }
        }
        let mut normalized = deduped
            .into_iter()
            .map(|(id, value)| {
                let node = self.node_for_id(&id)?;
                Ok(SetChange::Source { id, value, node })
            })
            .collect::<Result<Vec<_>, ForgeSignalJsError>>()?;
        normalized.extend(packed);
        Ok(normalized)
    }

    fn lock_store(&self) -> Result<std::sync::MutexGuard<'_, RuntimeStore>, ForgeSignalJsError> {
        self.store
            .lock()
            .map_err(|_| ForgeSignalJsError::internal("runtime store mutex poisoned"))
    }

    fn restore_store(&self, previous: RuntimeStore) -> Result<(), ForgeSignalJsError> {
        let mut store = self.lock_store()?;
        *store = previous;
        Ok(())
    }

    fn node_for_id(&self, id: &str) -> Result<NodeId, ForgeSignalJsError> {
        self.catalog
            .get(id)
            .map(|entry| entry.node)
            .ok_or_else(|| ForgeSignalJsError::invalid_input(format!("unknown signal id `{id}`")))
    }

    fn snapshot_branch_metadata(&self) -> BranchRuntimeMetadata {
        BranchRuntimeMetadata {
            catalog: self.catalog.clone(),
            nodes_by_id: self.nodes_by_id.clone(),
            dense_grids: self.dense_grids.clone(),
        }
    }

    fn snapshot_branch_state(&self) -> BranchRuntimeState {
        BranchRuntimeState {
            metadata: self.snapshot_branch_metadata(),
            store: self
                .lock_store()
                .map(|store| store.snapshot())
                .unwrap_or_default(),
        }
    }

    fn restore_branch_metadata(&mut self, metadata: BranchRuntimeMetadata) {
        self.catalog = metadata.catalog;
        self.nodes_by_id = metadata.nodes_by_id;
        self.dense_grids = metadata.dense_grids;
    }

    fn restore_branch_state(
        &mut self,
        state: BranchRuntimeState,
    ) -> Result<(), ForgeSignalJsError> {
        self.restore_branch_metadata(state.metadata);
        let mut store = self.lock_store()?;
        store.restore_snapshot(state.store);
        Ok(())
    }

    fn state_for_branch(&self, branch_id: u64) -> BranchRuntimeState {
        if self.runtime.current_branch().id.0 == branch_id {
            self.snapshot_branch_state()
        } else {
            self.branch_states
                .get(&branch_id)
                .cloned()
                .unwrap_or_default()
        }
    }
}

fn build_branch_state_proof_basis(
    state: &BranchRuntimeState,
) -> BranchStateProofBasis<RuntimeStoreSnapshot> {
    let mut catalog_ids = state.metadata.catalog.keys().cloned().collect::<Vec<_>>();
    catalog_ids.sort();

    let mut dense_grids = state
        .metadata
        .dense_grids
        .iter()
        .map(|(family_id, grid)| BranchStateDenseGridProofBasis {
            family_id: family_id.clone(),
            width: grid.width,
            height: grid.height,
            key_count: grid.key_to_index.len(),
            ids: grid.ids.clone(),
        })
        .collect::<Vec<_>>();
    dense_grids.sort_by(|left, right| left.family_id.cmp(&right.family_id));
    for grid in &mut dense_grids {
        grid.ids.sort();
    }

    BranchStateProofBasis {
        proof_schema_version: BRANCH_STATE_PROOF_BASIS_VERSION.to_owned(),
        catalog_ids,
        dense_grids,
        store: state.store.clone(),
    }
}

#[derive(Debug, Clone)]
enum SetChange {
    Source {
        id: String,
        value: SignalValue,
        node: NodeId,
    },
    DenseGridRgba {
        family_id: String,
        rgba: Vec<u8>,
    },
}

fn merge_branch_metadata(
    target: &BranchRuntimeMetadata,
    source: &BranchRuntimeMetadata,
) -> BranchRuntimeMetadata {
    let mut merged = target.clone();
    for (node, id) in &source.nodes_by_id {
        merged
            .nodes_by_id
            .entry(*node)
            .or_insert_with(|| id.clone());
    }
    for (id, entry) in &source.catalog {
        merged
            .catalog
            .entry(id.clone())
            .or_insert_with(|| entry.clone());
    }
    for (family_id, grid) in &source.dense_grids {
        merged
            .dense_grids
            .entry(family_id.clone())
            .or_insert_with(|| grid.clone());
    }
    merged
}

fn merge_branch_store(
    target: &RuntimeStoreSnapshot,
    source: &RuntimeStoreSnapshot,
    source_metadata: &BranchRuntimeMetadata,
    merged_metadata: &BranchRuntimeMetadata,
    result: &BranchMergeResult,
) -> RuntimeStoreSnapshot {
    let mut merged = target.clone();
    let mut merged_sources: BTreeMap<String, StoredSourceSnapshot> = merged
        .sources
        .into_iter()
        .map(|entry| (entry.id.clone(), entry))
        .collect();
    let source_sources: BTreeMap<String, StoredSourceSnapshot> = source
        .sources
        .iter()
        .cloned()
        .map(|entry| (entry.id.clone(), entry))
        .collect();

    for record in &result.records {
        let should_adopt = matches!(
            record.action,
            ArtifactMergeAction::Adopted
                | ArtifactMergeAction::Replaced
                | ArtifactMergeAction::IntroducedIntoTarget
                | ArtifactMergeAction::EquivalentUnchanged
        );
        if !should_adopt {
            continue;
        }

        let Some(source_id) = source_metadata.nodes_by_id.get(&record.source_node) else {
            continue;
        };
        let Some(source_value) = source_sources.get(source_id) else {
            continue;
        };
        let target_id = record
            .target_node
            .and_then(|node| merged_metadata.nodes_by_id.get(&node).cloned())
            .unwrap_or_else(|| source_id.clone());
        merged_sources.insert(
            target_id.clone(),
            StoredSourceSnapshot {
                id: target_id,
                value: source_value.value.clone(),
                version: source_value.version,
            },
        );
    }

    merged.sources = merged_sources.into_values().collect();
    for recipe in &mut merged.recipes {
        recipe.value = SignalValue::Null;
        recipe.initialized = false;
        recipe.output_identity = None;
    }
    merged
}

pub fn new_shared_core(policy: RuntimePolicySpec) -> Result<SharedCore, ForgeSignalJsError> {
    Ok(Rc::new(RefCell::new(RuntimeCore::new(policy)?)))
}

fn evaluate_node(
    view: &mut EvaluationContext<'_, SharedStore>,
    store: &SharedStore,
    nodes_by_id: &BTreeMap<NodeId, String>,
) -> Result<EvaluationOutput, SignalError> {
    let Some(id) = nodes_by_id.get(&view.node()) else {
        return Err(SignalError::invalid_input(
            "missing runtime node id mapping",
        ));
    };

    let mut locked = store
        .lock()
        .map_err(|_| SignalError::internal("runtime store mutex poisoned"))?;

    if let Some(source) = locked.sources.get(id) {
        return Ok(view.finish(NodeEvaluationResult::from_version(
            AspectVersion::from_updates([(DEFAULT_ASPECT, source.version)]),
        )));
    }

    let reads = locked
        .recipes
        .get(id)
        .map(|recipe| recipe.spec.reads.clone())
        .ok_or_else(|| SignalError::invalid_input(format!("unknown runtime recipe `{id}`")))?;

    let mut read_values = BTreeMap::new();
    for read in &reads {
        let Some(read_node) =
            nodes_by_id
                .iter()
                .find_map(|(node, candidate)| if candidate == read { Some(*node) } else { None })
        else {
            return Err(SignalError::invalid_input(format!(
                "recipe `{id}` references unknown read `{read}`"
            )));
        };
        let _ = view.read_aspect_version(read_node, DEFAULT_ASPECT)?;
        let value = locked.read_value(read).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "recipe `{id}` could not read current value for `{read}`"
            ))
        })?;
        read_values.insert(read.clone(), value);
    }

    let env = ExprEnvironment::new(&read_values);
    let recipe = locked
        .recipes
        .get_mut(id)
        .ok_or_else(|| SignalError::invalid_input(format!("unknown runtime recipe `{id}`")))?;

    if let Some(condition) = &recipe.spec.when {
        match env.evaluate(&condition.expr) {
            Ok(SignalValue::Bool(false)) if recipe.initialized => {
                let mut result =
                    NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                        DEFAULT_ASPECT,
                        recipe.version,
                    )]))
                    .with_output_change(OutputChange::Unchanged);
                if let Some(identity) = &recipe.output_identity {
                    result = result.with_output_identity(identity.clone());
                }
                return Ok(view.finish(result));
            }
            Ok(SignalValue::Bool(_)) => {}
            Ok(_) => {
                return Err(SignalError::invalid_input(
                    "recipe condition must evaluate to a boolean",
                ));
            }
            Err(err) => return Err(SignalError::invalid_input(err.message)),
        }
    }

    let next_value = env
        .evaluate(&recipe.spec.expr)
        .map_err(|err| SignalError::invalid_input(err.message))?;
    let next_identity = resolve_identity(&recipe.spec.identity, &env, &next_value)
        .map_err(|err| SignalError::invalid_input(err.message))?;

    let output_change = if !recipe.initialized {
        OutputChange::Replaced
    } else if recipe.output_identity == next_identity && recipe.value == next_value {
        OutputChange::Unchanged
    } else if recipe.output_identity == next_identity {
        OutputChange::Refreshed
    } else {
        OutputChange::Replaced
    };

    if !recipe.initialized || !matches!(output_change, OutputChange::Unchanged) {
        recipe.version = recipe.version.saturating_add(1);
        recipe.value = next_value;
        recipe.initialized = true;
        recipe.output_identity = next_identity.clone();
    }

    let mut result = NodeEvaluationResult::from_version(AspectVersion::from_updates([(
        DEFAULT_ASPECT,
        recipe.version,
    )]))
    .with_output_change(output_change);
    if let Some(identity) = next_identity {
        result = result.with_output_identity(identity);
    }
    Ok(view.finish(result))
}

fn resolve_identity(
    spec: &Option<IdentitySpec>,
    env: &ExprEnvironment<'_>,
    value: &SignalValue,
) -> Result<Option<String>, ForgeSignalJsError> {
    match spec {
        Some(IdentitySpec::Exact) => Ok(Some(canonical_value_string(value)?)),
        Some(IdentitySpec::Expr { expr }) => {
            Ok(Some(canonical_value_string(&env.evaluate(expr)?)?))
        }
        None => Ok(None),
    }
}

fn canonical_value_string(value: &SignalValue) -> Result<String, ForgeSignalJsError> {
    serde_json::to_string(value).map_err(|err| {
        ForgeSignalJsError::internal(format!("failed to canonicalize signal value: {err}"))
    })
}

fn rgba_signal_value(r: u8, g: u8, b: u8, a: u8) -> SignalValue {
    SignalValue::Object(vec![
        ("r".to_owned(), SignalValue::Number(r as f64)),
        ("g".to_owned(), SignalValue::Number(g as f64)),
        ("b".to_owned(), SignalValue::Number(b as f64)),
        ("a".to_owned(), SignalValue::Number(a as f64)),
    ])
}

fn set_rgba_signal_value(value: &mut SignalValue, r: u8, g: u8, b: u8, a: u8) {
    match value {
        SignalValue::Object(fields) if fields.len() == 4 => {
            fields[0].1 = SignalValue::Number(r as f64);
            fields[1].1 = SignalValue::Number(g as f64);
            fields[2].1 = SignalValue::Number(b as f64);
            fields[3].1 = SignalValue::Number(a as f64);
        }
        _ => {
            *value = rgba_signal_value(r, g, b, a);
        }
    }
}

fn composite_keyed_id(family_id: &str, key: &str) -> String {
    format!("{family_id}::{key}")
}

fn rewrite_keyed_expr(
    expr: &crate::expression::model::Expr,
    reads: &[RecipeFamilyReadSpec],
    key: &str,
) -> crate::expression::model::Expr {
    use crate::expression::model::Expr;

    match expr {
        Expr::Value { value } => Expr::Value {
            value: value.clone(),
        },
        Expr::Read { id } => {
            let rewritten = reads
                .iter()
                .find_map(|read| match read {
                    RecipeFamilyReadSpec::Signal { .. } => None,
                    RecipeFamilyReadSpec::Keyed { family_id } if family_id == id => {
                        Some(composite_keyed_id(family_id, key))
                    }
                    RecipeFamilyReadSpec::Keyed { .. } => None,
                })
                .unwrap_or_else(|| id.clone());
            Expr::Read { id: rewritten }
        }
        Expr::Get { target, field } => Expr::Get {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            field: field.clone(),
        },
        Expr::At { target, index } => Expr::At {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            index: Box::new(rewrite_keyed_expr(index, reads, key)),
        },
        Expr::First { target } => Expr::First {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Last { target } => Expr::Last {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Slice { target, start, end } => Expr::Slice {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            start: Box::new(rewrite_keyed_expr(start, reads, key)),
            end: end
                .as_ref()
                .map(|value| Box::new(rewrite_keyed_expr(value, reads, key))),
        },
        Expr::Join { target, separator } => Expr::Join {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            separator: Box::new(rewrite_keyed_expr(separator, reads, key)),
        },
        Expr::Flatten { target } => Expr::Flatten {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Object { fields } => Expr::Object {
            fields: fields
                .iter()
                .map(|(name, value)| (name.clone(), rewrite_keyed_expr(value, reads, key)))
                .collect(),
        },
        Expr::Array { items } => Expr::Array {
            items: items
                .iter()
                .map(|item| rewrite_keyed_expr(item, reads, key))
                .collect(),
        },
        Expr::Sum { args } => Expr::Sum {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Multiply { args } => Expr::Multiply {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Concat { args } => Expr::Concat {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Coalesce { args } => Expr::Coalesce {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Length { target } => Expr::Length {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Contains { target, value } => Expr::Contains {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            value: Box::new(rewrite_keyed_expr(value, reads, key)),
        },
        Expr::MergeObjects { args } => Expr::MergeObjects {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Keys { target } => Expr::Keys {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Values { target } => Expr::Values {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::HasField { target, field } => Expr::HasField {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            field: field.clone(),
        },
        Expr::Pick { target, fields } => Expr::Pick {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            fields: fields.clone(),
        },
        Expr::Omit { target, fields } => Expr::Omit {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            fields: fields.clone(),
        },
        Expr::Append { target, value } => Expr::Append {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
            value: Box::new(rewrite_keyed_expr(value, reads, key)),
        },
        Expr::Abs { target } => Expr::Abs {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Min { args } => Expr::Min {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Max { args } => Expr::Max {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Sqrt { target } => Expr::Sqrt {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Sin { target } => Expr::Sin {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Cos { target } => Expr::Cos {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Floor { target } => Expr::Floor {
            target: Box::new(rewrite_keyed_expr(target, reads, key)),
        },
        Expr::Mod { left, right } => Expr::Mod {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Clamp { value, min, max } => Expr::Clamp {
            value: Box::new(rewrite_keyed_expr(value, reads, key)),
            min: Box::new(rewrite_keyed_expr(min, reads, key)),
            max: Box::new(rewrite_keyed_expr(max, reads, key)),
        },
        Expr::Atan2 { y, x } => Expr::Atan2 {
            y: Box::new(rewrite_keyed_expr(y, reads, key)),
            x: Box::new(rewrite_keyed_expr(x, reads, key)),
        },
        Expr::Subtract { left, right } => Expr::Subtract {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Divide { left, right } => Expr::Divide {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Eq { left, right } => Expr::Eq {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Neq { left, right } => Expr::Neq {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Gt { left, right } => Expr::Gt {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Gte { left, right } => Expr::Gte {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Lt { left, right } => Expr::Lt {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::Lte { left, right } => Expr::Lte {
            left: Box::new(rewrite_keyed_expr(left, reads, key)),
            right: Box::new(rewrite_keyed_expr(right, reads, key)),
        },
        Expr::And { args } => Expr::And {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Or { args } => Expr::Or {
            args: args
                .iter()
                .map(|arg| rewrite_keyed_expr(arg, reads, key))
                .collect(),
        },
        Expr::Not { arg } => Expr::Not {
            arg: Box::new(rewrite_keyed_expr(arg, reads, key)),
        },
        Expr::If {
            condition,
            then_expr,
            else_expr,
        } => Expr::If {
            condition: Box::new(rewrite_keyed_expr(condition, reads, key)),
            then_expr: Box::new(rewrite_keyed_expr(then_expr, reads, key)),
            else_expr: Box::new(rewrite_keyed_expr(else_expr, reads, key)),
        },
    }
}
