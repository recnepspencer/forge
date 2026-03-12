use crate::data::output::IntoNodeEvaluationResult;
use crate::facade::{
    Aspect, AspectMask, AspectVersion, ComparatorPolicyResolver, ComputationSpec,
    DefinedComputation, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    DefaultConditionResolver, EvaluationRequestMode, ExecutionReadView, NodeContract, NodeId,
    PreparedEvaluation, SignalError, SignalGraph, SignalRuntime, VersionComparatorPolicy,
    VersionComparatorResolver,
};
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, execute_plan_with_policy_and_condition,
    StageExecutor,
};
use std::ops::DerefMut;

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
    mut graph: impl DerefMut<Target = SignalGraph>,
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
        graph.deref_mut(),
        node,
        compute,
        &mut comparator,
        &mut condition,
        EvaluationRequestMode::Default,
    )
}

pub(crate) fn evaluate_on_demand<F, O>(
    mut graph: impl DerefMut<Target = SignalGraph>,
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
        graph.deref_mut(),
        node,
        compute,
        &mut comparator,
        &mut condition,
        EvaluationRequestMode::ForceOnDemand,
    )
}

pub(crate) fn evaluate_with_resolver<F, O, R>(
    mut graph: impl DerefMut<Target = SignalGraph>,
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
        graph.deref_mut(),
        node,
        compute,
        resolver,
        &mut condition,
        EvaluationRequestMode::Default,
    )
}

pub(crate) fn evaluate_with_resolvers<F, O, R, C>(
    mut graph: impl DerefMut<Target = SignalGraph>,
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
        graph.deref_mut(),
        node,
        compute,
        &mut policy,
        condition_resolver,
        request_mode,
    )
}

pub(crate) fn evaluate_with_policy_and_condition_resolvers<F, O, R, C>(
    mut graph: impl DerefMut<Target = SignalGraph>,
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
    let graph = graph.deref_mut();
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

fn unsupported_keyed_evaluator(
    _node: NodeId,
    _view: &ExecutionReadView<'_>,
) -> Result<PreparedEvaluation, SignalError> {
    Err(SignalError::internal(
        "test helper computation definition should not use its built-in evaluator",
    ))
}

pub(crate) fn define_keyed_computation<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    family: impl Into<crate::facade::ComputationFamily>,
    tier: T,
) -> DefinedComputation<
    T,
    fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError>,
>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    runtime
        .define_computation(ComputationSpec {
            family: family.into(),
            contract: NodeContract::wildcard(),
            tier,
            comparator: VersionComparatorPolicy::Exact,
            evaluator: unsupported_keyed_evaluator
                as fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError>,
        })
        .expect("test keyed computation should define cleanly")
}
