use worth_query::facade::runtime::WorthQueryDeclarationBoundContributionTarget;
use worth_query::facade::WorthQueryDeclarationRoutePlanBindingTarget;

fn route_target() -> WorthQueryDeclarationRoutePlanBindingTarget {
    loop {}
}

fn main() {
    let route_target = route_target();
    let _: WorthQueryDeclarationBoundContributionTarget = route_target;
}
