#[cfg(test)]
mod boundary_tests;
mod query_native_boundary;

pub(crate) use query_native_boundary::topology_primitive_construction_birth_layout_violation_registration;
pub use query_native_boundary::{
    execute_primitive_construction_birth_compose,
    prepare_primitive_construction_query_admitted_handoff,
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    prepare_primitive_construction_query_envelope, prepare_primitive_construction_query_handoff,
    prepare_primitive_construction_query_receipt,
    topology_primitive_construction_birth_graph_obligation_registration,
    TopologyConstructionQueryAdmittedHandoffError, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactRow, TopologyConstructionQueryHandoffError,
    TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface, TopologyConstructionQueryReceiptError,
    TopologyPrimitiveConstructionBirthComposeEvidence,
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
    TopologyPrimitiveConstructionBirthComposeProgram, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionBirthMaterializationCoverage,
    TopologyPrimitiveConstructionBirthSelectedObligationRow,
    TopologyPrimitiveConstructionBirthTopologyKind,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis, TopologyPrimitiveConstructionQueryEnvelope,
    TopologyPrimitiveConstructionQueryHandoff, TopologyPrimitiveConstructionQueryReceipt,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};
