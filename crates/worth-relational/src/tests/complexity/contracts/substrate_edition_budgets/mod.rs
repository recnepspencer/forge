// CONTRACT: the record substrate is lent, not copied, on ordinary read paths
// LANES: edition-acquisition, adjacency-leasing, ordinary-copy, reconstructive-copy

mod adjacency_leasing;
mod allocation_slope;
mod edition_copy_lanes;
mod partition_copy_on_write;
