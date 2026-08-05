use std::num::NonZeroU32;

use worth_foundational::facade::CanonicalDigestWorkBudget;
use worth_query_declaration::facade::application_schema::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};
use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode, WorthQueryResourceDimension,
    WorthQueryResourceLimitRequest, WorthQuerySemanticScaleRequest,
};

use super::{
    WorthQueryInstalledAbilityRequirement, WorthQueryInstalledApplicationOperationAuthorization,
    WorthQueryInstalledApplicationOperationExecutionPosture,
    WorthQueryInstalledMutationPrecondition,
};
use crate::canonical_work::WorthQueryCanonicalWorkEvidence;
use crate::domain_computation::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionProviderRequirements,
    WorthQueryExecutionResourceContract, WorthQueryExecutionResourceEnvelope,
    WorthQueryExecutionStrategyContract, WorthQueryExecutionStrategyName,
};
use crate::domain_operation::{
    WorthQueryDecisionFactFamily, WorthQueryDecisionFactKind,
    WorthQueryInstalledInvariantExecutionRequirement, WorthQueryInvariantEnforcement,
    WorthQueryInvariantExecutionContract, WorthQueryOperationDecisionFactContract,
    WorthQueryOperationEffectContract, WorthQueryOperationEffectFamily,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationGraphReadContract, WorthQueryOperationGraphReadRole,
    WorthQueryOperationInvariantContract, WorthQueryOperationTouchContract,
};

pub const APPLICATION_EXECUTION_PROVIDER_FAMILY: &str = "primary-relational-provider";
pub const APPLICATION_EXECUTION_ACCESS_PRODUCT_FAMILY: &str = "typed-primary-graph";
pub const APPLICATION_EXECUTION_ALLOCATOR_FAMILY: &str = "primary-attempt-arena";
pub const APPLICATION_EXECUTION_SAFE_POINT_FAMILY: &str = "application-attempt-boundary";
pub const APPLICATION_DECISION_FACT_FAMILY: &str = "application-operation-decision-facts";
pub const APPLICATION_AUTHORIZATION_FACT_FAMILY: &str = "application-operation-authorization-facts";
pub const APPLICATION_INVARIANT_SLOT: &str = "application-touched-graph";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCompiledApplicationOperationContracts {
    authorization: WorthQueryInstalledApplicationOperationAuthorization,
    ability_requirements: Vec<WorthQueryInstalledAbilityRequirement>,
    graph_reads: WorthQueryOperationGraphReadContract,
    touches: WorthQueryOperationTouchContract,
    effects: WorthQueryOperationEffectContract,
    invariants: WorthQueryOperationInvariantContract,
    decision_facts: WorthQueryOperationDecisionFactContract,
    invariant_execution: WorthQueryInvariantExecutionContract,
    resources: WorthQueryExecutionResourceContract,
    program: Vec<ApplicationOperationProgramTarget>,
    decision_reads: Vec<ApplicationOperationDecisionReadTarget>,
    decision_fact_budget: usize,
    projection_work_budget: usize,
    additional_authorization_fact_count: usize,
    mutation_preconditions: Vec<WorthQueryInstalledMutationPrecondition>,
    execution_posture: WorthQueryInstalledApplicationOperationExecutionPosture,
}

