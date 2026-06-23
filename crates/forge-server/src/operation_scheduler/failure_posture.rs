use super::ForgeServerSchedulerRuntimeFailure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerSchedulerCancellationPosture {
    BeforeAdmission,
    AfterAdmissionBeforeExecution,
    DuringExecution,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerSchedulerFailurePosture {
    IsolatedRuntimeFailure {
        runtime_failure: ForgeServerSchedulerRuntimeFailure,
    },
    DependentSharedBasisFailure {
        shared_basis_key: String,
        failed_slot_ordinal: usize,
    },
    StaleMutationBasis {
        expected_basis_digest: String,
        observed_basis_digest: String,
    },
    OrderedLaneClosed {
        scheduler_lane: String,
        failed_slot_ordinal: usize,
    },
}
