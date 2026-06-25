use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::authority::{WireInterpretationClass, WireInterpretationRecord};
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::{
    status_rows_from_loop_cycle_migration_closeout, status_rows_from_migrated_family_closeouts,
    CoveredDerivedProductStatusRow,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::loop_cycles::{
    close_loop_cycle_migration_slice, LoopCycleBoundarySourceRow, LoopCycleExecutionInput,
    LoopCycleMigrationCloseout, LoopCycleReadSource, LoopCycleReadStageExecutor,
};
use crate::derived_topology::invalidation_plan::migrated_products::materialized_graph::{
    close_materialized_graph_migration_slice, MaterializedGraphExecutionInput,
    MaterializedGraphReadSource, MaterializedGraphReadStageExecutor,
};
use crate::derived_topology::invalidation_plan::migrated_products::radial_rings::{
    close_radial_ring_migration_slice, RadialRingBoundarySourceRow, RadialRingExecutionInput,
    RadialRingReadSource, RadialRingReadStageExecutor,
};
use crate::derived_topology::invalidation_plan::migrated_products::shell_views::{
    close_shell_view_migration_slice, ShellViewBoundarySourceRow, ShellViewExecutionInput,
    ShellViewReadSource, ShellViewReadStageExecutor,
};
use crate::derived_topology::invalidation_plan::migrated_products::traversal_views::{
    close_traversal_views_migration_slice, TraversalViewsExecutionInput, TraversalViewsReadSource,
    TraversalViewsReadStageExecutor,
};
use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::{
    close_vertex_disk_migration_slice, VertexDiskBoundarySourceRow, VertexDiskExecutionInput,
    VertexDiskReadSource, VertexDiskReadStageExecutor,
};
use crate::derived_topology::invalidation_plan::migrated_products::wire_views::{
    close_wire_view_migration_slice, WireViewExecutionInput, WireViewMigrationCloseout,
    WireViewReadSource, WireViewReadStageCounters, WireViewReadStageExecutor, WireViewSourceRow,
};
use crate::derived_topology::invalidation_plan::migrated_products::{
    CoveredDerivedProductMigrationSweepCloseout, MigratedDerivedProductFamilyCloseout,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    loop_cycles_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
    DerivedInvalidationTouchedClosure,
};
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(super) fn selected_loop_cycle_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("phase-six-sweep"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn loop_cycle_closeout() -> LoopCycleMigrationCloseout {
    let plan = selected_loop_cycle_plan();
    let read_source = LoopCycleReadSource::from_rows(
        vec![LoopCycleBoundarySourceRow::new(entity_id(1), 1, 4)],
        1,
    )
    .unwrap();
    let read_receipt = LoopCycleReadStageExecutor::execute(&plan, read_source).unwrap();
    let input =
        LoopCycleExecutionInput::from_selected_plan_and_read_stage(&plan, read_receipt).unwrap();
    close_loop_cycle_migration_slice(&plan, input).unwrap()
}

pub(super) fn loop_cycle_bridge_rows() -> Vec<CoveredDerivedProductStatusRow> {
    status_rows_from_loop_cycle_migration_closeout(&loop_cycle_closeout())
}

pub(super) fn loop_cycle_and_wire_view_bridge_rows() -> Vec<CoveredDerivedProductStatusRow> {
    let loop_cycle_closeout = loop_cycle_closeout();
    let wire_view_closeout = wire_view_closeout();
    status_rows_from_migrated_family_closeouts(
        &[
            loop_cycle_closeout.migrated_family_closeout(),
            wire_view_closeout.migrated_family_closeout(),
        ],
        wire_view_closeout.old_authority_residue_digest(),
    )
}

pub(super) fn all_family_real_migration_sweep() -> CoveredDerivedProductMigrationSweepCloseout {
    let touched_closure = all_required_family_touched_closure();
    let plan = all_required_family_plan(&touched_closure);
    let closeouts = all_family_migration_closeouts(&plan, &touched_closure);
    let closeout_refs = closeouts.iter().collect::<Vec<_>>();
    super::super::close_covered_derived_product_migration_sweep(
        &plan,
        status_rows_from_migrated_family_closeouts(
            &closeout_refs,
            "all-family-real-migration-no-residue",
        ),
    )
    .unwrap()
}

pub(super) fn wire_view_closeout() -> WireViewMigrationCloseout {
    let plan = selected_loop_cycle_plan();
    let read_source = WireViewReadSource::from_rows_with_query_reports(
        &plan,
        &loop_cycles_touched_closure("phase-six-sweep"),
        vec![wire_view_source_row(7)],
        1,
        WireViewReadStageCounters::for_selected_rows(1, 1),
        vec!["query.native.read.receipt".to_string()],
    )
    .unwrap();
    let read_receipt = WireViewReadStageExecutor::execute(&plan, read_source).unwrap();
    let input =
        WireViewExecutionInput::from_selected_plan_and_read_stage(&plan, read_receipt).unwrap();
    close_wire_view_migration_slice(&plan, input).unwrap()
}

pub(super) fn rows_without_family(
    missing_family: DerivedTopologyProductFamilyIdentity,
) -> Vec<CoveredDerivedProductStatusRow> {
    loop_cycle_bridge_rows()
        .into_iter()
        .filter(|row| row.family_identity() != missing_family)
        .collect()
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}

fn all_required_family_plan(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> DerivedInvalidationSelectedPlan {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        touched_closure,
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap();
    assert_eq!(
        plan.selected_rows().len(),
        DerivedTopologyProductFamilyIdentity::REQUIRED.len()
    );
    plan
}

fn all_required_family_touched_closure() -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(1)),
            TopologyTouchedEntity::new(entity_id(2)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(3))],
        vec![
            TopologyRelationKind::HalfEdgeNext,
            TopologyRelationKind::HalfEdgeRadialNext,
        ],
        vec![
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedAspect::TopologyStructure,
            TopologyTouchedAspect::TopologyRadial,
        ],
        vec![
            TopologyTouchedScope::Loop,
            TopologyTouchedScope::Relation,
            TopologyTouchedScope::Entity,
        ],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis(
        "all-required-family-migration-sweep",
        basis,
    )
    .unwrap();
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}

