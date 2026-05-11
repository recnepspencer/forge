mod support;

#[cfg(test)]
mod query_rows;
#[cfg(test)]
mod row_lookup;

#[cfg(test)]
pub(crate) use query_rows::{query_entity_id_from_row, query_relation_id_from_row};
#[cfg(test)]
pub(crate) use row_lookup::TopologyQueryRowLookup;
pub(crate) use support::{
    parse_entity_identity, parse_relation_identity, query_entity_identity, required_text,
};
