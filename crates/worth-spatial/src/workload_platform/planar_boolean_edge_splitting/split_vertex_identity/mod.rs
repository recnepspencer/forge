mod coalescence;
mod counters;
mod decision_record;
mod denial;
mod identity;
mod input_rows;
mod normalization;
mod vertex_set;

#[cfg(test)]
mod tests;

pub use counters::PlanarBooleanSplitVertexIdentityCounters;
pub use decision_record::{
    PlanarBooleanSplitVertexCoalescenceDecision, PlanarBooleanSplitVertexCoalescenceReason,
};
pub use denial::{
    PlanarBooleanSplitVertexIdentityDenial, PlanarBooleanSplitVertexIdentityDenialKind,
};
pub use vertex_set::{
    PlanarBooleanSplitVertexIdentityRow, PlanarBooleanSplitVertexIdentitySchedule,
    PlanarBooleanSplitVertexIdentitySet,
};
