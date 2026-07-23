use worth_proof::{AuthorityMarker, AuthorityWitness};
use worth_query::facade::domain::{
    WorthQueryCompiledSemanticAspectDependency, WorthQueryCompiledSemanticAspectDependencyClosure,
    WorthQueryDependencyClosureSemanticComparison, WorthQueryImpactDecision,
    WorthQuerySemanticDependencyRole,
};

struct ForgedAuthority;
impl AuthorityMarker for ForgedAuthority {}

fn forge_closure(
    raw: Vec<WorthQueryCompiledSemanticAspectDependency>,
    marker: AuthorityWitness<ForgedAuthority>,
) -> WorthQueryCompiledSemanticAspectDependencyClosure {
    WorthQueryCompiledSemanticAspectDependencyClosure::mint(raw, marker)
}

fn forge_impact(
    raw_roles: Vec<WorthQuerySemanticDependencyRole>,
    marker: AuthorityWitness<ForgedAuthority>,
) -> WorthQueryImpactDecision {
    (raw_roles, marker).into()
}

fn promote_description(
    comparison: WorthQueryDependencyClosureSemanticComparison,
) -> WorthQueryImpactDecision {
    comparison.into()
}

fn main() {}
