mod projection_aspect_filter;
mod projection_aspect_scope;
mod projection_contract_admission;
mod projection_mask_basis;

#[cfg(test)]
mod tests;

pub use projection_aspect_filter::{ProjectionAspectFilter, ProjectionAspectFilterMode};
pub use projection_aspect_scope::{ProjectionAspectRequirement, ProjectionAspectScope};
pub(super) use projection_contract_admission::assert_declared_projection_aspects;
