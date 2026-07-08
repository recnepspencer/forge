//! Execution lane — handle allocation, plan lowering input, and host-facing lane adapters.

#[path = "../handle_allocation/mod.rs"]
pub mod handle_allocation;
#[path = "../lane_admission/mod.rs"]
pub mod lane_admission;
#[path = "../ordinary_lane/mod.rs"]
pub mod ordinary_lane;
#[path = "../canvas_spatial_lane/mod.rs"]
pub mod canvas_spatial_lane;
#[path = "../realtime_overlay_lane/mod.rs"]
pub mod realtime_overlay_lane;
#[path = "../virtualized_data_lane/mod.rs"]
pub mod virtualized_data_lane;
#[path = "../lane_frame_cost_certification/mod.rs"]
pub mod lane_frame_cost_certification;
#[path = "../lane_meaning_parity/mod.rs"]
pub mod lane_meaning_parity;
#[path = "../steady_frame_counter_boundary/mod.rs"]
pub mod steady_frame_counter_boundary;
#[path = "../reload_counter_boundary/mod.rs"]
pub mod reload_counter_boundary;

pub mod host_lanes;
mod transitions;

pub use transitions::WorthUiExecutionLaneInput;