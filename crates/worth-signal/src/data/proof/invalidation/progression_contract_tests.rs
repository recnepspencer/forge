use worth_proof::{
    AdmitExecutionReadyRecipeTransition, AssumptionBasis, ContextualTransition, CurrentValidity,
    ExecuteReadyRecipeTransition, ExecutedRecipe, ExecutionReadinessContext, ExecutionReadyRecipe,
    FreshnessScopedBasis, LowerRecipeTransition, Lowered, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Resolved, Transition, Unresolved,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PrototypeOrigin {
    SourceRecompute,
    DependencyCommit,
    StructuralRecompute,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PrototypeWork {
    target_slot: u32,
    origin: PrototypeOrigin,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PrototypeBinding {
    graph_instance: u64,
    dependency_revision: u64,
    readiness_epoch: u64,
}

type CurrentBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<PrototypeBinding>>;
type ResolvedPrototypeWork = Recipe<Resolved, PrototypeWork, CurrentBasis>;
type LoweredPrototypeBatch = Recipe<Lowered, PrototypeWork, CurrentBasis>;
type ReadyPrototypeBatch = ExecutionReadyRecipe<PrototypeWork, CurrentBasis>;
type ExecutedPrototypeBatch = ExecutedRecipe<PrototypeWork, CurrentBasis>;

struct PreparedSourceRecompute(Recipe<Unresolved, PrototypeWork>);
struct PreparedDirectInvalidation(Recipe<Unresolved, PrototypeWork>);
struct PreparedStructuralRecompute(Recipe<Unresolved, PrototypeWork>);

worth_proof::authority_marker!(SourceRecomputeAuthority);
worth_proof::authority_marker!(DependencyCommitAuthority);
worth_proof::authority_marker!(StructuralRecomputeAuthority);
worth_proof::capability_marker!(TopologyLoweringCapability);
worth_proof::authority_marker!(ReadinessAdmissionAuthority);

fn resolve_source(
    prepared: PreparedSourceRecompute,
    binding: PrototypeBinding,
) -> ResolvedPrototypeWork {
    ResolveRecipeTransition
        .transition(
            prepared.0,
            RecipeResolutionContext::new(binding, SourceRecomputeAuthority::witness()),
        )
        .into_value()
}

fn resolve_dependency(
    prepared: PreparedDirectInvalidation,
    binding: PrototypeBinding,
) -> ResolvedPrototypeWork {
    ResolveRecipeTransition
        .transition(
            prepared.0,
            RecipeResolutionContext::new(binding, DependencyCommitAuthority::witness()),
        )
        .into_value()
}

fn resolve_structural(
    prepared: PreparedStructuralRecompute,
    binding: PrototypeBinding,
) -> ResolvedPrototypeWork {
    ResolveRecipeTransition
        .transition(
            prepared.0,
            RecipeResolutionContext::new(binding, StructuralRecomputeAuthority::witness()),
        )
        .into_value()
}

fn lower(resolved: ResolvedPrototypeWork) -> LoweredPrototypeBatch {
    LowerRecipeTransition::new(TopologyLoweringCapability::witness())
        .transition(resolved)
        .into_value()
}

fn admit_ready(lowered: LoweredPrototypeBatch) -> ReadyPrototypeBatch {
    AdmitExecutionReadyRecipeTransition
        .transition(
            lowered,
            ExecutionReadinessContext::new((), ReadinessAdmissionAuthority::witness()),
        )
        .into_value()
}

fn execute_ready(ready: ReadyPrototypeBatch) -> ExecutedPrototypeBatch {
    ExecuteReadyRecipeTransition.transition(ready).into_value()
}

fn prepared(origin: PrototypeOrigin) -> Recipe<Unresolved, PrototypeWork> {
    Recipe::new(PrototypeWork {
        target_slot: 7,
        origin,
    })
}

fn binding() -> PrototypeBinding {
    PrototypeBinding {
        graph_instance: 11,
        dependency_revision: 13,
        readiness_epoch: 17,
    }
}

#[test]
fn existing_proof_surface_supports_all_three_signal_origins_and_ready_only_execution() {
    let resolved = [
        resolve_source(
            PreparedSourceRecompute(prepared(PrototypeOrigin::SourceRecompute)),
            binding(),
        ),
        resolve_dependency(
            PreparedDirectInvalidation(prepared(PrototypeOrigin::DependencyCommit)),
            binding(),
        ),
        resolve_structural(
            PreparedStructuralRecompute(prepared(PrototypeOrigin::StructuralRecompute)),
            binding(),
        ),
    ];

    for resolved in resolved {
        let expected_origin = resolved.payload().origin;
        let executed = execute_ready(admit_ready(lower(resolved)));
        assert_eq!(executed.payload().origin, expected_origin);
        assert_eq!(executed.strong_basis().value(), &binding());
    }
}
