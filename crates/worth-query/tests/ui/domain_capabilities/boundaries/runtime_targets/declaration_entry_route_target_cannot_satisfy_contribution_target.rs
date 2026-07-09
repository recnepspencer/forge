use worth_query::facade::{
    WorthQueryDeclarationBoundContributionTarget, WorthQueryDeclarationRoutePlanBindingTarget,
};

fn route_target() -> WorthQueryDeclarationRoutePlanBindingTarget {
    loop {}
}

fn main() {
    let route_target = route_target();
    let _: WorthQueryDeclarationBoundContributionTarget = route_target;
}
