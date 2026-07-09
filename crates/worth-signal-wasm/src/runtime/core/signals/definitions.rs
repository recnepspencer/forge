use worth_signal::facade::{AspectVersion, DependencyEdge};

use crate::boundary::errors::WorthSignalJsError;
use crate::boundary::signals_model::InputOptions;
use crate::expression::model::{Expr, SignalValue};
use crate::recipe::model::{
    KeyedRecipeFamilySpec, KeyedSourceFamilySpec, RecipeReadSpec, RecipeSpec, SourceSpec,
    WasmAspectId,
};
use crate::runtime::compute_callbacks;

use super::super::aspects::{
    aspect_mask_from_list, defaulted_produced_aspects, initial_aspect_version,
    resolve_selected_aspects,
};
use super::super::evaluation::canonicalize_callback_reads;
use super::super::state::{
    CallbackDiagnosticState, CatalogEntry, StoredComputeCallbackRecipe, StoredRecipe,
    StoredRecipeDefinition, StoredRecipeFamily, StoredRecipeOrigin, StoredSource,
    StoredSourceFamily, WebSignalKind,
};
use super::super::{RuntimeCore, DEFAULT_ASPECT};

impl RuntimeCore {
    pub fn define_source_family(
        &mut self,
        spec: KeyedSourceFamilySpec,
    ) -> Result<(), WorthSignalJsError> {
        let mut store = self.lock_store()?;
        if store.source_families.contains_key(&spec.family_id)
            || store.recipe_families.contains_key(&spec.family_id)
        {
            return Err(WorthSignalJsError::invalid_input(format!(
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
    ) -> Result<(), WorthSignalJsError> {
        let mut store = self.lock_store()?;
        if store.recipe_families.contains_key(&spec.family_id)
            || store.source_families.contains_key(&spec.family_id)
        {
            return Err(WorthSignalJsError::invalid_input(format!(
                "family `{}` already exists",
                spec.family_id
            )));
        }
        for read in &spec.reads {
            match read {
                crate::recipe::model::RecipeFamilyReadSpec::Signal { id, .. } => {
                    if !self.catalog.contains_key(id) {
                        return Err(WorthSignalJsError::invalid_input(format!(
                            "keyed family `{}` reads unknown signal `{id}`",
                            spec.family_id
                        )));
                    }
                }
                crate::recipe::model::RecipeFamilyReadSpec::Keyed { family_id, .. } => {
                    if !store.source_families.contains_key(family_id)
                        && !store.recipe_families.contains_key(family_id)
                    {
                        return Err(WorthSignalJsError::invalid_input(format!(
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

    pub fn define_source(&mut self, spec: SourceSpec) -> Result<(), WorthSignalJsError> {
        self.ensure_unique_id(&spec.id)?;
        let source_id = spec.id.clone();
        let produced_aspects = defaulted_produced_aspects(spec.produces_aspects.as_deref());
        let node = self
            .runtime
            .graph_mut()
            .node()
            .produces_aspects(aspect_mask_from_list(&produced_aspects))
            .build();
        self.catalog.insert(
            source_id.clone(),
            CatalogEntry {
                node,
                produced_aspects: produced_aspects.clone(),
            },
        );
        self.nodes_by_id.insert(node, source_id.clone());
        let mut store = self.lock_store()?;
        store.sources.insert(
            source_id.clone(),
            StoredSource {
                value: spec.initial,
                version: initial_aspect_version(&produced_aspects),
            },
        );
        drop(store);

        let evaluator = self.evaluator();
        self.runtime
            .read(node, &self.store, &evaluator)
            .map_err(WorthSignalJsError::from)?;
        self.runtime.clear_live_branch_mutation_residue();
        Ok(())
    }

    pub fn define_web_input(
        &mut self,
        id: String,
        initial: SignalValue,
        options: Option<InputOptions>,
    ) -> Result<(), WorthSignalJsError> {
        self.define_source(SourceSpec {
            id: id.clone(),
            initial,
            produces_aspects: options.and_then(|options| options.produces_aspects),
        })?;
        self.web_signals.insert(id, WebSignalKind::Input);
        Ok(())
    }

    pub fn define_recipe(&mut self, spec: RecipeSpec) -> Result<(), WorthSignalJsError> {
        let id = spec.id.clone();
        self.insert_recipe_definition(
            id,
            spec.reads.clone(),
            spec.produces_aspects.clone(),
            StoredRecipeOrigin::ExprSpec,
            StoredRecipeDefinition::Expr(spec),
        )
    }

    pub fn define_web_computed(
        &mut self,
        id: String,
        spec: RecipeSpec,
    ) -> Result<(), WorthSignalJsError> {
        self.define_recipe(spec)?;
        self.web_signals.insert(id, WebSignalKind::Computed);
        Ok(())
    }

    pub fn install_web_computed_callback_recipe(
        &mut self,
        id: String,
        token: compute_callbacks::ComputeCallbackToken,
        invocation: compute_callbacks::ComputeCallbackInvocationResult,
    ) -> Result<(), WorthSignalJsError> {
        let reads = canonicalize_callback_reads(invocation.captured_read_ids);
        let current_reads = reads.iter().map(|read| read.id().to_owned()).collect();
        let value = invocation.value;
        if reads.is_empty() && invocation.captured_host_capability_reads.is_empty() {
            let disposed = compute_callbacks::dispose_compute(token);
            debug_assert!(
                disposed,
                "compute callback tokens should dispose after constant callback lowering"
            );
            self.insert_recipe_definition(
                id.clone(),
                Vec::new(),
                None,
                StoredRecipeOrigin::CallbackConstantizedNoSignalReads,
                StoredRecipeDefinition::Expr(RecipeSpec {
                    id: id.clone(),
                    reads: Vec::new(),
                    expr: Expr::Value {
                        value: value.clone(),
                    },
                    when: None,
                    identity: None,
                    produces_aspects: None,
                }),
            )?;
            let mut store = self.lock_store()?;
            let recipe = store.recipes.get_mut(&id).ok_or_else(|| {
                WorthSignalJsError::internal(format!(
                    "constantized callback recipe `{id}` missing after definition"
                ))
            })?;
            let produced_aspects = self
                .catalog
                .get(&id)
                .map(|entry| entry.produced_aspects.clone())
                .unwrap_or_else(|| vec![DEFAULT_ASPECT]);
            recipe.value = value;
            recipe.initialized = true;
            recipe.version = initial_aspect_version(&produced_aspects);
            drop(store);
            self.lock_callback_diagnostics()?.insert(
                id.clone(),
                CallbackDiagnosticState {
                    purity_posture: Some("constantizedNoSignalReads".to_owned()),
                    ..CallbackDiagnosticState::default()
                },
            );
            self.web_signals.insert(id, WebSignalKind::Computed);
            self.web_metrics
                .compute_callback_constant_no_signal_read_classification_count = self
                .web_metrics
                .compute_callback_constant_no_signal_read_classification_count
                .saturating_add(1);
            return Ok(());
        }

        let definition = StoredRecipeDefinition::Callback(StoredComputeCallbackRecipe {
            id: id.clone(),
            token,
            reads: reads.clone(),
            host_capability_reads: invocation.captured_host_capability_reads.clone(),
            produces_aspects: None,
        });
        self.insert_recipe_definition(
            id.clone(),
            reads,
            None,
            StoredRecipeOrigin::CallbackSignalTracked,
            definition,
        )?;
        let mut store = self.lock_store()?;
        let recipe = store.recipes.get_mut(&id).ok_or_else(|| {
            WorthSignalJsError::internal(format!("callback recipe `{id}` missing after definition"))
        })?;
        let produced_aspects = self
            .catalog
            .get(&id)
            .map(|entry| entry.produced_aspects.clone())
            .unwrap_or_else(|| vec![DEFAULT_ASPECT]);
        recipe.value = value;
        recipe.initialized = true;
        recipe.version = initial_aspect_version(&produced_aspects);
        drop(store);
        self.lock_callback_diagnostics()?.insert(
            id.clone(),
            CallbackDiagnosticState {
                current_reads,
                host_capability_reads: invocation.captured_host_capability_reads,
                purity_posture: Some("signalTracked".to_owned()),
                last_runtime_read_breadth: invocation.runtime_read_breadth,
                ..CallbackDiagnosticState::default()
            },
        );
        self.web_signals.insert(id, WebSignalKind::Computed);
        self.web_metrics
            .compute_callback_signal_tracked_classification_count = self
            .web_metrics
            .compute_callback_signal_tracked_classification_count
            .saturating_add(1);
        Ok(())
    }

    #[cfg(test)]
    pub fn define_web_computed_native_callback(
        &mut self,
        id: String,
        callback: Box<
            dyn Fn() -> Result<
                compute_callbacks::ComputeCallbackInvocationResult,
                compute_callbacks::ComputeCallbackFailure,
            >,
        >,
    ) -> Result<(), WorthSignalJsError> {
        let token = compute_callbacks::register_native_compute_result(callback);
        let invocation = match compute_callbacks::invoke_compute(token) {
            Ok(invocation) => invocation,
            Err(failure) => {
                let _ = compute_callbacks::dispose_compute(token);
                return Err(WorthSignalJsError::from_compute_callback_failure(failure));
            }
        };
        self.install_web_computed_callback_recipe(id, token, invocation)
    }

    #[cfg(test)]
    pub fn dispose_web_computed_callback_for_test(
        &mut self,
        id: &str,
    ) -> Result<bool, WorthSignalJsError> {
        let store = self.lock_store()?;
        let recipe = store.recipes.get(id).ok_or_else(|| {
            WorthSignalJsError::invalid_input(format!("unknown callback recipe `{id}`"))
        })?;
        let StoredRecipeDefinition::Callback(callback) = &recipe.definition else {
            return Err(WorthSignalJsError::invalid_input(format!(
                "signal `{id}` is not callback-backed"
            )));
        };
        Ok(compute_callbacks::dispose_compute(callback.token))
    }

    pub fn define_web_output(
        &mut self,
        id: String,
        spec: RecipeSpec,
    ) -> Result<(), WorthSignalJsError> {
        self.define_recipe(spec)?;
        self.web_signals.insert(id, WebSignalKind::Output);
        Ok(())
    }

    pub(crate) fn mark_worker_public_outputs(
        &mut self,
        output_ids: Vec<String>,
    ) -> Result<(), WorthSignalJsError> {
        for output_id in output_ids {
            if !self.catalog.contains_key(&output_id) {
                return Err(WorthSignalJsError::invalid_input(format!(
                    "worker public output `{output_id}` is not published"
                )));
            }
            if !self.lock_store()?.recipes.contains_key(&output_id) {
                return Err(WorthSignalJsError::invalid_input(format!(
                    "worker public output `{output_id}` must be a recipe"
                )));
            }
            self.web_signals.insert(output_id, WebSignalKind::Output);
        }
        Ok(())
    }

    pub(crate) fn is_web_output_signal(&self, id: &str) -> bool {
        matches!(self.web_signals.get(id), Some(WebSignalKind::Output))
    }

    #[cfg(test)]
    pub fn web_signal_kind(&self, id: &str) -> Option<WebSignalKind> {
        self.web_signals.get(id).copied()
    }

    pub(super) fn insert_recipe_definition(
        &mut self,
        id: String,
        reads: Vec<RecipeReadSpec>,
        produces_aspects_spec: Option<Vec<WasmAspectId>>,
        origin: StoredRecipeOrigin,
        definition: StoredRecipeDefinition,
    ) -> Result<(), WorthSignalJsError> {
        self.ensure_unique_id(&id)?;
        self.ensure_known_reads(&reads)?;
        let mut read_aspects = Vec::new();
        let mut dependencies = Vec::new();
        for read in &reads {
            let entry = self.catalog.get(read.id()).ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!("unknown read `{}`", read.id()))
            })?;
            let aspects = resolve_selected_aspects(read.aspect_spec())?;
            read_aspects.extend(aspects.iter().copied());
            for aspect in aspects {
                let edge = match read.scope() {
                    Some(scope) => {
                        DependencyEdge::with_partition_scope(entry.node, aspect, scope.clone())
                    }
                    None => DependencyEdge::new(entry.node, aspect),
                };
                dependencies.push(edge);
            }
        }
        read_aspects.sort_by_key(|aspect| aspect.id());
        read_aspects.dedup_by_key(|aspect| aspect.id());
        let produced_aspects = defaulted_produced_aspects(produces_aspects_spec.as_deref());
        let mut graph = self.runtime.graph_mut();
        let mut builder = graph
            .node()
            .on_demand()
            .produces_aspects(aspect_mask_from_list(&produced_aspects));
        if !read_aspects.is_empty() {
            builder = builder.reads_aspects(aspect_mask_from_list(&read_aspects));
        }
        let node = builder.build();
        graph
            .set_dependencies(node, dependencies)
            .map_err(WorthSignalJsError::from)?;
        drop(graph);
        self.catalog.insert(
            id.clone(),
            CatalogEntry {
                node,
                produced_aspects,
            },
        );
        self.nodes_by_id.insert(node, id.clone());
        let mut store = self.lock_store()?;
        store.recipes.insert(
            id,
            StoredRecipe {
                definition,
                origin,
                value: SignalValue::Null,
                version: AspectVersion::zero(),
                initialized: false,
                output_identity: None,
            },
        );
        Ok(())
    }

    fn ensure_unique_id(&self, id: &str) -> Result<(), WorthSignalJsError> {
        if self.catalog.contains_key(id) {
            return Err(WorthSignalJsError::invalid_input(format!(
                "signal id `{id}` already exists"
            )));
        }
        Ok(())
    }

    fn ensure_known_reads(&self, reads: &[RecipeReadSpec]) -> Result<(), WorthSignalJsError> {
        for read in reads {
            if !self.catalog.contains_key(read.id()) {
                return Err(WorthSignalJsError::invalid_input(format!(
                    "unknown read `{}`",
                    read.id()
                )));
            }
        }
        Ok(())
    }
}
