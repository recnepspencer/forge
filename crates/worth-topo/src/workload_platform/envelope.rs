use super::{TopologyWorkloadDeclarationIdentity, TopologyWorkloadSupportPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyWorkloadCounters {
    declaration_rows: usize,
    support_rows: usize,
}

impl TopologyWorkloadCounters {
    pub(crate) fn new(declaration_rows: usize, support_rows: usize) -> Self {
        Self {
            declaration_rows,
            support_rows,
        }
    }

    pub fn declaration_rows(&self) -> usize {
        self.declaration_rows
    }

    pub fn support_rows(&self) -> usize {
        self.support_rows
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyWorkloadEnvelope {
    identity: TopologyWorkloadDeclarationIdentity,
    support_posture: TopologyWorkloadSupportPosture,
    counters: TopologyWorkloadCounters,
}

impl TopologyWorkloadEnvelope {
    pub(crate) fn new(
        identity: TopologyWorkloadDeclarationIdentity,
        support_posture: TopologyWorkloadSupportPosture,
        counters: TopologyWorkloadCounters,
    ) -> Self {
        Self {
            identity,
            support_posture,
            counters,
        }
    }

    pub fn identity(&self) -> &TopologyWorkloadDeclarationIdentity {
        &self.identity
    }

    pub fn support_posture(&self) -> &TopologyWorkloadSupportPosture {
        &self.support_posture
    }

    pub fn counters(&self) -> TopologyWorkloadCounters {
        self.counters
    }
}
