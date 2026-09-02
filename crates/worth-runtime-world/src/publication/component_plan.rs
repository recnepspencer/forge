use worth_relational::facade::branch::{
    AdmittedRelationalBranchBasis, AdmittedRelationalForkSourceBasis,
};
use worth_relational::facade::mvcc::PreparedRelationalCommitCandidate;
use worth_signal::facade::branch::{AdmittedSignalBranchBasis, ValidatedSignalBranchName};

use crate::budget::RuntimeWorldBudgets;
use crate::identity::{CompositeCommitIdentity, CompositePublicationAttemptIdentity};
use crate::lifecycle::{
    RuntimeWorldCancellationBoundary, RuntimeWorldCancellationToken, RuntimeWorldInstant,
};
use crate::retention::PublicationRetentionObligation;

use super::reservation::{ReservedHistorySlot, ReservedPinAcquisitionSlots, ReservedRecoverySlot};
use crate::publication::{CompositeComponentIntent, ResolvedExpectedProductHead};

/// Relational owner posture. It is separate from Signal posture so a sibling
/// cannot be silently refreshed or omitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalComponentPlanPosture {
    RetainExact,
    PublishPrepared,
    ForkThenPublish,
}

/// Signal owner posture. Mutation input is borrowed later at execution time;
/// this plan does not retain a callback or caller context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalComponentPlanPosture {
    RetainExact,
    AdvanceExact,
    ForkThenAdvance,
}

#[derive(Debug)]
pub struct RelationalComponentPlan {
    posture: RelationalComponentPlanPosture,
    expected: AdmittedRelationalBranchBasis,
    prepared_candidate: Option<PreparedRelationalCommitCandidate>,
    fork_source: Option<AdmittedRelationalForkSourceBasis>,
}

impl RelationalComponentPlan {
    pub fn posture(&self) -> RelationalComponentPlanPosture {
        self.posture
    }

    pub fn expected(&self) -> &AdmittedRelationalBranchBasis {
        &self.expected
    }

    pub(crate) fn retain_exact(expected: AdmittedRelationalBranchBasis) -> Self {
        Self {
            posture: RelationalComponentPlanPosture::RetainExact,
            expected,
            prepared_candidate: None,
            fork_source: None,
        }
    }

    pub(crate) fn publish_prepared(
        expected: AdmittedRelationalBranchBasis,
        prepared_candidate: PreparedRelationalCommitCandidate,
    ) -> Self {
        Self {
            posture: RelationalComponentPlanPosture::PublishPrepared,
            expected,
            prepared_candidate: Some(prepared_candidate),
            fork_source: None,
        }
    }

    pub(crate) fn fork_then_publish(
        expected: AdmittedRelationalBranchBasis,
        fork_source: AdmittedRelationalForkSourceBasis,
    ) -> Self {
        Self {
            posture: RelationalComponentPlanPosture::ForkThenPublish,
            expected,
            prepared_candidate: None,
            fork_source: Some(fork_source),
        }
    }

    pub(crate) fn planned(
        posture: RelationalComponentPlanPosture,
        expected: AdmittedRelationalBranchBasis,
    ) -> Self {
        Self {
            posture,
            expected,
            prepared_candidate: None,
            fork_source: None,
        }
    }
}

#[derive(Debug)]
pub struct SignalComponentPlan {
    posture: SignalComponentPlanPosture,
    expected: AdmittedSignalBranchBasis,
    requested_branch_name: Option<ValidatedSignalBranchName>,
}

impl SignalComponentPlan {
    pub fn posture(&self) -> SignalComponentPlanPosture {
        self.posture
    }

    pub fn expected(&self) -> &AdmittedSignalBranchBasis {
        &self.expected
    }

    pub(crate) fn retain_exact(expected: AdmittedSignalBranchBasis) -> Self {
        Self {
            posture: SignalComponentPlanPosture::RetainExact,
            expected,
            requested_branch_name: None,
        }
    }

    pub(crate) fn advance_exact(expected: AdmittedSignalBranchBasis) -> Self {
        Self {
            posture: SignalComponentPlanPosture::AdvanceExact,
            expected,
            requested_branch_name: None,
        }
    }

    pub(crate) fn fork_then_advance(
        expected: AdmittedSignalBranchBasis,
        requested_branch_name: ValidatedSignalBranchName,
    ) -> Self {
        Self {
            posture: SignalComponentPlanPosture::ForkThenAdvance,
            expected,
            requested_branch_name: Some(requested_branch_name),
        }
    }

