use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::mvcc::PreparedRelationalCommitCandidate;
use worth_signal::facade::branch::{AdmittedSignalBranchBasis, ValidatedSignalBranchName};

use crate::branch::ProductBranchObservation;
use crate::publication::{
    CompositeComponentIntent, RelationalForkPlanInput, ResolvedExpectedProductHead,
};

mod compatibility;
mod lowering;
pub(crate) use lowering::lower_component_plans;

/// Relational owner posture. It is separate from Signal posture so a sibling
/// cannot be silently refreshed or omitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationalComponentPlanPosture {
    RetainExact,
    PublishPrepared,
    ForkExact,
    ForkAndAdvance,
}

/// Signal owner posture. Mutation input is borrowed later at execution time;
/// this plan does not retain a callback or caller context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalComponentPlanPosture {
    RetainExact,
    AdvanceExact,
    ForkExact,
    ForkAndAdvance,
}

#[derive(Debug)]
pub struct RelationalComponentPlan {
    posture: RelationalComponentPlanPosture,
    expected: AdmittedRelationalBranchBasis,
    prepared_candidate: Option<PreparedRelationalCommitCandidate>,
    fork_input: Option<RelationalForkPlanInput>,
}

impl RelationalComponentPlan {
    pub fn posture(&self) -> RelationalComponentPlanPosture {
        self.posture
    }

    pub fn expected(&self) -> &AdmittedRelationalBranchBasis {
        &self.expected
    }

    /// The owner-issued candidate, when this plan carries ordinary
    /// Relational publication evidence. Borrowing it never transfers or
    /// duplicates the candidate's linear authority.
    pub fn prepared_candidate(&self) -> Option<&PreparedRelationalCommitCandidate> {
        self.prepared_candidate.as_ref()
    }

    /// The complete owner-issued fork route, when this plan carries one.
    pub fn fork_input(&self) -> Option<&RelationalForkPlanInput> {
        self.fork_input.as_ref()
    }

    pub(crate) fn retain_exact(expected: AdmittedRelationalBranchBasis) -> Self {
        Self {
            posture: RelationalComponentPlanPosture::RetainExact,
            expected,
            prepared_candidate: None,
            fork_input: None,
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
            fork_input: None,
        }
    }

    pub(crate) fn fork_exact(
        expected: AdmittedRelationalBranchBasis,
        fork_input: RelationalForkPlanInput,
    ) -> Self {
        Self {
            posture: RelationalComponentPlanPosture::ForkExact,
            expected,
            prepared_candidate: None,
            fork_input: Some(fork_input),
        }
    }

    pub(crate) fn fork_and_advance(
        expected: AdmittedRelationalBranchBasis,
        fork_input: RelationalForkPlanInput,
    ) -> Self {
        Self {
            posture: RelationalComponentPlanPosture::ForkAndAdvance,
            expected,
            prepared_candidate: None,
            fork_input: Some(fork_input),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        RelationalComponentPlanPosture,
        AdmittedRelationalBranchBasis,
        Option<PreparedRelationalCommitCandidate>,
        Option<RelationalForkPlanInput>,
    ) {
        (
            self.posture,
            self.expected,
            self.prepared_candidate,
            self.fork_input,
        )
    }

    pub(crate) fn take_prepared_candidate(&mut self) -> Option<PreparedRelationalCommitCandidate> {
        self.prepared_candidate.take()
    }

    pub(crate) fn take_fork_input(&mut self) -> Option<RelationalForkPlanInput> {
        self.fork_input.take()
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

    /// A validated Signal name is retained only for a fork route. The
    /// execution callback and caller context remain outside this plan.
    pub fn requested_branch_name(&self) -> Option<&ValidatedSignalBranchName> {
        self.requested_branch_name.as_ref()
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

    pub(crate) fn fork_exact(
        expected: AdmittedSignalBranchBasis,
        requested_branch_name: ValidatedSignalBranchName,
    ) -> Self {
        Self {
            posture: SignalComponentPlanPosture::ForkExact,
            expected,
            requested_branch_name: Some(requested_branch_name),
        }
    }

    pub(crate) fn fork_and_advance(
        expected: AdmittedSignalBranchBasis,
        requested_branch_name: ValidatedSignalBranchName,
    ) -> Self {
        Self {
            posture: SignalComponentPlanPosture::ForkAndAdvance,
            expected,
            requested_branch_name: Some(requested_branch_name),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SignalComponentPlanPosture,
        AdmittedSignalBranchBasis,
        Option<ValidatedSignalBranchName>,
    ) {
        (self.posture, self.expected, self.requested_branch_name)
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

    /// Recheck the complete plan against the exact product predecessor before
    /// any bounded reservation is acquired. Component basis equality is
    /// owner-issued equality, not a digest or branch-name comparison.
    pub(crate) fn is_compatible_with(&self, expected: &ProductBranchObservation) -> bool {
        compatibility::plan_is_compatible_with(self, expected)
    }

    pub(crate) fn take_relational_candidate(
        &mut self,
    ) -> Option<PreparedRelationalCommitCandidate> {
        self.relational.take_prepared_candidate()
    }

    pub(crate) fn take_relational_fork_input(&mut self) -> Option<RelationalForkPlanInput> {
        self.relational.take_fork_input()
    }
}
