mod bridge_change;
mod classification;
mod query_contract;

pub use bridge_change::{
    BridgeChangeSummary, BridgeFieldDelta, BridgeLocalitySlice, BridgeRelationDelta,
    BridgeSliceCategory, MaterializationScopeTransition, MembershipTransition,
};
pub use classification::{ChangeRelevance, IrrelevantChangeClass, RelevantChangeClass};
pub use query_contract::{QueryFieldKey, QueryRelevanceContract};
