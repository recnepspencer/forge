//! Assembly boundary for the concrete Relational owner-service bundle.

mod basis_port;
mod lifecycle_port;
mod owner_binding;
mod service_ports;

pub use basis_port::RelationalBranchBasisPort;
pub use lifecycle_port::{RelationalBranchLifecyclePort, RelationalOwnerLifecycleObservation};
pub use service_ports::RelationalOwnerServicePorts;
