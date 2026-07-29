use super::{
    UiRebindConflictFootprint, UiRebindEffectSet, UiRebindExecutionPolicy,
    UiRebindParallelAdmission, UiRebindPlanBasis, UiRebindPlanCost, UiRebindSubsystemKind,
    UiRebindSubsystemPlan,
};

pub struct UiRebindPlan {
    basis: UiRebindPlanBasis,
    scope: Option<super::super::UiResolvedAffectedScope>,
    identity_decisions: Box<[super::super::UiIdentityLifecycleEntry]>,
    subsystems: Box<[UiRebindSubsystemPlan]>,
    effects: UiRebindEffectSet,
    conflicts: UiRebindConflictFootprint,
    parallel: UiRebindParallelAdmission,
    policy: UiRebindExecutionPolicy,
    budget: crate::runtime::rebind::UiRebindBudgetInput,
    cost: UiRebindPlanCost,
    semantic_proof: UiRebindSemanticProof,
}

pub(crate) enum UiRebindSemanticProof {
    Changed(Box<crate::runtime::WorthUiReplacementLoweringReady>),
    EvidenceOnly(Box<crate::runtime::observation::UiAuthoredSourceSuccession>),
    NonSource,
}

pub(crate) struct UiRebindPlanInput {
    pub(crate) basis: UiRebindPlanBasis,
    pub(crate) scope: Option<super::super::UiResolvedAffectedScope>,
    pub(crate) identity_decisions: Box<[super::super::UiIdentityLifecycleEntry]>,
    pub(crate) subsystems: Box<[UiRebindSubsystemPlan]>,
    pub(crate) effects: UiRebindEffectSet,
    pub(crate) conflicts: UiRebindConflictFootprint,
    pub(crate) parallel: UiRebindParallelAdmission,
    pub(crate) policy: UiRebindExecutionPolicy,
    pub(crate) budget: crate::runtime::rebind::UiRebindBudgetInput,
    pub(crate) cost: UiRebindPlanCost,
    pub(crate) semantic_proof: UiRebindSemanticProof,
}

impl UiRebindPlan {
    pub(crate) fn new(input: UiRebindPlanInput) -> Self {
        Self {
            basis: input.basis,
            scope: input.scope,
            identity_decisions: input.identity_decisions,
            subsystems: input.subsystems,
            effects: input.effects,
            conflicts: input.conflicts,
            parallel: input.parallel,
            policy: input.policy,
            budget: input.budget,
            cost: input.cost,
            semantic_proof: input.semantic_proof,
        }
    }

    pub const fn basis(&self) -> &UiRebindPlanBasis {
        &self.basis
    }

    pub fn scope(&self) -> Option<&super::super::UiResolvedAffectedScope> {
        self.scope.as_ref()
    }

    pub fn identity_decisions(&self) -> &[super::super::UiIdentityLifecycleEntry] {
        &self.identity_decisions
    }

    pub fn subsystems(&self) -> &[UiRebindSubsystemPlan] {
        &self.subsystems
    }

    pub fn subsystem(&self, kind: UiRebindSubsystemKind) -> Option<&UiRebindSubsystemPlan> {
        self.subsystems
            .binary_search_by_key(&kind, UiRebindSubsystemPlan::kind)
            .ok()
            .map(|index| &self.subsystems[index])
    }

    pub const fn effects(&self) -> &UiRebindEffectSet {
        &self.effects
    }

    pub const fn conflicts(&self) -> &UiRebindConflictFootprint {
        &self.conflicts
    }

    pub const fn parallel_admission(&self) -> &UiRebindParallelAdmission {
        &self.parallel
    }

    pub const fn execution_policy(&self) -> UiRebindExecutionPolicy {
        self.policy
    }

    pub const fn budget(&self) -> crate::runtime::rebind::UiRebindBudgetInput {
        self.budget
    }

    pub const fn cost(&self) -> UiRebindPlanCost {
        self.cost
    }

    pub fn source_candidate_artifact_digest(&self) -> Option<u64> {
        let admitted = match &self.semantic_proof {
            UiRebindSemanticProof::Changed(lowering) => Some(lowering.admitted()),
            UiRebindSemanticProof::EvidenceOnly(succession) => succession.admitted_candidate(),
            UiRebindSemanticProof::NonSource => None,
        }?;
        Some(admitted.candidate().basis().artifact_digest().raw())
    }

    #[cfg(test)]
    pub(crate) const fn semantic_proof(&self) -> &UiRebindSemanticProof {
        &self.semantic_proof
    }
}
