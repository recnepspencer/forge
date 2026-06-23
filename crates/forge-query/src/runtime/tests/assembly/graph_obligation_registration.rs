use super::super::support::*;
use forge_relational::facade::config::CrossContextPolicy;
use forge_relational::facade::identity::KindId;
use forge_relational::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};
use forge_relational::facade::schema::{
    ContractId, LoweredEndpointKindContract, RelationIntegrityPlanRevision,
};
use std::collections::BTreeSet;

#[test]
fn query_builder_exposes_graph_obligation_registration_catalog() {
    let registration = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap(),
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
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
    let selector = ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let registrations = vec![
        ForgeQueryGraphObligationRegistration::blocking_invariant(
            ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "blocking", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        ForgeQueryGraphObligationRegistration::schema_contract_validator(
            ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "schema", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        ForgeQueryGraphObligationRegistration::advisory_obligation(
            ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "advisory", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        ForgeQueryGraphObligationRegistration::preflight_sequencing_obligation(
            ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "preflight", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        ForgeQueryGraphObligationRegistration::capability_gap_screen(
            ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "capability", "v1")
                .unwrap(),
            selector.clone(),
            world,
        ),
        ForgeQueryGraphObligationRegistration::operating_context_gate(
            ForgeQueryGraphObligationRuleIdentity::new(
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
    let selector = ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();
    let left = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "schema", "v1")
            .unwrap(),
        selector.clone(),
        world,
    );
    let right = ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "blocking", "v1")
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
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let selector = ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let world = ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority();

    let error = match complete_backend_from_parts_builder()
        .graph_obligation(
            ForgeQueryGraphObligationRegistration::schema_contract_validator(
                rule_identity.clone(),
                selector.clone(),
                world,
            ),
        )
        .graph_obligation(ForgeQueryGraphObligationRegistration::blocking_invariant(
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
        ForgeQueryRuntimeError::InvariantRegistration { stage, message } => {
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
        .graph_obligation(ForgeQueryGraphObligationRegistration::blocking_invariant(
            ForgeQueryGraphObligationRuleIdentity::new(
                "relational-schema-contract",
                "endpoint-kind:loop-successor-endpoint-kind",
                "v1",
            )
            .unwrap(),
            ForgeQueryGraphTouchSelector::relational_kind_id(KindId(77)),
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ))
        .build_backend_from_parts()
        .build()
    {
        Ok(_) => panic!("query runtime should reject explicit conflict with auto-lowered schema"),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::InvariantRegistration { stage, message } => {
            assert_eq!(stage, "graph_obligation_registration_catalog_assembly");
            assert!(message.contains("conflicting registrations"));
        }
        other => panic!("unexpected runtime error: {other:?}"),
    }
}

#[test]
fn query_builder_rejects_explicit_backend_when_graph_obligations_are_queued() {
    let explicit_backend = ForgeQueryBridgeBackedRuntimeBackend::from_parts(
        ForgeQueryRuntimeBackendParts::new()
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
    .expect("explicit backend should build for graph obligation conflict test");

    let registration = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap(),
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let error = match ForgeQueryRuntime::builder()
        .graph_obligation(registration)
        .backend(explicit_backend)
        .build()
    {
        Ok(_) => panic!("explicit backend should reject queued graph obligations"),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::InvariantRegistration { stage, message } => {
            assert_eq!(stage, "runtime_backend_selection");
            assert!(message.contains("graph obligation registrations"));
            assert!(message.contains("backend(...)"));
        }
        other => panic!("unexpected runtime error: {other:?}"),
    }
}
