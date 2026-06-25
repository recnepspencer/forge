use crate::{
    PhysicalOracleJudgment, PhysicalProofOracleVerdict, PhysicalScenarioPlanIdentity,
    RuntimeVerifierParityTrace, ScenarioCounterTrace, ScenarioDenialTrace, ShortcutRejectionTrace,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalStoryTranscript {
    plan_identity: PhysicalScenarioPlanIdentity,
    counter_trace: ScenarioCounterTrace,
    denial_trace: ScenarioDenialTrace,
    parity_trace: RuntimeVerifierParityTrace,
    shortcut_trace: ShortcutRejectionTrace,
    judgments: Vec<PhysicalOracleJudgment>,
}

impl PhysicalStoryTranscript {
    pub(crate) fn from_verdict(verdict: PhysicalProofOracleVerdict) -> Self {
        let trace = verdict.observed_trace();
        Self {
            plan_identity: trace.plan_identity().clone(),
            counter_trace: trace.counter_trace().clone(),
            denial_trace: trace.denial_trace().clone(),
            parity_trace: trace.parity_trace(),
            shortcut_trace: trace.shortcut_trace().clone(),
            judgments: verdict.judgments().to_vec(),
        }
    }

    pub const fn plan_identity(&self) -> &PhysicalScenarioPlanIdentity {
        &self.plan_identity
    }

    pub const fn counter_trace(&self) -> &ScenarioCounterTrace {
        &self.counter_trace
    }

    pub const fn denial_trace(&self) -> &ScenarioDenialTrace {
        &self.denial_trace
    }

    pub const fn parity_trace(&self) -> RuntimeVerifierParityTrace {
        self.parity_trace
    }

    pub const fn shortcut_trace(&self) -> &ShortcutRejectionTrace {
        &self.shortcut_trace
    }

    pub fn judgments(&self) -> &[PhysicalOracleJudgment] {
        &self.judgments
    }
}
