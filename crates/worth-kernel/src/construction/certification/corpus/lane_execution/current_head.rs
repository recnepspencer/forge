use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::tests::support::runtime_truth::prepare_primitive_construction_certification_runtime_truth;

use super::PrimitiveConstructionCorpusCurrentHeadLane;

pub(crate) fn prepare_current_head_lane(
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionCorpusCurrentHeadLane {
    PrimitiveConstructionCorpusCurrentHeadLane::new(
        prepare_primitive_construction_certification_runtime_truth(intent.into_request()),
    )
}
