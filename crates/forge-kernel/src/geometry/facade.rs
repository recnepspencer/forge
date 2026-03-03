//! Public API for the geometry domain.

// Contracts
pub use super::contracts::GeometryView;

// Data types
pub use super::data::layer::{PropertyLayer, PropertyPatch};
pub use super::data::position::ExactPosition;
pub use super::data::store::GeometryStore;
pub use super::data::draft::GeometryDraft;

// Logic — operations
pub use super::logic::coalescence::{snap_or_coalesce_vertex, CoalescenceResult};
pub use super::logic::measurements::{
    face_area, all_face_areas, edge_length, solid_volume, bounding_box, Aabb,
};
pub use super::logic::split::propagate_curve_on_split;
pub use super::logic::transforms::{transform_geometry, inverse_transform_geometry};
pub use super::logic::validation::validate_bindings;
pub use super::logic::tolerance::{
    compute_model_scale, compute_global_tolerance, GeometryToleranceProvider,
};
pub use super::logic::source_adapter::GeometrySourceAdapter;
