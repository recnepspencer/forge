use super::ValidationManualFlowId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationManualFlowExpectation {
    status: &'static str,
    visible_result: &'static str,
    counter_posture: &'static str,
    replay_posture: &'static str,
    changed_facts: &'static [&'static str],
    rebuilt_projections: &'static [&'static str],
    preserved_projections: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationManualFlowExpectationSet {
    flow_id: ValidationManualFlowId,
    expectation: ValidationManualFlowExpectation,
}

impl ValidationManualFlowExpectation {
    pub const fn new(
        status: &'static str,
        visible_result: &'static str,
        counter_posture: &'static str,
        replay_posture: &'static str,
        changed_facts: &'static [&'static str],
        rebuilt_projections: &'static [&'static str],
        preserved_projections: &'static [&'static str],
    ) -> Self {
        Self {
            status,
            visible_result,
            counter_posture,
            replay_posture,
            changed_facts,
            rebuilt_projections,
            preserved_projections,
        }
    }

    pub fn status(self) -> &'static str {
        self.status
    }

    pub fn visible_result(self) -> &'static str {
        self.visible_result
    }

    pub fn counter_posture(self) -> &'static str {
        self.counter_posture
    }

    pub fn replay_posture(self) -> &'static str {
        self.replay_posture
    }

    pub fn changed_facts(self) -> &'static [&'static str] {
        self.changed_facts
    }

    pub fn rebuilt_projections(self) -> &'static [&'static str] {
        self.rebuilt_projections
    }

    pub fn preserved_projections(self) -> &'static [&'static str] {
        self.preserved_projections
    }
}

impl ValidationManualFlowExpectationSet {
    pub const fn new(
        flow_id: ValidationManualFlowId,
        expectation: ValidationManualFlowExpectation,
    ) -> Self {
        Self {
            flow_id,
            expectation,
        }
    }

    pub fn flow_id(self) -> ValidationManualFlowId {
        self.flow_id
    }

    pub fn expectation(self) -> ValidationManualFlowExpectation {
        self.expectation
    }
}
