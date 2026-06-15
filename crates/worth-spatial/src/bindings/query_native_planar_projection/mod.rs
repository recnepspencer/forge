mod authoring;
mod domain;
mod facts;

pub use authoring::{
    project_point_to_certified_plane_2d_entry, ProjectPointToCertifiedPlane2DCase,
    ProjectPointToCertifiedPlane2DEntry,
};
pub use domain::{
    ProjectPointToCertifiedPlane2DDeclarationFamily, ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld,
};
pub use facts::{
    project_point_to_certified_plane_2d_facts, ProjectPointToCertifiedPlane2DFactError,
};