fn all_family_migration_closeouts(
    plan: &DerivedInvalidationSelectedPlan,
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> Vec<MigratedDerivedProductFamilyCloseout> {
    vec![
        materialized_graph_closeout(plan),
        traversal_views_closeout(plan),
        loop_cycles_closeout(plan),
        radial_rings_closeout(plan),
        shell_views_closeout(plan),
        vertex_disks_closeout(plan),
        wire_views_closeout(plan, touched_closure),
    ]
}

fn materialized_graph_closeout(
    plan: &DerivedInvalidationSelectedPlan,
) -> MigratedDerivedProductFamilyCloseout {
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            4,
        );
    let read_source =
        MaterializedGraphReadSource::from_topology_view_with_selected_prefix(&topology, 2, 1)
            .unwrap();
    let read_receipt = MaterializedGraphReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        MaterializedGraphExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt)
            .unwrap();
    close_materialized_graph_migration_slice(plan, input)
        .unwrap()
        .migrated_family_closeout()
        .clone()
}

fn traversal_views_closeout(
    plan: &DerivedInvalidationSelectedPlan,
) -> MigratedDerivedProductFamilyCloseout {
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);
    let read_source =
        TraversalViewsReadSource::from_topology_view_with_selected_prefix(&topology, 1).unwrap();
    let read_receipt = TraversalViewsReadStageExecutor::execute(plan, read_source).unwrap();
    let input = TraversalViewsExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt)
        .unwrap();
    close_traversal_views_migration_slice(plan, input)
        .unwrap()
        .migrated_family_closeout()
        .clone()
}

