use super::{
    budget::{require_compiled_plan_budget, require_terminal_decision_budget},
    currentness::{require_classification_currentness, require_scope_currentness},
    effect_compiler::{compile_conflicts, compile_effects, compile_parallel_admission},
    subsystem_compiler::compile_subsystems,
    UiChangedRebindSemanticProof, UiRebindCandidatePreparationDenial, UiRebindConflictFootprint,
    UiRebindEffectSet, UiRebindExecutionPolicy, UiRebindParallelAdmission, UiRebindPlan,
    UiRebindPlanBasis, UiRebindPlanCost, UiRebindPlanInput, UiRebindPlanTarget,
    UiRebindPlanningContext, UiRebindPlanningDenial, UiRebindSemanticProof, UiRebindSubsystemKind,
    UiRebindSubsystemPlan,
};
use crate::runtime::rebind::UiResolvedIdentityLifecycle;

pub(crate) struct UiRebindPlanCompiler;

impl UiRebindPlanCompiler {
    pub(crate) fn compile(
        context: UiRebindPlanningContext<'_>,
        lifecycle: UiResolvedIdentityLifecycle,
        policy: UiRebindExecutionPolicy,
    ) -> Result<UiRebindPlan, UiRebindPlanningDenial> {
        let (mut scope, identity_decisions) = lifecycle.into_parts();
        require_scope_currentness(&context, &scope)?;
        require_policy_session(context.session(), policy)?;
        let budget = context.budget();
        require_terminal_decision_budget(&identity_decisions, budget)?;
        let semantic_proof = finish_semantic_proof(context.runtime(), &mut scope)?;
        let basis = UiRebindPlanBasis::new(
            scope.basis().classification().clone(),
            semantic_proof_candidate_generation(&semantic_proof)
                .unwrap_or_else(|| scope.basis().candidate_generation())
                .clone(),
        );
        let binding_targets = binding_targets(&semantic_proof);
        let subsystems = compile_subsystems(&scope, &identity_decisions, binding_targets);
        require_compiled_plan_budget(&scope, &subsystems, budget)?;
        let effects = compile_effects(&subsystems);
        let conflicts = compile_conflicts(&subsystems, &effects);
        let parallel = compile_parallel_admission(&effects);
        let content = super::content_plan::compile_content_plan(&scope)?;
        let cost = compile_cost(&identity_decisions, &subsystems, &effects);
        Ok(UiRebindPlan::new(UiRebindPlanInput {
            basis,
            scope: Some(scope),
            identity_decisions,
            subsystems,
            effects,
            conflicts,
            parallel,
            content,
            policy,
            budget,
            effecting_observation_capacity: context.effecting_observation_capacity(),
            cost,
            semantic_proof,
        }))
    }

    pub(crate) fn compile_preservation(
        context: UiRebindPlanningContext<'_>,
        evidence: crate::runtime::observation::UiEvidenceOnlySourceChange,
        policy: UiRebindExecutionPolicy,
    ) -> Result<UiRebindPlan, UiRebindPlanningDenial> {
        let (classification, succession) = evidence.into_parts();
        require_classification_currentness(&context, &classification)?;
        require_policy_session(context.session(), policy)?;
        if !matches!(
            succession,
            crate::runtime::observation::UiAuthoredSourceSuccession::EvidenceOnly { .. }
        ) {
            return Err(UiRebindPlanningDenial::WrongSourceSuccessionPosture);
        }
        let basis = UiRebindPlanBasis::new(
            classification,
            succession
                .successor_authority()
                .generation_identity()
                .clone(),
        );
        let subsystems = UiRebindSubsystemKind::all()
            .into_iter()
            .map(|kind| UiRebindSubsystemPlan::new(kind, Vec::new()))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(UiRebindPlan::new(UiRebindPlanInput {
            basis,
            scope: None,
            identity_decisions: Box::new([]),
            subsystems,
            effects: UiRebindEffectSet::new(Vec::new()),
            conflicts: UiRebindConflictFootprint::new(Vec::new(), Vec::new(), Vec::new()),
            parallel: UiRebindParallelAdmission::new(Vec::new()),
            content: crate::mounting::UiMountedSemanticContentInput::empty(),
            policy,
            budget: context.budget(),
            effecting_observation_capacity: context.effecting_observation_capacity(),
            cost: UiRebindPlanCost::new(0, 0, 0, 0, 0),
            semantic_proof: UiRebindSemanticProof::EvidenceOnly(Box::new(succession)),
        }))
    }
}

