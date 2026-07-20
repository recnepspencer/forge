use super::super::support::*;
use std::collections::BTreeSet;
use worth_relational::facade::config::CrossContextPolicy;
use worth_relational::facade::identity::KindId;
use worth_relational::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};
use worth_relational::facade::schema::{
    ContractId, LoweredEndpointKindContract, RelationIntegrityPlanRevision,
};

#[test]
fn query_builder_exposes_graph_obligation_registration_catalog() {
    let registration = WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap(),
        WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let runtime = complete_backend_from_parts_builder()
        .graph_obligation(registration)
        .build_backend_from_parts()
        .build()
        .expect("query runtime should assemble graph obligation registrations");

    assert_eq!(
        runtime
            .graph_obligation_registration_catalog()
            .registration_count(),
        1
    );
}

#[test]
fn query_builder_accepts_registration_for_every_obligation_kind() {
    let selector = WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let registrations = vec![
        WorthQueryGraphObligationRegistration::blocking_invariant(
            WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "blocking", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        WorthQueryGraphObligationRegistration::schema_contract_validator(
            WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "schema", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        WorthQueryGraphObligationRegistration::advisory_obligation(
            WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "advisory", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        WorthQueryGraphObligationRegistration::preflight_sequencing_obligation(
            WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "preflight", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        WorthQueryGraphObligationRegistration::capability_gap_screen(
            WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "capability", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        WorthQueryGraphObligationRegistration::operating_context_gate(
            WorthQueryGraphObligationRuleIdentity::new(
                "test.graph-obligation",
                "operating-context",
                "v1",
            )
            .unwrap(),
            selector,
            world,
        ),
    ];
    let runtime = registrations
        .into_iter()
        .fold(
            complete_backend_from_parts_builder(),
            |builder, registration| builder.graph_obligation(registration),
        )
        .build_backend_from_parts()
        .build()
        .expect("query runtime should assemble every graph obligation kind");

    assert_eq!(
        runtime
            .graph_obligation_registration_catalog()
            .registration_count(),
        6
    );
    assert_eq!(
        runtime
            .graph_obligation_registration_catalog()
            .registrations()
            .iter()
            .map(|registration| registration.kind().as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "advisory-obligation",
            "blocking-invariant",
            "capability-gap-screen",
            "operating-context-gate",
            "preflight-sequencing-obligation",
            "schema-contract-validator",
        ])
    );
}

#[test]
fn graph_obligation_registration_catalog_digest_is_order_independent() {
    let selector = WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let left = WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "schema", "v1")
            .unwrap(),
        selector.clone(),
        world,
    );
    let right = WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "blocking", "v1")
            .unwrap(),
        selector,
        world,
    );

    let first = complete_backend_from_parts_builder()
        .graph_obligation(left.clone())
        .graph_obligation(right.clone())
        .build_backend_from_parts()
        .build()
        .unwrap();
    let second = complete_backend_from_parts_builder()
        .graph_obligation(right)
        .graph_obligation(left)
        .build_backend_from_parts()
        .build()
        .unwrap();

    assert_eq!(
        first
            .graph_obligation_registration_catalog()
            .catalog_digest(),
        second
            .graph_obligation_registration_catalog()
            .catalog_digest()
    );
    assert_eq!(
        first
            .graph_obligation_registration_catalog()
            .registrations()
            .iter()
            .map(|registration| registration.registration_digest())
            .collect::<Vec<_>>(),
        second
            .graph_obligation_registration_catalog()
            .registrations()
            .iter()
            .map(|registration| registration.registration_digest())
            .collect::<Vec<_>>()
    );
}

#[test]
fn query_builder_auto_indexes_relational_schema_contract_obligations() {
    let runtime = complete_backend_from_parts_builder()
        .invariant_catalog(InvariantCatalog {
            registrations: vec![InvariantRegistration::commit_boundary_blocking(
                InvariantRule::EndpointKindContract(LoweredEndpointKindContract {
                    contract_id: ContractId::new("loop-successor-endpoint-kind"),
                    relation_kind_id: KindId(77),
                    allowed_source_kinds: vec![KindId(1)],
                    allowed_target_kinds: vec![KindId(2)],
                    self_edges_allowed: false,
                    cross_context_policy: CrossContextPolicy::SchemaControlled,
                    plan_revision: RelationIntegrityPlanRevision(1),
                }),
            )],
        })
        .build_backend_from_parts()
        .build()
        .expect("query runtime should auto-index relational schema contract obligations");

    let catalog = runtime.graph_obligation_registration_catalog();
    assert_eq!(catalog.registration_count(), 1);
    assert_eq!(
        catalog.registrations()[0].rule_identity().namespace(),
        "relational-schema-contract"
    );
    assert_eq!(
        catalog.registrations()[0].rule_identity().name(),
        "endpoint-kind:loop-successor-endpoint-kind"
    );
    assert_eq!(
        catalog.registrations()[0]
            .touch_selector()
            .terminal_selector_kind_for_boundary(),
        "relation-kind-id"
    );
    assert_eq!(
        catalog.registrations()[0]
            .touch_selector()
            .terminal_selector_value_for_boundary()
            .as_deref(),
        Some("77")
    );
}