fn loop_cycles_closeout(
    plan: &DerivedInvalidationSelectedPlan,
) -> MigratedDerivedProductFamilyCloseout {
    let read_source = LoopCycleReadSource::from_rows(
        vec![LoopCycleBoundarySourceRow::new(entity_id(10), 1, 4)],
        1,
    )
    .unwrap();
    let read_receipt = LoopCycleReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        LoopCycleExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_loop_cycle_migration_slice(plan, input)
        .unwrap()
        .migrated_family_closeout()
        .clone()
}

fn radial_rings_closeout(
    plan: &DerivedInvalidationSelectedPlan,
) -> MigratedDerivedProductFamilyCloseout {
    let read_source = RadialRingReadSource::from_rows(
        vec![RadialRingBoundarySourceRow::new(
            "he:radial-source",
            "edge:radial",
            "he:radial-target",
            "edge:radial",
            "rel:radial-next",
            2,
            true,
            false,
        )],
        1,
    )
    .unwrap();
    let read_receipt = RadialRingReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        RadialRingExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_radial_ring_migration_slice(plan, input)
        .unwrap()
        .migrated_family_closeout()
        .clone()
}

fn shell_views_closeout(
    plan: &DerivedInvalidationSelectedPlan,
) -> MigratedDerivedProductFamilyCloseout {
    let read_source = ShellViewReadSource::from_rows(
        vec![ShellViewBoundarySourceRow::new(
            "shell:main",
            "he:shell-source",
            "he:shell-source",
            "edge:shell",
            "he:shell-target",
            "edge:shell",
            "rel:shell-radial-next",
            2,
            true,
            false,
        )],
        1,
    )
    .unwrap();
    let read_receipt = ShellViewReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        ShellViewExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_shell_view_migration_slice(plan, input)
        .unwrap()
        .migrated_family_closeout()
        .clone()
}

fn vertex_disks_closeout(
    plan: &DerivedInvalidationSelectedPlan,
) -> MigratedDerivedProductFamilyCloseout {
    let read_source = VertexDiskReadSource::from_rows(
        vec![VertexDiskBoundarySourceRow::new(
            vec!["vertex:a".to_string()],
            "vertex:a",
            "he:vertex-source",
            "edge:vertex",
            vec![
                "he:vertex-source".to_string(),
                "he:vertex-neighbor".to_string(),
            ],
            vec!["he:vertex-neighbor".to_string()],
            vec!["edge:vertex".to_string()],
        )],
        1,
    )
    .unwrap();
    let read_receipt = VertexDiskReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        VertexDiskExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_vertex_disk_migration_slice(plan, input)
        .unwrap()
        .migrated_family_closeout()
        .clone()
}

fn wire_views_closeout(
    plan: &DerivedInvalidationSelectedPlan,
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> MigratedDerivedProductFamilyCloseout {
    let read_source = WireViewReadSource::from_rows_with_query_reports(
        plan,
        touched_closure,
        vec![WireViewSourceRow::from_interpretation(
            &WireInterpretationRecord {
                wire_id: entity_id(40),
                class: WireInterpretationClass::OpenChain,
                connected_component_count: 1,
                terminal_vertex_ids: vec![entity_id(41), entity_id(42)],
                branch_vertex_ids: Vec::new(),
            },
        )],
        1,
        WireViewReadStageCounters::for_selected_rows(1, 1),
        vec!["query.native.read.receipt".to_string()],
    )
    .unwrap();
    let read_receipt = WireViewReadStageExecutor::execute(plan, read_source).unwrap();
    let input =
        WireViewExecutionInput::from_selected_plan_and_read_stage(plan, read_receipt).unwrap();
    close_wire_view_migration_slice(plan, input)
        .unwrap()
        .migrated_family_closeout()
        .clone()
}

fn wire_view_source_row(slot: u64) -> WireViewSourceRow {
    WireViewSourceRow::from_interpretation(
        &schema::facade::platform::authority::WireInterpretationRecord {
            wire_id: entity_id(slot),
            class: schema::facade::platform::authority::WireInterpretationClass::OpenChain,
            connected_component_count: 1,
            terminal_vertex_ids: vec![entity_id(slot + 1), entity_id(slot + 2)],
            branch_vertex_ids: Vec::new(),
        },
    )
}
