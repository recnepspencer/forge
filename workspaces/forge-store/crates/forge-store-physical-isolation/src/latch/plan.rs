use forge_proof::{CanonicalVec, NonEmpty, Proof};

use super::{
    CanonicalLatchAcquisitionOrder, LatchAcquisitionDenial, LatchAcquisitionStep,
    LatchCounterEvidenceDenial, LatchDeniedBeforeWaitEvidence, LatchWaitCounterSnapshot,
    PhysicalLatchKey,
};

pub type LatchOrderProof =
    Proof<forge_proof::CanonicalOrder, forge_proof::StructuralProofAuthority>;

#[derive(Debug, Clone)]
pub struct LatchAcquisitionRequest {
    steps: Vec<LatchAcquisitionStep>,
}

#[derive(Debug)]
pub struct LatchAcquisitionPlan {
    steps: NonEmpty<LatchAcquisitionStep>,
    canonical_steps: CanonicalVec<LatchAcquisitionStep>,
}

impl LatchAcquisitionRequest {
    pub fn for_declared_footprint(steps: Vec<LatchAcquisitionStep>) -> Self {
        Self { steps }
    }

    pub fn steps(&self) -> &[LatchAcquisitionStep] {
        &self.steps
    }
}

impl LatchAcquisitionPlan {
    pub fn steps(&self) -> &[LatchAcquisitionStep] {
        self.steps.as_slice()
    }

    pub fn canonical_steps(&self) -> &CanonicalVec<LatchAcquisitionStep> {
        &self.canonical_steps
    }

    pub fn order_proof(&self) -> &LatchOrderProof {
        self.canonical_steps.proof()
    }
}

pub fn lower_latch_acquisition_plan(
    request: LatchAcquisitionRequest,
) -> Result<LatchAcquisitionPlan, LatchAcquisitionDenial> {
    let mut canonical_steps = request.steps;
    if canonical_steps.is_empty() {
        return Err(LatchAcquisitionDenial::EmptyPlan);
    }
    CanonicalLatchAcquisitionOrder::sort_steps(&mut canonical_steps);
    reject_duplicate_or_conflicting_steps(&canonical_steps)?;
    let non_empty_steps = NonEmpty::try_from_vec(canonical_steps.clone())
        .map_err(|_| LatchAcquisitionDenial::EmptyPlan)?;
    let canonical_steps = CanonicalVec::try_from_sorted(canonical_steps)
        .map_err(|_| LatchAcquisitionDenial::HierarchyInversion)?;
    Ok(LatchAcquisitionPlan {
        steps: non_empty_steps,
        canonical_steps,
    })
}

pub fn pre_wait_denial_for_unordered_latch_set(
    steps: &[LatchAcquisitionStep],
) -> Result<Option<LatchDeniedBeforeWaitEvidence>, LatchCounterEvidenceDenial> {
    if CanonicalLatchAcquisitionOrder::is_canonical(steps) {
        Ok(None)
    } else {
        pre_wait_denial(
            LatchAcquisitionDenial::UnorderedLockSet,
            LatchWaitCounterSnapshot::empty().with_attempts(steps.len() as u64),
        )
        .map(Some)
    }
}

pub fn pre_wait_denial_for_hierarchy_inversion(
    steps: &[LatchAcquisitionStep],
) -> Result<Option<LatchDeniedBeforeWaitEvidence>, LatchCounterEvidenceDenial> {
    if CanonicalLatchAcquisitionOrder::is_canonical(steps) {
        Ok(None)
    } else {
        pre_wait_denial(
            LatchAcquisitionDenial::HierarchyInversion,
            LatchWaitCounterSnapshot::empty().with_attempts(steps.len() as u64),
        )
        .map(Some)
    }
}

pub fn pre_wait_denial_for_execution_time_latch_discovery(
    key: PhysicalLatchKey,
) -> Result<LatchDeniedBeforeWaitEvidence, LatchCounterEvidenceDenial> {
    pre_wait_denial(
        LatchAcquisitionDenial::ExecutionTimeLatchDiscovery(key),
        LatchWaitCounterSnapshot::empty()
            .with_attempts(1)
            .with_execution_time_discovery_denial(),
    )
}

pub fn pre_wait_denial_for_unauthorized_latch_upgrade(
    key: PhysicalLatchKey,
) -> Result<LatchDeniedBeforeWaitEvidence, LatchCounterEvidenceDenial> {
    pre_wait_denial(
        LatchAcquisitionDenial::UnauthorizedUpgrade(key),
        LatchWaitCounterSnapshot::empty()
            .with_attempts(1)
            .with_denied_upgrade(),
    )
}

fn reject_duplicate_or_conflicting_steps(
    steps: &[LatchAcquisitionStep],
) -> Result<(), LatchAcquisitionDenial> {
    for window in steps.windows(2) {
        let [first, second] = window else {
            continue;
        };
        if first.key() == second.key() {
            return duplicate_or_conflicting_step_denial(*first, *second);
        }
    }
    Ok(())
}

fn duplicate_or_conflicting_step_denial(
    first: LatchAcquisitionStep,
    second: LatchAcquisitionStep,
) -> Result<(), LatchAcquisitionDenial> {
    let key = first.key();
    let first_mode = first.mode();
    let second_mode = second.mode();
    if first_mode == second_mode {
        Err(LatchAcquisitionDenial::DuplicateLatchKey(key))
    } else {
        Err(LatchAcquisitionDenial::ConflictingLatchMode {
            key,
            first: first_mode,
            second: second_mode,
        })
    }
}

fn pre_wait_denial(
    denial: LatchAcquisitionDenial,
    counters: LatchWaitCounterSnapshot,
) -> Result<LatchDeniedBeforeWaitEvidence, LatchCounterEvidenceDenial> {
    LatchDeniedBeforeWaitEvidence::new(denial, counters)
}

impl PartialEq for LatchAcquisitionStep {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
            && self.mode() == other.mode()
            && self.is_upgrade() == other.is_upgrade()
    }
}

impl Eq for LatchAcquisitionStep {}

impl PartialOrd for LatchAcquisitionStep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LatchAcquisitionStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        CanonicalLatchAcquisitionOrder::compare_steps(self, other)
    }
}
