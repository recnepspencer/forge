use worth_query_installation::facade::{
    WorthQueryInstalledGraphReadContract, WorthQueryInstalledGraphRelation,
};

use super::{
    allocate_claimed_result_vector, order_collection, projection_denial, ResultTreeWork,
    WorthQueryApplicationProjectionNode, WorthQueryApplicationReadExecutionDenial,
};
use crate::domain_computation::primary_graph::application_query::{
    projection::WorthQueryApplicationProjectedRelation,
    resource_lifecycle::WorthQueryApplicationResultBufferReservation,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn distribute_relation_rows(
    parents: &mut [WorthQueryApplicationProjectionNode],
    relation: &WorthQueryInstalledGraphRelation,
    counts: Vec<usize>,
    children: Vec<WorthQueryApplicationProjectionNode>,
    contract: &WorthQueryInstalledGraphReadContract,
    governance: &crate::domain_computation::primary_graph::application_query::disclosure::WorthQueryApplicationQueryGovernance,
    work: &mut ResultTreeWork,
    already_ordered: bool,
    result_buffer: &mut WorthQueryApplicationResultBufferReservation,
) -> Result<(), WorthQueryApplicationReadExecutionDenial> {
    let temporary_child_buffer_bytes = children
        .capacity()
        .saturating_mul(std::mem::size_of::<WorthQueryApplicationProjectionNode>());
    let mut children = children.into_iter();
    for (parent, count) in parents.iter_mut().zip(counts) {
        let mut rows = allocate_claimed_result_vector::<WorthQueryApplicationProjectionNode>(
            result_buffer,
            count,
            relation.result_path(),
        )?;
        rows.extend(children.by_ref().take(count));
        if !already_ordered {
            order_collection(
                contract,
                governance,
                relation.result_path(),
                &mut rows,
                work,
            )?;
        }
        if rows.len() != count
            || !parent.insert_relation(WorthQueryApplicationProjectedRelation::new(relation, rows))
        {
            return Err(projection_denial(relation.result_path()));
        }
    }
    if children.next().is_some() {
        return Err(projection_denial(relation.result_path()));
    }
    drop(children);
    result_buffer.release_temporary(temporary_child_buffer_bytes);
    Ok(())
}
