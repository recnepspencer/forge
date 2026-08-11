use crate::boundary::errors::WorthSignalJsError;
use crate::expression::model::{Expr, SignalValue};
use crate::recipe::model::RecipeSpec;
use crate::runtime::compute_callbacks;

use super::super::super::aspects::initial_aspect_version;
use super::super::super::evaluation::canonicalize_callback_reads;
use super::super::super::state::{
    CallbackDiagnosticState, StoredComputeCallbackRecipe, StoredRecipeDefinition,
    StoredRecipeOrigin, WebSignalKind,
};
use super::super::super::{RuntimeCore, DEFAULT_ASPECT};

impl RuntimeCore {
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
        if reads.is_empty() && invocation.captured_host_capability_reads.is_empty() {
            return self.install_constantized_callback_recipe(id, token, invocation.value);
        }
        let host_capability_reads = invocation.captured_host_capability_reads;
        let runtime_read_breadth = invocation.runtime_read_breadth;
        let value = invocation.value;
        self.install_signal_tracked_callback_recipe(
            id,
            token,
            host_capability_reads,
            runtime_read_breadth,
            reads,
            current_reads,
            value,
        )
    }

    fn install_constantized_callback_recipe(
        &mut self,
        id: String,
        token: compute_callbacks::ComputeCallbackToken,
        value: SignalValue,
    ) -> Result<(), WorthSignalJsError> {
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
        Ok(())
    }

    fn install_signal_tracked_callback_recipe(
        &mut self,
        id: String,
        token: compute_callbacks::ComputeCallbackToken,
        host_capability_reads: Vec<compute_callbacks::CapturedHostCapabilityRead>,
        runtime_read_breadth: u64,
        reads: Vec<crate::recipe::model::RecipeReadSpec>,
        current_reads: Vec<String>,
        value: SignalValue,
    ) -> Result<(), WorthSignalJsError> {
        let definition = StoredRecipeDefinition::Callback(StoredComputeCallbackRecipe {
            id: id.clone(),
            token,
            reads: reads.clone(),
            host_capability_reads: host_capability_reads.clone(),
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
                host_capability_reads,
                purity_posture: Some("signalTracked".to_owned()),
                last_runtime_read_breadth: runtime_read_breadth,
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
}
