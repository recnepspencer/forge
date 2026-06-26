use crate::query_obligation_selection::boundary_inventory::inventory_record::{
    QuerySelectionAuthorityPosture as Posture, QuerySelectionBoundaryInventoryRow,
    QuerySelectionDeletionAction as Action, QuerySelectionProofStrength as Proof,
    QuerySelectionSurfaceClassification as Class,
};
use crate::query_obligation_selection::boundary_inventory::row_constructors::{
    topo_operator, topo_operator_application, topo_operator_residue, topo_operator_surface,
};

pub(super) fn rows() -> Vec<QuerySelectionBoundaryInventoryRow> {
    vec![
        topo_operator("topology_operator_relation_touch_descriptor", "catalog/operator_touch_descriptor.rs", Class::SourceDescriptor, Posture::DescriptorInput, Proof::SourceDescriptorOnly, Action::KeepAsSourceDescriptor),
        topo_operator("topology_operator_graph_obligation_catalog", "catalog/mod.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, Action::MigrateToParallelSelectionSubstrate),
        topo_operator("topology_operator_graph_obligation_registration_declaration", "catalog/registration_declaration.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, Action::MigrateToParallelSelectionSubstrate),
        topo_operator("topology_operator_runtime_graph_obligation_registrations", "catalog/registration_declaration.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, Action::MigrateToParallelSelectionSubstrate),
        topo_operator("topology_operator_graph_obligation_selector_coverage", "catalog/selector_coverage.rs", Class::MigrationProjection, Posture::SelectorCoverageDeclaration, Proof::RegistrationOnly, Action::CollapseToQueryOwnedSelection),
        topo_operator("topology_operator_graph_obligation_support_pin", "catalog/support_pin.rs", Class::CertificationOnlySupport, Posture::SupportPin, Proof::SupportOnly, Action::CertificationOnly),
        topo_operator("topology_operator_graph_obligation_support_matrix", "catalog/support_pin.rs", Class::CertificationOnlySupport, Posture::SupportMatrix, Proof::SupportOnly, Action::CertificationOnly),
        topo_operator("topology_operator_graph_obligation_local_ceremony_audit", "residue/local_ceremony_audit.rs", Class::CertificationOnlySupport, Posture::LocalCeremonyAudit, Proof::LocalCeremonyOnly, Action::CertificationOnly),
        topo_operator_residue("topology_operator_graph_obligation_residue_manifest", "operator residue rows capped by class until each operator family emits covered Query obligation evidence", "multiple topology operators remain residue instead of selected obligations", "each operator residue class has a covered catalog row, selector coverage, support pin, and runtime envelope test"),
        topo_operator("topology_operator_graph_obligation_adoption_proof", "proof.rs", Class::MigrationProjection, Posture::InMemorySelectionAdoption, Proof::InMemorySelection, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_application("TopologyMutationApplicationEvidence::from_inspection_and_graph_obligation_projection", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_application("TopologyMutationApplicationEvidence::graph_obligation_envelope_digest", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_application("TopologyMutationApplicationEvidence::graph_obligation_dispatch_digest", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_application("TopologyMutationApplicationEvidence::graph_obligation_execution_point", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_application("TopologyMutationApplicationEvidence::graph_obligation_selected_count", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_surface("TopologyRetainedApplicationHandoff::graph_obligation_dispatch_projection", "application/declaration_entry/retained_application_handoff.rs", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_surface("TopologyMutationApplicationStop::graph_obligation_envelope_digest", "application/mod.rs", Class::MigrationProjection, Posture::PublicFacadeStatus, Proof::PublicStatusOnly, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_surface("TopologyDeclaredMutationArtifact::graph_obligation_envelope_digest", "application/declared_mutation_artifact.rs", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_surface("TopologyDeclaredMutationArtifact::graph_obligation_orchestration", "application/declared_mutation_artifact.rs", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_surface("TopologyMutationApplicationError::declaration_entry_graph_obligation_envelope_digest", "application/error.rs", Class::MigrationProjection, Posture::SelectedObligationExecutionEvidence, Proof::ExecutionEnvelope, Action::MigrateToParallelSelectionSubstrate),
        topo_operator_surface("topology_rewire_loop_successor_graph_obligation_registration", "declaration_entry/grouped/rewire_loop_successor_program.rs", Class::MigrationProjection, Posture::RegistrationDeclaration, Proof::RegistrationOnly, Action::MigrateToParallelSelectionSubstrate),
    ]
}
