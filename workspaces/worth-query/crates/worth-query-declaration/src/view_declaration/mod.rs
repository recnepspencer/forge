mod admission;
mod compatibility;
mod descriptor;
mod digest;
mod error;
mod family;
mod grouped_binding;
mod identity;

pub use admission::{admit_view_shape, AdmittedViewShape};
pub use compatibility::ViewShapeCompatibilityMatrixArtifact;
pub use descriptor::ViewShapeDescriptor;
pub use digest::ViewShapeDigest;
pub use error::{ViewShapeError, ViewShapeFailureClass};
pub use family::ViewShapeFamily;
pub use grouped_binding::QueryResultBindingProof;
pub use identity::{
    InspectorIdentityClassification, InspectorIdentityDigest, ViewShapeIdentityBinding,
    ViewShapeIdentityConsumption,
};
