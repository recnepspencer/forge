use worth_query_declaration::facade::identity::CanonicalEquivalence;

use super::super::WorthQueryPortableDomainOperationDefinition;
use super::evidence::{MismatchEvidence, WorthQueryPortableOperationComparisonWork};
use super::{
    WorthQueryPortableOperationCostDimension as Cost,
    WorthQueryPortableOperationDimension as Dimension,
    WorthQueryPortableOperationSupportDimension as Support,
};

pub(super) fn compare_identity(
    left: &WorthQueryPortableDomainOperationDefinition,
    right: &WorthQueryPortableDomainOperationDefinition,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    require_equal(
        left.identity().name(),
        right.identity().name(),
        Dimension::IdentityName,
        work,
    )?;
    require_equal(
        &left.identity().version(),
        &right.identity().version(),
        Dimension::IdentityVersion,
        work,
    )
}

pub(super) fn compare_before_conditionals(
    left: &WorthQueryPortableDomainOperationDefinition,
    right: &WorthQueryPortableDomainOperationDefinition,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    let left = left.semantics();
    let right = right.semantics();
    work.submit_variable_items(
        super::comparison_width::parameters(&left.parameters)
            + super::comparison_width::parameters(&right.parameters),
    );
    require_equal(
        &left.parameters,
        &right.parameters,
        Dimension::Parameters,
        work,
    )?;
    work.submit_variable_items(
        super::comparison_width::canonical_query(&left.canonical_query)
            + super::comparison_width::canonical_query(&right.canonical_query),
    );
    work.inspect_owner_dimension();
    if left.canonical_query.equivalence_to(&right.canonical_query)
        != CanonicalEquivalence::Equivalent
    {
        return Err(MismatchEvidence::declaration_owner(
            Dimension::CanonicalQuery,
        ));
    }
    work.submit_variable_items(
        super::comparison_width::collection(&left.collection)
            + super::comparison_width::collection(&right.collection),
    );
    require_equal(
        &left.collection,
        &right.collection,
        Dimension::Collection,
        work,
    )?;
    work.submit_variable_items(
        left.required_capabilities.len() + right.required_capabilities.len(),
    );
    require_equal(
        &left.required_capabilities,
        &right.required_capabilities,
        Dimension::RequiredCapabilities,
        work,
    )?;
    work.submit_variable_items(left.required_domains.len() + right.required_domains.len());
    require_equal(
        &left.required_domains,
        &right.required_domains,
        Dimension::RequiredDomains,
        work,
    )?;
    require_equal(
        &left.evidence,
        &right.evidence,
        Dimension::DomainEvidence,
        work,
    )
}

