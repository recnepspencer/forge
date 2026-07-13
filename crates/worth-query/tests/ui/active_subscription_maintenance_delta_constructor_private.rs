use worth_query::facade::runtime::QuerySubscriptionMaintenanceDelta;

fn main() {
    let _ = QuerySubscriptionMaintenanceDelta {
        kind: todo!(),
        active_lane_digest: todo!(),
        affected_scope_digest: "scope".to_string(),
        width: todo!(),
        maintenance_delta_digest: "delta".to_string(),
    };
}
