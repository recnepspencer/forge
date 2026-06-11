mod certified_support;
mod geometry_snapshot;
mod support_receipt;
mod support_workload;
mod surface_family;
mod unsupported_support;

pub use certified_support::{CertifiedPlaneSupport, CertifiedSurfaceSupport};
pub use geometry_snapshot::{SurfaceSupportCarrierRow, SurfaceSupportGeometrySnapshot};
pub use support_receipt::{
    SurfaceSupportCounters, SurfaceSupportReceiptSet, UnsupportedSurfaceSupportReceipt,
};
pub use support_workload::SurfaceSupportWorkload;
pub use surface_family::{SurfaceFamily, SurfaceSupportMatrixRow, SurfaceSupportStatus};
pub use unsupported_support::{UnsupportedSurfaceSupport, UnsupportedSurfaceSupportReasonCode};

pub(crate) use surface_family::support_matrix_rows;
