use crate::data::output::IntoNodeEvaluationResult;
use crate::facade::{
    Aspect, AspectMask, AspectVersion, ComparatorPolicyResolver, DefaultComparatorPolicyResolver,
    DefaultComparatorResolver, DefaultConditionResolver, EvaluationRequestMode, NodeId,
    SignalError, SignalGraph, VersionComparatorPolicy, VersionComparatorResolver,
};
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, execute_plan_with_policy_and_condition,
    StageExecutor,
};

pub const ASPECT_A: Aspect = Aspect::new(0);
pub const ASPECT_B: Aspect = Aspect::new(1);

pub fn mask_a() -> AspectMask {
    AspectMask::from_aspect(ASPECT_A)
}

pub fn mask_b() -> AspectMask {
    AspectMask::from_aspect(ASPECT_B)
}

pub fn version_ab(a: u64, b: u64) -> AspectVersion {
    AspectVersion::from_updates([(ASPECT_A, a), (ASPECT_B, b)])
}

pub(crate) fn evaluate<F, O>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
{
    let mut comparator = DefaultComparatorResolver;
    let mut condition = DefaultConditionResolver;
    evaluate_with_resolvers(
        graph,
        node,
        compute,
        &mut comparator,
        &mut condition,
        EvaluationRequestMode::Default,
    )
}

pub(crate) fn evaluate_on_demand<F, O>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
{
    let mut comparator = DefaultComparatorResolver;
    let mut condition = DefaultConditionResolver;
    evaluate_with_resolvers(
        graph,
        node,
        compute,
        &mut comparator,
        &mut condition,
        EvaluationRequestMode::ForceOnDemand,
    )
}

pub(crate) fn evaluate_with_resolver<F, O, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
    R: VersionComparatorResolver,
{
    let mut condition = DefaultConditionResolver;
    evaluate_with_resolvers(
        graph,
        node,
        compute,
        resolver,
        &mut condition,
        EvaluationRequestMode::Default,
    )
}

pub(crate) fn evaluate_with_resolvers<F, O, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
    R: VersionComparatorResolver,
    C: crate::facade::ConditionResolver,
{
    let mut policy = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: comparator_resolver,
    };
    evaluate_with_policy_and_condition_resolvers(
        graph,
        node,
        compute,
        &mut policy,
        condition_resolver,
        request_mode,
    )
}

pub(crate) fn evaluate_with_policy_and_condition_resolvers<F, O, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
    R: ComparatorPolicyResolver,
    C: crate::facade::ConditionResolver,
{
    let plan = build_evaluation_plan_with_policy_resolver(
        graph,
        &[node],
        request_mode,
        comparator_resolver,
    )?;
    execute_plan_with_policy_and_condition(
        graph,
        &plan,
        compute,
        comparator_resolver,
        condition_resolver,
        StageExecutor::Serial,
        None,
    )?;
    Ok(())
}
