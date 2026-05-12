use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::placement::{PlacementDeclarationCandidate, PlacementDeclarationOrigin};

use super::state::{StoredRecipe, StoredRecipeDefinition, StoredRecipeOrigin};
use super::RuntimeCore;

impl RuntimeCore {
    pub(crate) fn collect_worker_placement_declaration_candidates(
        &self,
    ) -> Result<Vec<PlacementDeclarationCandidate>, ForgeSignalJsError> {
        let store = self.lock_store()?;
        Ok(store
            .recipes
            .iter()
            .map(|(id, recipe)| {
                derive_worker_placement_declaration_candidate_from_stored_recipe(self, id, recipe)
            })
            .collect())
    }
}

fn derive_worker_placement_declaration_candidate_from_stored_recipe(
    runtime: &RuntimeCore,
    id: &str,
    recipe: &StoredRecipe,
) -> PlacementDeclarationCandidate {
    let callback_posture = stored_recipe_callback_posture(&recipe.definition);
    PlacementDeclarationCandidate {
        id: id.to_owned(),
        signal_kind: runtime.web_signals.get(id).copied(),
        origin: stored_recipe_origin_to_placement_declaration_origin(recipe.origin),
        has_live_callback: callback_posture.has_live_callback,
        callback_runtime_read_count: callback_posture.callback_runtime_read_count,
        host_capability_read_count: callback_posture.host_capability_read_count,
        is_unavailable: false,
    }
}

struct StoredRecipeCallbackPosture {
    has_live_callback: bool,
    callback_runtime_read_count: usize,
    host_capability_read_count: usize,
}

fn stored_recipe_callback_posture(
    definition: &StoredRecipeDefinition,
) -> StoredRecipeCallbackPosture {
    match definition {
        StoredRecipeDefinition::Expr(_) => StoredRecipeCallbackPosture {
            has_live_callback: false,
            callback_runtime_read_count: 0,
            host_capability_read_count: 0,
        },
        StoredRecipeDefinition::Callback(callback) => StoredRecipeCallbackPosture {
            has_live_callback: true,
            callback_runtime_read_count: callback.reads.len(),
            host_capability_read_count: callback.host_capability_reads.len(),
        },
    }
}

fn stored_recipe_origin_to_placement_declaration_origin(
    origin: StoredRecipeOrigin,
) -> PlacementDeclarationOrigin {
    match origin {
        StoredRecipeOrigin::ExprSpec => PlacementDeclarationOrigin::ExprSpec,
        StoredRecipeOrigin::CallbackSignalTracked => {
            PlacementDeclarationOrigin::CallbackSignalTracked
        }
        StoredRecipeOrigin::CallbackConstantizedNoSignalReads => {
            PlacementDeclarationOrigin::CallbackConstantizedNoSignalReads
        }
    }
}
