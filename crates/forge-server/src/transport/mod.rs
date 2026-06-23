mod axum_runtime;
mod route_assembly;

pub(crate) use axum_runtime::serve_runtime;
pub(crate) use route_assembly::project_axum_router;
pub use route_assembly::{
    ForgeServerDeclaredRoute, ForgeServerOperationRouter, ForgeServerOperationalRoute,
    ForgeServerOperationalRouteKind, ForgeServerOperationalRouteOutcome,
    ForgeServerProjectedRouter, ForgeServerRouteAssembly, ForgeServerRouteAssemblyError,
    ForgeServerRouteBranchTarget, ForgeServerRouteExecutionBridge,
    ForgeServerRouteExecutionOutcome, ForgeServerRouteInventory, ForgeServerRouteInventoryRow,
    ForgeServerRouteTransportRequest, ForgeServerTransportDenial, ForgeServerTransportDenialCode,
};
