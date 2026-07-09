use worth_runtime_bridge::facade::{BridgeRouteRequest, RuntimeBridge};

fn main() {
    let _ = BridgeRouteRequest::for_commit(sealed_authority_placeholder::<&str>());
}

fn runtime_route_requires_native_request(runtime: &RuntimeBridge) {
    let _ = runtime.route(sealed_authority_placeholder::<&str>());
}

fn sealed_authority_placeholder<T>() -> T {
    panic!("compile-fail fixture never executes")
}