fn require_policy_session(
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    policy: UiRebindExecutionPolicy,
) -> Result<(), UiRebindPlanningDenial> {
    if policy.admits_session(session) {
        Ok(())
    } else {
        Err(UiRebindPlanningDenial::ForeignExecutionPolicySession)
    }
}

fn finish_semantic_proof(
    runtime: &crate::runtime::WorthUiRuntime,
    scope: &mut super::super::UiResolvedAffectedScope,
) -> Result<UiRebindSemanticProof, UiRebindPlanningDenial> {
    let Some(succession) = scope.take_source_succession() else {
        return Ok(UiRebindSemanticProof::NonSource);
    };
    let Some((mut candidate, _comparison, replacement)) = succession.into_changed_parts() else {
        return Err(UiRebindPlanningDenial::WrongSourceSuccessionPosture);
    };
    let candidate_graph_changed_nodes =
        candidate.prepare_rebind_mount_eligibility().map_err(|_| {
            UiRebindPlanningDenial::CandidatePreparation(
                UiRebindCandidatePreparationDenial::MountEligibility,
            )
        })?;
    let lowering = runtime
        .finish_precomputed_replacement_lowering(replacement, &candidate)
        .map_err(|denial| UiRebindPlanningDenial::Replacement(Box::new(denial)))?;
    Ok(UiRebindSemanticProof::Changed(Box::new(
        UiChangedRebindSemanticProof {
            successor_authority: candidate,
            lowering,
            candidate_graph_changed_nodes,
        },
    )))
}

fn binding_targets(proof: &UiRebindSemanticProof) -> Vec<UiRebindPlanTarget> {
    let UiRebindSemanticProof::Changed(changed) = proof else {
        return Vec::new();
    };
    changed
        .lowering
        .query_rebind_plan()
        .entries()
        .iter()
        .map(|entry| UiRebindPlanTarget::QueryBinding(entry.identity().view_binding_id().into()))
        .collect()
}

fn semantic_proof_candidate_generation(
    proof: &UiRebindSemanticProof,
) -> Option<
    &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
> {
    match proof {
        UiRebindSemanticProof::Changed(changed) => {
            Some(changed.successor_authority.generation_identity())
        }
        UiRebindSemanticProof::EvidenceOnly(succession) => {
            Some(succession.successor_authority().generation_identity())
        }
        UiRebindSemanticProof::NonSource => None,
        UiRebindSemanticProof::Transferred => {
            unreachable!("planning never receives a transferred semantic proof")
        }
    }
}

fn compile_cost(
    decisions: &[super::super::UiIdentityLifecycleEntry],
    subsystems: &[UiRebindSubsystemPlan],
    effects: &UiRebindEffectSet,
) -> UiRebindPlanCost {
    UiRebindPlanCost::new(
        decisions.len(),
        subsystem_target_count(subsystems, UiRebindSubsystemKind::Graph)
            + subsystem_target_count(subsystems, UiRebindSubsystemKind::Mount),
        subsystem_target_count(subsystems, UiRebindSubsystemKind::Measurement)
            + subsystem_target_count(subsystems, UiRebindSubsystemKind::Allocation),
        subsystem_target_count(subsystems, UiRebindSubsystemKind::Binding),
        effects.effects().len(),
    )
}

fn subsystem_target_count(
    subsystems: &[UiRebindSubsystemPlan],
    kind: UiRebindSubsystemKind,
) -> usize {
    subsystems
        .binary_search_by_key(&kind, UiRebindSubsystemPlan::kind)
        .ok()
        .map(|index| subsystems[index].targets().len())
        .unwrap_or(0)
}

impl UiRebindSubsystemKind {
    pub(super) const fn all() -> [Self; 9] {
        [
            Self::Preservation,
            Self::Graph,
            Self::Mount,
            Self::Measurement,
            Self::Allocation,
            Self::Binding,
            Self::Obligation,
            Self::Surface,
            Self::Retirement,
        ]
    }
}
