use super::support::*;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::facade::RuntimeBridgeBuilder;

#[test]
fn build_freezes_mapping_registry_before_runtime_construction() {
    let runtime = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .register_mapping(exact_registration("user-profile-name"))
        .register_aspect_mapping(exact_aspect_registration("user-profile-name-field"))
        .build()
        .expect("builder should freeze mapping registrations");

    assert_eq!(
        runtime.policy(),
        &crate::policy::BridgeRuntimePolicy::default()
    );
}

#[test]
fn build_accepts_custom_diagnostics_sink() {
    let diagnostics_sink = std::sync::Arc::new(BridgeDiagnosticsFacade::new(
        crate::policy::BridgeRuntimePolicy::default(),
    ));
    let runtime = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_diagnostics_sink(diagnostics_sink)
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("builder should accept an injected diagnostics sink");

    assert_eq!(
        runtime.policy(),
        &crate::policy::BridgeRuntimePolicy::default()
    );
}
