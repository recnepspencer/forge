use topology::derived_invalidation_migrated_products::TraversalViewsReadSource;
use topology::derived_invalidation_selected_plan::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};
use topology::facade::TopologyView;

fn main() {
    fn attempt(
        plan: &DerivedInvalidationSelectedPlan,
        closure: &DerivedInvalidationTouchedClosure,
        topology: &TopologyView,
    ) {
        let _ = TraversalViewsReadSource::select_from_touched_closure(plan, closure, topology);
    }

    let _ = attempt;
}
