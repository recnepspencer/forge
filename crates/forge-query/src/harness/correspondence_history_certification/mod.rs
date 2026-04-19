mod adapter;
mod fixtures;
mod model;
mod row_catalog;
mod rows;
mod tests;

pub use adapter::MilestoneFivePointFourCorrespondenceHistoryCertificationAdapter;
pub(crate) use row_catalog::{
    CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES,
    CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES,
};
