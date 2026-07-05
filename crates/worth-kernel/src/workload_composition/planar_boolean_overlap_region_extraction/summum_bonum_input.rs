use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityReadinessInput;
use topology::facade::TopologyMilestoneSevenFiveOverlapReadinessConsumer;
use worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapReadinessLoopLedgerBinding;

pub struct PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'a> {
    readiness: &'a TouchedGraphParityReadinessInput,
    readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
    readiness_binding: &'a PlanarBooleanOverlapReadinessLoopLedgerBinding,
}

impl<'a> PlanarBooleanOverlapRegionSummumBonumCloseoutInput<'a> {
    pub fn new(
        readiness: &'a TouchedGraphParityReadinessInput,
        readiness_consumer: &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer,
        readiness_binding: &'a PlanarBooleanOverlapReadinessLoopLedgerBinding,
    ) -> Self {
        Self {
            readiness,
            readiness_consumer,
            readiness_binding,
        }
    }

    pub fn readiness(&self) -> &'a TouchedGraphParityReadinessInput {
        self.readiness
    }

    pub fn readiness_consumer(&self) -> &'a TopologyMilestoneSevenFiveOverlapReadinessConsumer {
        self.readiness_consumer
    }

    pub fn readiness_binding(&self) -> &'a PlanarBooleanOverlapReadinessLoopLedgerBinding {
        self.readiness_binding
    }
}
