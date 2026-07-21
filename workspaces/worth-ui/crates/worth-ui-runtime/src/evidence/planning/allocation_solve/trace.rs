use crate::evidence::{
    UiConstraintCycleParticipationPosture, UiConstraintEqualShareDistributionPolicy,
    UiConstraintNormalizationPosture, UiConstraintResizePermissionPosture,
    UiConstraintSiblingNegotiationFixedPointPolicy,
};
use crate::runtime::WorthUiAllocationPlanningDenialReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationSolvePass {
    ViewportInput,
    ScrollOwnerInput,
    PortalAnchorInput,
    SiblingNegotiation,
    EqualShareDistribution,
    BoundedReconciliation,
    DurableResizeInput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationSolveRemainderPolicy {
    None,
    ExactFractional,
    DeterministicRemainderLeftToRightByStablePeerIdentity,
    DeterministicRemainderCenterOutByStablePeerIdentity,
    DenyIfNonIntegralRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationSolveConvergencePosture {
    AcyclicDeterministic,
    FixedPointDeterministic,
    DeniedByMeasurementBasis,
    DeniedByConstraintSet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationSolveTrace {
    planning_identity_digest: u64,
    pass_order: Box<[UiAllocationSolvePass]>,
    remainder_policy: UiAllocationSolveRemainderPolicy,
    convergence_posture: UiAllocationSolveConvergencePosture,
    resize_permission_posture: Option<UiConstraintResizePermissionPosture>,
    normalization_posture: Option<UiConstraintNormalizationPosture>,
}

impl UiAllocationSolveTrace {
    pub(crate) fn new(
        planning_identity_digest: u64,
        pass_order: Vec<UiAllocationSolvePass>,
        remainder_policy: UiAllocationSolveRemainderPolicy,
        convergence_posture: UiAllocationSolveConvergencePosture,
        resize_permission_posture: Option<UiConstraintResizePermissionPosture>,
        normalization_posture: Option<UiConstraintNormalizationPosture>,
    ) -> Self {
        Self {
            planning_identity_digest,
            pass_order: pass_order.into_boxed_slice(),
            remainder_policy,
            convergence_posture,
            resize_permission_posture,
            normalization_posture,
        }
    }

    pub fn planning_identity_digest(&self) -> u64 {
        self.planning_identity_digest
    }

    pub fn pass_order(&self) -> &[UiAllocationSolvePass] {
        &self.pass_order
    }

    pub fn remainder_policy(&self) -> UiAllocationSolveRemainderPolicy {
        self.remainder_policy
    }

    pub fn convergence_posture(&self) -> UiAllocationSolveConvergencePosture {
        self.convergence_posture
    }

    pub fn resize_permission_posture(&self) -> Option<UiConstraintResizePermissionPosture> {
        self.resize_permission_posture
    }

    pub fn normalization_posture(&self) -> Option<UiConstraintNormalizationPosture> {
        self.normalization_posture
    }

    pub fn is_deterministic(&self) -> bool {
        matches!(
            self.convergence_posture,
            UiAllocationSolveConvergencePosture::AcyclicDeterministic
                | UiAllocationSolveConvergencePosture::FixedPointDeterministic
        )
    }

    pub fn answers_boundedness_question(&self) -> bool {
        !matches!(
            self.convergence_posture,
            UiAllocationSolveConvergencePosture::DeniedByConstraintSet
        )
    }
}

pub(crate) fn remainder_policy_for_equal_share(
    policy: Option<UiConstraintEqualShareDistributionPolicy>,
) -> UiAllocationSolveRemainderPolicy {
    match policy {
        Some(UiConstraintEqualShareDistributionPolicy::ExactFractional) => {
            UiAllocationSolveRemainderPolicy::ExactFractional
        }
        Some(
            UiConstraintEqualShareDistributionPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity,
        ) => UiAllocationSolveRemainderPolicy::DeterministicRemainderLeftToRightByStablePeerIdentity,
        Some(
            UiConstraintEqualShareDistributionPolicy::DeterministicRemainderCenterOutByStablePeerIdentity,
        ) => UiAllocationSolveRemainderPolicy::DeterministicRemainderCenterOutByStablePeerIdentity,
        Some(UiConstraintEqualShareDistributionPolicy::DenyIfNonIntegralRequired) => {
            UiAllocationSolveRemainderPolicy::DenyIfNonIntegralRequired
        }
        None => UiAllocationSolveRemainderPolicy::None,
    }
}

pub(crate) fn convergence_posture_for_cycle_and_denial(
    cycle_posture: Option<UiConstraintCycleParticipationPosture>,
    fixed_point_policy: Option<UiConstraintSiblingNegotiationFixedPointPolicy>,
    denial_reason: Option<WorthUiAllocationPlanningDenialReason>,
) -> UiAllocationSolveConvergencePosture {
    match denial_reason {
        Some(WorthUiAllocationPlanningDenialReason::MeasurementBasisDenied) => {
            UiAllocationSolveConvergencePosture::DeniedByMeasurementBasis
        }
        Some(WorthUiAllocationPlanningDenialReason::ConstraintSetDenied) => {
            UiAllocationSolveConvergencePosture::DeniedByConstraintSet
        }
        None => match (cycle_posture, fixed_point_policy) {
            (Some(UiConstraintCycleParticipationPosture::AdmittedFixedPoint), _)
            | (_, Some(UiConstraintSiblingNegotiationFixedPointPolicy::AdmittedStablePeerMutual)) => {
                UiAllocationSolveConvergencePosture::FixedPointDeterministic
            }
            _ => UiAllocationSolveConvergencePosture::AcyclicDeterministic,
        },
    }
}
