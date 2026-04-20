mod lane;
mod matrix;
mod row_catalog;

pub use lane::{HistoricalDiffFailureClass, HistoricalDiffPerturbationClass};
pub(crate) use row_catalog::{
    HISTORICAL_DIFF_REQUIRED_CANONICAL_ROW_NAMES, HISTORICAL_DIFF_REQUIRED_REJECTION_ROW_NAMES,
};

#[cfg(test)]
mod tests;
