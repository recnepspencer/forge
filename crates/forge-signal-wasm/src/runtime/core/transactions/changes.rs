use forge_signal::facade::{Aspect, ChangedRegion};

use crate::expression::model::SignalValue;

#[derive(Debug, Clone)]
pub(super) enum SetChange {
    Source {
        id: String,
        value: SignalValue,
        node: forge_signal::facade::NodeId,
        changed_regions: Vec<ChangedRegion>,
        aspects: Vec<Aspect>,
    },
    DenseGridRgba {
        family_id: String,
        rgba: Vec<u8>,
        aspects: Vec<Aspect>,
    },
}
