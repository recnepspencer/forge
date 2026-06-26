use crate::query_obligation_selection::public_facade::WorthQueryObligationSelectionMilestoneSixSeed;

use super::super::closeout::{
    WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventoryCollector,
};
use super::super::inventory_error::WorthGraphReadAccessInventoryError;
use super::super::scope::graph_read_scope_binding_for_covered_source;
use super::super::seed::WorthGraphReadAccessInventorySeed;
use super::catalog::covered_graph_read_sources;
use super::guard::validate_current_graph_read_surfaces;

pub fn current_worth_graph_read_access_surface_inventory(
    milestone_five_seed: WorthQueryObligationSelectionMilestoneSixSeed,
) -> Result<WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventoryError> {
    let seed = WorthGraphReadAccessInventorySeed::from_milestone_five_seed(milestone_five_seed)?;
    current_worth_graph_read_access_surface_inventory_from_seed(seed)
}

fn current_worth_graph_read_access_surface_inventory_from_seed(
    seed: WorthGraphReadAccessInventorySeed,
) -> Result<WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventoryError> {
    let covered_sources = covered_graph_read_sources()?;
    let guard_report = validate_current_graph_read_surfaces(covered_sources)?;

    let mut collector =
        WorthGraphReadAccessInventoryCollector::from_seed(seed).with_guard_report(guard_report);
    for source in covered_sources {
        let scope_binding =
            graph_read_scope_binding_for_covered_source(source.source_path(), collector.seed())?;
        collector = collector.admit_row(source.into_row_builder().scope_binding(scope_binding))?;
    }
    collector.closeout()
}

#[cfg(test)]
pub(crate) fn current_worth_graph_read_access_surface_inventory_for_tests(
    seed: WorthGraphReadAccessInventorySeed,
) -> Result<WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventoryError> {
    current_worth_graph_read_access_surface_inventory_from_seed(seed)
}