pub(super) fn compare_after_conditionals(
    left: &WorthQueryPortableDomainOperationDefinition,
    right: &WorthQueryPortableDomainOperationDefinition,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    let left = left.semantics();
    let right = right.semantics();
    super::workflow_structure::compare_workflow_structure(&left.workflow, &right.workflow, work)?;
    work.submit_variable_items(
        super::comparison_width::graph_reads(&left.graph_reads)
            + super::comparison_width::graph_reads(&right.graph_reads),
    );
    require_equal(
        &left.graph_reads,
        &right.graph_reads,
        Dimension::GraphReads,
        work,
    )?;
    work.submit_variable_items(
        left.decision_facts.required_families().len()
            + right.decision_facts.required_families().len(),
    );
    require_equal(
        &left.decision_facts,
        &right.decision_facts,
        Dimension::DecisionFacts,
        work,
    )?;
    work.submit_variable_items(
        super::comparison_width::touches(&left.touches)
            + super::comparison_width::touches(&right.touches),
    );
    require_equal(&left.touches, &right.touches, Dimension::Touches, work)?;
    work.submit_variable_items(
        super::comparison_width::effects(&left.effects)
            + super::comparison_width::effects(&right.effects),
    );
    require_equal(&left.effects, &right.effects, Dimension::Effects, work)?;
    work.submit_variable_items(
        super::comparison_width::invariants(&left.invariants)
            + super::comparison_width::invariants(&right.invariants),
    );
    require_equal(
        &left.invariants,
        &right.invariants,
        Dimension::Invariants,
        work,
    )?;
    work.submit_variable_items(
        super::comparison_width::invariant_execution(&left.invariant_execution)
            + super::comparison_width::invariant_execution(&right.invariant_execution),
    );
    require_equal(
        &left.invariant_execution,
        &right.invariant_execution,
        Dimension::InvariantExecution,
        work,
    )?;
    require_equal(&left.replay, &right.replay, Dimension::Replay, work)?;
    require_equal(&left.reversal, &right.reversal, Dimension::Reversal, work)?;
    require_equal(&left.lineage, &right.lineage, Dimension::Lineage, work)?;
    require_equal(
        &left.promotion,
        &right.promotion,
        Dimension::Promotion,
        work,
    )?;
    require_equal(
        &left.publication,
        &right.publication,
        Dimension::Publication,
        work,
    )?;
    require_equal(
        &left.projection_consumption,
        &right.projection_consumption,
        Dimension::ProjectionConsumption,
        work,
    )?;
    work.submit_variable_items(
        left.terminal.result_states.len() + right.terminal.result_states.len(),
    );
    require_equal(
        &left.terminal.result_states,
        &right.terminal.result_states,
        Dimension::TerminalResultStates,
        work,
    )?;
    work.submit_variable_items(
        left.terminal.failure_classes.len() + right.terminal.failure_classes.len(),
    );
    require_equal(
        &left.terminal.failure_classes,
        &right.terminal.failure_classes,
        Dimension::TerminalFailureClasses,
        work,
    )?;
    compare_cost(left.cost, right.cost, work)?;
    compare_support(left.support, right.support, work)?;
    require_equal(
        &left.lowering.family,
        &right.lowering.family,
        Dimension::LoweringFamily,
        work,
    )?;
    require_equal(
        &left.lowering.deterministic,
        &right.lowering.deterministic,
        Dimension::LoweringDeterminism,
        work,
    )
}

fn compare_cost(
    left: super::super::WorthQueryOperationCostContract,
    right: super::super::WorthQueryOperationCostContract,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    require_equal(
        &left.lookup,
        &right.lookup,
        Dimension::Cost(Cost::Lookup),
        work,
    )?;
    require_equal(
        &left.execution,
        &right.execution,
        Dimension::Cost(Cost::Execution),
        work,
    )?;
    require_equal(
        &left.result_width,
        &right.result_width,
        Dimension::Cost(Cost::ResultWidth),
        work,
    )
}

fn compare_support(
    left: super::super::WorthQueryOperationSupportRequirements,
    right: super::super::WorthQueryOperationSupportRequirements,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    for (left, right, dimension) in [
        (left.live, right.live, Support::Live),
        (left.continuation, right.continuation, Support::Continuation),
        (
            left.async_result_state,
            right.async_result_state,
            Support::AsyncResultState,
        ),
        (left.recovery, right.recovery, Support::Recovery),
        (left.inspection, right.inspection, Support::Inspection),
        (
            left.projection_consumption,
            right.projection_consumption,
            Support::ProjectionConsumption,
        ),
        (
            left.dependency_impact,
            right.dependency_impact,
            Support::DependencyImpact,
        ),
        (left.sharing, right.sharing, Support::Sharing),
        (left.invalidation, right.invalidation, Support::Invalidation),
        (
            left.collection_delivery,
            right.collection_delivery,
            Support::CollectionDelivery,
        ),
        (
            left.conditional_evaluation,
            right.conditional_evaluation,
            Support::ConditionalEvaluation,
        ),
        (
            left.conditional_comparator,
            right.conditional_comparator,
            Support::ConditionalComparator,
        ),
        (
            left.conditional_trigger,
            right.conditional_trigger,
            Support::ConditionalTrigger,
        ),
        (
            left.conditional_temporal_or_on_demand,
            right.conditional_temporal_or_on_demand,
            Support::ConditionalTemporalOrOnDemand,
        ),
    ] {
        require_equal(&left, &right, Dimension::Support(dimension), work)?;
    }
    Ok(())
}

fn require_equal<T: PartialEq + ?Sized>(
    left: &T,
    right: &T,
    dimension: Dimension,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    work.inspect_owner_dimension();
    if left == right {
        Ok(())
    } else {
        Err(MismatchEvidence::installation_owner(dimension))
    }
}