impl WorthQueryCompiledApplicationOperationContracts {
    pub(crate) fn compile(
        authorization: WorthQueryInstalledApplicationOperationAuthorization,
        mut ability_requirements: Vec<WorthQueryInstalledAbilityRequirement>,
        mut program: Vec<ApplicationOperationProgramTarget>,
        mut decision_reads: Vec<ApplicationOperationDecisionReadTarget>,
        decision_fact_budget: usize,
        projection_work_budget: usize,
        additional_authorization_fact_count: usize,
        mutation_preconditions: Vec<WorthQueryInstalledMutationPrecondition>,
        execution_posture: WorthQueryInstalledApplicationOperationExecutionPosture,
    ) -> Self {
        ability_requirements.sort();
        ability_requirements.dedup();
        program.sort();
        program.dedup();
        decision_reads.sort();
        decision_reads.dedup();
        let authorization_fact_count = authorization
            .exact_fact_count(ability_requirements.len())
            .saturating_add(additional_authorization_fact_count);
        let primary_role = "primary".to_string();
        let graph_reads = WorthQueryOperationGraphReadContract::Declared {
            roles: vec![WorthQueryOperationGraphReadRole {
                role: primary_role.clone(),
                participation: WorthQueryOperationGraphParticipation::PrimaryLogicalGraph,
                access: WorthQueryOperationGraphAccess::Project,
                semantic_reads: Vec::new(),
            }],
        };
        let (touches, effects, invariants, invariant_execution) = if program.is_empty() {
            (
                WorthQueryOperationTouchContract::NotRequired,
                WorthQueryOperationEffectContract::NotRequired,
                WorthQueryOperationInvariantContract::NotRequired,
                WorthQueryInvariantExecutionContract::NotRequired,
            )
        } else {
            (
                WorthQueryOperationTouchContract::Declared {
                    graph_roles: vec![primary_role],
                    scopes: program.iter().map(program_scope).collect(),
                },
                WorthQueryOperationEffectContract::Declared {
                    effect_families: vec![WorthQueryOperationEffectFamily::Mutation],
                },
                WorthQueryOperationInvariantContract::Declared {
                    invariant_slots: vec![APPLICATION_INVARIANT_SLOT.to_owned()],
                },
                application_invariant_execution_contract(decision_fact_budget, program.len()),
            )
        };
        let decision_facts =
            application_decision_fact_contract(decision_fact_budget, authorization_fact_count);
        let resources = application_resource_contract(
            decision_fact_budget.saturating_add(authorization_fact_count),
            program.len(),
        );
        Self {
            authorization,
            ability_requirements,
            graph_reads,
            touches,
            effects,
            invariants,
            decision_facts,
            invariant_execution,
            resources,
            program,
            decision_reads,
            decision_fact_budget,
            projection_work_budget,
            additional_authorization_fact_count,
            mutation_preconditions,
            execution_posture,
        }
    }

    pub fn mutation_preconditions(&self) -> &[WorthQueryInstalledMutationPrecondition] {
        &self.mutation_preconditions
    }

    pub const fn authorization(&self) -> WorthQueryInstalledApplicationOperationAuthorization {
        self.authorization
    }

    pub fn precondition_canonical_work_budget(&self) -> Option<CanonicalDigestWorkBudget> {
        let count = u32::try_from(self.mutation_preconditions.len()).ok()?;
        let entries = count.checked_mul(5)?.checked_add(1)?;
        CanonicalDigestWorkBudget::new(entries, 256 * 1_024)
    }

    /// Bounds the canonical digest over one complete delegation proposal.
    ///
    /// Entry width follows the installed program so adding a governed axis
    /// cannot retain a stale fixed-width hashing allowance.
    pub fn delegation_activation_proposal_canonical_work_budget(
        &self,
    ) -> Option<CanonicalDigestWorkBudget> {
        if !self.execution_posture.requires_delegation_activation() {
            return None;
        }
        let width = u32::try_from(self.program.len()).ok()?;
        let entries = width.checked_mul(6)?.checked_add(16)?;
        CanonicalDigestWorkBudget::new(entries, 256 * 1_024)
    }

    pub fn capability_revocation_proposal_canonical_work_budget(
        &self,
    ) -> Option<CanonicalDigestWorkBudget> {
        if !self.execution_posture.requires_capability_revocation() {
            return None;
        }
        CanonicalDigestWorkBudget::new(16, 64 * 1_024)
    }

    pub fn canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.ability_requirements.iter().fold(
            WorthQueryCanonicalWorkEvidence::zero(),
            |work, requirement| work.combine(requirement.canonical_work()),
        )
    }

    pub fn ability_requirements(&self) -> &[WorthQueryInstalledAbilityRequirement] {
        &self.ability_requirements
    }

    pub fn graph_reads(&self) -> &WorthQueryOperationGraphReadContract {
        &self.graph_reads
    }

    pub fn touches(&self) -> &WorthQueryOperationTouchContract {
        &self.touches
    }

    pub fn effects(&self) -> &WorthQueryOperationEffectContract {
        &self.effects
    }

    pub fn invariants(&self) -> &WorthQueryOperationInvariantContract {
        &self.invariants
    }

    pub fn decision_facts(&self) -> &WorthQueryOperationDecisionFactContract {
        &self.decision_facts
    }

    pub fn invariant_execution(&self) -> &WorthQueryInvariantExecutionContract {
        &self.invariant_execution
    }

    pub fn resources(&self) -> &WorthQueryExecutionResourceContract {
        &self.resources
    }

    pub fn execution_strategy(&self) -> Option<&WorthQueryExecutionStrategyContract> {
        let [strategy] = self.resources.strategies() else {
            return None;
        };
        Some(strategy)
    }

    pub fn program(&self) -> &[ApplicationOperationProgramTarget] {
        &self.program
    }

    pub fn decision_reads(&self) -> &[ApplicationOperationDecisionReadTarget] {
        &self.decision_reads
    }

    pub const fn decision_fact_budget(&self) -> usize {
        self.decision_fact_budget
    }

    pub const fn projection_work_budget(&self) -> usize {
        self.projection_work_budget
    }

    pub const fn execution_posture(
        &self,
    ) -> WorthQueryInstalledApplicationOperationExecutionPosture {
        self.execution_posture
    }

    pub(crate) const fn additional_authorization_fact_count(&self) -> usize {
        self.additional_authorization_fact_count
    }
}

