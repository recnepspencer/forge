use super::{PhysicalActorId, PhysicalActorStep, ScheduleReplayDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScheduleFailureClass {
    CounterMismatch,
    OracleViolation,
    FaultDeliveryMismatch,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalFaultLocus {
    actor_id: PhysicalActorId,
    yieldpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CounterMismatchSummary {
    counter_contract: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OracleVerdictSummary {
    oracle_family: String,
    verdict: OracleVerdictKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OracleVerdictKind {
    Satisfied,
    Violated,
    Indeterminate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleShrinkTrace {
    failure: ScheduleFailureSignature,
    minimized_steps: Vec<PhysicalActorStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleFailureSignature {
    failure_class: ScheduleFailureClass,
    fault_locus: PhysicalFaultLocus,
    counter_mismatch: CounterMismatchSummary,
    oracle_verdict: OracleVerdictSummary,
}

impl PhysicalFaultLocus {
    pub fn from_actor_step(step: &PhysicalActorStep) -> Self {
        Self {
            actor_id: step.actor_id_proof().clone(),
            yieldpoint: step.yieldpoint().to_owned(),
        }
    }

    pub fn actor_id(&self) -> &str {
        self.actor_id.as_str()
    }

    pub fn yieldpoint(&self) -> &str {
        &self.yieldpoint
    }
}

impl CounterMismatchSummary {
    pub fn new(counter_contract: impl Into<String>) -> Self {
        Self {
            counter_contract: counter_contract.into(),
        }
    }

    pub fn counter_contract(&self) -> &str {
        &self.counter_contract
    }
}

impl OracleVerdictSummary {
    pub fn satisfied(oracle_family: impl Into<String>) -> Self {
        Self {
            oracle_family: oracle_family.into(),
            verdict: OracleVerdictKind::Satisfied,
        }
    }

    pub fn violated(oracle_family: impl Into<String>) -> Self {
        Self {
            oracle_family: oracle_family.into(),
            verdict: OracleVerdictKind::Violated,
        }
    }

    pub fn indeterminate(oracle_family: impl Into<String>) -> Self {
        Self {
            oracle_family: oracle_family.into(),
            verdict: OracleVerdictKind::Indeterminate,
        }
    }

    pub fn oracle_family(&self) -> &str {
        &self.oracle_family
    }

    pub const fn verdict(&self) -> OracleVerdictKind {
        self.verdict
    }
}

impl ScheduleShrinkTrace {
    pub fn shrink_reproducing_failure(
        failure: ScheduleFailureSignature,
        candidate_steps: impl IntoIterator<Item = PhysicalActorStep>,
        mut observe_failure: impl FnMut(&[PhysicalActorStep]) -> Option<ScheduleFailureSignature>,
    ) -> Result<Self, ScheduleReplayDenial> {
        let mut minimized_steps: Vec<_> = candidate_steps.into_iter().collect();
        require_fault_locus_step(&failure.fault_locus, &minimized_steps)?;
        if observe_failure(&minimized_steps).as_ref() != Some(&failure) {
            return Err(ScheduleReplayDenial::ShrinkInputDoesNotReproduceFailure);
        }
        let mut index = 0;
        while index < minimized_steps.len() {
            if step_is_fault_locus(&failure.fault_locus, &minimized_steps[index]) {
                index += 1;
                continue;
            }
            let mut candidate = minimized_steps.clone();
            candidate.remove(index);
            if observe_failure(&candidate).as_ref() == Some(&failure) {
                minimized_steps = candidate;
            } else {
                index += 1;
            }
        }
        Ok(Self {
            failure,
            minimized_steps,
        })
    }

    pub const fn failure_class(&self) -> ScheduleFailureClass {
        self.failure.failure_class
    }

    pub const fn fault_locus(&self) -> &PhysicalFaultLocus {
        &self.failure.fault_locus
    }

    pub const fn counter_mismatch(&self) -> &CounterMismatchSummary {
        &self.failure.counter_mismatch
    }

    pub const fn oracle_verdict(&self) -> &OracleVerdictSummary {
        &self.failure.oracle_verdict
    }

    pub fn minimized_steps(&self) -> &[PhysicalActorStep] {
        &self.minimized_steps
    }
}

fn require_fault_locus_step(
    fault_locus: &PhysicalFaultLocus,
    minimized_steps: &[PhysicalActorStep],
) -> Result<(), ScheduleReplayDenial> {
    if minimized_steps
        .iter()
        .any(|step| step_is_fault_locus(fault_locus, step))
    {
        return Ok(());
    }
    Err(ScheduleReplayDenial::ShrinkErasedFaultLocus {
        actor_id: fault_locus.actor_id().to_owned(),
        yieldpoint: fault_locus.yieldpoint().to_owned(),
    })
}

impl ScheduleFailureSignature {
    pub const fn new(
        failure_class: ScheduleFailureClass,
        fault_locus: PhysicalFaultLocus,
        counter_mismatch: CounterMismatchSummary,
        oracle_verdict: OracleVerdictSummary,
    ) -> Self {
        Self {
            failure_class,
            fault_locus,
            counter_mismatch,
            oracle_verdict,
        }
    }
}

fn step_is_fault_locus(fault_locus: &PhysicalFaultLocus, step: &PhysicalActorStep) -> bool {
    step.actor_id() == fault_locus.actor_id() && step.yieldpoint() == fault_locus.yieldpoint()
}
