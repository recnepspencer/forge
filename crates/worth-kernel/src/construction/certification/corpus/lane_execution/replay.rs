use crate::construction::intent::PrimitiveConstructionIntent;

use super::{PrimitiveConstructionCorpusLaneGap, PrimitiveConstructionCorpusReplayLane};

pub(crate) fn prepare_replay_lane(
    intent: &PrimitiveConstructionIntent,
) -> PrimitiveConstructionCorpusReplayLane {
    PrimitiveConstructionCorpusReplayLane::new(PrimitiveConstructionCorpusLaneGap::new(
        "historical_replay_execution_surface_missing",
        format!(
            "primitive construction for {} does not yet expose a Query historical replay execution receipt that can certify direct-result parity without local reconstruction",
            intent.family().as_str()
        ),
    ))
}
