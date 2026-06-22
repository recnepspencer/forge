mod counters;
mod inventory;
mod lifecycle;
mod match_report;
mod match_selection;
mod planner;
mod posture;
mod support_row;
mod support_row_defaults;

pub use counters::ForgeQueryGraphIndexInventoryCounters;
pub use inventory::{forge_query_graph_index_inventory, ForgeQueryGraphIndexInventory};
pub use lifecycle::{ForgeQueryGraphIndexLifecycleClass, ForgeQueryGraphIndexLifecycleOwner};
pub use match_report::{
    ForgeQueryGraphIndexInventoryMatch, ForgeQueryGraphIndexInventoryMatchReport,
};
pub use planner::match_current_graph_index_inventory_for_requirements;
pub(crate) use planner::match_graph_index_inventory_for_requirements;
pub use posture::{
    ForgeQueryGraphIndexInventoryMatchOutcome, ForgeQueryGraphIndexPosture,
    ForgeQueryGraphIndexSupportState,
};
pub use support_row::ForgeQueryGraphIndexSupportRow;
