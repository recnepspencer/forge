use topology::derived_invalidation_migrated_products::{
    close_covered_derived_product_migration_sweep, close_loop_cycle_migration_slice,
    close_materialized_graph_migration_slice, close_traversal_views_migration_slice,
    close_wire_view_migration_slice, status_rows_from_loop_cycle_migration_closeout,
    status_rows_from_migrated_family_closeouts, CoveredDerivedProductMigrationCounters,
    CoveredDerivedProductMigrationError,
    CoveredDerivedProductMigrationStatus, CoveredDerivedProductMigrationSweepCloseout,
    CoveredDerivedProductPhaseSevenSeed, CoveredDerivedProductStatusRow,
    LoopCycleBoundarySourceRow, LoopCycleDerivedProductOutput, LoopCycleExecutionInput,
    LoopCycleMigrationCloseout, LoopCycleMigrationCounters, LoopCycleMigrationError,
    LoopCycleOldAuthorityResidue, LoopCycleOldAuthorityResidueRow, LoopCyclePhaseSixSeed,
    LoopCycleProductRow, LoopCycleReadSource, LoopCycleReadStageExecutor,
    LoopCycleReadStageReceipt, MaterializedGraphDerivedProductOutput,
    MaterializedGraphDiagnosticProjection, MaterializedGraphExecutionInput,
    MaterializedGraphMigrationCloseout, MaterializedGraphMigrationCounters,
    MaterializedGraphMigrationError, MaterializedGraphOldAuthorityResidue,
    MaterializedGraphOldAuthorityResidueRow, MaterializedGraphPhaseTenSeed,
    MaterializedGraphProductEntityRow, MaterializedGraphProductRelationRow,
    MaterializedGraphReadEntityRow, MaterializedGraphReadRelationRow, MaterializedGraphReadSource,
    MaterializedGraphReadStageExecutor, MaterializedGraphReadStageReceipt,
    MigratedDerivedProductFamilyCloseout, TraversalViewsDerivedProductOutput,
    TraversalViewsDiagnosticProjection, TraversalViewsExecutionInput,
    TraversalViewsMigrationCloseout, TraversalViewsMigrationCounters, TraversalViewsMigrationError,
    TraversalViewsOldAuthorityResidue, TraversalViewsOldAuthorityResidueRow,
    TraversalViewsPhaseElevenSeed, TraversalViewsProductRow, TraversalViewsReadSource,
    TraversalViewsReadStageExecutor, TraversalViewsReadStageReceipt, TraversalViewsSourceRow,
    VertexDiskBoundarySourceRow, VertexDiskDerivedProductOutput, VertexDiskExecutionInput,
    VertexDiskFamilyCloseoutSeed, VertexDiskMigrationCloseout, VertexDiskMigrationCounters,
    VertexDiskMigrationError, VertexDiskOldAuthorityResidue, VertexDiskOldAuthorityResidueRow,
    VertexDiskProductRow, VertexDiskReadSource, VertexDiskReadStageExecutor,
    VertexDiskReadStageReceipt, WireViewDerivedProductOutput, WireViewExecutionInput,
    WireViewFamilyCloseoutSeed, WireViewMigrationCloseout, WireViewMigrationCounters,
    WireViewMigrationError, WireViewOldAuthorityResidue, WireViewOldAuthorityResidueRow,
    WireViewProductRow, WireViewReadSource, WireViewReadStageCounters,
    WireViewReadStageExecutor, WireViewReadStageReceipt, WireViewSourceRow,
    close_vertex_disk_migration_slice,
};
use topology::derived_invalidation_selected_plan::{
    DerivedInvalidationSelectedPlan as MigratedProductsSelectedPlan,
    DerivedInvalidationTouchedClosure as MigratedProductsTouchedClosure,
};

