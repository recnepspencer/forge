use worth_relational::facade::config::{CascadeDeletePolicy, CrossContextPolicy};
use worth_relational::facade::identity::KindId;
use worth_relational::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};
use worth_relational::facade::schema::{
    AllowedCycleClass, ConnectivityMinimumEnforcement, ContractId, DirectedTraversalKind,
    EndpointDeletionIntegrityMode, LoweredAcyclicityContract, LoweredCardinalityMaximumContract,
    LoweredCardinalityMinimumContract, LoweredConnectivityMinimumContract,
    LoweredEndpointDeletionIntegrityContract, LoweredEndpointKindContract,
    LoweredPartitionIsolationContract, LoweredSymmetryContract, LoweredUniquenessContract,
    MinimumCardinalityEnforcement, PairMinimumSemantics, PartitionIsolationMode,
    RelationIntegrityPlanRevision, SymmetryMode, UniquenessScope,
};

use crate::runtime::{
    registrations_from_relational_invariant_catalog, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistrationCatalog,
};

#[test]
fn relational_schema_contracts_lower_to_exact_schema_validator_registrations() {
    let cases = schema_contract_cases();
    let catalog = InvariantCatalog {
        registrations: cases
            .iter()
            .map(|case| registration_for_schema_rule(case.rule.clone()))
            .collect(),
    };

    let registrations = registrations_from_relational_invariant_catalog(&catalog).unwrap();
    let graph_catalog =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(registrations.clone())
            .unwrap();

    assert_eq!(registrations.len(), cases.len());
    assert_eq!(graph_catalog.registration_count(), cases.len());
    for case in cases {
        let registration = registrations
            .iter()
            .find(|registration| registration.rule_identity().name() == case.expected_rule_name)
            .unwrap_or_else(|| panic!("missing graph obligation for {}", case.expected_rule_name));

        assert_eq!(
            registration.kind(),
            WorthQueryGraphObligationKind::SchemaContractValidator
        );
        assert_eq!(
            registration.rule_identity().namespace(),
            "relational-schema-contract"
        );
        assert_eq!(registration.rule_identity().name(), case.expected_rule_name);
        assert_eq!(registration.rule_identity().semantic_version(), "v1");
        assert_eq!(
            registration
                .touch_selector()
                .terminal_selector_kind_for_boundary(),
            "relation-kind-id"
        );
        assert_eq!(
            registration
                .touch_selector()
                .terminal_selector_value_for_boundary()
                .as_deref(),
            Some(case.expected_relation_kind_id)
        );
        assert_eq!(
            registration.operating_world_selector(),
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority()
        );
    }
}

#[test]
fn non_schema_invariants_do_not_lower_to_graph_schema_contracts() {
    let catalog = InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(3),
        )],
    };

    let registrations = registrations_from_relational_invariant_catalog(&catalog).unwrap();

    assert!(registrations.is_empty());
}

fn registration_for_schema_rule(rule: InvariantRule) -> InvariantRegistration {
    if matches!(rule, InvariantRule::ConnectivityMinimumContract(_)) {
        InvariantRegistration::snapshot_publication_blocking(rule)
    } else {
        InvariantRegistration::commit_boundary_blocking(rule)
    }
}

#[derive(Clone)]
struct SchemaContractCase {
    rule: InvariantRule,
    expected_rule_name: &'static str,
    expected_relation_kind_id: &'static str,
}

