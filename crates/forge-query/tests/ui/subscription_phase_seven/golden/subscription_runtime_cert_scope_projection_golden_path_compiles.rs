use forge_query::facade::QuerySubscriptionRuntimeCertificationScope;

fn runtime_cert_scope_projection_golden_path(scope: &QuerySubscriptionRuntimeCertificationScope) {
    let _ = scope.scope_projection().label();
}

fn main() {}
