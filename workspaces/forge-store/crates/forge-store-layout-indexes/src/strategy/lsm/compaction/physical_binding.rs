use forge_store_physical_isolation::CompactionRewritePublication;

use super::super::{
    BaselineLsmExecutionAdmissionDenial, BaselineLsmExecutionAdmissionDenialKind,
    LsmExecutionOperation, LsmExecutionOwnerCaseDeclaration, LsmExecutionOwnerCaseId,
    LsmExecutionOwnerCaseObservation, PreparedLsmCompaction,
};

#[derive(Debug, Clone)]
pub struct InterlockedLsmCompaction {
    pub(crate) prepared: PreparedLsmCompaction,
    pub(crate) physical: CompactionRewritePublication,
}

#[derive(Debug)]
enum PhysicalCompactionBindingCase {
    Admitted(InterlockedLsmCompaction),
    Denied(BaselineLsmExecutionAdmissionDenial),
}

#[derive(Debug)]
pub struct LsmPhysicalCompactionBindingOutcome {
    case: PhysicalCompactionBindingCase,
}

#[derive(Debug)]
pub enum LsmPhysicalCompactionBindingView<'a> {
    Admitted(&'a InterlockedLsmCompaction),
    Denied(&'a BaselineLsmExecutionAdmissionDenial),
}

impl LsmPhysicalCompactionBindingOutcome {
    fn issue(
        result: Result<InterlockedLsmCompaction, BaselineLsmExecutionAdmissionDenial>,
    ) -> Self {
        Self {
            case: match result {
                Ok(value) => PhysicalCompactionBindingCase::Admitted(value),
                Err(denial) => PhysicalCompactionBindingCase::Denied(denial),
            },
        }
    }

    pub const fn view(&self) -> LsmPhysicalCompactionBindingView<'_> {
        match &self.case {
            PhysicalCompactionBindingCase::Admitted(value) => {
                LsmPhysicalCompactionBindingView::Admitted(value)
            }
            PhysicalCompactionBindingCase::Denied(denial) => {
                LsmPhysicalCompactionBindingView::Denied(denial)
            }
        }
    }

    pub fn into_result(
        self,
    ) -> Result<InterlockedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
        match self.case {
            PhysicalCompactionBindingCase::Admitted(value) => Ok(value),
            PhysicalCompactionBindingCase::Denied(denial) => Err(denial),
        }
    }

    pub const fn owner_case_observation(&self) -> LsmExecutionOwnerCaseObservation {
        LsmExecutionOwnerCaseObservation::new(match &self.case {
            PhysicalCompactionBindingCase::Admitted(_) => {
                LsmExecutionOwnerCaseId::admitted(LsmExecutionOperation::BindPhysicalCompaction)
            }
            PhysicalCompactionBindingCase::Denied(denial) => LsmExecutionOwnerCaseId::denied(
                LsmExecutionOperation::BindPhysicalCompaction,
                denial.kind(),
            ),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmPhysicalCompactionRuntime;

pub const fn lsm_physical_compaction_runtime() -> LsmPhysicalCompactionRuntime {
    LsmPhysicalCompactionRuntime
}

impl LsmPhysicalCompactionRuntime {
    pub fn admit(
        self,
        prepared: PreparedLsmCompaction,
        physical: CompactionRewritePublication,
    ) -> LsmPhysicalCompactionBindingOutcome {
        let old_root = physical.publication().old_root();
        let new_root = physical.publication().new_root();
        let expected = &prepared.physical_intent;
        let result = if physical.delta().plan() != expected.plan()
            || old_root.scope() != expected.root_scope()
            || new_root.epoch().get() != expected.target_epoch()
            || new_root.manifest_epoch().get() != expected.manifest_epoch()
        {
            Err(BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch)
        } else {
            Ok(InterlockedLsmCompaction { prepared, physical })
        };
        LsmPhysicalCompactionBindingOutcome::issue(result)
    }
}

pub(super) fn owner_cases() -> impl Iterator<Item = LsmExecutionOwnerCaseDeclaration> {
    [
        LsmExecutionOwnerCaseDeclaration::new(LsmExecutionOwnerCaseId::admitted(
            LsmExecutionOperation::BindPhysicalCompaction,
        )),
        LsmExecutionOwnerCaseDeclaration::new(LsmExecutionOwnerCaseId::denied(
            LsmExecutionOperation::BindPhysicalCompaction,
            BaselineLsmExecutionAdmissionDenialKind::PhysicalPublicationBindingMismatch,
        )),
    ]
    .into_iter()
}
