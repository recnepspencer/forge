//! Compatibility shim — re-exports from `provenance` component.

pub use crate::provenance::data::lineage::lineage_record as lineage;
pub use crate::provenance::data::lineage::tracking_store as lineage_store;
pub use crate::provenance::data::reidentification_link as lineage_link;
pub use crate::provenance::data::replay::replay_log as replay;
pub use crate::provenance::logic::bulk_stamping as bulk_stamp;
