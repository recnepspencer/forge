#[cfg(test)]
mod boundary_tests;
mod query_native_boundary;

pub(crate) use query_native_boundary::topology_primitive_construction_birth_layout_violation_registration;
pub use query_native_boundary::{
    prepare_primitive_construction_query_admitted_handoff,
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    prepare_primitive_construction_query_envelope, prepare_primitive_construction_query_handoff,
    prepare_primitive_construction_query_receipt,
    run_primitive_construction_birth_declared_touched_basis_compose,
    topology_primitive_construction_birth_graph_authority_proof,
    topology_primitive_construction_birth_graph_obligation_registration,
    TopologyConstructionQueryAdmittedHandoffError, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactRow, TopologyConstructionQueryHandoffError,
    TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface, TopologyConstructionQueryReceiptError,
    TopologyPrimitiveConstructionBirthComposeEvidence,
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
    TopologyPrimitiveConstructionBirthComposeProgram,
    TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
    TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionBirthGraphAuthorityProof,
    TopologyPrimitiveConstructionBirthMaterializationCoverage,
    TopologyPrimitiveConstructionBirthSelectedObligationRow,
    TopologyPrimitiveConstructionBirthTopologyKind,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis, TopologyPrimitiveConstructionQueryEnvelope,
    TopologyPrimitiveConstructionQueryHandoff, TopologyPrimitiveConstructionQueryReceipt,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};
