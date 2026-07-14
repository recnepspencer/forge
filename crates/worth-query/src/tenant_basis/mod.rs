mod admission;
mod artifacts;
mod authority;
mod counters;
mod errors;

pub(crate) use admission::admit_tenant_bases;
pub use artifacts::{
    TenantBasisEpoch, TenantResolutionClass, TenantSchemaBasis, TenantSchemaBasisIdentity,
    TenantTruthBasis, TenantTruthBasisIdentity,
};
pub use authority::{SchemaVariantSnapshot, TenantBindingSnapshot};
pub use counters::TenantBasisCounters;
pub use errors::{TenantBasisAdmissionError, TenantBasisAdmissionFailureClass};

#[cfg(test)]
mod tests;
