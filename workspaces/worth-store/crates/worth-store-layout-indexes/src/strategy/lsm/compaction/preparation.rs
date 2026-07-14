use super::super::{
    AdmittedLsmCompactionDemand, BaselineLsmExecutionAdmissionDenial,
    BaselineLsmExecutionAdmissionDenialKind, LsmExecutionOperation,
    LsmExecutionOwnerCaseDeclaration, LsmExecutionOwnerCaseId, LsmExecutionOwnerCaseObservation,
    PreparedLsmCompaction,
};

#[derive(Debug)]
enum CompactionPreparationCase {
    Admitted(PreparedLsmCompaction),
    Denied(BaselineLsmExecutionAdmissionDenial),
}

#[derive(Debug)]
pub struct LsmCompactionPreparationOutcome {
    case: CompactionPreparationCase,
}

#[derive(Debug)]
pub enum LsmCompactionPreparationView<'a> {
    Admitted(&'a PreparedLsmCompaction),
    Denied(&'a BaselineLsmExecutionAdmissionDenial),
}

impl LsmCompactionPreparationOutcome {
    fn issue(result: Result<PreparedLsmCompaction, BaselineLsmExecutionAdmissionDenial>) -> Self {
        Self {
            case: match result {
                Ok(value) => CompactionPreparationCase::Admitted(value),
                Err(denial) => CompactionPreparationCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmCompactionPreparationView<'_> {
        match &self.case {
            CompactionPreparationCase::Admitted(value) => {
                LsmCompactionPreparationView::Admitted(value)
            }
            CompactionPreparationCase::Denied(denial) => {
                LsmCompactionPreparationView::Denied(denial)
            }
        }
    }

    pub fn into_result(self) -> Result<PreparedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
        match self.case {
            CompactionPreparationCase::Admitted(value) => Ok(value),
            CompactionPreparationCase::Denied(denial) => Err(denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmExecutionOwnerCaseObservation {
        LsmExecutionOwnerCaseObservation::new(match &self.case {
            CompactionPreparationCase::Admitted(_) => {
                LsmExecutionOwnerCaseId::admitted(LsmExecutionOperation::PrepareCompaction)
            }
            CompactionPreparationCase::Denied(denial) => LsmExecutionOwnerCaseId::denied(
                LsmExecutionOperation::PrepareCompaction,
                denial.kind(),
            ),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmCompactionRuntime;

pub const fn lsm_compaction_runtime() -> LsmCompactionRuntime {
    LsmCompactionRuntime
}

impl LsmCompactionRuntime {
    pub fn execute(self, demand: AdmittedLsmCompactionDemand) -> LsmCompactionPreparationOutcome {
        LsmCompactionPreparationOutcome::issue(prepare(demand))
    }
}

fn prepare(
    demand: AdmittedLsmCompactionDemand,
) -> Result<PreparedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
    let key = demand.key().canonical();
    let value = demand.value_record().identity();
    let generation = demand.generation_record().identity();
    let tombstone = demand.tombstone_record().identity();
    if value.sequence() >= generation.sequence() {
        return Err(BaselineLsmExecutionAdmissionDenial::SortedRunsNotCanonical);
    }
    if tombstone.sequence() <= generation.sequence() {
        return Err(BaselineLsmExecutionAdmissionDenial::MemtableDoesNotFollowSortedRuns);
    }
    if key == [0; 8] {
        return Err(BaselineLsmExecutionAdmissionDenial::CanonicalKeyRequired);
    }
    Ok(PreparedLsmCompaction {
        membership: demand.membership().clone(),
        replay_tail: super::LsmCompactionReplayTail::issue(
            demand.value_record().clone(),
            demand.generation_record().clone(),
            demand.tombstone_record().clone(),
        ),
        output: demand.output().clone(),
        physical_intent: demand.physical_intent().clone(),
    })
}

pub(super) fn owner_cases() -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration> {
    use BaselineLsmExecutionAdmissionDenialKind as Denial;
    const DENIALS: [Denial; 2] = [
        Denial::SortedRunsNotCanonical,
        Denial::MemtableDoesNotFollowSortedRuns,
    ];
    std::iter::once(LsmExecutionOwnerCaseDeclaration::new(
        LsmExecutionOwnerCaseId::admitted(LsmExecutionOperation::PrepareCompaction),
    ))
    .chain(DENIALS.into_iter().map(|denial| {
        LsmExecutionOwnerCaseDeclaration::new(LsmExecutionOwnerCaseId::denied(
            LsmExecutionOperation::PrepareCompaction,
            denial,
        ))
    }))
}
