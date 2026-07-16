use std::any::type_name;
use worth_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, WorthQueryBoundaryAuditSourceSet,
    WorthQueryGraphObligationAdoptionManifest, WorthQueryGraphObligationAdoptionProof,
    WorthQueryGraphObligationConsumerKitErrorKind,
    WorthQueryGraphObligationConsumerRegistrationDeclaration,
    WorthQueryGraphObligationExecutionBackedAdoptionProof, WorthQueryGraphObligationExecutionProof,
    WorthQueryGraphObligationInMemoryProof, WorthQueryGraphObligationLocalCeremonyAudit,
    WorthQueryGraphObligationResidueManifest, WorthQueryGraphObligationResidueRow,
    WorthQueryGraphObligationSelectorCoverageDeclaration, WorthQueryGraphObligationSupportPin,
};
use worth_query::facade::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryGraphObligationExecutionResultEnvelope,
    WorthQueryGraphObligationExecutionStatus, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportMatrix, WorthQueryGraphObligationSupportPosture,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector,
    WorthQueryMutationFamily,
};

use crate::support;

use support::aspect_touch as touch;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthGraphAuthorityQueryContractRole {
    ConsumerKitEntry,
    AdoptionPosture,
    SelectorCoverage,
    SupportPinning,
    SelectedObligation,
    ExecutedObligation,
    ReceiptEnvelope,
    ResidueManifest,
    QueryCapabilityGap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthGraphAuthorityProofRequirement {
    ExecutionRequired,
    SelectionOnlyInsufficient,
    ExplicitQueryGapRow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthGraphAuthorityQueryContractSurface {
    role: WorthGraphAuthorityQueryContractRole,
    canonical_surface: &'static str,
    proof_requirement: WorthGraphAuthorityProofRequirement,
}

const WORTH_GRAPH_AUTHORITY_QUERY_CONTRACT: &[WorthGraphAuthorityQueryContractSurface] = &[
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::ConsumerKitEntry,
        canonical_surface: "worth_query::facade::consumer_kit::graph_obligation_consumer_kit",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::AdoptionPosture,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationAdoptionProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::AdoptionPosture,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationExecutionBackedAdoptionProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::SelectorCoverage,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationSelectorCoverageDeclaration",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::SupportPinning,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationSupportPin",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::SelectedObligation,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationInMemoryProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::ExecutedObligation,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationExecutionProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::ReceiptEnvelope,
        canonical_surface:
            "worth_query::facade::runtime::WorthQueryGraphObligationExecutionResultEnvelope",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::ResidueManifest,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationResidueManifest",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::QueryCapabilityGap,
        canonical_surface:
            "worth_query::facade::consumer_kit::WorthQueryGraphObligationResidueRow::explicit",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExplicitQueryGapRow,
    },
];

#[test]
fn worth_graph_authority_contract_names_canonical_query_surfaces() {
    let expected_roles = [
        WorthGraphAuthorityQueryContractRole::ConsumerKitEntry,
        WorthGraphAuthorityQueryContractRole::AdoptionPosture,
        WorthGraphAuthorityQueryContractRole::SelectorCoverage,
        WorthGraphAuthorityQueryContractRole::SupportPinning,
        WorthGraphAuthorityQueryContractRole::SelectedObligation,
        WorthGraphAuthorityQueryContractRole::ExecutedObligation,
        WorthGraphAuthorityQueryContractRole::ReceiptEnvelope,
        WorthGraphAuthorityQueryContractRole::ResidueManifest,
        WorthGraphAuthorityQueryContractRole::QueryCapabilityGap,
    ];
    for role in expected_roles {
        assert!(
            WORTH_GRAPH_AUTHORITY_QUERY_CONTRACT
                .iter()
                .any(|surface| surface.role == role),
            "missing Worth graph-authority Query contract role {role:?}"
        );
    }

    assert!(WORTH_GRAPH_AUTHORITY_QUERY_CONTRACT.iter().all(|surface| {
        surface
            .canonical_surface
            .starts_with("worth_query::facade::")
    }));
    assert_eq!(
        WORTH_GRAPH_AUTHORITY_QUERY_CONTRACT
            .iter()
            .filter(|surface| surface.proof_requirement
                == WorthGraphAuthorityProofRequirement::ExecutionRequired)
            .count(),
        4
    );

    let canonical_type_names = [
        type_name::<WorthQueryGraphObligationAdoptionManifest>(),
        type_name::<WorthQueryGraphObligationAdoptionProof>(),
        type_name::<WorthQueryGraphObligationExecutionBackedAdoptionProof>(),
        type_name::<WorthQueryGraphObligationConsumerRegistrationDeclaration>(),
        type_name::<WorthQueryGraphObligationSelectorCoverageDeclaration>(),
        type_name::<WorthQueryGraphObligationSupportPin>(),
        type_name::<WorthQueryGraphObligationInMemoryProof>(),
        type_name::<WorthQueryGraphObligationExecutionProof>(),
        type_name::<WorthQueryGraphObligationResidueManifest>(),
        type_name::<WorthQueryGraphObligationResidueRow>(),
        type_name::<WorthQueryGraphObligationExecutionResultEnvelope>(),
    ];
    for expected_type in [
        "WorthQueryGraphObligationAdoptionProof",
        "WorthQueryGraphObligationExecutionBackedAdoptionProof",
        "WorthQueryGraphObligationSelectorCoverageDeclaration",
        "WorthQueryGraphObligationSupportPin",
        "WorthQueryGraphObligationInMemoryProof",
        "WorthQueryGraphObligationExecutionProof",
        "WorthQueryGraphObligationResidueManifest",
        "WorthQueryGraphObligationExecutionResultEnvelope",
    ] {
        assert!(
            canonical_type_names
                .iter()
                .any(|type_name| type_name.ends_with(expected_type)),
            "canonical Query contract type `{expected_type}` is not named"
        );
    }
}

#[test]
fn graph_obligation_consumer_kit_is_available_from_public_facade() {
    let registration = WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new("worth.topo", "operator-scope", "1.0.0")
            .unwrap(),
        WorthQueryGraphTouchSelector::collection("worth_topo").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
    ));

    let proof = graph_obligation_consumer_kit("worth-topo")
        .register_obligations(
            WorthQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-operators",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            WorthQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator catalog read coverage",
                WorthQueryGraphTouchSelector::collection("worth_topo").unwrap(),
            )]),
        )
        .pin_support(WorthQueryGraphObligationSupportPin::supported([(
            WorthQueryGraphObligationKind::BlockingInvariant,
            WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
        )]))
        .audit_local_ceremony(WorthQueryGraphObligationLocalCeremonyAudit::evaluate(
            &WorthQueryBoundaryAuditSourceSet::new("worth-topo").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(WorthQueryGraphObligationResidueManifest::empty())
        .prove_in_memory_selection(
            &WorthQueryGraphTouchDescriptor::read_family(
                "worth_topo",
                [WorthQueryGraphTouchReadVerb::ObservesCollection],
            )
            .unwrap(),
            &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption()
        .unwrap();

    assert_eq!(proof.manifest().consumer_name(), "worth-topo");
}

#[test]
fn graph_obligation_consumer_kit_public_facade_proves_execution_backed_adoption() {
    let authority_budget =
        WorthQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface()
            .rows_for_lane(WorthQueryGraphObligationSupportLane::GraphComposition)
            .find(|row| row.obligation_kind() == WorthQueryGraphObligationKind::BlockingInvariant)
            .expect("blocking graph-composition row")
            .execution_budget()
            .clone();
    let registration = WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new(
            "worth.topo",
            "public-facade-operator-scope",
            "1.0.0",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ))
    .with_execution_budget(authority_budget.clone());

    let proof = graph_obligation_consumer_kit("worth-topo-public-facade")
        .register_obligations(
            WorthQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-public-operators",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            WorthQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator graph mutation coverage",
                WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
            )]),
        )
        .pin_support(WorthQueryGraphObligationSupportPin::supported_with_budget(
            [(
                WorthQueryGraphObligationKind::BlockingInvariant,
                WorthQueryGraphObligationSupportLane::GraphComposition,
                authority_budget,
            )],
        ))
        .against_support_matrix(
            WorthQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface(),
        )
        .audit_local_ceremony(WorthQueryGraphObligationLocalCeremonyAudit::evaluate(
            &WorthQueryBoundaryAuditSourceSet::new("worth-topo-public-facade").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(WorthQueryGraphObligationResidueManifest::empty())
        .prove_execution_with(
            &WorthQueryGraphTouchDescriptor::declared_mutation_collection(
                "topology.edge",
                WorthQueryMutationFamily::Update,
                None,
                [set_operation("capacity")],
                [touch("capacity")],
            )
            .unwrap(),
            &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption_with_execution()
        .unwrap();

    let execution_proof = proof.execution_proof();
    let execution_proof_digest = proof
        .manifest()
        .execution_proof_digest()
        .expect("manifest execution proof digest");

    assert_eq!(proof.manifest().consumer_name(), "worth-topo-public-facade");
    assert_eq!(execution_proof_digest, execution_proof.proof_digest());
    assert!(execution_proof.has_real_executor_rows());
    assert_eq!(
        execution_proof.execution_statuses(),
        vec![WorthQueryGraphObligationExecutionStatus::Executed]
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.residue_manifest().rows().len(), 0);
}

#[test]
fn selection_only_adoption_cannot_claim_execution_backed_closeout() {
    let registration = WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new(
            "worth.topo",
            "selection-only-is-not-execution",
            "1.0.0",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ));

    let error = graph_obligation_consumer_kit("worth-topo-selection-only")
        .register_obligations(
            WorthQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-selection-only",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            WorthQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator graph mutation coverage",
                WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
            )]),
        )
        .pin_support(WorthQueryGraphObligationSupportPin::supported([(
            WorthQueryGraphObligationKind::BlockingInvariant,
            WorthQueryGraphObligationSupportLane::GraphComposition,
        )]))
        .audit_local_ceremony(WorthQueryGraphObligationLocalCeremonyAudit::evaluate(
            &WorthQueryBoundaryAuditSourceSet::new("worth-topo-selection-only").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(WorthQueryGraphObligationResidueManifest::empty())
        .prove_in_memory_selection(
            &WorthQueryGraphTouchDescriptor::declared_mutation_collection(
                "topology.edge",
                WorthQueryMutationFamily::Update,
                None,
                [set_operation("capacity")],
                [touch("capacity")],
            )
            .unwrap(),
            &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption_with_execution()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        WorthQueryGraphObligationConsumerKitErrorKind::MissingInMemoryProof
    );
    assert!(error.message().contains("real execution proof"));
}

fn set_operation(authored_touch_text: &str) -> WorthQueryAspectMutationOperation {
    WorthQueryAspectMutationOperation::set(touch(authored_touch_text))
}
