use std::sync::OnceLock;

use super::{
    ordinary_consumer_cutover_from_inventory, WorthWorkloadOrdinaryConsumerCutover,
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};
use crate::workload_composition::current_conflict_batch_admission_inventory;
use crate::workload_composition::performance_trace::trace_scope;

pub fn current_worth_workload_ordinary_consumer_cutover(
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    static CACHE: OnceLock<WorthWorkloadOrdinaryConsumerCutover> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    trace_scope("current_worth_workload_ordinary_consumer_cutover", || {
        let inventory = current_conflict_batch_admission_inventory().map_err(|error| {
            WorthWorkloadOrdinaryConsumerCutoverError::new(
                WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingInventory,
                format!("phase 13 ordinary-consumer cutover inventory did not load: {error:?}"),
            )
        })?;
        let cutover = ordinary_consumer_cutover_from_inventory(&inventory)?;
        let _ = CACHE.set(cutover.clone());
        Ok(cutover)
    })
}
