mod audit;
mod model;
mod registry;

pub use audit::audit_reference_consumer_adoption_sources;
pub use model::{
    WorthQueryReferenceConsumerAdoptionAudit, WorthQueryReferenceConsumerAdoptionFinding,
    WorthQueryReferenceConsumerAdoptionFindingKind, WorthQueryReferenceConsumerAdoptionRow,
    WorthQueryReferenceConsumerDeletedResidue, WorthQueryReferenceConsumerDxCounters,
    WorthQueryReferenceConsumerResidueKind, WorthQueryReferenceConsumerSource,
};
pub use registry::{
    worth_query_reference_consumer_adoption_rows, worth_query_reference_consumer_deleted_residue,
};

#[cfg(test)]
pub(super) use audit::workspace_reference_consumer_adoption_audit;
#[cfg(test)]
mod tests;
