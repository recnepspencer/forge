use super::classification::{
    DerivedInvalidationAuthorityDisposition as Disposition,
    DerivedInvalidationAuthorityOwner as Owner, DerivedInvalidationOldAuthorityKind as Kind,
    DerivedInvalidationProductCategory as Category, DerivedInvalidationReplacementPhase as Phase,
};
use super::report::DerivedInvalidationAuthorityInventoryReport;
use super::row::DerivedInvalidationAuthorityInventoryRow;

pub fn current_derived_invalidation_authority_inventory(
) -> DerivedInvalidationAuthorityInventoryReport {
    DerivedInvalidationAuthorityInventoryReport::new(current_rows())
}

fn current_rows() -> Vec<DerivedInvalidationAuthorityInventoryRow> {
    vec![
        certification_residue_row(
            "crates/worth-topo/src/derived_topology/materialized_graph/mod.rs",
            "TopologyMaterializer::materialize_from_truth",
            Category::CertificationBootstrap,
            Kind::WholeViewMaterialization,
        ),
        row_with_disposition(
            "crates/worth-topo/src/derived_topology/materialized_graph/mod.rs",
            "TopologyMaterializer::materialize_query_input",
            Category::MaterializedGraph,
            Kind::QueryInputMaterialization,
            Disposition::Delete,
            Phase::PhaseSixProductMigrationSweep,
            true,
        ),
        certification_residue_row(
            "crates/worth-topo/src/derived_topology/materialized_graph/types.rs",
            "MaterializationFallbackClass::WholeViewRebuild",
            Category::CertificationBootstrap,
            Kind::WholeViewMaterialization,
        ),
        certification_residue_row(
            "crates/worth-topo/src/derived_topology/materialized_graph/mod.rs",
            "TopologyMaterializer::materialize_from_rows WholeViewRebuild fallback",
            Category::CertificationBootstrap,
            Kind::WholeViewMaterialization,
        ),
        row_with_disposition(
            "crates/worth-topo/src/derived_topology/traversal_views/facade.rs",
            "interpret_topology_view",
            Category::TraversalViews,
            Kind::TraversalInterpretation,
            Disposition::Delete,
            Phase::PhaseSixProductMigrationSweep,
            true,
        ),
        row_with_disposition(
            "crates/worth-topo/src/derived_topology/loop_cycles/mod.rs",
            "deleted_loop_cycle_direct_interpreter_path",
            Category::LoopCycles,
            Kind::TraversalInterpretation,
            Disposition::Delete,
            Phase::PhaseEightDeletionFirewall,
            true,
        ),
        row_with_disposition(
            "crates/worth-topo/src/derived_topology/radial_rings/mod.rs",
            "deleted_radial_ring_direct_interpreter_path",
            Category::RadialRings,
            Kind::TraversalInterpretation,
            Disposition::Delete,
            Phase::PhaseEightDeletionFirewall,
            true,
        ),
        row_with_disposition(
            "crates/worth-topo/src/derived_topology/traversal_views/shell_compatibility.rs",
            "interpret_shells interpret_shell_radial_surface",
            Category::ShellViews,
            Kind::TraversalInterpretation,
            Disposition::Delete,
            Phase::PhaseEightDeletionFirewall,
            true,
        ),
        row_with_disposition(
            "crates/worth-topo/src/derived_topology/traversal_views/vertex_disk_compatibility.rs",
            "interpret_wire_branching",
            Category::VertexDisks,
            Kind::TraversalInterpretation,
            Disposition::Delete,
            Phase::PhaseEightDeletionFirewall,
            true,
        ),
        row_with_disposition(
            "crates/worth-topo/src/derived_topology/traversal_views/wire_compatibility.rs",
            "interpret_wires",
            Category::WireViews,
            Kind::TraversalInterpretation,
            Disposition::Delete,
            Phase::PhaseEightDeletionFirewall,
            true,
        ),
        row_with_disposition(
            "crates/worth-topo/src/projection/runtime_boundary/read_stage.rs",
            "deleted_projection_read_stage_ordinary_expansion",
            Category::ProjectionReadStage,
            Kind::ProjectionReadStage,
            Disposition::Delete,
            Phase::PhaseSixProductMigrationSweep,
            true,
        ),
        certification_residue_row(
            "crates/worth-topo/src/projection/runtime_boundary/read_stage.rs",
            "stage_topology_read_from_view bootstrap_topology_interpretation",
            Category::CertificationBootstrap,
            Kind::ProjectionReadStage,
        ),
        certification_residue_row(
            "crates/worth-topo/src/certification/topology_operator_closeout/derived_fallout/derived_work_breadth_rows.rs",
            "MilestoneThreeDerivedWorkBreadthRow",
            Category::CertificationBootstrap,
            Kind::OperatorDerivedBreadthCloseout,
        ),
        certification_residue_row(
            "crates/worth-topo/src/certification/topology_operator_closeout/derived_fallout/derived_work_breadth.rs",
            "derived_work_breadth acceptance over declared_derived_region_count and fallback_count",
            Category::CertificationBootstrap,
            Kind::OperatorDerivedBreadthCloseout,
        ),
        certification_residue_row(
            "crates/worth-topo/src/certification/topology_operator_closeout/derived_fallout/fallback_policy_denial_rows.rs",
            "MilestoneThreeDerivedFallbackPolicyDenialRow",
            Category::CertificationBootstrap,
            Kind::FallbackPolicyDenial,
        ),
        certification_residue_row(
            "crates/worth-topo/src/certification/topology_operator_closeout/derived_fallout/fallback_policy_denial.rs",
            "fallback_policy_denial acceptance over observed_fallback_count",
            Category::CertificationBootstrap,
            Kind::FallbackPolicyDenial,
        ),
        certification_residue_row(
            "crates/worth-topo/src/derived_topology/traversal_views/tests.rs",
            "MaterializedTopologyView::whole_view",
            Category::CertificationBootstrap,
            Kind::TestOnlyWholeViewFixture,
        ),
        certification_residue_row(
            "crates/worth-topo/src/certification/topology_operator_closeout/shared.rs",
            "derived_validation_report_from_materialized",
            Category::CertificationBootstrap,
            Kind::DerivedValidationDiagnostic,
        ),
    ]
}