fn application_decision_fact_contract(
    maximum: usize,
    authorization_fact_count: usize,
) -> WorthQueryOperationDecisionFactContract {
    let application = WorthQueryDecisionFactFamily::new(
        APPLICATION_DECISION_FACT_FAMILY,
        WorthQueryDecisionFactKind::DomainStructuralProof,
    )
    .and_then(|family| family.with_bounded_fact_count(maximum))
    .expect("installed application decision-fact budget is nonzero and canonical");
    let mut families = vec![application];
    if authorization_fact_count > 0 {
        families.push(
            WorthQueryDecisionFactFamily::new(
                APPLICATION_AUTHORIZATION_FACT_FAMILY,
                WorthQueryDecisionFactKind::DomainStructuralProof,
            )
            .and_then(|family| family.with_exact_fact_count(authorization_fact_count))
            .expect("installed authorization requirement count is nonzero and canonical"),
        );
    }
    WorthQueryOperationDecisionFactContract::declared(families)
        .expect("application decision-fact families are valid")
}

fn application_invariant_execution_contract(
    decision_fact_budget: usize,
    program_width: usize,
) -> WorthQueryInvariantExecutionContract {
    let maximum_state_facts = decision_fact_budget.saturating_add(program_width).max(1);
    let requirement = WorthQueryInstalledInvariantExecutionRequirement::new(
        APPLICATION_INVARIANT_SLOT,
        "application-installed-invariants",
        NonZeroU32::new(1).expect("one is nonzero"),
        WorthQueryInvariantEnforcement::Blocking,
        "primary",
        ["application-proposed-state"],
        maximum_state_facts,
        maximum_state_facts as u64,
    )
    .expect("static application invariant requirement is valid");
    WorthQueryInvariantExecutionContract::declared([requirement])
        .expect("one application invariant requirement is valid")
}

fn application_resource_contract(
    decision_fact_budget: usize,
    program_width: usize,
) -> WorthQueryExecutionResourceContract {
    let semantic_width = decision_fact_budget.saturating_add(program_width).max(1) as u64;
    let envelope = WorthQueryExecutionResourceEnvelope::new(
        WorthQuerySemanticScaleRequest::bounded(semantic_width),
        WorthQueryResourceLimitRequest::bounded(semantic_width)
            .with(WorthQueryResourceDimension::RetainedBytes, 262_144),
        WorthQueryExecutionMode::Synchronous,
        None,
        WorthQueryCancellationSafePointFamily::new(APPLICATION_EXECUTION_SAFE_POINT_FAMILY)
            .expect("static application safe-point family is canonical"),
    );
    let requirements = WorthQueryExecutionProviderRequirements::new(
        WorthQueryExecutionProviderFamily::new(APPLICATION_EXECUTION_PROVIDER_FAMILY)
            .expect("static application provider family is canonical"),
        WorthQueryExecutionAccessProductFamily::new(APPLICATION_EXECUTION_ACCESS_PRODUCT_FAMILY)
            .expect("static application access-product family is canonical"),
        WorthQueryExecutionAllocatorFamily::new(APPLICATION_EXECUTION_ALLOCATOR_FAMILY)
            .expect("static application allocator family is canonical"),
    );
    WorthQueryExecutionResourceContract::declared([WorthQueryExecutionStrategyContract::new(
        WorthQueryExecutionStrategyName::new("primary-application-atomic")
            .expect("static application strategy name is canonical"),
        envelope,
        requirements,
    )])
    .expect("installed application execution resource contract is valid")
}

fn program_scope(target: &ApplicationOperationProgramTarget) -> String {
    match target {
        ApplicationOperationProgramTarget::Create { entity } => format!("create:{entity}"),
        ApplicationOperationProgramTarget::Delete { entity } => format!("delete:{entity}"),
        ApplicationOperationProgramTarget::Write {
            entity,
            aspect,
            field,
        } => format!("write:{entity}/{aspect}/{field}"),
        ApplicationOperationProgramTarget::Link { relation, from, to } => {
            format!("link:{relation}:{from}->{to}")
        }
        ApplicationOperationProgramTarget::Unlink { relation, from, to } => {
            format!("unlink:{relation}:{from}->{to}")
        }
        ApplicationOperationProgramTarget::Emit { effect } => format!("emit:{effect}"),
    }
}
