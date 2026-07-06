use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use super::built_recipe::BuiltBooleanOperandPairRecipe;
use super::recipe_kind::WorkloadCatalogRecipeKind;

pub(super) fn boolean_operand_pair_cache_key(
    kind: WorkloadCatalogRecipeKind,
    declaration: &str,
    retained_replay_artifacts: bool,
) -> String {
    format!(
        "{}::{}::retained-replay-artifacts={}",
        kind.query_key(),
        declaration,
        retained_replay_artifacts
    )
}

pub(super) fn cached_boolean_operand_pair(key: &str) -> Option<BuiltBooleanOperandPairRecipe> {
    cache()
        .lock()
        .expect("boolean operand-pair cache should not be poisoned")
        .get(key)
        .cloned()
}

pub(super) fn cache_boolean_operand_pair(key: String, pair: BuiltBooleanOperandPairRecipe) {
    cache()
        .lock()
        .expect("boolean operand-pair cache should not be poisoned")
        .insert(key, pair);
}

fn cache() -> &'static Mutex<BTreeMap<String, BuiltBooleanOperandPairRecipe>> {
    static CACHE: OnceLock<Mutex<BTreeMap<String, BuiltBooleanOperandPairRecipe>>> =
        OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(BTreeMap::new()))
}
