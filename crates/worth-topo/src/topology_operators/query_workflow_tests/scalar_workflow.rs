use forge_query::facade::{ForgeQueryOrdinaryOutcome, ForgeQueryRecoveryAction};

use crate::topology_operators::{
    topology_operator_continuation_target, topology_operator_signal_workflow,
    TopologyCreateTopologyEntityDeclaration, TopologyOperatorWorkflowHandleExt,
};

use super::current_head_handle::current_head_handle;

fn assert_linked_artifacts_preserve_query_lineage(
    declaration_digest: Option<&str>,
    route_plan_digest: Option<&str>,
    receipt_digest: Option<&str>,
    envelope_digest: Option<&str>,
) -> String {
    assert!(declaration_digest.is_some());
    assert!(route_plan_digest.is_some());
    assert!(receipt_digest.is_some());
    let envelope_digest = envelope_digest
        .expect("workflow proof should preserve envelope digest")
        .to_string();
    assert!(!envelope_digest.is_empty());
    envelope_digest
}

fn assert_check_support_recovery_action(brief: forge_query::facade::ForgeQueryRecoveryBrief) {
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::CheckSupport
    );
}

#[test]
fn scalar_operator_lane_uses_query_declaration_outcome_and_recovery() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.scalar",
        schema::facade::platform::entities::TopologyEntityKind::Face,
    );

    let canonical = handle
        .declare_topology_operator(declaration.clone())
        .expect("scalar topology declaration should canonicalize");
    assert_eq!(
        canonical.declaration_family_key(),
        "topology.create_topology_entity"
    );
    handle
        .review_topology_operator(canonical)
        .expect("scalar topology declaration should be legal");

    let outcome = handle.orchestrate_topology_operator_outcome(declaration);
    assert!(handle.recover_topology_operator_outcome(&outcome).is_none());
    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            assert_eq!(
                envelope.declaration_family_key(),
                "topology.create_topology_entity"
            );
        }
        _ => panic!("expected bound scalar operator outcome"),
    }
}

#[test]
fn scalar_operator_lane_exposes_topology_named_envelope_and_recovery() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.envelope",
        schema::facade::platform::entities::TopologyEntityKind::Region,
    );

    let envelope = handle
        .orchestrate_topology_operator_envelope(declaration.clone())
        .unwrap_or_else(|_| panic!("scalar topology declaration should envelope"));
    assert_eq!(
        envelope.declaration_family_key(),
        "topology.create_topology_entity"
    );

    assert!(handle
        .recover_topology_operator_envelope_checked(
            handle.orchestrate_topology_operator_envelope_checked(declaration.clone()),
        )
        .is_none());
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration.clone());
    match checked {
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(
            checked_envelope,
        ) => {
            assert_eq!(
                envelope.envelope_digest(),
                checked_envelope.envelope_digest()
            );
        }
        _ => panic!("expected checked topology envelope lane to stay enveloped"),
    }

    let proof = handle.orchestrate_topology_operator_envelope_proof(declaration);
    assert!(handle
        .recover_topology_operator_envelope_proof(&proof)
        .is_none());
    match proof.outcome() {
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(
            proof_envelope,
        ) => {
            assert_eq!(envelope.envelope_digest(), proof_envelope.envelope_digest());
        }
        _ => panic!("expected proof topology envelope lane to stay enveloped"),
    }
}

#[test]
fn scalar_operator_signal_lane_keeps_unsupported_posture_typed_and_recoverable() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.signal",
        schema::facade::platform::entities::TopologyEntityKind::Region,
    );

    let checked = handle.orchestrate_topology_operator_signal_compatibility_checked(
        topology_operator_signal_workflow(
            handle
                .orchestrate_topology_operator_envelope(declaration.clone())
                .unwrap_or_else(|_| panic!("scalar topology declaration should envelope")),
        ),
    );
    let checked_envelope_digest = assert_linked_artifacts_preserve_query_lineage(
        checked.linked_artifacts().declaration_digest(),
        checked.linked_artifacts().route_plan_digest(),
        checked.linked_artifacts().receipt_digest(),
        checked.linked_artifacts().envelope_digest(),
    );
    match checked.outcome() {
        forge_query::facade::ForgeQuerySignalCompatibilityOrchestrationOutcome::Unsupported(
            reason,
        ) => {
            assert!(!reason.is_empty());
        }
        other => panic!(
            "topology signal lane should stay typed unsupported for topology families, got {:?}",
            std::mem::discriminant(other)
        ),
    }

    let proof = handle.orchestrate_topology_operator_signal_compatibility_proof(
        topology_operator_signal_workflow(
            handle
                .orchestrate_topology_operator_envelope(declaration)
                .unwrap_or_else(|_| panic!("scalar topology declaration should envelope")),
        ),
    );
    assert_linked_artifacts_preserve_query_lineage(
        proof.linked_artifacts().declaration_digest(),
        proof.linked_artifacts().route_plan_digest(),
        proof.linked_artifacts().receipt_digest(),
        proof.linked_artifacts().envelope_digest(),
    );
    assert_eq!(
        proof.linked_artifacts().envelope_digest(),
        Some(checked_envelope_digest.as_str())
    );
    let brief = handle
        .recover_topology_operator_signal_compatibility_proof(proof)
        .expect("topology signal denial proof should recover");
    assert_check_support_recovery_action(brief);
}

