use crate::boundary::errors::ForgeSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::{KeyedSetValue, SetValue, TransactionOp, WasmAspectId};
use crate::runtime::summaries::RunSummary;

use super::super::RuntimeCore;

impl RuntimeCore {
    pub fn set_keyed_value(
        &mut self,
        family_id: &str,
        key: &str,
        value: SignalValue,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let id = self.ensure_source_key(family_id, key, Some(value.clone()))?;
        self.apply_transaction(vec![TransactionOp::Set {
            id,
            value,
            aspect: None,
            aspects: None,
        }])
    }

    pub fn set_keyed_value_with_aspects(
        &mut self,
        family_id: &str,
        key: &str,
        value: SignalValue,
        aspects: Vec<WasmAspectId>,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        let id = self.ensure_source_key(family_id, key, Some(value.clone()))?;
        self.apply_transaction(vec![TransactionOp::Set {
            id,
            value,
            aspect: None,
            aspects: Some(aspects),
        }])
    }

    pub fn set_keyed_values(
        &mut self,
        family_id: &str,
        values: Vec<KeyedSetValue>,
    ) -> Result<RunSummary, ForgeSignalJsError> {
        if self.try_fast_seed_keyed_grid_coords(family_id, &values)? {
            return Ok(RunSummary {
                touched_nodes: 0,
                nodes_evaluated: 0,
                nodes_recomputed: 0,
                nodes_suppressed: 0,
                plans_built: 0,
                stages_executed: 0,
                total_nanos: "0".to_owned(),
                evaluation_nanos: "0".to_owned(),
                commit_nanos: "0".to_owned(),
            });
        }
        let mut normalized = Vec::with_capacity(values.len());
        for entry in values {
            let id = self.ensure_source_key(family_id, &entry.key, Some(entry.value.clone()))?;
            normalized.push(SetValue {
                id,
                value: entry.value,
                aspect: entry.aspect,
                aspects: entry.aspects,
            });
        }

        self.apply_transaction(vec![TransactionOp::SetMany { values: normalized }])
    }
}
