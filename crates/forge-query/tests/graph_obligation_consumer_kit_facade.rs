use forge_query::facade::consumer_kit::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationAdoptionManifest, ForgeQueryGraphObligationAdoptionProof,
    ForgeQueryGraphObligationConsumerKitErrorKind,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof, ForgeQueryGraphObligationExecutionProof,
    ForgeQueryGraphObligationInMemoryProof, ForgeQueryGraphObligationLocalCeremonyAudit,
    ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationResidueRow,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};
use forge_query::facade::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryGraphObligationExecutionResultEnvelope,
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector,
    ForgeQueryMutationFamily,
};
use std::any::type_name;

mod support;

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
        canonical_surface: "forge_query::facade::consumer_kit::graph_obligation_consumer_kit",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::AdoptionPosture,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationAdoptionProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::AdoptionPosture,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionBackedAdoptionProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::SelectorCoverage,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationSelectorCoverageDeclaration",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::SupportPinning,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationSupportPin",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::SelectedObligation,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationInMemoryProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::ExecutedObligation,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionProof",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::ReceiptEnvelope,
        canonical_surface:
            "forge_query::facade::runtime::ForgeQueryGraphObligationExecutionResultEnvelope",
        proof_requirement: WorthGraphAuthorityProofRequirement::ExecutionRequired,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::ResidueManifest,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationResidueManifest",
        proof_requirement: WorthGraphAuthorityProofRequirement::SelectionOnlyInsufficient,
    },
    WorthGraphAuthorityQueryContractSurface {
        role: WorthGraphAuthorityQueryContractRole::QueryCapabilityGap,
        canonical_surface:
            "forge_query::facade::consumer_kit::ForgeQueryGraphObligationResidueRow::explicit",
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
            .starts_with("forge_query::facade::")
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
        type_name::<ForgeQueryGraphObligationAdoptionManifest>(),
        type_name::<ForgeQueryGraphObligationAdoptionProof>(),
        type_name::<ForgeQueryGraphObligationExecutionBackedAdoptionProof>(),
        type_name::<ForgeQueryGraphObligationConsumerRegistrationDeclaration>(),
        type_name::<ForgeQueryGraphObligationSelectorCoverageDeclaration>(),
        type_name::<ForgeQueryGraphObligationSupportPin>(),
        type_name::<ForgeQueryGraphObligationInMemoryProof>(),
        type_name::<ForgeQueryGraphObligationExecutionProof>(),
        type_name::<ForgeQueryGraphObligationResidueManifest>(),
        type_name::<ForgeQueryGraphObligationResidueRow>(),
        type_name::<ForgeQueryGraphObligationExecutionResultEnvelope>(),
    ];
    for expected_type in [
        "ForgeQueryGraphObligationAdoptionProof",
        "ForgeQueryGraphObligationExecutionBackedAdoptionProof",
        "ForgeQueryGraphObligationSelectorCoverageDeclaration",
        "ForgeQueryGraphObligationSupportPin",
        "ForgeQueryGraphObligationInMemoryProof",
        "ForgeQueryGraphObligationExecutionProof",
        "ForgeQueryGraphObligationResidueManifest",
        "ForgeQueryGraphObligationExecutionResultEnvelope",
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
    let registration = ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new("worth.topo", "operator-scope", "1.0.0")
            .unwrap(),
        ForgeQueryGraphTouchSelector::collection("worth_topo").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
    ));

    let proof = graph_obligation_consumer_kit("worth-topo")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-operators",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator catalog read coverage",
                ForgeQueryGraphTouchSelector::collection("worth_topo").unwrap(),
            )]),
        )
        .pin_support(ForgeQueryGraphObligationSupportPin::supported([(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
        )]))
        .audit_local_ceremony(ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
            &ForgeQueryBoundaryAuditSourceSet::new("worth-topo").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_in_memory_selection(
            &ForgeQueryGraphTouchDescriptor::read_family(
                "worth_topo",
                [ForgeQueryGraphTouchReadVerb::ObservesCollection],
            )
            .unwrap(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption()
        .unwrap();

    assert_eq!(proof.manifest().consumer_name(), "worth-topo");
}

#[test]
fn graph_obligation_consumer_kit_public_facade_proves_execution_backed_adoption() {
    let authority_budget =
        ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface()
            .rows_for_lane(ForgeQueryGraphObligationSupportLane::GraphComposition)
            .find(|row| row.obligation_kind() == ForgeQueryGraphObligationKind::BlockingInvariant)
            .expect("blocking graph-composition row")
            .execution_budget()
            .clone();
    let registration = ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new(
            "worth.topo",
            "public-facade-operator-scope",
            "1.0.0",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    ))
    .with_execution_budget(authority_budget.clone());

    let proof = graph_obligation_consumer_kit("worth-topo-public-facade")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-public-operators",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator graph mutation coverage",
                ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
            )]),
        )
        .pin_support(ForgeQueryGraphObligationSupportPin::supported_with_budget(
            [(
                ForgeQueryGraphObligationKind::BlockingInvariant,
                ForgeQueryGraphObligationSupportLane::GraphComposition,
                authority_budget,
            )],
        ))
        .against_support_matrix(
            ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface(),
        )
        .audit_local_ceremony(ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
            &ForgeQueryBoundaryAuditSourceSet::new("worth-topo-public-facade").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_execution_with(
            &ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
                "topology.edge",
                ForgeQueryMutationFamily::Update,
                None,
                [set_operation("capacity")],
                [touch("capacity")],
            )
            .unwrap(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
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
        vec![ForgeQueryGraphObligationExecutionStatus::Executed]
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.residue_manifest().rows().len(), 0);
}

#[test]
fn selection_only_adoption_cannot_claim_execution_backed_closeout() {
    let registration = ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new(
            "worth.topo",
            "selection-only-is-not-execution",
            "1.0.0",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    ));

    let error = graph_obligation_consumer_kit("worth-topo-selection-only")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-topo-selection-only",
                [registration],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "operator graph mutation coverage",
                ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
            )]),
        )
        .pin_support(ForgeQueryGraphObligationSupportPin::supported([(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            ForgeQueryGraphObligationSupportLane::GraphComposition,
        )]))
        .audit_local_ceremony(ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
            &ForgeQueryBoundaryAuditSourceSet::new("worth-topo-selection-only").source_file(
                "operator-consumer.rs",
                "crates/worth-topo/src/operator_consumer.rs",
                "pub fn operator_consumer_uses_graph_obligation_kit() {}",
            ),
        ))
        .account_for_residue(ForgeQueryGraphObligationResidueManifest::empty())
        .prove_in_memory_selection(
            &ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
                "topology.edge",
                ForgeQueryMutationFamily::Update,
                None,
                [set_operation("capacity")],
                [touch("capacity")],
            )
            .unwrap(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )
        .unwrap()
        .prove_adoption_with_execution()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::MissingInMemoryProof
    );
    assert!(error.message().contains("real execution proof"));
}

fn set_operation(authored_touch_text: &str) -> ForgeQueryAspectMutationOperation {
    ForgeQueryAspectMutationOperation::set(touch(authored_touch_text))
}
