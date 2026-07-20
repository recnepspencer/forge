//! Semantic integration suite. Individual responsibilities remain in named child modules.

#[path = "../admission_report_runtime.rs"]
mod admission_report_runtime;
#[path = "../admission_support_runtime.rs"]
mod admission_support_runtime;
#[path = "../application_contracts/allocation_observing_host.rs"]
mod allocation_observing_host;
#[path = "../application_contracts/canvas_spatial_execution.rs"]
mod canvas_spatial_execution;
#[path = "../application_contracts/canvas_spatial_replacement.rs"]
mod canvas_spatial_replacement;
#[path = "../application_contracts/cross_lane_bundle_execution.rs"]
mod cross_lane_bundle_execution;
#[path = "../application_contracts/egui_allocation_attribution.rs"]
mod egui_allocation_attribution;
#[path = "../application_contracts/egui_host_execution.rs"]
mod egui_host_execution;
#[path = "../application_contracts/executor_allocator_observation.rs"]
mod executor_allocator_observation;
#[path = "../facade_lifecycle_path_runtime.rs"]
mod facade_lifecycle_path_runtime;
#[path = "../application_contracts/filesystem_contract_workspace.rs"]
mod filesystem_contract_workspace;
#[path = "../application_contracts/filesystem_replacement_support.rs"]
mod filesystem_replacement_support;
#[path = "../application_contracts/filesystem_source_acquisition.rs"]
mod filesystem_source_acquisition;
#[path = "../application_contracts/filesystem_watcher_settlement.rs"]
mod filesystem_watcher_settlement;
#[path = "../application_contracts/headless_host_execution.rs"]
mod headless_host_execution;
#[path = "../application_contracts/headless_output_observer.rs"]
mod headless_output_observer;
#[path = "../application_contracts/public_application_lifecycle.rs"]
mod public_application_lifecycle;
#[path = "../application_contracts/query_consumer_kit_lifecycle.rs"]
mod query_consumer_kit_lifecycle;
#[path = "../application_contracts/query_replacement_lifecycle.rs"]
mod query_replacement_lifecycle;
#[path = "../application_contracts/realtime_overlay_execution.rs"]
mod realtime_overlay_execution;
#[path = "../application_contracts/realtime_overlay_replacement.rs"]
mod realtime_overlay_replacement;
