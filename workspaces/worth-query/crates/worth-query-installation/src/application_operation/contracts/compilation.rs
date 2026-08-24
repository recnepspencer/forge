use std::num::NonZeroU32;

use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode, WorthQueryResourceDimension,
    WorthQueryResourceLimitRequest, WorthQuerySemanticScaleRequest,
};

use super::compiled_contract::WorthQueryCompiledApplicationOperationContracts;
use crate::application_operation::WorthQuerySealedOperationContractCompilation;
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
    WorthQueryOperationInvariantContract, WorthQueryOperationReadTouchOverlapIndex,
};

pub const APPLICATION_EXECUTION_PROVIDER_FAMILY: &str = "primary-relational-provider";
pub const APPLICATION_EXECUTION_ACCESS_PRODUCT_FAMILY: &str = "typed-primary-graph";
pub const APPLICATION_EXECUTION_ALLOCATOR_FAMILY: &str = "primary-attempt-arena";
pub const APPLICATION_EXECUTION_SAFE_POINT_FAMILY: &str = "application-attempt-boundary";
pub const APPLICATION_DECISION_FACT_FAMILY: &str = "application-operation-decision-facts";
pub const APPLICATION_AUTHORIZATION_FACT_FAMILY: &str = "application-operation-authorization-facts";
pub const APPLICATION_INVARIANT_SLOT: &str = "application-touched-graph";

impl WorthQueryCompiledApplicationOperationContracts {
    pub(in crate::application_operation) fn compile(
        compilation: WorthQuerySealedOperationContractCompilation,
    ) -> Self {
        let (
            authorization,
            mut ability_requirements,
            authored_program_width,
            decision_fact_budget,
            projection_work_budget,
            additional_authorization_fact_count,
            mutation_preconditions,
            execution_posture,
            external_effect,
            aftermath,
            graph_reads,
            touches,
            emissions,
            graph_mutation_count,
        ) = compilation.into_parts();
        ability_requirements.sort();
        ability_requirements.dedup();
        let authorization_fact_count = authorization
            .exact_fact_count(ability_requirements.len())
            .saturating_add(additional_authorization_fact_count);
        let (effects, invariants, invariant_execution) =
            mutation_contracts(decision_fact_budget, graph_mutation_count);
        let overlap_index = WorthQueryOperationReadTouchOverlapIndex::new(
            graph_reads
                .roles()
                .iter()
                .flat_map(|role| role.read_scopes().iter().cloned())
                .collect(),
            touches.scopes().to_vec(),
        );
        let decision_facts =
            application_decision_fact_contract(decision_fact_budget, authorization_fact_count);
        let resources = application_resource_contract(
            decision_fact_budget.saturating_add(authorization_fact_count),
            authored_program_width,
        );
        Self {
            authorization,
            ability_requirements,
            graph_reads,
            touches,
            emissions,
            effects,
            invariants,
            decision_facts,
            invariant_execution,
            resources,
            decision_fact_budget,
            projection_work_budget,
            additional_authorization_fact_count,
            mutation_preconditions,
            execution_posture,
            external_effect,
            aftermath,
            overlap_index,
        }
    }
}

fn mutation_contracts(
    decision_fact_budget: usize,
    graph_mutation_count: usize,
) -> (
    WorthQueryOperationEffectContract,
    WorthQueryOperationInvariantContract,
    WorthQueryInvariantExecutionContract,
) {
    if graph_mutation_count == 0 {
        return (
            WorthQueryOperationEffectContract::NotRequired,
            WorthQueryOperationInvariantContract::NotRequired,
            WorthQueryInvariantExecutionContract::NotRequired,
        );
    }
    (
        WorthQueryOperationEffectContract::Declared {
            effect_families: vec![WorthQueryOperationEffectFamily::Mutation],
        },
        WorthQueryOperationInvariantContract::Declared {
            invariant_slots: vec![APPLICATION_INVARIANT_SLOT.to_owned()],
        },
        application_invariant_execution_contract(decision_fact_budget, graph_mutation_count),
    )
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
