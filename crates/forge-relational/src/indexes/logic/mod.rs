mod access;
mod authority;
mod observed_field_indexes;
mod unique_entity_aspect_field_index;

#[cfg(test)]
pub(crate) use access::index_query_scratch_hint_count;
#[cfg(test)]
pub(crate) use access::index_query_scratch_hint_exists;
pub(crate) use access::purge_index_query_scratch_hints;
