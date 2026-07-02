#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphTouchRuntimeLane {
    Structural,
    Participation,
    Measurement,
    QueryBinding,
    IntentOperability,
    Service,
    HostCapability,
    Diagnostic,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphTouchAspectPosture {
    Read,
    Written,
    Invalidated,
    Preserved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiGraphTouchAspectFact {
    lane: UiGraphTouchRuntimeLane,
    posture: UiGraphTouchAspectPosture,
}

impl UiGraphTouchAspectFact {
    pub(crate) const fn new(
        lane: UiGraphTouchRuntimeLane,
        posture: UiGraphTouchAspectPosture,
    ) -> Self {
        Self { lane, posture }
    }

    pub fn lane(self) -> UiGraphTouchRuntimeLane {
        self.lane
    }

    pub fn posture(self) -> UiGraphTouchAspectPosture {
        self.posture
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UiGraphTouchAspects {
    facts: Vec<UiGraphTouchAspectFact>,
}

impl UiGraphTouchAspects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn structural(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::Structural,
            posture,
        ));
        self
    }

    pub fn participation(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::Participation,
            posture,
        ));
        self
    }

    pub fn measurement(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::Measurement,
            posture,
        ));
        self
    }

    pub fn query_binding(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::QueryBinding,
            posture,
        ));
        self
    }

    pub fn intent_operability(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::IntentOperability,
            posture,
        ));
        self
    }

    pub fn service(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::Service,
            posture,
        ));
        self
    }

    pub fn host_capability(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::HostCapability,
            posture,
        ));
        self
    }

    pub fn diagnostic(mut self, posture: UiGraphTouchAspectPosture) -> Self {
        self.facts.push(UiGraphTouchAspectFact::new(
            UiGraphTouchRuntimeLane::Diagnostic,
            posture,
        ));
        self
    }

    pub(crate) fn facts(&self) -> &[UiGraphTouchAspectFact] {
        &self.facts
    }
}
