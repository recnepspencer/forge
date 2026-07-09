mod trace;

pub(crate) use trace::{
    convergence_posture_for_cycle_and_denial, remainder_policy_for_equal_share,
};
pub use trace::{
    UiAllocationSolveConvergencePosture, UiAllocationSolvePass, UiAllocationSolveRemainderPolicy,
    UiAllocationSolveTrace,
};
