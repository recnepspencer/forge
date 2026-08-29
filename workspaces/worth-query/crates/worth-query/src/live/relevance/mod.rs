mod bridge_change;
mod classification;
mod query_contract;

pub use bridge_change::{
    BridgeChangeSummary, BridgeSliceCategory, MaterializationScopeTransition, MembershipTransition,
};
#[cfg(test)]
pub use bridge_change::{BridgeFieldDelta, BridgeRelationDelta};
pub use classification::{ChangeRelevance, IrrelevantChangeClass, RelevantChangeClass};
pub use query_contract::{QueryFieldKey, QueryRelevanceContract};
