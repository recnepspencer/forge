mod audit;
mod model;
mod registry;
mod source;

pub use audit::{audit_declarative_surface_sources, current_declarative_surface_audit};
pub use model::{
    WorthQueryDeclarativeCapabilityFamily, WorthQueryDeclarativePhaseResponsibility,
    WorthQueryDeclarativeSurfaceClass, WorthQueryDeclarativeSurfaceRow,
};
pub use registry::worth_query_declarative_surface_rows;
pub use source::{
    WorthQueryDeclarativeSurfaceAudit, WorthQueryDeclarativeSurfaceFinding,
    WorthQueryDeclarativeSurfaceFindingKind, WorthQueryDeclarativeSurfaceSource,
    WorthQueryDeclarativeSurfaceSourceSite,
};

#[cfg(test)]
mod tests;
