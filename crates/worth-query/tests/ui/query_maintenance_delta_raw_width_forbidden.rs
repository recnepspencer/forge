use worth_query::facade::runtime::{QuerySubscriptionMaintenanceDelta, QuerySubscriptionMaintenanceDeltaKind};

fn main() {
    let _delta = QuerySubscriptionMaintenanceDelta::admitted_with_scope_label(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        todo!(),
        "employee.name",
        1,
    );
}
