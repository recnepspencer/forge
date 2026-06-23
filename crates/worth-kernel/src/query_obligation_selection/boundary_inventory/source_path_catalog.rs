pub(super) fn kernel_path(file: &'static str) -> &'static str {
    match file {
        "catalog.rs" => "crates/worth-kernel/src/construction/graph_obligation_adoption/catalog.rs",
        "family_execution_matrix.rs" => {
            "crates/worth-kernel/src/construction/graph_obligation_adoption/family_execution_matrix.rs"
        }
        "query_authority/declaration.rs" => {
            "crates/worth-kernel/src/construction/query_authority/declaration.rs"
        }
        "outcome.rs" => "crates/worth-kernel/src/construction/result_surface/outcome.rs",
        "residue.rs" => "crates/worth-kernel/src/construction/graph_obligation_adoption/residue.rs",
        "result.rs" => "crates/worth-kernel/src/construction/result_surface/result.rs",
        "selector_matrix.rs" => {
            "crates/worth-kernel/src/construction/graph_obligation_adoption/selector_matrix.rs"
        }
        _ => "crates/worth-kernel/src/construction/graph_obligation_adoption/mod.rs",
    }
}

pub(super) fn forge_query_path(file: &'static str) -> &'static str {
    match file {
        "kit.rs" => "crates/forge-query/src/consumer_kit/graph_obligation_adoption/kit.rs",
        "consumer_declaration.rs" => {
            "crates/forge-query/src/consumer_kit/graph_obligation_adoption/consumer_declaration.rs"
        }
        "selector_coverage.rs" => {
            "crates/forge-query/src/consumer_kit/graph_obligation_adoption/selector_coverage.rs"
        }
        "local_ceremony_audit.rs" => {
            "crates/forge-query/src/consumer_kit/graph_obligation_adoption/local_ceremony_audit.rs"
        }
        "residue_manifest.rs" => {
            "crates/forge-query/src/consumer_kit/graph_obligation_adoption/residue_manifest.rs"
        }
        "in_memory_proof/mod.rs" => {
            "crates/forge-query/src/consumer_kit/graph_obligation_adoption/in_memory_proof/mod.rs"
        }
        _ => "crates/forge-query/src/consumer_kit/graph_obligation_adoption/mod.rs",
    }
}

pub(super) fn topo_operator_path(file: &'static str) -> &'static str {
    match file {
        "catalog/mod.rs" => "crates/worth-topo/src/topology_operators/adoption/catalog/mod.rs",
        "catalog/operator_touch_descriptor.rs" => {
            "crates/worth-topo/src/topology_operators/adoption/catalog/operator_touch_descriptor.rs"
        }
        "catalog/registration_declaration.rs" => {
            "crates/worth-topo/src/topology_operators/adoption/catalog/registration_declaration.rs"
        }
        "catalog/selector_coverage.rs" => {
            "crates/worth-topo/src/topology_operators/adoption/catalog/selector_coverage.rs"
        }
        "catalog/support_pin.rs" => {
            "crates/worth-topo/src/topology_operators/adoption/catalog/support_pin.rs"
        }
        "proof.rs" => "crates/worth-topo/src/topology_operators/adoption/proof.rs",
        "residue/local_ceremony_audit.rs" => {
            "crates/worth-topo/src/topology_operators/adoption/residue/local_ceremony_audit.rs"
        }
        "residue/residue_manifest.rs" => {
            "crates/worth-topo/src/topology_operators/adoption/residue/residue_manifest.rs"
        }
        _ => "crates/worth-topo/src/topology_operators/adoption/mod.rs",
    }
}

pub(super) fn topo_operator_surface_path(file: &'static str) -> &'static str {
    match file {
        "application/declaration_entry/retained_application_handoff.rs" => {
            "crates/worth-topo/src/topology_operators/application/declaration_entry/retained_application_handoff.rs"
        }
        "application/declared_mutation_artifact.rs" => {
            "crates/worth-topo/src/topology_operators/application/declared_mutation_artifact.rs"
        }
        "application/error.rs" => "crates/worth-topo/src/topology_operators/application/error.rs",
        "application/mod.rs" => "crates/worth-topo/src/topology_operators/application/mod.rs",
        "declaration_entry/grouped/rewire_loop_successor_program.rs" => {
            "crates/worth-topo/src/topology_operators/declaration_entry/grouped/rewire_loop_successor_program.rs"
        }
        _ => "crates/worth-topo/src/topology_operators/mod.rs",
    }
}

pub(super) fn topo_path(file: &'static str) -> &'static str {
    match file {
        "query_native_boundary/compose_execution/touched_basis.rs" => "crates/worth-topo/src/construction/query_native_boundary/compose_execution/touched_basis.rs",
        "query_native_boundary/compose_execution/obligation_registration.rs" => "crates/worth-topo/src/construction/query_native_boundary/compose_execution/obligation_registration.rs",
        "query_native_boundary/compose_execution/execution.rs" => "crates/worth-topo/src/construction/query_native_boundary/compose_execution/execution.rs",
        "query_native_boundary/compose_execution/evidence.rs" => "crates/worth-topo/src/construction/query_native_boundary/compose_execution/evidence.rs",
        _ => "crates/worth-topo/src/construction/query_native_boundary/mod.rs",
    }
}

pub(super) fn spatial_path(file: &'static str) -> &'static str {
    match file {
        "query_adoption/consumer_kit.rs" => {
            "crates/worth-spatial/src/query_adoption/consumer_kit.rs"
        }
        "query_adoption/support_projection.rs" => {
            "crates/worth-spatial/src/query_adoption/support_projection.rs"
        }
        "workload_platform/evidence_ledger/spatial_touch_admission/query_lowering.rs" => {
            "crates/worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission/query_lowering.rs"
        }
        _ => "crates/worth-spatial/src/query_adoption.rs",
    }
}
