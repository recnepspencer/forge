mod built_in;
mod domain_registration;
mod outcome;
mod registry;
mod registry_admission_error;
mod requirement;
mod resolved_operation;
mod resolver;
mod unsupported_denial;

pub use built_in::ForgeQueryBuiltInGraphReadOperation;
pub use domain_registration::{
    ForgeQueryDomainRegisteredGraphReadOperation, ForgeQueryGraphReadOperationRegistration,
};
pub use outcome::ForgeQueryGraphReadOperationOutcome;
pub use registry::ForgeQueryGraphReadOperationRegistry;
pub use registry_admission_error::ForgeQueryGraphReadRegistryAdmissionError;
pub use requirement::{
    ForgeQueryGraphReadOperationCapabilityRequirement,
    ForgeQueryGraphReadOperationCapabilityRequirementDeclaration,
    ForgeQueryGraphReadOperationCapabilityRequirementKind,
};
pub use resolved_operation::{
    ForgeQueryGraphReadOperationResolution, ForgeQueryGraphReadResolvedOperation,
    ForgeQueryGraphReadResolvedOperationFamily, ForgeQueryGraphReadResolvedOperationKind,
};
pub use unsupported_denial::{
    ForgeQueryGraphReadOperationUnsupportedDenial,
    ForgeQueryGraphReadOperationUnsupportedDenialKind,
    ForgeQueryGraphReadOperationUnsupportedShapeDeclaration,
};

pub(crate) use resolver::resolve_graph_read_operations_for_read_graph;
