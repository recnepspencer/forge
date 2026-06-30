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
mod tests;

pub use admitted_input::TopologyCompiledProductFamilyAdmittedInput;
pub(crate) use admitted_input::{
    triggered_invalidation_targets_from_read_basis,
    triggered_invalidation_targets_from_touched_aspects,
};
pub use catalog::{
    current_topology_compiled_product_family_catalog, TopologyCompiledProductFamilyCatalog,
    TopologyCompiledProductFamilyCatalogCounters,
};
pub(crate) use compiled_product::topology_invalidation_closure_digest;
pub use compiled_product::{
    digest_derived_validation_report, digest_interpreted_topology_view,
    digest_materialized_topology_view, DeterministicDigest, TopologyCompiledProductLoweredIdentity,
};
pub use consumer::TopologyCompiledProductConsumer;
pub use declaration::TopologyCompiledProductFamilyDeclaration;
pub use error::{TopologyCompiledProductFamilyError, TopologyCompiledProductFamilyErrorKind};
pub use family_identity::TopologyCompiledProductFamilyIdentity;
pub use posture::{
    TopologyAuthorityBasisPosture, TopologyEquivalencePolicyPosture,
    TopologyLocalityFootprintPosture, TopologyPriorProofPosture, TopologyStageIdentityPosture,
    TopologyValidatorEvidenceRolePosture,
};
pub use selection::{
    select_topology_compiled_product_family, SelectedTopologyCompiledProductFamily,
};
