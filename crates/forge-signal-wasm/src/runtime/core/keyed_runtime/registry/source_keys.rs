use crate::boundary::errors::ForgeSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::SourceSpec;

use super::super::super::state::KeyedEnsureStats;
use super::super::super::RuntimeCore;

impl RuntimeCore {
    pub fn ensure_source_key(
        &mut self,
        family_id: &str,
        key: &str,
        initial: Option<SignalValue>,
    ) -> Result<String, ForgeSignalJsError> {
        self.ensure_source_key_with_stats(family_id, key, initial, &mut KeyedEnsureStats::default())
    }

    pub(super) fn ensure_source_key_with_stats(
        &mut self,
        family_id: &str,
        key: &str,
        initial: Option<SignalValue>,
        stats: &mut KeyedEnsureStats,
    ) -> Result<String, ForgeSignalJsError> {
        if let Some(grid) = self.dense_grids.get(family_id) {
            if let Some(index) = grid.key_to_index.get(key) {
                stats.source_hits = stats.source_hits.saturating_add(1);
                return Ok(grid.ids[*index].clone());
            }
            return Err(ForgeSignalJsError::invalid_input(format!(
                "key `{key}` is outside dense grid family `{family_id}`"
            )));
        }

        let composite_id = crate::runtime::core::keyed_families::composite_keyed_id(family_id, key);
        if self.catalog.contains_key(&composite_id) {
            stats.source_hits = stats.source_hits.saturating_add(1);
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
                produces_aspects: family.spec.produces_aspects.clone(),
            }
        };
        self.define_source(spec)?;
        stats.source_created = stats.source_created.saturating_add(1);
        Ok(composite_id)
    }
}
