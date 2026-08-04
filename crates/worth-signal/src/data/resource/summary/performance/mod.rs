use serde::{Deserialize, Serialize};

mod accessors;
mod admission;
mod async_capability;
mod completion;
mod construction;
mod contract;
mod declaration;
mod lifecycle;
mod policy;
mod replay;

pub use contract::{
    ResourceBoundaryKind, ResourceCostContractId, ResourceCostPosture, ResourceDensityStrategy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBoundaryPerformanceEnvelope {
    pub(in crate::data::resource::summary::performance) boundary: ResourceBoundaryKind,
    pub(in crate::data::resource::summary::performance) input_width: u32,
    pub(in crate::data::resource::summary::performance) lifecycle_transition_count: u32,
    pub(in crate::data::resource::summary::performance) admitted_count: u32,
    pub(in crate::data::resource::summary::performance) denied_count: u32,
    pub(in crate::data::resource::summary::performance) broad_scan_denial_count: u32,
    pub(in crate::data::resource::summary::performance) coalescing_width: u32,
    pub(in crate::data::resource::summary::performance) output_continuity_classification_width: u32,
    pub(in crate::data::resource::summary::performance) retry_budget_scope_touch_count: u32,
    pub(in crate::data::resource::summary::performance) temporal_wake_footprint: u32,
    pub(in crate::data::resource::summary::performance) operational_allocation_count: u32,
    pub(in crate::data::resource::summary::performance) retained_history_allocation_count: u32,
    pub(in crate::data::resource::summary::performance) diagnostics_allocation_count: u32,
    pub(in crate::data::resource::summary::performance) facade_report_allocation_count: u32,
    pub(in crate::data::resource::summary::performance) density_strategy: ResourceDensityStrategy,
    pub(in crate::data::resource::summary::performance) cost_contract: ResourceCostContractId,
    pub(in crate::data::resource::summary::performance) cost_posture: ResourceCostPosture,
}
