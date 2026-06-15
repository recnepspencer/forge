use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionLaneDescriptor, WorthUiLaneCostRegime,
    WorthUiLaneFailureMode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiLaneSupportStatus {
    Supported,
    Deferred,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiLaneTeachingPosture {
    OrdinaryRuntimeDx,
    SupportGateOnly,
    VisibleButDeferred,
    VisibleVocabularyOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneSupportDiagnostic {
    lane: WorthUiExecutionLane,
    status: WorthUiLaneSupportStatus,
    teaching_posture: WorthUiLaneTeachingPosture,
    cost_regime: WorthUiLaneCostRegime,
    failure_mode: WorthUiLaneFailureMode,
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneSupportRow {
    descriptor: WorthUiExecutionLaneDescriptor,
    status: WorthUiLaneSupportStatus,
    teaching_posture: WorthUiLaneTeachingPosture,
    admission_fail_closed: bool,
    support_contract_digest: u64,
}

impl WorthUiLaneSupportRow {
    pub(crate) fn supported(descriptor: WorthUiExecutionLaneDescriptor) -> Self {
        let support_contract_digest = support_digest_for_descriptor(&descriptor);
        Self {
            descriptor,
            status: WorthUiLaneSupportStatus::Supported,
            teaching_posture: WorthUiLaneTeachingPosture::OrdinaryRuntimeDx,
            admission_fail_closed: true,
            support_contract_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn unsupported(descriptor: WorthUiExecutionLaneDescriptor) -> Self {
        let support_contract_digest = support_digest_for_descriptor(&descriptor);
        Self {
            descriptor,
            status: WorthUiLaneSupportStatus::Unsupported,
            teaching_posture: WorthUiLaneTeachingPosture::VisibleVocabularyOnly,
            admission_fail_closed: true,
            support_contract_digest,
        }
    }

    pub fn descriptor(&self) -> &WorthUiExecutionLaneDescriptor {
        &self.descriptor
    }

    pub fn lane(&self) -> WorthUiExecutionLane {
        self.descriptor.lane()
    }

    pub fn cost_regime(&self) -> WorthUiLaneCostRegime {
        self.descriptor.cost_regime()
    }

    pub fn failure_mode(&self) -> WorthUiLaneFailureMode {
        self.descriptor.failure_mode()
    }

    pub fn status(&self) -> WorthUiLaneSupportStatus {
        self.status
    }

    pub fn teaching_posture(&self) -> WorthUiLaneTeachingPosture {
        self.teaching_posture
    }

    pub fn admission_fail_closed(&self) -> bool {
        self.admission_fail_closed
    }

    pub fn support_contract_digest(&self) -> u64 {
        self.support_contract_digest
    }
}

impl WorthUiLaneSupportDiagnostic {
    pub(crate) fn unsupported(row: &WorthUiLaneSupportRow) -> Self {
        Self {
            lane: row.lane(),
            status: row.status(),
            teaching_posture: row.teaching_posture(),
            cost_regime: row.cost_regime(),
            failure_mode: row.failure_mode(),
            reason: "execution lane is not admitted by the lane support matrix",
        }
    }

    pub fn lane(&self) -> WorthUiExecutionLane {
        self.lane
    }

    pub fn status(&self) -> WorthUiLaneSupportStatus {
        self.status
    }

    pub fn teaching_posture(&self) -> WorthUiLaneTeachingPosture {
        self.teaching_posture
    }

    pub fn cost_regime(&self) -> WorthUiLaneCostRegime {
        self.cost_regime
    }

    pub fn failure_mode(&self) -> WorthUiLaneFailureMode {
        self.failure_mode
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

fn support_digest_for_descriptor(descriptor: &WorthUiExecutionLaneDescriptor) -> u64 {
    let mut digest = 0xcbf29ce484222325u64;
    digest = fold(digest, descriptor.lane().canonical_tag());
    digest = fold(digest, descriptor.cost_regime().canonical_tag());
    digest = fold(digest, descriptor.failure_mode().canonical_tag());
    fold(digest, u64::from(descriptor.is_query_bound()))
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
