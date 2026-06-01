use forge_query::facade::{
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDeclarationRoutePlanBindingTarget,
};

fn route_target() -> ForgeQueryDeclarationRoutePlanBindingTarget {
    loop {}
}

fn main() {
    let route_target = route_target();
    let _: ForgeQueryDeclarationBoundContributionTarget = route_target;
}
