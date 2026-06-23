mod candidate_index;
mod counters;
mod denial;
mod identity;
mod product;
mod product_validation;
mod query_index;
mod receipt;
mod work_item;
mod worklist;

pub use counters::PlanarBooleanSegmentPairEnumerationCounters;
pub use denial::{
    PlanarBooleanSegmentPairEnumerationDenial, PlanarBooleanSegmentPairEnumerationDenialKind,
};
#[cfg(test)]
pub(crate) use product::PlanarBooleanSegmentCandidateIndexProductInput;
pub use product::{
    PlanarBooleanCandidateBroadPhaseReason, PlanarBooleanCandidateEnvelopeBasis,
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanSegmentCandidateIndexProduct,
    PlanarBooleanSegmentCandidateRowReceipt,
};
pub use receipt::PlanarBooleanSegmentPairEnumerationReceipt;
pub use work_item::PlanarBooleanSegmentPairWorkItem;

pub(crate) use worklist::enumerate_segment_pairs;