#[test]
fn query_builder_rejects_conflicting_graph_obligation_registration_slot() {
    let rule_identity =
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let selector = WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let world = WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority();

    let error = match complete_backend_from_parts_builder()
        .graph_obligation(
            WorthQueryGraphObligationRegistration::schema_contract_validator(
                rule_identity.clone(),
                selector.clone(),
                world,
            ),
        )
        .graph_obligation(WorthQueryGraphObligationRegistration::blocking_invariant(
            rule_identity,
            selector,
            world,
        ))
        .build_backend_from_parts()
        .build()
    {
        Ok(_) => panic!("query runtime should reject conflicting graph obligation slot"),
        Err(error) => error,
    };

    match error {
        WorthQueryRuntimeError::InvariantRegistration { stage, message } => {
            assert_eq!(stage, "graph_obligation_registration_catalog_assembly");
            assert!(message.contains("conflicting registrations"));
        }
        other => panic!("unexpected runtime error: {other:?}"),
    }
}

#[test]
fn query_builder_rejects_explicit_registration_conflicting_with_auto_lowered_schema_contract() {
    let error = match complete_backend_from_parts_builder()
        .invariant_catalog(InvariantCatalog {
            registrations: vec![InvariantRegistration::commit_boundary_blocking(
                InvariantRule::EndpointKindContract(LoweredEndpointKindContract {
                    contract_id: ContractId::new("loop-successor-endpoint-kind"),
                    relation_kind_id: KindId(77),
                    allowed_source_kinds: vec![KindId(1)],
                    allowed_target_kinds: vec![KindId(2)],
                    self_edges_allowed: false,
                    cross_context_policy: CrossContextPolicy::SchemaControlled,
                    plan_revision: RelationIntegrityPlanRevision(1),
                }),
            )],
        })
        .graph_obligation(WorthQueryGraphObligationRegistration::blocking_invariant(
            WorthQueryGraphObligationRuleIdentity::new(
                "relational-schema-contract",
                "endpoint-kind:loop-successor-endpoint-kind",
                "v1",
            )
            .unwrap(),
            WorthQueryGraphTouchSelector::relational_kind_id(KindId(77)),
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ))
        .build_backend_from_parts()
        .build()
    {
        Ok(_) => panic!("query runtime should reject explicit conflict with auto-lowered schema"),
        Err(error) => error,
    };

    match error {
        WorthQueryRuntimeError::InvariantRegistration { stage, message } => {
            assert_eq!(stage, "graph_obligation_registration_catalog_assembly");
            assert!(message.contains("conflicting registrations"));
        }
        other => panic!("unexpected runtime error: {other:?}"),
    }
}

#[test]
fn query_builder_keeps_query_owned_graph_obligations_with_explicit_backend() {
    let explicit_backend = WorthQueryBridgeBackedRuntimeBackend::from_parts(
        WorthQueryRuntimeBackendParts::new()
            .runtime_bridge(test_bridge())
            .schema_adapter(TestSchemaAdapter)
            .source_adapter(TestSourceAdapter::default())
            .write_authority(TestWriteAuthority)
            .snapshot_identity(TestSnapshotIdentityAdapter)
            .signal_sink(TestSignalSink)
            .subscription_activation(TestSubscriptionActivation)
            .preview_basis(TestPreviewBasis)
            .inspector_evidence(TestInspectorEvidence),
    )
    .expect("explicit backend should build for query-owned graph obligation test");

    let registration = WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap(),
        WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let runtime = WorthQueryRuntime::builder()
        .graph_obligation(registration)
        .backend(explicit_backend)
        .build()
        .expect("query-owned graph obligations should compose with an explicit backend");

    assert_eq!(
        runtime
            .graph_obligation_registration_catalog()
            .registration_count(),
        1
    );
}
