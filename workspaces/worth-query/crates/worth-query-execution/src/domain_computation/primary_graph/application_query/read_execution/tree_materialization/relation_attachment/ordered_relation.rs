use worth_query_installation::facade::{
    WorthQueryInstalledGraphReadContract, WorthQueryInstalledGraphRelation,
};
use worth_relational::facade::indexes::{
    BoundedIndexParityMode, BoundedRelatedEntityOrderedLookupDenialKind,
    BoundedRelatedEntityOrderedLookupRequest,
};

use super::super::relation_distribution::distribute_relation_rows;
use super::super::{
    project_nodes, ActiveOrderedCollectionWindow, ActiveResultTreeCollectionSelection,
    OrderedCollectionProgress, ResultTreeWork, WorthQueryApplicationProjectionNode,
    WorthQueryApplicationReadExecutionDenial,
};
use super::traversal_denial;
use crate::domain_computation::primary_graph::application_query::{
    read_execution::{read_execution_denial, WorthQueryApplicationReadExecutionDenialKind},
    resource_lifecycle::WorthQueryApplicationResultBufferReservation,
};

pub(in crate::domain_computation::primary_graph::application_query::read_execution::tree_materialization) fn advance_omitted_relation(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    graph: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraph,
    relation: &WorthQueryInstalledGraphRelation,
    parents: &[WorthQueryApplicationProjectionNode],
    work: &mut ResultTreeWork,
    collection_selection: &mut ActiveResultTreeCollectionSelection,
) -> Result<(), WorthQueryApplicationReadExecutionDenial> {
    let ActiveResultTreeCollectionSelection::Ordered(window) = collection_selection else {
        return Ok(());
    };
    if window
        .request
        .as_ref()
        .is_none_or(|request| request.collection_path != relation.result_path())
    {
        return Ok(());
    }
    let request = window
        .request
        .take()
        .ok_or_else(|| traversal_denial(relation.result_path()))?;
    let [parent] = parents else {
        return Err(traversal_denial(relation.result_path()));
    };
    let child_kind = graph
        .layout
        .entity_kind(relation.child_entity())
        .ok_or_else(|| traversal_denial(relation.result_path()))?;
    let page = execute_ordered_lookup(runtime, relation, parent, child_kind, request)?;
    work.charge_ordered_index_entries(page.examined_entry_count(), relation.result_path())?;
    let generation_id = page.generation_id();
    let has_more = page.has_more();
    let next_boundary = page.into_next_boundary();
    window.progress = Some(OrderedCollectionProgress {
        generation_id,
        next_boundary,
        has_more,
    });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn attach_ordered_relation(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    projection: worth_relational::facade::runtime::VisibilityProjectionView<'_>,
    graph: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraph,
    contract: &WorthQueryInstalledGraphReadContract,
    governance: &crate::domain_computation::primary_graph::application_query::disclosure::WorthQueryApplicationQueryGovernance,
    relation: &WorthQueryInstalledGraphRelation,
    parents: &mut [WorthQueryApplicationProjectionNode],
    work: &mut ResultTreeWork,
    window: &mut ActiveOrderedCollectionWindow,
    result_buffer: &mut WorthQueryApplicationResultBufferReservation,
) -> Result<(), WorthQueryApplicationReadExecutionDenial> {
    let request = window
        .request
        .take()
        .ok_or_else(|| traversal_denial(relation.result_path()))?;
    let [parent] = parents else {
        return Err(traversal_denial(relation.result_path()));
    };
    let child_kind = graph
        .layout
        .entity_kind(relation.child_entity())
        .ok_or_else(|| traversal_denial(relation.result_path()))?;
    let page = execute_ordered_lookup(runtime, relation, parent, child_kind, request)?;
    work.charge_ordered_index_entries(page.examined_entry_count(), relation.result_path())?;
    let child_ids = page.child_entity_ids().to_vec();
    let mut nested_selection = ActiveResultTreeCollectionSelection::Complete;
    let children = project_nodes(
        runtime,
        projection,
        graph,
        contract,
        governance,
        relation.result_path(),
        relation.child_entity(),
        &child_ids,
        work,
        &mut nested_selection,
        result_buffer,
    )?;
    let has_more = page.has_more();
    window.progress = Some(OrderedCollectionProgress {
        generation_id: page.generation_id(),
        next_boundary: page.into_next_boundary(),
        has_more,
    });
    distribute_relation_rows(
        parents,
        relation,
        vec![children.len()],
        children,
        contract,
        work,
        true,
        result_buffer,
    )
}

fn execute_ordered_lookup(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    relation: &WorthQueryInstalledGraphRelation,
    parent: &WorthQueryApplicationProjectionNode,
    child_kind: worth_relational::facade::identity::KindId,
    request: super::super::OrderedCollectionWindow,
) -> Result<
    worth_relational::facade::indexes::BoundedRelatedEntityOrderedLookupOutcome,
    WorthQueryApplicationReadExecutionDenial,
> {
    let mut lookup = BoundedRelatedEntityOrderedLookupRequest::new(
        request.snapshot,
        request.index_id,
        parent.entity_id(),
        child_kind,
        request.after,
        request.page_width,
    )
    .map_err(|denial| ordered_lookup_denial(denial.kind(), relation.result_path()))?;
    if let Some(expected_generation) = request.expected_generation {
        lookup = lookup.expect_generation(expected_generation);
    }
    runtime
        .index_access()
        .execute_bounded_related_entity_ordered_lookup(lookup, BoundedIndexParityMode::Production)
        .map_err(|denial| ordered_lookup_denial(denial.kind(), relation.result_path()))
}

fn ordered_lookup_denial(
    kind: BoundedRelatedEntityOrderedLookupDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationReadExecutionDenial {
    let kind = match kind {
        BoundedRelatedEntityOrderedLookupDenialKind::InvalidPageWidth => {
            WorthQueryApplicationReadExecutionDenialKind::ContinuationPageWidthInvalid
        }
        BoundedRelatedEntityOrderedLookupDenialKind::ForeignBoundary => {
            WorthQueryApplicationReadExecutionDenialKind::ContinuationBoundaryRejected
        }
        BoundedRelatedEntityOrderedLookupDenialKind::ExpectedGenerationMismatch => {
            WorthQueryApplicationReadExecutionDenialKind::ContinuationGenerationChanged
        }
        _ => WorthQueryApplicationReadExecutionDenialKind::ContinuationIndexUnavailable,
    };
    read_execution_denial(kind, subject)
}