fn schema_contract_cases() -> Vec<SchemaContractCase> {
    vec![
        SchemaContractCase {
            rule: InvariantRule::EndpointKindContract(LoweredEndpointKindContract {
                contract_id: ContractId::new("edge-endpoint-kind"),
                relation_kind_id: KindId(42),
                allowed_source_kinds: vec![KindId(1)],
                allowed_target_kinds: vec![KindId(2)],
                self_edges_allowed: false,
                cross_context_policy: CrossContextPolicy::SchemaControlled,
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "endpoint-kind:edge-endpoint-kind",
            expected_relation_kind_id: "42",
        },
        SchemaContractCase {
            rule: InvariantRule::CardinalityMaximumContract(LoweredCardinalityMaximumContract {
                contract_id: ContractId::new("edge-cardinality-maximum"),
                relation_kind_id: KindId(43),
                source_max: Some(1),
                target_max: Some(8),
                pair_max: Some(1),
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "cardinality-maximum:edge-cardinality-maximum",
            expected_relation_kind_id: "43",
        },
        SchemaContractCase {
            rule: InvariantRule::CardinalityMinimumContract(LoweredCardinalityMinimumContract {
                contract_id: ContractId::new("edge-cardinality-minimum"),
                relation_kind_id: KindId(44),
                source_min: Some(1),
                target_min: Some(1),
                pair_min: Some(1),
                pair_min_semantics: PairMinimumSemantics::ObservedDirectedPairs,
                candidate_source_kinds: vec![KindId(1)],
                candidate_target_kinds: vec![KindId(2)],
                minimum_enforcement: MinimumCardinalityEnforcement::CommitBoundary,
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "cardinality-minimum:edge-cardinality-minimum",
            expected_relation_kind_id: "44",
        },
        SchemaContractCase {
            rule: InvariantRule::UniquenessContract(LoweredUniquenessContract {
                contract_id: ContractId::new("edge-uniqueness"),
                relation_kind_id: KindId(45),
                scope: UniquenessScope::DirectedSemanticEdge,
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "uniqueness:edge-uniqueness",
            expected_relation_kind_id: "45",
        },
        SchemaContractCase {
            rule: InvariantRule::SymmetryContract(LoweredSymmetryContract {
                contract_id: ContractId::new("edge-symmetry"),
                relation_kind_id: KindId(46),
                mode: SymmetryMode::CanonicalUndirected,
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "symmetry:edge-symmetry",
            expected_relation_kind_id: "46",
        },
        SchemaContractCase {
            rule: InvariantRule::EndpointDeletionIntegrityContract(
                LoweredEndpointDeletionIntegrityContract {
                    contract_id: ContractId::new("edge-endpoint-delete"),
                    relation_kind_id: KindId(47),
                    mode: EndpointDeletionIntegrityMode::RejectDeleteWithLiveRelations,
                    cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
                    plan_revision: RelationIntegrityPlanRevision(7),
                },
            ),
            expected_rule_name: "endpoint-deletion-integrity:edge-endpoint-delete",
            expected_relation_kind_id: "47",
        },
        SchemaContractCase {
            rule: InvariantRule::AcyclicityContract(LoweredAcyclicityContract {
                contract_id: ContractId::new("edge-acyclicity"),
                relation_kind_id: KindId(48),
                traversal_direction: DirectedTraversalKind::SourceToTarget,
                allowed_cycle_class: AllowedCycleClass::NoCycles,
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "acyclicity:edge-acyclicity",
            expected_relation_kind_id: "48",
        },
        SchemaContractCase {
            rule: InvariantRule::PartitionIsolationContract(LoweredPartitionIsolationContract {
                contract_id: ContractId::new("edge-partition-isolation"),
                relation_kind_id: KindId(49),
                isolation_mode: PartitionIsolationMode::SamePartitionEndpoints,
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "partition-isolation:edge-partition-isolation",
            expected_relation_kind_id: "49",
        },
        SchemaContractCase {
            rule: InvariantRule::ConnectivityMinimumContract(LoweredConnectivityMinimumContract {
                contract_id: ContractId::new("edge-connectivity-minimum"),
                source_kind_ids: vec![KindId(1)],
                relation_kind_id: KindId(50),
                target_kind_ids: vec![KindId(2)],
                minimum_reachable_targets: 1,
                enforcement_boundary: ConnectivityMinimumEnforcement::SnapshotPublication,
                plan_revision: RelationIntegrityPlanRevision(7),
            }),
            expected_rule_name: "connectivity-minimum:edge-connectivity-minimum",
            expected_relation_kind_id: "50",
        },
    ]
}
