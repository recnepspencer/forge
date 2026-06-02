use topology::facade::{
    declare_persistent_name_live_view, declare_topology_diagnostics_surface,
    declare_topology_entity_live_view, declare_topology_equivalence_contract_surface,
    declare_topology_interpreted_surface, declare_topology_materialized_surface,
    declare_topology_relation_live_view, declare_topology_validation_surface,
    naming_attachment_report_from_query_input, persistent_name_live_view_declaration,
    topology_diagnostics_computed_declaration, topology_entity_live_view_declaration,
    topology_equivalence_contract_computed_declaration, topology_interpreted_computed_declaration,
    topology_materialized_computed_declaration, topology_relation_live_view_declaration,
    topology_validation_computed_declaration, TopologyDiagnosticsMaintainer,
    TopologyEquivalenceContractMaintainer, TopologyInterpretedMaintainer,
    TopologyMaterializedMaintainer, TopologyNamingAttachmentInput, TopologyQueryMutationEvidence,
    TopologyQuerySurfaceError, TopologyValidationMaintainer,
};

fn main() {
    let _ = declare_persistent_name_live_view::<serde_json::Value>;
    let _ = declare_topology_entity_live_view::<serde_json::Value>;
    let _ = declare_topology_relation_live_view::<serde_json::Value>;
    let _ = persistent_name_live_view_declaration;
    let _ = topology_entity_live_view_declaration;
    let _ = topology_relation_live_view_declaration;
    let _ = topology_materialized_computed_declaration;
    let _ = declare_topology_materialized_surface::<
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
    >;
    let _ = topology_interpreted_computed_declaration;
    let _ = declare_topology_interpreted_surface::<serde_json::Value, serde_json::Value>;
    let _ = topology_validation_computed_declaration;
    let _ = declare_topology_validation_surface::<
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
    >;
    let _ = topology_diagnostics_computed_declaration;
    let _ = declare_topology_diagnostics_surface::<
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
    >;
    let _ = topology_equivalence_contract_computed_declaration;
    let _ = declare_topology_equivalence_contract_surface::<
        serde_json::Value,
        serde_json::Value,
    >;
    let _ = naming_attachment_report_from_query_input;
    let _ = TopologyMaterializedMaintainer::new;
    let _ = TopologyInterpretedMaintainer::new;
    let _ = TopologyValidationMaintainer::new;
    let _ = TopologyDiagnosticsMaintainer::new;
    let _ = TopologyEquivalenceContractMaintainer::new;
    let _ = TopologyQuerySurfaceError::to_string;
    let _ = TopologyNamingAttachmentInput::new;
    let _ = TopologyQueryMutationEvidence::metadata_key;
}
