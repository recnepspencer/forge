//! Public API for the geometry domain.

// Contracts
pub use super::contracts::GeometryView;

// Data types
pub use super::data::draft::GeometryDraft;
pub use super::data::layer::{PropertyLayer, PropertyPatch};
pub use super::data::position::ExactPosition;
pub use super::data::store::GeometryStore;

// Logic — operations
pub use super::logic::coalescence::{snap_or_coalesce_vertex, CoalescenceResult};
pub use super::logic::measurements::{
    all_face_areas, bounding_box, collect_face_positions, edge_length, face_area, solid_centroid,
    solid_volume, Aabb,
};
pub use super::logic::source_adapter::GeometrySourceAdapter;
pub use super::logic::split::propagate_curve_on_split;
pub use super::logic::tolerance::{
    compute_global_tolerance, compute_model_scale, GeometryToleranceProvider,
};
pub use super::logic::transforms::{inverse_transform_geometry, transform_geometry};
pub use super::logic::validation::validate_bindings;
