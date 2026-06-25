use schema::facade::platform::authority::WireInterpretationClass;

use super::support::{
    branching_wire_view_query_read_rows, close_wire_view_slice_from_query_read_source,
    closed_wire_view_query_read_rows, selected_wire_view_read_source,
    selected_wire_view_read_source_fixture, selected_wire_view_touched_closure,
};

#[test]
fn migrated_wire_views_close_from_receipt_backed_touched_read_stage() {
    let closeout = close_wire_view_slice_from_query_read_source("wire-view-touch");

    assert_eq!(
        closeout.family_closeout_seed().migrated_family(),
        "wire_views"
    );
    assert!(!closeout.family_closeout_seed().seed_digest().is_empty());
    assert!(!closeout.execution_receipt_digest().is_empty());
    assert!(!closeout.wire_view_output_digest().is_empty());
    assert!(!closeout.closeout_digest().is_empty());
    assert_eq!(closeout.counters().output_row_count(), 1);
    assert_eq!(closeout.counters().selected_source_row_count(), 1);
    assert_eq!(closeout.counters().available_source_row_count(), 1);
    assert_eq!(closeout.counters().execution_work_count(), 1);
    assert_eq!(closeout.counters().whole_view_fallback_count(), 0);
    assert_eq!(closeout.counters().read_stage_touched_wire_count(), 1);
    assert_eq!(
        closeout
            .counters()
            .read_stage_touched_half_edge_lookup_count(),
        4
    );
    assert_eq!(
        closeout
            .counters()
            .read_stage_unrelated_wire_breadth_count(),
        0
    );
    assert_eq!(
        closeout.counters().old_authority_residue_count(),
        super::super::WireViewOldAuthorityResidue::required_capped_callers().len()
    );
    assert!(closeout.counters().non_wire_placeholder_execution_count() > 0);
}

#[test]
fn touched_wire_read_stage_preserves_open_wire_interpretation_without_unrelated_rows() {
    let read_source = selected_wire_view_read_source("wire-view-locality");

    assert_eq!(read_source.selected_rows().len(), 1);
    assert_eq!(read_source.available_source_row_count(), 1);
    assert_eq!(read_source.counters().touched_wire_count(), 1);
    assert_eq!(read_source.counters().touched_half_edge_lookup_count(), 4);
    assert_eq!(read_source.counters().unrelated_wire_breadth_count(), 0);
    assert_eq!(read_source.counters().whole_view_fallback_count(), 0);

    let row = &read_source.selected_rows()[0];
    assert_eq!(row.class(), WireInterpretationClass::OpenChain);
    assert_eq!(row.connected_component_count(), 1);
    assert_eq!(row.half_edge_count(), 4);
    assert_eq!(row.terminal_vertex_ids().len(), 2);
    assert!(row.branch_vertex_ids().is_empty());
}

#[test]
fn product_rows_carry_wire_locality_breadth_from_read_stage_rows() {
    let closeout = close_wire_view_slice_from_query_read_source("wire-view-product-locality");
    let fixture = selected_wire_view_read_source_fixture("wire-view-product-locality");
    let read_receipt =
        super::super::WireViewReadStageExecutor::execute(&fixture.plan, fixture.read_source)
            .unwrap();
    let input = super::super::WireViewExecutionInput::from_selected_plan_and_read_stage(
        &fixture.plan,
        read_receipt,
    )
    .unwrap();
    let output = super::super::WireViewDerivedProductOutput::from_execution_input(&input);
    let output_rows = output.rows();

    assert_eq!(output_rows.len(), 1);
    assert_eq!(output_rows[0].class(), WireInterpretationClass::OpenChain);
    assert_eq!(output_rows[0].connected_component_count(), 1);
    assert_eq!(output_rows[0].half_edge_count(), 4);
    assert_eq!(output_rows[0].terminal_vertex_ids().len(), 2);
    assert!(output_rows[0].branch_vertex_ids().is_empty());
    assert_eq!(closeout.counters().output_row_count(), output_rows.len());
}

#[test]
fn query_read_rows_preserve_closed_cycle_wire_semantics() {
    let touched_closure = selected_wire_view_touched_closure("wire-view-closed-cycle");
    let provisional_plan = super::support::selected_wire_view_plan("wire-view-closed-cycle");
    let rows = closed_wire_view_query_read_rows();
    let provisional_source = super::super::WireViewReadSource::from_query_wire_views(
        &provisional_plan,
        &touched_closure,
        &rows,
    )
    .unwrap();
    let plan = super::support::selected_wire_view_plan_with_query_read_digest(
        "wire-view-closed-cycle",
        provisional_source.query_report_digests()[0].as_str(),
    );
    let read_source =
        super::super::WireViewReadSource::from_query_wire_views(&plan, &touched_closure, &rows)
            .unwrap();

    let row = &read_source.selected_rows()[0];
    assert_eq!(row.class(), WireInterpretationClass::ClosedCycle);
    assert_eq!(row.half_edge_count(), 4);
    assert!(row.terminal_vertex_ids().is_empty());
    assert!(row.branch_vertex_ids().is_empty());
}

#[test]
fn query_read_rows_preserve_branching_wire_semantics() {
    let touched_closure = selected_wire_view_touched_closure("wire-view-branch");
    let provisional_plan = super::support::selected_wire_view_plan("wire-view-branch");
    let rows = branching_wire_view_query_read_rows();
    let provisional_source = super::super::WireViewReadSource::from_query_wire_views(
        &provisional_plan,
        &touched_closure,
        &rows,
    )
    .unwrap();
    let plan = super::support::selected_wire_view_plan_with_query_read_digest(
        "wire-view-branch",
        provisional_source.query_report_digests()[0].as_str(),
    );
    let read_source =
        super::super::WireViewReadSource::from_query_wire_views(&plan, &touched_closure, &rows)
            .unwrap();

    let row = &read_source.selected_rows()[0];
    assert_eq!(row.class(), WireInterpretationClass::ConnectedBranch);
    assert_eq!(row.half_edge_count(), 5);
    assert_eq!(row.terminal_vertex_ids().len(), 1);
    assert_eq!(row.branch_vertex_ids().len(), 1);
}
