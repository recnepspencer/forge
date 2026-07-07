mod admitted_input;
mod catalog;
mod compiled_product;
mod consumer;
mod declaration;
mod error;
mod family_identity;
mod posture;
mod selection;

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

pub use admitted_input::SpatialCompiledProductFamilyAdmittedInput;
#[cfg(test)]
pub(crate) use admitted_input::{
    admit_evidence_lookup_spatial_compiled_product_family_input,
    admit_retained_cancellation_spatial_compiled_product_family_input,
    admit_retained_replay_spatial_compiled_product_family_input,
};
pub(crate) use admitted_input::{
    admit_spatial_compiled_product_family_input, SpatialCompiledProductSupportBasis,
};
pub use catalog::{
    current_spatial_compiled_product_family_catalog, SpatialCompiledProductFamilyCatalog,
    SpatialCompiledProductFamilyCatalogCounters,
};
pub use compiled_product::SpatialCompiledProductLoweredIdentity;
pub use consumer::SpatialCompiledProductConsumer;
pub use declaration::SpatialCompiledProductFamilyDeclaration;
#[cfg(test)]
pub(crate) use declaration::SpatialCompiledProductFamilyDeclarationBuilder;
pub use error::{SpatialCompiledProductFamilyError, SpatialCompiledProductFamilyErrorKind};
pub use family_identity::SpatialCompiledProductFamilyIdentity;
pub use selection::{select_spatial_compiled_product_family, SelectedSpatialCompiledProductFamily};

#[cfg(test)]
pub(crate) use test_support::{
    real_retained_cancellation_receipt, retained_and_projected_receipts,
};
