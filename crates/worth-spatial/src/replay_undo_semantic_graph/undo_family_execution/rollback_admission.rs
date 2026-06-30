use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_undo_scope_identity, UndoScopeIdentityInput,
};

use crate::undo_family_catalog::SpatialUndoFamilyIdentityAuthority;

use super::{BooleanEventLedgerRollbackRequest, ProjectionReceiptRollbackRequest};
use crate::replay_undo_semantic_graph::scope_product::lower_spatial_undo_scope_product_from_selected_plan;
use crate::replay_undo_semantic_graph::{
    admit_spatial_undo_semantic_graph_input,
    lower_spatial_undo_equivalence_basis_from_scope_product, select_spatial_undo_plan,
    SpatialReplaySemanticGraphAdmissionError, SpatialUndoPlanError, SpatialUndoScopeProduct,
    SpatialUndoScopeProductCounters, SpatialUndoSemanticGraphAdmissionRequest,
};
use crate::undo_family_catalog::current_spatial_undo_family_catalog;

pub fn lower_spatial_undo_scope_product_from_boolean_event_ledger_request<'a>(
    request: BooleanEventLedgerRollbackRequest<'a>,
) -> Result<SpatialUndoScopeProduct<'a>, SpatialUndoFamilyExecutionError> {
    let admitted_input = admit_spatial_undo_semantic_graph_input(
        SpatialUndoSemanticGraphAdmissionRequest::new(
            SpatialUndoFamilyIdentityAuthority::boolean_event_ledger().identity(),
            request.spatial_touch_authority(),
            request.evidence_lookup_receipt(),
            request.stage_index_product(),
        )
        .with_lookup_consumed_workload_handoff(request.lookup_consumed_workload_handoff()),
    )?;
    let catalog = current_spatial_undo_family_catalog();
    let declaration = catalog
        .require_family(admitted_input.family_identity())
        .expect("admitted spatial undo input must come from a declared undo family");
    let undo_plan = select_spatial_undo_plan(&admitted_input)?;
    let scope_product = lower_spatial_undo_scope_product_from_selected_plan(&undo_plan)?;
    let counters = SpatialUndoScopeProductCounters::new(
        scope_product.equivalence_basis().touched_subjects().len(),
        usize::from(admitted_input.lookup_consumed_workload_handoff().is_some()),
        admitted_input
            .lookup_consumed_workload_handoff()
            .map_or(0, |value| value.counters().raw_row_scan_count()),
        admitted_input
            .lookup_consumed_workload_handoff()
            .map_or(0, |value| value.counters().broad_receipt_scan_count()),
        admitted_input
            .lookup_consumed_workload_handoff()
            .map_or(0, |value| value.counters().caller_owned_scan_count()),
    );
    Ok(SpatialUndoScopeProduct::new(
        admitted_input.family_identity(),
        declaration.workload_dependency_posture(),
        admitted_input.semantic_graph_identity().to_string(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.stage_index_identity().clone(),
        admitted_input.lookup_consumed_workload_handoff(),
        counters,
        lower_spatial_undo_equivalence_basis_from_scope_product(&scope_product),
        admit_undo_scope_identity(UndoScopeIdentityInput::new(
            scope_product.equivalence_basis().clone(),
        )),
    ))
}

pub fn lower_spatial_undo_scope_product_from_projection_receipt_request<'a>(
    request: ProjectionReceiptRollbackRequest<'a>,
) -> Result<SpatialUndoScopeProduct<'a>, SpatialUndoFamilyExecutionError> {
    let admitted_input =
        admit_spatial_undo_semantic_graph_input(SpatialUndoSemanticGraphAdmissionRequest::new(
            SpatialUndoFamilyIdentityAuthority::projection_receipt().identity(),
            request.spatial_touch_authority(),
            request.evidence_lookup_receipt(),
            request.stage_index_product(),
        ))?;
    let catalog = current_spatial_undo_family_catalog();
    let declaration = catalog
        .require_family(admitted_input.family_identity())
        .expect("admitted spatial undo input must come from a declared undo family");
    let undo_plan = select_spatial_undo_plan(&admitted_input)?;
    let scope_product = lower_spatial_undo_scope_product_from_selected_plan(&undo_plan)?;
    let counters = SpatialUndoScopeProductCounters::new(
        scope_product.equivalence_basis().touched_subjects().len(),
        usize::from(admitted_input.lookup_consumed_workload_handoff().is_some()),
        admitted_input
            .lookup_consumed_workload_handoff()
            .map_or(0, |value| value.counters().raw_row_scan_count()),
        admitted_input
            .lookup_consumed_workload_handoff()
            .map_or(0, |value| value.counters().broad_receipt_scan_count()),
        admitted_input
            .lookup_consumed_workload_handoff()
            .map_or(0, |value| value.counters().caller_owned_scan_count()),
    );
    Ok(SpatialUndoScopeProduct::new(
        admitted_input.family_identity(),
        declaration.workload_dependency_posture(),
        admitted_input.semantic_graph_identity().to_string(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.stage_index_identity().clone(),
        admitted_input.lookup_consumed_workload_handoff(),
        counters,
        lower_spatial_undo_equivalence_basis_from_scope_product(&scope_product),
        admit_undo_scope_identity(UndoScopeIdentityInput::new(
            scope_product.equivalence_basis().clone(),
        )),
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialUndoFamilyExecutionError {
    Admission(SpatialReplaySemanticGraphAdmissionError),
    Plan(SpatialUndoPlanError),
}

impl From<SpatialReplaySemanticGraphAdmissionError> for SpatialUndoFamilyExecutionError {
    fn from(value: SpatialReplaySemanticGraphAdmissionError) -> Self {
        Self::Admission(value)
    }
}

impl From<SpatialUndoPlanError> for SpatialUndoFamilyExecutionError {
    fn from(value: SpatialUndoPlanError) -> Self {
        Self::Plan(value)
    }
}
