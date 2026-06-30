mod admitted_input;
mod denial;
mod locality_basis;
mod prior_proof_basis;
mod request;
mod source_authority_basis;
mod support_posture;

#[cfg(test)]
mod tests;

pub(crate) use admitted_input::admit_spatial_compiled_product_input;
pub(crate) use admitted_input::SpatialCompiledProductAdmissionWitness;
pub(crate) use denial::SpatialCompiledProductAdmissionErrorKind;
pub(crate) use request::SpatialCompiledProductAdmissionRequest;
