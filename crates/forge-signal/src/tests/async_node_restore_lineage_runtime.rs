use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::define_keyed_computation;

#[test]
fn async_capable_node_handle_restored_before_attachment_fails_closed_and_cannot_rediscover() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let snapshot = runtime.capture_snapshot();
    let declaration = async_node_capability_declaration(node);
    let handle = runtime
        .attach_async_capability(declaration.clone())
        .expect("async capability should attach after the checkpoint");

    let baseline = runtime
        .async_node_capability_equivalence_report(
            &handle,
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("attached handle should certify before restore");
    assert_eq!(baseline.node(), node);

    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rewind to the pre-attachment checkpoint");

    assert!(
        runtime.async_capable_node(node).is_none(),
        "pre-attachment checkpoint must not rediscover an undeclared capability"
    );

    let history_denial = runtime
        .async_node_historical_parity_report(
            &handle,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("stale public handle should fail closed after restore removes attachment");
    assert_eq!(
        history_denial.denial_class(),
        AsyncNodeHistoricalParityDenialClass::UndeclaredCapability
    );

    let equivalence_denial = runtime
        .async_node_capability_equivalence_report(
            &handle,
            &declaration,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("equivalence should not certify a handle whose attachment was restored away");
    assert_eq!(
        equivalence_denial.denial_class(),
        AsyncNodeCapabilityEquivalenceDenialClass::HistoricalParityDenied
    );
    assert_eq!(
        equivalence_denial
            .historical_parity_denial()
            .expect("equivalence denial should preserve the historical cause")
            .denial_class(),
        AsyncNodeHistoricalParityDenialClass::UndeclaredCapability
    );
}

#[test]
fn async_keyed_node_restore_rebind_rejects_old_lineage_and_requires_public_rediscovery() {
    let mut runtime = TestRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "async-restore-lineage", ());
    let keyed = family.keyed("left-wing");
    let snapshot = runtime.capture_snapshot();

    let payload_a = AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(61))
        .with_max_payload_bytes(1024);
    let declaration_a = keyed.async_capability_declaration(&mut runtime, payload_a.clone());
    let binding_a = keyed
        .declare_async_capability(&mut runtime, payload_a)
        .expect("first keyed attachment should succeed");
    let handle_a = keyed
        .async_capable_node(&mut runtime)
        .expect("first keyed handle should exist");

    let baseline = runtime
        .async_keyed_node_capability_equivalence_report(
            &binding_a,
            &handle_a,
            &declaration_a,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("first keyed attachment should certify before restore");
    assert_eq!(baseline.key(), keyed.key());

    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rewind to the pre-attachment checkpoint");
    assert!(
        keyed.async_capable_node(&mut runtime).is_none(),
        "pre-attachment checkpoint must not rediscover a keyed capability"
    );

    let stale_history_denial = runtime
        .async_keyed_node_historical_parity_report(
            &binding_a,
            &handle_a,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("old keyed binding/handle should fail closed after restore removes attachment");
    assert_eq!(
        stale_history_denial.denial_class(),
        AsyncKeyedNodeHistoricalParityDenialClass::HistoricalParityDenied
    );
    assert_eq!(
        stale_history_denial
            .historical_parity_denial()
            .expect("wrapped denial should preserve the inner historical cause")
            .denial_class(),
        AsyncNodeHistoricalParityDenialClass::UndeclaredCapability
    );

    let payload_b = AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(62))
        .with_max_payload_bytes(4096);
    let declaration_b = keyed.async_capability_declaration(&mut runtime, payload_b.clone());
    let binding_b = keyed
        .declare_async_capability(&mut runtime, payload_b)
        .expect("restored-away keyed node should be able to attach a new capability lineage");
    let handle_b = keyed
        .async_capable_node(&mut runtime)
        .expect("new keyed handle should be rediscoverable");

    assert_ne!(
        binding_a.payload_contract_digest(),
        binding_b.payload_contract_digest(),
        "restore-era rebind should create a meaningfully different capability lineage"
    );
    assert_ne!(
        handle_a.payload_contract_digest(),
        handle_b.payload_contract_digest(),
        "rediscovered public handle should reflect the new payload lineage"
    );

    let old_binding_new_handle = runtime
        .async_keyed_node_historical_parity_report(
            &binding_a,
            &handle_b,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("old binding must not certify against rediscovered new-lineage handle");
    assert_eq!(
        old_binding_new_handle.denial_class(),
        AsyncKeyedNodeHistoricalParityDenialClass::BindingHandleDigestMismatch
    );

    let new_binding_old_handle = runtime
        .async_keyed_node_historical_parity_report(
            &binding_b,
            &handle_a,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("rediscovered binding must reject an old public handle lineage");
    assert_eq!(
        new_binding_old_handle.denial_class(),
        AsyncKeyedNodeHistoricalParityDenialClass::BindingHandleDigestMismatch
    );

    let wrong_declaration_denial = runtime
        .async_keyed_node_capability_equivalence_report(
            &binding_b,
            &handle_b,
            &declaration_a,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect_err("same-node old declaration must not certify against new-lineage keyed handle");
    assert_eq!(
        wrong_declaration_denial.denial_class(),
        AsyncKeyedNodeCapabilityEquivalenceDenialClass::CapabilityEquivalenceDenied
    );
    assert_eq!(
        wrong_declaration_denial
            .capability_equivalence_denial()
            .expect("wrapped denial should preserve the inner equivalence cause")
            .denial_class(),
        AsyncNodeCapabilityEquivalenceDenialClass::HandleDeclarationDigestMismatch
    );

    let rebound = runtime
        .async_keyed_node_capability_equivalence_report(
            &binding_b,
            &handle_b,
            &declaration_b,
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("rediscovered keyed capability should certify once declaration lineage matches");
    assert_eq!(
        rebound.equivalence_report().capability_declaration_digest(),
        canonical_digest(&declaration_b)
    );
}