fn row_with_disposition(
    source_path: &'static str,
    surface: &'static str,
    category: Category,
    kind: Kind,
    disposition: Disposition,
    replacement_phase: Phase,
    ordinary_path: bool,
) -> DerivedInvalidationAuthorityInventoryRow {
    DerivedInvalidationAuthorityInventoryRow::new(
        source_path,
        surface,
        category,
        kind,
        disposition,
        owner_for_category(category),
        "Milestone 10 requires declare-once derived product invalidation",
        "covered product migrates through derived_topology::invalidation_plan",
        replacement_phase,
        ordinary_path,
        false,
        None,
    )
}

fn certification_residue_row(
    source_path: &'static str,
    surface: &'static str,
    category: Category,
    kind: Kind,
) -> DerivedInvalidationAuthorityInventoryRow {
    DerivedInvalidationAuthorityInventoryRow::new(
        source_path,
        surface,
        category,
        kind,
        Disposition::CertificationBootstrapResidue,
        Owner::WorthTopoCertification,
        "certification/bootstrap comparison still needs whole-view oracle",
        "ordinary derived invalidation receipts replace certification comparison",
        Phase::CertificationBootstrapResidue,
        false,
        true,
        Some(1),
    )
}

fn owner_for_category(category: Category) -> Owner {
    match category {
        Category::ProjectionReadStage => Owner::WorthTopoProjectionRuntimeBoundary,
        Category::OperatorCloseout => Owner::WorthTopoOperatorCloseout,
        Category::CertificationBootstrap => Owner::WorthTopoCertification,
        _ => Owner::WorthTopoDerivedTopology,
    }
}
