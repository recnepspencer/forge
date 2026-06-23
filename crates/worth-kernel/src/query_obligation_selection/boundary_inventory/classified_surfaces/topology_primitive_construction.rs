use crate::query_obligation_selection::boundary_inventory::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class,
};
use crate::query_obligation_selection::boundary_inventory::row_constructors::topo_primitive;

pub(super) fn rows() -> Vec<QuerySelectionBoundaryInventoryRow> {
    vec![
        topo_primitive("TopologyPrimitiveConstructionBirthDeclaredTouchedBasis", "query_native_boundary/compose_execution/touched_basis.rs", Class::SourceDescriptor, Posture::DescriptorInput, Proof::SourceDescriptorOnly, Action::KeepAsSourceDescriptor),
        topo_primitive("topology_primitive_construction_birth_graph_obligation_registration", "query_native_boundary/compose_execution/obligation_registration.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, Action::MigrateToParallelSelectionSubstrate),
        topo_primitive("TopologyPrimitiveConstructionBirthGraphAuthorityProof", "query_native_boundary/compose_execution/execution.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::KeepAsQueryOwnedSelection),
        topo_primitive("TopologyPrimitiveConstructionBirthGraphAuthorityProof::graph_obligation_envelope_digest", "query_native_boundary/compose_execution/execution.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::KeepAsQueryOwnedSelection),
        topo_primitive("TopologyPrimitiveConstructionBirthGraphAuthorityProof::graph_obligation_selected_count", "query_native_boundary/compose_execution/execution.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::KeepAsQueryOwnedSelection),
        topo_primitive("TopologyPrimitiveConstructionBirthComposeExecution::graph_obligation_envelope_digest", "query_native_boundary/compose_execution/execution.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::KeepAsQueryOwnedSelection),
        topo_primitive("TopologyPrimitiveConstructionBirthComposeExecution::graph_authority_proof", "query_native_boundary/compose_execution/execution.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::KeepAsQueryOwnedSelection),
        topo_primitive("TopologyPrimitiveConstructionBirthComposeEvidence::graph_obligation_envelope_digest", "query_native_boundary/compose_execution/evidence.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::KeepAsQueryOwnedSelection),
        topo_primitive("TopologyPrimitiveConstructionBirthComposeEvidence::graph_obligation_selected_count", "query_native_boundary/compose_execution/evidence.rs", Class::QueryOwnedSelection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::KeepAsQueryOwnedSelection),
    ]
}