#[test]
fn scalar_operator_lane_retains_progression_and_receipt_truth() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.progressed",
        schema::facade::platform::entities::TopologyEntityKind::Shell,
    );

    let progressed = handle
        .declare_review_and_progress_topology_operator(declaration)
        .unwrap_or_else(|_| panic!("scalar topology declaration should progress"));
    assert_eq!(
        progressed.declaration_family_key(),
        "topology.create_topology_entity"
    );
    assert!(!progressed.progression_digest().is_empty());

    let receipt = match handle.orchestrate_topology_operator_receipt_checked(progressed.clone()) {
        forge_query::facade::ForgeQueryDeclarationReceiptChecked::Issued(receipt) => receipt,
        _ => panic!("progressed scalar topology declaration should issue a receipt"),
    };

    assert_eq!(
        receipt.declaration_family_key(),
        "topology.create_topology_entity"
    );
    assert_eq!(
        receipt.progression_digest(),
        Some(progressed.progression_digest())
    );
    assert!(receipt.receipt_digest().metadata().entry_count() > 0);
    assert!(handle
        .recover_topology_operator_receipt_checked(
            forge_query::facade::ForgeQueryDeclarationReceiptChecked::Issued(receipt)
        )
        .is_none());
}

#[test]
fn scalar_operator_lane_exposes_topology_named_receipt_and_proof() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.receipt",
        schema::facade::platform::entities::TopologyEntityKind::Shell,
    );

    let progressed = handle
        .declare_review_and_progress_topology_operator(declaration)
        .unwrap_or_else(|_| panic!("scalar topology declaration should progress"));
    let receipt = handle
        .orchestrate_topology_operator_receipt(progressed.clone())
        .unwrap_or_else(|_| panic!("progressed scalar topology declaration should issue receipt"));
    assert_eq!(
        receipt.declaration_family_key(),
        "topology.create_topology_entity"
    );
    assert_eq!(
        receipt.progression_digest(),
        Some(progressed.progression_digest())
    );

    let proof = handle.orchestrate_topology_operator_receipt_proof(progressed);
    match proof.outcome() {
        forge_query::facade::ForgeQueryDeclarationReceiptChecked::Issued(proof_receipt) => {
            assert_eq!(proof_receipt.receipt_digest(), receipt.receipt_digest());
            assert_eq!(
                proof_receipt.progression_digest(),
                receipt.progression_digest()
            );
        }
        _ => panic!("expected progressed receipt proof lane to stay issued"),
    }
}

#[test]
fn scalar_operator_lane_retains_route_truth_before_receipt() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.route",
        schema::facade::platform::entities::TopologyEntityKind::Wire,
    );

    let progressed = handle
        .declare_review_and_progress_topology_operator(declaration)
        .unwrap_or_else(|_| panic!("scalar topology declaration should progress"));
    let route = handle
        .orchestrate_topology_operator_route(progressed.clone())
        .unwrap_or_else(|_| panic!("scalar topology declaration should produce a route plan"));
    assert_eq!(
        route.declaration_family_key(),
        "topology.create_topology_entity"
    );
    assert_eq!(route.progression_digest(), progressed.progression_digest());
    assert!(!route.route_plan_digest().is_empty());

    let checked = handle.orchestrate_topology_operator_route_checked(progressed.clone());
    match checked {
        forge_query::facade::ForgeQueryDeclarationRoutePlanChecked::Planned(checked_route) => {
            assert_eq!(route.route_plan_digest(), checked_route.route_plan_digest());
        }
        _ => panic!("expected checked topology route lane to stay planned"),
    }

    let proof = handle.orchestrate_topology_operator_route_proof(progressed);
    assert_eq!(
        proof.plan().product(),
        forge_query::facade::ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan
    );
    match proof.outcome() {
        forge_query::facade::ForgeQueryDeclarationRoutePlanChecked::Planned(proof_route) => {
            assert_eq!(route.route_plan_digest(), proof_route.route_plan_digest());
        }
        _ => panic!("expected proof topology route lane to stay planned"),
    }
}

