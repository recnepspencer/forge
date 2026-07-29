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
    source_candidate_artifact_digest: Option<u64>,
    semantic_proof: UiRebindSemanticProof,
}

pub(crate) enum UiRebindSemanticProof {
    Changed(Box<UiChangedRebindSemanticProof>),
    EvidenceOnly(Box<crate::runtime::observation::UiAuthoredSourceSuccession>),
    NonSource,
    Transferred,
}

pub(crate) struct UiChangedRebindSemanticProof {
    pub(crate) successor_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    pub(crate) lowering: crate::runtime::WorthUiReplacementLoweringReady,
    pub(crate) candidate_graph_changed_nodes:
        std::collections::BTreeSet<crate::graph::UiGraphNodeIdentity>,
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
        let source_candidate_artifact_digest = semantic_proof_admitted(&input.semantic_proof)
            .map(|admitted| admitted.candidate().basis().artifact_digest().raw());
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
            source_candidate_artifact_digest,
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
        self.source_candidate_artifact_digest
    }

    pub(crate) fn semantic_candidate_generation(
        &self,
    ) -> Option<
        &crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    >{
        match &self.semantic_proof {
            UiRebindSemanticProof::Changed(changed) => {
                Some(changed.successor_authority.generation_identity())
            }
            UiRebindSemanticProof::EvidenceOnly(succession) => {
                Some(succession.successor_authority().generation_identity())
            }
            UiRebindSemanticProof::NonSource | UiRebindSemanticProof::Transferred => None,
        }
    }

    pub(crate) fn take_semantic_proof(&mut self) -> UiRebindSemanticProof {
        std::mem::replace(&mut self.semantic_proof, UiRebindSemanticProof::Transferred)
    }

    #[cfg(test)]
    pub(crate) const fn semantic_proof(&self) -> &UiRebindSemanticProof {
        &self.semantic_proof
    }
}

fn semantic_proof_admitted(
    proof: &UiRebindSemanticProof,
) -> Option<&crate::runtime::WorthUiAdmittedReplacementCandidate> {
    match proof {
        UiRebindSemanticProof::Changed(changed) => Some(changed.lowering.admitted()),
        UiRebindSemanticProof::EvidenceOnly(succession) => succession.admitted_candidate(),
        UiRebindSemanticProof::NonSource | UiRebindSemanticProof::Transferred => None,
    }
}
