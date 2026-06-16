mod support;

mod admitted_commit_identity;
mod identity_reporting;

#[cfg(test)]
mod query_rows;
#[cfg(test)]
mod row_lookup;

#[cfg(test)]
pub(crate) use query_rows::{query_entity_id_from_row, query_relation_id_from_row};
#[cfg(test)]
pub(crate) use row_lookup::TopologyQueryRowLookup;
pub(crate) use admitted_commit_identity::derived_surface_commit_identity;
pub(crate) use identity_reporting::{
    bridge_identity_projection, query_entity_identity_reporting_label,
};
pub(crate) use support::{entity_id_from_query_identity, query_entity_identity, required_text};
