mod declaration;
mod field_path;
mod materialized_posture;
mod request;

pub(crate) use declaration::NativeFactDeclarationConflict;
pub use declaration::ProjectMaterializedFacts;
pub(crate) use field_path::projection_fact_field_path_from_segments;
pub use field_path::ProjectionFactFieldPath;
pub use materialized_posture::{
    ProjectionMaterializedFactPosture, ProjectionMaterializedFactPostureKind,
};
pub use request::{ProjectionFactKind, ProjectionFactRequest};
