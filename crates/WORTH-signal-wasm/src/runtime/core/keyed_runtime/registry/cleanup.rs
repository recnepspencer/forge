use crate::boundary::errors::WORTHSignalJsError;

use super::super::super::RuntimeCore;

impl RuntimeCore {
    pub fn clear_keyed_family_cache(&mut self, family_id: &str) -> Result<(), WORTHSignalJsError> {
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
        store.recipes.retain(|id, recipe| {
            if id.starts_with(&prefix) {
                super::super::super::dispose_callback_recipe_token(recipe);
                return false;
            }
            true
        });
        Ok(())
    }
}
