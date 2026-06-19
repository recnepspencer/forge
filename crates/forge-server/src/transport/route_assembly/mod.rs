mod assembly_error;
mod axum_projection;
mod declared_route;
mod execution_bridge;
mod facade;
mod inventory;
mod operational_route;
mod projected_router;
mod request_decoding;
mod response_projection;
mod transport_denial;

pub use assembly_error::ForgeServerRouteAssemblyError;
pub(crate) use axum_projection::project_axum_router;
pub use declared_route::ForgeServerDeclaredRoute;
pub use execution_bridge::{
    ForgeServerOperationalRouteOutcome, ForgeServerRouteExecutionBridge,
    ForgeServerRouteExecutionOutcome,
};
pub use facade::{ForgeServerOperationRouter, ForgeServerRouteAssembly};
pub use inventory::{ForgeServerRouteInventory, ForgeServerRouteInventoryRow};
pub use operational_route::{ForgeServerOperationalRoute, ForgeServerOperationalRouteKind};
pub use projected_router::ForgeServerProjectedRouter;
pub use request_decoding::{ForgeServerRouteBranchTarget, ForgeServerRouteTransportRequest};
pub use transport_denial::{ForgeServerTransportDenial, ForgeServerTransportDenialCode};
