#[path = "correspondence_history_parity/bundle.rs"]
mod bundle;
#[path = "correspondence_history_parity/digests.rs"]
mod digests;
#[path = "correspondence_history_parity/lowering.rs"]
mod lowering;
#[cfg(test)]
#[path = "correspondence_history_parity/tests.rs"]
#[cfg(test)]
mod tests;

pub use bundle::{
    CorrespondenceHistoricalParityBundle, CorrespondenceHistoricalParityBundleError,
    CorrespondenceHistoricalParityVariant,
};
pub use lowering::build_correspondence_historical_parity_bundle;
