mod evidence;
mod source_scan;

pub use evidence::AspectNativeRejectedInputKind;
pub(crate) use evidence::{
    audit_current_aspect_native_boundaries, AspectNativeBoundaryAudit,
    AspectNativeBoundaryAuditDenial,
};
