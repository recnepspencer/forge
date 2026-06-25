use super::super::{DerivedInvalidationProductExecutionReport, DerivedInvalidationProductExecutor};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    loop_cycles_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
    DerivedInvalidationSelectedRow,
};
use crate::derived_topology::materialized_graph::{
    MaterializationBreadthReport, MaterializationFallbackClass, MaterializationReport,
};

pub(super) fn selected_loop_cycles_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("loop-touch"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn whole_view_materialization_report() -> MaterializationReport {
    MaterializationReport {
        breadth: MaterializationBreadthReport {
            entity_count: 10,
            relation_count: 10,
            topology_entity_count: 10,
            topology_relation_count: 10,
        },
        whole_view_materialization: true,
        fallback_class: Some(MaterializationFallbackClass::WholeViewRebuild),
    }
}

pub(super) fn bounded_materialization_report() -> MaterializationReport {
    MaterializationReport {
        breadth: MaterializationBreadthReport {
            entity_count: 1,
            relation_count: 1,
            topology_entity_count: 1,
            topology_relation_count: 1,
        },
        whole_view_materialization: false,
        fallback_class: None,
    }
}

pub(super) struct MeasuredExecutionExecutor {
    execution_work_count: usize,
    caller_owned_graph_work_count: usize,
    materialization_report: Option<MaterializationReport>,
}

impl MeasuredExecutionExecutor {
    pub(super) fn new(
        execution_work_count: usize,
        caller_owned_graph_work_count: usize,
        materialization_report: Option<MaterializationReport>,
    ) -> Self {
        Self {
            execution_work_count,
            caller_owned_graph_work_count,
            materialization_report,
        }
    }
}

impl DerivedInvalidationProductExecutor for MeasuredExecutionExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<
        DerivedInvalidationProductExecutionReport,
        super::super::DerivedInvalidationExecutionError,
    > {
        DerivedInvalidationProductExecutionReport::from_selected_row(
            row,
            self.execution_work_count,
            self.caller_owned_graph_work_count,
            self.materialization_report.as_ref(),
        )
    }
}
