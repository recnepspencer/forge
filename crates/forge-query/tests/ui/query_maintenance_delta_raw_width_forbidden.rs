use forge_query::facade::{
    QuerySubscriptionMaintenanceDelta, QuerySubscriptionMaintenanceDeltaKind,
};

fn main() {
    let _delta = QuerySubscriptionMaintenanceDelta::admitted(
        QuerySubscriptionMaintenanceDeltaKind::DetailFieldDelta,
        todo!(),
        "employee.name",
        1,
    );
}