    pub(crate) fn planned(
        posture: SignalComponentPlanPosture,
        expected: AdmittedSignalBranchBasis,
    ) -> Self {
        Self {
            posture,
            expected,
            requested_branch_name: None,
        }
    }
}

/// Lowered component plans retain the exact expected composite basis and
/// never infer a sibling plan from currentness.
#[derive(Debug)]
pub struct LoweredOwnerComponentPlan {
    expected: ResolvedExpectedProductHead,
    intent: CompositeComponentIntent,
    relational: RelationalComponentPlan,
    signal: SignalComponentPlan,
}

impl LoweredOwnerComponentPlan {
    pub(crate) fn new(
        expected: ResolvedExpectedProductHead,
        intent: CompositeComponentIntent,
        relational: RelationalComponentPlan,
        signal: SignalComponentPlan,
    ) -> Self {
        Self {
            expected,
            intent,
            relational,
            signal,
        }
    }

    pub fn expected(&self) -> &ResolvedExpectedProductHead {
        &self.expected
    }

    pub fn component_intent(&self) -> CompositeComponentIntent {
        self.intent.clone()
    }

    pub fn relational(&self) -> &RelationalComponentPlan {
        &self.relational
    }

    pub fn signal(&self) -> &SignalComponentPlan {
        &self.signal
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedExpectedProductHead,
        CompositeComponentIntent,
        RelationalComponentPlan,
        SignalComponentPlan,
    ) {
        (self.expected, self.intent, self.relational, self.signal)
    }

    /// The only preparation-to-reservation transition. The plan supplies the
    /// expected head and predecessor basis; callers cannot replace either
    /// value while reserving bounded Runtime World capacity. The supplied
    /// publication obligation is for the prospective successor and is checked
    /// against the owner-issued commit before the ready token is issued.
    pub(crate) fn reserve(
        self,
        attempt_identity: CompositePublicationAttemptIdentity,
        commit_identity: CompositeCommitIdentity,
        budgets: &RuntimeWorldBudgets,
        cancellation: &RuntimeWorldCancellationToken,
        retention_obligation: PublicationRetentionObligation,
        deadline: Option<RuntimeWorldInstant>,
    ) -> Result<super::ReservedCompositePublicationAttempt, super::NoEffectCompositePublication>
    {
        if cancellation
            .check(RuntimeWorldCancellationBoundary::BeforeReservation)
            .is_err()
        {
            return Err(super::NoEffectCompositePublication::new(
                super::NoEffectCause::CancelledBeforeEffect,
                None,
            ));
        }

        let expected_head = self.expected.expected().clone();
        let owner = expected_head.owner_identity();
        if attempt_identity.owner_identity() != owner
            || commit_identity.owner_identity() != owner
            || expected_head.basis().owner_identity() != owner
        {
            return Err(super::NoEffectCompositePublication::new(
                super::NoEffectCause::OwnerDeniedBeforeEffect,
                Some(expected_head),
            ));
        }
        let predecessor_basis = expected_head.basis().clone();
        Ok(super::ReservedCompositePublicationAttempt::new(
            attempt_identity,
            expected_head,
            predecessor_basis,
            self,
            commit_identity,
            ReservedHistorySlot {
                limit: budgets.retained_composite_commits(),
            },
            ReservedRecoverySlot {
                limit: budgets.retained_product_unpublished_records(),
            },
            ReservedPinAcquisitionSlots {
                limit: budgets.in_flight_pin_acquisition_reservations(),
            },
            retention_obligation,
            deadline,
        ))
    }
}

pub(crate) fn lower_component_plans(
    expected: ResolvedExpectedProductHead,
    intent: CompositeComponentIntent,
) -> LoweredOwnerComponentPlan {
    let basis = expected.expected().basis();
    let relational = if intent.changes_relational() {
        RelationalComponentPlan::planned(
            RelationalComponentPlanPosture::PublishPrepared,
            basis.relational_basis().clone(),
        )
    } else {
        RelationalComponentPlan::retain_exact(basis.relational_basis().clone())
    };
    let signal = if intent.changes_signal() {
        SignalComponentPlan::planned(
            SignalComponentPlanPosture::AdvanceExact,
            basis.signal_basis().clone(),
        )
    } else {
        SignalComponentPlan::retain_exact(basis.signal_basis().clone())
    };
    LoweredOwnerComponentPlan::new(expected, intent, relational, signal)
}
