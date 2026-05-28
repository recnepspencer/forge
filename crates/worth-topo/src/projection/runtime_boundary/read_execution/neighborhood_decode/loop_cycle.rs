use forge_query::facade::{ForgeQueryEntity, RelationName};

use crate::projection::read_views::domain::error::TopologyDomainQueryError;

use super::super::row_decode::{cycle_identities_from_successors, RetainedTopologyRows};

pub(crate) fn decode_loop_cycle(
    rows: &[ForgeQueryEntity],
    start_identity: &str,
    count: usize,
    successor_relation: &RelationName,
    label: &str,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    cycle_identities_from_successors(
        &RetainedTopologyRows::new(rows),
        start_identity,
        count,
        successor_relation,
        label,
    )
}




