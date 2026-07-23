//! Execution lane — handle allocation, plan lowering input, and host-facing lane adapters.

pub mod canvas_spatial_lane;
pub mod handle_allocation;
pub mod lane_admission;
pub mod lane_frame_cost_certification;
pub mod lane_meaning_parity;
pub mod ordinary_lane;
pub mod realtime_overlay_lane;
pub mod reload_counter_boundary;
pub mod steady_frame_counter_boundary;
pub mod virtualized_data_lane;

pub mod host_lanes;