#[test]
fn scalar_operator_lane_exposes_envelope_from_progressed_and_proof() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.progressed-envelope",
        schema::facade::platform::entities::TopologyEntityKind::Wire,
    );

    let progressed = handle
        .declare_review_and_progress_topology_operator(declaration)
        .unwrap_or_else(|_| panic!("scalar topology declaration should progress"));
    let envelope = handle
        .orchestrate_topology_operator_envelope_from_progressed(progressed.clone())
        .unwrap_or_else(|_| panic!("progressed scalar topology declaration should envelope"));
    assert_eq!(
        envelope.declaration_family_key(),
        "topology.create_topology_entity"
    );
    assert_eq!(
        envelope.progression_digest(),
        Some(progressed.progression_digest())
    );

    let checked =
        handle.orchestrate_topology_operator_envelope_from_progressed_checked(progressed.clone());
    match checked {
        forge_query::facade::ForgeQueryDeclarationEnvelopeChecked::Enveloped(checked_envelope) => {
            assert_eq!(
                checked_envelope.envelope_digest(),
                envelope.envelope_digest()
            );
        }
        _ => panic!("expected progressed envelope lane to stay enveloped"),
    }

    let proof = handle.orchestrate_topology_operator_envelope_from_progressed_proof(progressed);
    match proof.outcome() {
        forge_query::facade::ForgeQueryDeclarationEnvelopeChecked::Enveloped(proof_envelope) => {
            assert_eq!(proof_envelope.envelope_digest(), envelope.envelope_digest());
            assert_eq!(
                proof_envelope.progression_digest(),
                envelope.progression_digest()
            );
        }
        _ => panic!("expected progressed envelope proof lane to stay enveloped"),
    }
}

#[test]
fn scalar_operator_continuation_lane_keeps_unsupported_posture_typed_and_linked() {
    let handle = current_head_handle();
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "phase3.operator.continuation",
        schema::facade::platform::entities::TopologyEntityKind::Vertex,
    );

    let checked = handle.prepare_topology_operator_continuation_checked(
        topology_operator_continuation_target(
            handle
                .orchestrate_topology_operator_envelope(declaration.clone())
                .unwrap_or_else(|_| panic!("scalar topology declaration should envelope")),
        ),
    );
    let checked_envelope_digest = assert_linked_artifacts_preserve_query_lineage(
        checked.linked_artifacts().declaration_digest(),
        checked.linked_artifacts().route_plan_digest(),
        checked.linked_artifacts().receipt_digest(),
        checked.linked_artifacts().envelope_digest(),
    );
    match checked.outcome() {
        forge_query::facade::ForgeQueryPreparedContinuationOutcome::Unsupported(reason) => {
            assert!(
                reason.contains("no continuation contract exists")
                    || reason.contains("bridge continuation contract")
            );
        }
        _ => panic!("topology continuation preparation should stay unsupported"),
    }

    let outcome = handle.prepare_topology_operator_continuation_outcome(
        topology_operator_continuation_target(
            handle
                .orchestrate_topology_operator_envelope(declaration.clone())
                .unwrap_or_else(|_| panic!("scalar topology declaration should envelope")),
        ),
    );
    match outcome {
        ForgeQueryOrdinaryOutcome::Unsupported(_) => {}
        _ => panic!("topology continuation ordinary outcome should stay unsupported"),
    }

    let proof =
        handle.prepare_topology_operator_continuation_proof(topology_operator_continuation_target(
            handle
                .orchestrate_topology_operator_envelope(declaration)
                .unwrap_or_else(|_| panic!("scalar topology declaration should envelope")),
        ));
    assert_linked_artifacts_preserve_query_lineage(
        proof.linked_artifacts().declaration_digest(),
        proof.linked_artifacts().route_plan_digest(),
        proof.linked_artifacts().receipt_digest(),
        proof.linked_artifacts().envelope_digest(),
    );
    assert_eq!(
        proof.linked_artifacts().envelope_digest(),
        Some(checked_envelope_digest.as_str())
    );
    let brief = handle
        .recover_topology_operator_prepared_continuation_proof(proof)
        .expect("topology continuation denial proof should recover");
    assert_check_support_recovery_action(brief);
}