fn _derived_invalidation_migrated_products_contract() {
    let _: fn(
        &MigratedProductsSelectedPlan,
        LoopCycleExecutionInput,
    ) -> Result<LoopCycleMigrationCloseout, LoopCycleMigrationError> =
        close_loop_cycle_migration_slice;

    let _: fn(
        forge_relational::facade::identity::EntityId,
        usize,
        usize,
    ) -> LoopCycleBoundarySourceRow = LoopCycleBoundarySourceRow::new;
    let _: fn(
        &MigratedProductsSelectedPlan,
        LoopCycleReadStageReceipt,
    ) -> Result<LoopCycleExecutionInput, LoopCycleMigrationError> =
        LoopCycleExecutionInput::from_selected_plan_and_read_stage;
    let _: fn(&LoopCycleReadSource) -> &[LoopCycleBoundarySourceRow] =
        LoopCycleReadSource::selected_rows;
    let _: fn(&LoopCycleReadSource) -> usize = LoopCycleReadSource::available_source_row_count;
    let _: fn(&LoopCycleReadSource) -> &str = LoopCycleReadSource::read_source_digest;
    let _: fn(
        &MigratedProductsSelectedPlan,
        LoopCycleReadSource,
    ) -> Result<LoopCycleReadStageReceipt, LoopCycleMigrationError> =
        LoopCycleReadStageExecutor::execute;
    let _: fn(&LoopCycleReadStageReceipt) -> &str =
        LoopCycleReadStageReceipt::selected_plan_digest;
    let _: fn(&LoopCycleReadStageReceipt) -> &str =
        LoopCycleReadStageReceipt::touched_closure_digest;
    let _: fn(&LoopCycleReadStageReceipt) -> &str =
        LoopCycleReadStageReceipt::native_query_read_receipt_digest;
    let _: fn(&LoopCycleReadStageReceipt) -> &str =
        LoopCycleReadStageReceipt::selected_legality_receipt_digest;
    let _: fn(&LoopCycleReadStageReceipt) -> usize =
        LoopCycleReadStageReceipt::touched_closure_loop_cycle_bound;
    let _: fn(&LoopCycleReadStageReceipt) -> usize =
        LoopCycleReadStageReceipt::selected_source_row_count;
    let _: fn(&LoopCycleReadStageReceipt) -> usize =
        LoopCycleReadStageReceipt::available_source_row_count;
    let _: fn(&LoopCycleReadStageReceipt) -> &[LoopCycleBoundarySourceRow] =
        LoopCycleReadStageReceipt::selected_rows;
    let _: fn(&LoopCycleExecutionInput) -> &[LoopCycleBoundarySourceRow] =
        LoopCycleExecutionInput::selected_rows;
    let _: fn(&LoopCycleExecutionInput) -> usize =
        LoopCycleExecutionInput::available_source_row_count;
    let _: fn(&LoopCycleExecutionInput) -> &str = LoopCycleExecutionInput::selected_plan_digest;
    let _: fn(&LoopCycleExecutionInput) -> &str = LoopCycleExecutionInput::input_digest;

    let _: fn(&LoopCycleDerivedProductOutput) -> &[LoopCycleProductRow] =
        LoopCycleDerivedProductOutput::rows;
    let _: fn(&LoopCycleDerivedProductOutput) -> &str =
        LoopCycleDerivedProductOutput::selected_plan_digest;
    let _: fn(&LoopCycleDerivedProductOutput) -> &str =
        LoopCycleDerivedProductOutput::output_digest;
    let _: fn(&LoopCycleProductRow) -> forge_relational::facade::identity::EntityId =
        LoopCycleProductRow::shell_id;
    let _: fn(&LoopCycleProductRow) -> bool = LoopCycleProductRow::closed_boundary;

    let _: fn() -> LoopCycleOldAuthorityResidue = LoopCycleOldAuthorityResidue::current_source_scan;
    let _: fn(&LoopCycleOldAuthorityResidue) -> &[LoopCycleOldAuthorityResidueRow] =
        LoopCycleOldAuthorityResidue::capped_rows;
    let _: fn(&LoopCycleOldAuthorityResidue) -> usize =
        LoopCycleOldAuthorityResidue::capped_direct_interpreter_count;
    let _: fn(&LoopCycleOldAuthorityResidue) -> &str =
        LoopCycleOldAuthorityResidue::residue_digest;
    let _: fn(&LoopCycleOldAuthorityResidueRow) -> &str = LoopCycleOldAuthorityResidueRow::owner;
    let _: fn(&LoopCycleOldAuthorityResidueRow) -> &str = LoopCycleOldAuthorityResidueRow::blocker;
    let _: fn(&LoopCycleOldAuthorityResidueRow) -> &str =
        LoopCycleOldAuthorityResidueRow::removal_trigger;

    let _: fn(&LoopCycleMigrationCloseout) -> &LoopCycleMigrationCounters =
        LoopCycleMigrationCloseout::counters;
    let _: fn(&LoopCycleMigrationCloseout) -> &LoopCyclePhaseSixSeed =
        LoopCycleMigrationCloseout::phase_six_seed;
    let _: fn(&LoopCycleMigrationCloseout) -> &str =
        LoopCycleMigrationCloseout::closeout_digest;
    let _: fn(&LoopCycleMigrationCounters) -> usize =
        LoopCycleMigrationCounters::execution_work_count;
    let _: fn(&LoopCycleMigrationCounters) -> usize =
        LoopCycleMigrationCounters::whole_view_fallback_count;
    let _: fn(&LoopCycleMigrationCounters) -> usize =
        LoopCycleMigrationCounters::non_loop_placeholder_execution_count;
    let _: fn(&LoopCycleMigrationCounters) -> &str =
        LoopCycleMigrationCounters::counters_digest;
    let _: fn(&LoopCyclePhaseSixSeed) -> &'static str = LoopCyclePhaseSixSeed::migrated_family;
    let _: fn(&LoopCyclePhaseSixSeed) -> &str = LoopCyclePhaseSixSeed::seed_digest;

    let _: fn(
        &MigratedProductsSelectedPlan,
        WireViewExecutionInput,
    ) -> Result<WireViewMigrationCloseout, WireViewMigrationError> =
        close_wire_view_migration_slice;
    let _: fn(
        &schema::facade::platform::authority::WireInterpretationRecord,
    ) -> WireViewSourceRow = WireViewSourceRow::from_interpretation;
    let _: fn(
        &MigratedProductsSelectedPlan,
        WireViewReadStageReceipt,
    ) -> Result<WireViewExecutionInput, WireViewMigrationError> =
        WireViewExecutionInput::from_selected_plan_and_read_stage;
    let _: fn(&WireViewReadSource) -> &[WireViewSourceRow] = WireViewReadSource::selected_rows;
    let _: fn(&WireViewReadSource) -> usize = WireViewReadSource::available_source_row_count;
    let _: fn(&WireViewReadSource) -> &WireViewReadStageCounters = WireViewReadSource::counters;
    let _: fn(&WireViewReadSource) -> &[String] = WireViewReadSource::query_report_digests;
    let _: fn(&WireViewReadSource) -> &str = WireViewReadSource::read_source_digest;
    let _: fn(
        &MigratedProductsSelectedPlan,
        WireViewReadSource,
    ) -> Result<WireViewReadStageReceipt, WireViewMigrationError> =
        WireViewReadStageExecutor::execute;
    let _: fn(&WireViewReadStageReceipt) -> &str =
        WireViewReadStageReceipt::selected_plan_digest;
    let _: fn(&WireViewReadStageReceipt) -> &str =
        WireViewReadStageReceipt::touched_closure_digest;
    let _: fn(&WireViewReadStageReceipt) -> &str =
        WireViewReadStageReceipt::native_query_read_receipt_digest;
    let _: fn(&WireViewReadStageReceipt) -> &str =
        WireViewReadStageReceipt::selected_legality_receipt_digest;
    let _: fn(&WireViewReadStageReceipt) -> &[WireViewSourceRow] =
        WireViewReadStageReceipt::selected_rows;
    let _: fn(&WireViewReadStageReceipt) -> &str = WireViewReadStageReceipt::receipt_digest;
    let _: fn(&WireViewExecutionInput) -> &[WireViewSourceRow] =
        WireViewExecutionInput::selected_rows;
    let _: fn(&WireViewExecutionInput) -> usize =
        WireViewExecutionInput::available_source_row_count;
    let _: fn(&WireViewExecutionInput) -> &str = WireViewExecutionInput::selected_plan_digest;
    let _: fn(&WireViewExecutionInput) -> &str =
        WireViewExecutionInput::read_stage_receipt_digest;
    let _: fn(&WireViewExecutionInput) -> &str = WireViewExecutionInput::input_digest;

    let _: fn(&WireViewDerivedProductOutput) -> &[WireViewProductRow] =
        WireViewDerivedProductOutput::rows;
    let _: fn(&WireViewDerivedProductOutput) -> &str =
        WireViewDerivedProductOutput::selected_plan_digest;
    let _: fn(&WireViewDerivedProductOutput) -> &str = WireViewDerivedProductOutput::output_digest;
    let _: fn(&WireViewProductRow) -> forge_relational::facade::identity::EntityId =
        WireViewProductRow::wire_id;
    let _: fn(&WireViewProductRow) -> schema::facade::platform::authority::WireInterpretationClass =
        WireViewProductRow::class;
    let _: fn(&WireViewProductRow) -> usize = WireViewProductRow::half_edge_count;

    let _: fn() -> WireViewOldAuthorityResidue = WireViewOldAuthorityResidue::current_source_scan;
    let _: fn(&WireViewOldAuthorityResidue) -> &[WireViewOldAuthorityResidueRow] =
        WireViewOldAuthorityResidue::capped_rows;
    let _: fn(&WireViewOldAuthorityResidue) -> usize =
        WireViewOldAuthorityResidue::capped_direct_interpreter_count;
    let _: fn(&WireViewOldAuthorityResidue) -> &str = WireViewOldAuthorityResidue::residue_digest;
    let _: fn(&WireViewOldAuthorityResidueRow) -> &str = WireViewOldAuthorityResidueRow::owner;
    let _: fn(&WireViewOldAuthorityResidueRow) -> &str = WireViewOldAuthorityResidueRow::blocker;
    let _: fn(&WireViewOldAuthorityResidueRow) -> &str =
        WireViewOldAuthorityResidueRow::removal_trigger;

    let _: fn(&WireViewMigrationCloseout) -> &WireViewMigrationCounters =
        WireViewMigrationCloseout::counters;
    let _: fn(&WireViewMigrationCloseout) -> &WireViewFamilyCloseoutSeed =
        WireViewMigrationCloseout::family_closeout_seed;
    let _: fn(&WireViewMigrationCloseout) -> &str = WireViewMigrationCloseout::closeout_digest;
    let _: fn(&WireViewMigrationCounters) -> usize = WireViewMigrationCounters::execution_work_count;
    let _: fn(&WireViewMigrationCounters) -> usize =
        WireViewMigrationCounters::whole_view_fallback_count;
    let _: fn(&WireViewMigrationCounters) -> usize =
        WireViewMigrationCounters::read_stage_touched_wire_count;
    let _: fn(&WireViewMigrationCounters) -> usize =
        WireViewMigrationCounters::read_stage_touched_half_edge_lookup_count;
    let _: fn(&WireViewMigrationCounters) -> usize =
        WireViewMigrationCounters::read_stage_unrelated_wire_breadth_count;
    let _: fn(&WireViewMigrationCounters) -> usize =
        WireViewMigrationCounters::non_wire_placeholder_execution_count;
    let _: fn(&WireViewMigrationCounters) -> &str = WireViewMigrationCounters::counters_digest;
    let _: fn(&WireViewFamilyCloseoutSeed) -> &'static str =
        WireViewFamilyCloseoutSeed::migrated_family;
    let _: fn(&WireViewFamilyCloseoutSeed) -> &str = WireViewFamilyCloseoutSeed::seed_digest;

    let _: fn(
        &MigratedProductsSelectedPlan,
        MaterializedGraphExecutionInput,
    ) -> Result<MaterializedGraphMigrationCloseout, MaterializedGraphMigrationError> =
        close_materialized_graph_migration_slice;
    let _: fn(&MaterializedGraphReadSource) -> &[MaterializedGraphReadEntityRow] =
        MaterializedGraphReadSource::selected_entities;
    let _: fn(&MaterializedGraphReadSource) -> &[MaterializedGraphReadRelationRow] =
        MaterializedGraphReadSource::selected_relations;
    let _: fn(&MaterializedGraphReadSource) -> usize =
        MaterializedGraphReadSource::available_entity_count;
    let _: fn(&MaterializedGraphReadSource) -> usize =
        MaterializedGraphReadSource::available_relation_count;
    let _: fn(&MaterializedGraphReadEntityRow) -> forge_relational::facade::identity::EntityId =
        MaterializedGraphReadEntityRow::entity_id;
    let _: fn(&MaterializedGraphReadEntityRow) -> &'static str =
        MaterializedGraphReadEntityRow::topology_kind;
    let _: fn(&MaterializedGraphReadRelationRow) -> &'static str =
        MaterializedGraphReadRelationRow::relation_kind;
    let _: fn(&MaterializedGraphReadRelationRow) -> forge_relational::facade::identity::EntityId =
        MaterializedGraphReadRelationRow::source_entity_id;
    let _: fn(&MaterializedGraphReadRelationRow) -> forge_relational::facade::identity::EntityId =
        MaterializedGraphReadRelationRow::target_entity_id;
    let _: fn(
        &MigratedProductsSelectedPlan,
        MaterializedGraphReadSource,
    ) -> Result<MaterializedGraphReadStageReceipt, MaterializedGraphMigrationError> =
        MaterializedGraphReadStageExecutor::execute;
    let _: fn(
        &MigratedProductsSelectedPlan,
        MaterializedGraphReadStageReceipt,
    ) -> Result<MaterializedGraphExecutionInput, MaterializedGraphMigrationError> =
        MaterializedGraphExecutionInput::from_selected_plan_and_read_stage;
    let _: fn(&MaterializedGraphReadStageReceipt) -> &str =
        MaterializedGraphReadStageReceipt::receipt_digest;
    let _: fn(&MaterializedGraphDerivedProductOutput) -> &[MaterializedGraphProductEntityRow] =
        MaterializedGraphDerivedProductOutput::entity_rows;
    let _: fn(&MaterializedGraphDerivedProductOutput) -> &[MaterializedGraphProductRelationRow] =
        MaterializedGraphDerivedProductOutput::relation_rows;
    let _: fn(&MaterializedGraphProductEntityRow) -> forge_relational::facade::identity::EntityId =
        MaterializedGraphProductEntityRow::source_entity_id;
    let _: fn(&MaterializedGraphProductEntityRow) -> &'static str =
        MaterializedGraphProductEntityRow::topology_kind;
    let _: fn(&MaterializedGraphProductRelationRow) -> &'static str =
        MaterializedGraphProductRelationRow::relation_kind;
    let _: fn(&MaterializedGraphProductRelationRow) -> forge_relational::facade::identity::EntityId =
        MaterializedGraphProductRelationRow::source_entity_id;
    let _: fn(&MaterializedGraphProductRelationRow) -> forge_relational::facade::identity::EntityId =
        MaterializedGraphProductRelationRow::target_entity_id;
    let _: fn(&MaterializedGraphDiagnosticProjection) -> &str =
        MaterializedGraphDiagnosticProjection::projection_digest;
    let _: fn() -> MaterializedGraphOldAuthorityResidue =
        MaterializedGraphOldAuthorityResidue::current_source_scan;
    let _: fn(&MaterializedGraphOldAuthorityResidue) -> &[MaterializedGraphOldAuthorityResidueRow] =
        MaterializedGraphOldAuthorityResidue::capped_rows;
    let _: fn(&MaterializedGraphMigrationCloseout) -> &MaterializedGraphMigrationCounters =
        MaterializedGraphMigrationCloseout::counters;
    let _: fn(&MaterializedGraphMigrationCloseout) -> &MaterializedGraphPhaseTenSeed =
        MaterializedGraphMigrationCloseout::phase_ten_seed;

    let _: fn(
        &MigratedProductsSelectedPlan,
        TraversalViewsExecutionInput,
    ) -> Result<TraversalViewsMigrationCloseout, TraversalViewsMigrationError> =
        close_traversal_views_migration_slice;
    let _: fn(&TraversalViewsReadSource) -> &[TraversalViewsSourceRow] =
        TraversalViewsReadSource::selected_rows;
    let _: fn(&TraversalViewsReadSource) -> usize =
        TraversalViewsReadSource::available_traversal_count;
    let _: fn(&TraversalViewsSourceRow) -> &'static str = TraversalViewsSourceRow::traversal_kind;
    let _: fn(&TraversalViewsSourceRow) -> forge_relational::facade::identity::EntityId =
        TraversalViewsSourceRow::anchor_entity_id;
    let _: fn(&TraversalViewsSourceRow) -> usize = TraversalViewsSourceRow::reached_entity_count;
    let _: fn(&TraversalViewsSourceRow) -> &str = TraversalViewsSourceRow::row_digest;
    let _: fn(
        &MigratedProductsSelectedPlan,
        TraversalViewsReadSource,
    ) -> Result<TraversalViewsReadStageReceipt, TraversalViewsMigrationError> =
        TraversalViewsReadStageExecutor::execute;
    let _: fn(
        &MigratedProductsSelectedPlan,
        TraversalViewsReadStageReceipt,
    ) -> Result<TraversalViewsExecutionInput, TraversalViewsMigrationError> =
        TraversalViewsExecutionInput::from_selected_plan_and_read_stage;
    let _: fn(&TraversalViewsReadStageReceipt) -> &str =
        TraversalViewsReadStageReceipt::receipt_digest;
    let _: fn(&TraversalViewsReadStageReceipt) -> usize =
        TraversalViewsReadStageReceipt::touched_closure_traversal_bound;
    let _: fn(&TraversalViewsReadStageReceipt) -> usize =
        TraversalViewsReadStageReceipt::selected_traversal_count;
    let _: fn(&TraversalViewsReadStageReceipt) -> usize =
        TraversalViewsReadStageReceipt::available_traversal_count;
    let _: fn(&TraversalViewsReadStageReceipt) -> &[TraversalViewsSourceRow] =
        TraversalViewsReadStageReceipt::selected_rows;
    let _: fn(&TraversalViewsExecutionInput) -> &TraversalViewsReadStageReceipt =
        TraversalViewsExecutionInput::read_stage_receipt;
    let _: fn(&TraversalViewsExecutionInput) -> usize =
        TraversalViewsExecutionInput::selected_traversal_count;
    let _: fn(&TraversalViewsExecutionInput) -> &str = TraversalViewsExecutionInput::input_digest;
    let _: fn(&TraversalViewsDerivedProductOutput) -> &[TraversalViewsProductRow] =
        TraversalViewsDerivedProductOutput::rows;
    let _: fn(&TraversalViewsDerivedProductOutput) -> usize =
        TraversalViewsDerivedProductOutput::touched_closure_traversal_bound;
    let _: fn(&TraversalViewsDerivedProductOutput) -> usize =
        TraversalViewsDerivedProductOutput::selected_traversal_count;
    let _: fn(&TraversalViewsDerivedProductOutput) -> usize =
        TraversalViewsDerivedProductOutput::available_traversal_count;
    let _: fn(&TraversalViewsDerivedProductOutput) -> &str =
        TraversalViewsDerivedProductOutput::output_digest;
    let _: fn(&TraversalViewsProductRow) -> &'static str = TraversalViewsProductRow::traversal_kind;
    let _: fn(&TraversalViewsProductRow) -> forge_relational::facade::identity::EntityId =
        TraversalViewsProductRow::anchor_entity_id;
    let _: fn(&TraversalViewsProductRow) -> usize = TraversalViewsProductRow::reached_entity_count;
    let _: fn(&TraversalViewsProductRow) -> &str = TraversalViewsProductRow::row_digest;
    let _: fn(&TraversalViewsDiagnosticProjection) -> usize =
        TraversalViewsDiagnosticProjection::touched_closure_traversal_bound;
    let _: fn(&TraversalViewsDiagnosticProjection) -> usize =
        TraversalViewsDiagnosticProjection::selected_traversal_count;
    let _: fn(&TraversalViewsDiagnosticProjection) -> usize =
        TraversalViewsDiagnosticProjection::available_traversal_count;
    let _: fn(&TraversalViewsDiagnosticProjection) -> &str =
        TraversalViewsDiagnosticProjection::projection_digest;
    let _: fn() -> TraversalViewsOldAuthorityResidue =
        TraversalViewsOldAuthorityResidue::current_source_scan;
    let _: fn(&TraversalViewsOldAuthorityResidue) -> &[TraversalViewsOldAuthorityResidueRow] =
        TraversalViewsOldAuthorityResidue::capped_rows;
    let _: fn(&TraversalViewsOldAuthorityResidue) -> usize =
        TraversalViewsOldAuthorityResidue::capped_traversal_authority_count;
    let _: fn(&TraversalViewsOldAuthorityResidue) -> &str =
        TraversalViewsOldAuthorityResidue::residue_digest;
    let _: fn(&TraversalViewsOldAuthorityResidueRow) -> &str =
        TraversalViewsOldAuthorityResidueRow::owner;
    let _: fn(&TraversalViewsOldAuthorityResidueRow) -> &str =
        TraversalViewsOldAuthorityResidueRow::blocker;
    let _: fn(&TraversalViewsMigrationCloseout) -> &TraversalViewsMigrationCounters =
        TraversalViewsMigrationCloseout::counters;
    let _: fn(&TraversalViewsMigrationCloseout) -> &TraversalViewsPhaseElevenSeed =
        TraversalViewsMigrationCloseout::phase_eleven_seed;
    let _: fn(&TraversalViewsMigrationCloseout) -> &str =
        TraversalViewsMigrationCloseout::closeout_digest;
    let _: fn(&TraversalViewsMigrationCounters) -> usize =
        TraversalViewsMigrationCounters::touched_closure_traversal_bound;
    let _: fn(&TraversalViewsMigrationCounters) -> usize =
        TraversalViewsMigrationCounters::selected_traversal_count;
    let _: fn(&TraversalViewsMigrationCounters) -> usize =
        TraversalViewsMigrationCounters::available_traversal_count;
    let _: fn(&TraversalViewsMigrationCounters) -> usize =
        TraversalViewsMigrationCounters::execution_work_count;
    let _: fn(&TraversalViewsMigrationCounters) -> usize =
        TraversalViewsMigrationCounters::whole_view_fallback_count;
    let _: fn(&TraversalViewsPhaseElevenSeed) -> &'static str =
        TraversalViewsPhaseElevenSeed::migrated_family;
    let _: fn(&TraversalViewsPhaseElevenSeed) -> &str = TraversalViewsPhaseElevenSeed::seed_digest;

    let _: fn(
        &MigratedProductsSelectedPlan,
        VertexDiskExecutionInput,
    ) -> Result<VertexDiskMigrationCloseout, VertexDiskMigrationError> =
        close_vertex_disk_migration_slice;
    let _: fn(
        &MigratedProductsSelectedPlan,
        &MigratedProductsTouchedClosure,
        &[topology::query_domain::TopologyHalfEdgeSharedVertexNeighborhoodView],
    ) -> Result<VertexDiskReadSource, VertexDiskMigrationError> =
        VertexDiskReadSource::from_query_shared_vertex_neighborhood_views;
    let _: fn(&VertexDiskReadSource) -> &[VertexDiskBoundarySourceRow] =
        VertexDiskReadSource::selected_rows;
    let _: fn(&VertexDiskReadSource) -> usize = VertexDiskReadSource::available_source_row_count;
    let _: fn(
        &MigratedProductsSelectedPlan,
        VertexDiskReadSource,
    ) -> Result<VertexDiskReadStageReceipt, VertexDiskMigrationError> =
        VertexDiskReadStageExecutor::execute;
    let _: fn(
        &MigratedProductsSelectedPlan,
        VertexDiskReadStageReceipt,
    ) -> Result<VertexDiskExecutionInput, VertexDiskMigrationError> =
        VertexDiskExecutionInput::from_selected_plan_and_read_stage;
    let _: fn(&VertexDiskReadStageReceipt) -> &str =
        VertexDiskReadStageReceipt::native_query_read_receipt_digest;
    let _: fn(&VertexDiskReadStageReceipt) -> usize =
        VertexDiskReadStageReceipt::touched_closure_vertex_disk_bound;
    let _: fn(&VertexDiskReadStageReceipt) -> &[VertexDiskBoundarySourceRow] =
        VertexDiskReadStageReceipt::selected_rows;
    let _: fn(&VertexDiskExecutionInput) -> &[VertexDiskBoundarySourceRow] =
        VertexDiskExecutionInput::selected_rows;
    let _: fn(&VertexDiskDerivedProductOutput) -> &[VertexDiskProductRow] =
        VertexDiskDerivedProductOutput::rows;
    let _: fn(&VertexDiskProductRow) -> &[String] =
        VertexDiskProductRow::touched_vertex_identities;
    let _: fn(&VertexDiskProductRow) -> usize = VertexDiskProductRow::touched_incident_edge_count;
    let _: fn(&VertexDiskProductRow) -> bool = VertexDiskProductRow::branch_vertex_disk;
    let _: fn() -> VertexDiskOldAuthorityResidue =
        VertexDiskOldAuthorityResidue::current_source_scan;
    let _: fn(&VertexDiskOldAuthorityResidue) -> &[VertexDiskOldAuthorityResidueRow] =
        VertexDiskOldAuthorityResidue::capped_rows;
    let _: fn(&VertexDiskMigrationCloseout) -> &VertexDiskMigrationCounters =
        VertexDiskMigrationCloseout::counters;
    let _: fn(&VertexDiskMigrationCloseout) -> &VertexDiskFamilyCloseoutSeed =
        VertexDiskMigrationCloseout::family_closeout_seed;
    let _: fn(&VertexDiskMigrationCounters) -> usize =
        VertexDiskMigrationCounters::read_stage_touched_vertex_count;
    let _: fn(&VertexDiskMigrationCounters) -> usize =
        VertexDiskMigrationCounters::read_stage_touched_incident_edge_count;
    let _: fn(&VertexDiskFamilyCloseoutSeed) -> &'static str =
        VertexDiskFamilyCloseoutSeed::migrated_family;
    let _: fn(&VertexDiskFamilyCloseoutSeed) -> &str = VertexDiskFamilyCloseoutSeed::seed_digest;

    let _: fn(
        &MigratedProductsSelectedPlan,
        Vec<CoveredDerivedProductStatusRow>,
    ) -> Result<CoveredDerivedProductMigrationSweepCloseout, CoveredDerivedProductMigrationError> =
        close_covered_derived_product_migration_sweep;
    let _: fn(&LoopCycleMigrationCloseout) -> Vec<CoveredDerivedProductStatusRow> =
        status_rows_from_loop_cycle_migration_closeout;
    let _: fn(
        &[&MigratedDerivedProductFamilyCloseout],
        &str,
    ) -> Vec<CoveredDerivedProductStatusRow> = status_rows_from_migrated_family_closeouts;
    let _: fn(&CoveredDerivedProductStatusRow) -> CoveredDerivedProductMigrationStatus =
        CoveredDerivedProductStatusRow::status;
    let _: fn(&CoveredDerivedProductStatusRow) -> bool =
        CoveredDerivedProductStatusRow::ordinary_invalidation_consumable;
    let _: fn(&CoveredDerivedProductMigrationSweepCloseout) -> &[CoveredDerivedProductStatusRow] =
        CoveredDerivedProductMigrationSweepCloseout::status_rows;
    let _: fn(
        &CoveredDerivedProductMigrationSweepCloseout,
    ) -> &CoveredDerivedProductMigrationCounters =
        CoveredDerivedProductMigrationSweepCloseout::counters;
    let _: fn(&CoveredDerivedProductMigrationSweepCloseout) -> &CoveredDerivedProductPhaseSevenSeed =
        CoveredDerivedProductMigrationSweepCloseout::phase_seven_seed;
    let _: fn(&CoveredDerivedProductMigrationSweepCloseout) -> &str =
        CoveredDerivedProductMigrationSweepCloseout::closeout_digest;
    let _: fn(&CoveredDerivedProductMigrationCounters) -> usize =
        CoveredDerivedProductMigrationCounters::required_family_count;
    let _: fn(&CoveredDerivedProductMigrationCounters) -> usize =
        CoveredDerivedProductMigrationCounters::migrated_family_count;
    let _: fn(&CoveredDerivedProductMigrationCounters) -> usize =
        CoveredDerivedProductMigrationCounters::certification_residue_only_count;
    let _: fn(&CoveredDerivedProductPhaseSevenSeed) -> &str =
        CoveredDerivedProductPhaseSevenSeed::seed_digest;
}
