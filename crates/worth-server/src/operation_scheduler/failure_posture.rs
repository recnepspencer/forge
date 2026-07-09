use super::WorthServerSchedulerRuntimeFailure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerSchedulerCancellationPosture {
    BeforeAdmission,
    AfterAdmissionBeforeExecution,
    DuringExecution,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerSchedulerFailurePosture {
    IsolatedRuntimeFailure {
        runtime_failure: WorthServerSchedulerRuntimeFailure,
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
