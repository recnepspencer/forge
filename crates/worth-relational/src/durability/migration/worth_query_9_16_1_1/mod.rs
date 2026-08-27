mod lineage;
mod relation_integrity_revision;
mod schema_authority;
mod segment;

pub(crate) use segment::{decode_segment, segment_inventory, LegacySegmentDecodeError};
