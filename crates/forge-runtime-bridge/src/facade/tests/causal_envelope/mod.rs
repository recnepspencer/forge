use super::*;
use crate::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenialKind,
    BridgeCausalEvidenceBinding, BridgeCausalEvidenceBindingClass, BridgeCausalEvidenceFamily,
    BridgeCausalEvidenceOwner, BridgeCausalEvidenceReference,
    BridgeCausalEvidenceReferenceIdentity, BridgeTruthViewEvaluationRequest,
};

mod admission_summary;
mod external_authority;
mod mapping;
mod mapping_scale;
mod mapping_support;
mod receipt;
mod retained_mapping;
mod retained_mapping_bulk;
mod retained_mapping_digest_support;
mod retained_mapping_edges;
mod retained_mapping_stream_history;
mod retained_mapping_support;

use retained_mapping_digest_support::{
    expected_retained_causal_digest, ExpectedRetainedCausalDigestArtifact,
};

fn bridge_reference(
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    let family = identity.family();
    BridgeCausalEvidenceReference::new(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
        .expect("bridge reference should be valid")
}

fn external_reference(
    owner: BridgeCausalEvidenceOwner,
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    let family = identity.family();
    BridgeCausalEvidenceReference::new(owner, family, identity)
        .expect("external reference should be valid")
}

fn query_observation_reference(
    identity: BridgeCausalEvidenceReferenceIdentity,
) -> BridgeCausalEvidenceReference {
    external_reference(BridgeCausalEvidenceOwner::Query, identity)
}

fn missing_bridge_reference(
    family: BridgeCausalEvidenceFamily,
    identity: &str,
) -> BridgeCausalEvidenceReference {
    bridge_reference(
        BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
            family,
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(identity),
        )
        .expect("bridge reference identity should be valid"),
    )
}

fn bridge_route_reference(
    route_summary: &crate::routing::BridgeRouteResultSummary,
) -> BridgeCausalEvidenceReference {
    missing_bridge_reference(
        BridgeCausalEvidenceFamily::BridgeRoute,
        route_summary.route_identity().as_str(),
    )
}

fn bridge_historical_evaluation_reference(
    record: &crate::diagnostics::BridgeCanonicalHistoricalEvaluationRecord,
) -> BridgeCausalEvidenceReference {
    missing_bridge_reference(
        BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation,
        record.record_identity().as_str(),
    )
}

fn binding_for<'a>(
    bindings: &'a [BridgeCausalEvidenceBinding],
    owner: BridgeCausalEvidenceOwner,
    family: BridgeCausalEvidenceFamily,
    reference_identity: &str,
) -> &'a BridgeCausalEvidenceBinding {
    bindings
        .iter()
        .find(|binding| {
            binding.owner() == owner
                && binding.family() == family
                && binding.reference_evidence_identity().as_str() == reference_identity
        })
        .expect("expected causal evidence binding should be present")
}

fn expected_retained_route_digest(
    route_identity: &str,
    invalidation_identity: &str,
    source_commit: &str,
) -> String {
    expected_retained_causal_digest(
        ExpectedRetainedCausalDigestArtifact::RouteRecord,
        &[route_identity, invalidation_identity, source_commit],
    )
}

fn expected_retained_historical_digest(
    record_identity: &str,
    decision_log_identity: &str,
    snapshot_identity: &str,
) -> String {
    expected_retained_causal_digest(
        ExpectedRetainedCausalDigestArtifact::HistoricalEvaluationRecord,
        &[record_identity, decision_log_identity, snapshot_identity],
    )
}

#[test]
fn causal_envelope_denies_missing_retained_bridge_record_without_unindexed_scan() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "query-admission:missing",
            ),
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "causal-anchor:missing",
            ),
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                        "query-observation:missing-route",
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            missing_bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                "missing-route-identity",
            ),
        ],
    )
    .expect("request should be valid");

    let denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect_err("missing retained route should deny");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRetainedBridgeRecord
    );
    assert_eq!(denial.family(), BridgeCausalEvidenceFamily::BridgeRoute);
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 1);
    assert_eq!(denial.counters().missing_bridge_record_count(), 1);
    assert_eq!(denial.counters().bridge_record_unindexed_scan_count(), 0);
}

#[test]
fn causal_envelope_missing_historical_record_preserves_prior_lookup_counters() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route(crate::truth_identity_fixtures::truth_commit_fixture(
            "commit-causal-before-missing-history",
        ))
        .expect("route should succeed");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "query-admission:missing-history",
            ),
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "causal-anchor:missing-history",
            ),
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                        "query-observation:missing-history",
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            bridge_route_reference(routed.result().result_summary()),
            missing_bridge_reference(
                BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation,
                "missing-historical-record",
            ),
        ],
    )
    .expect("request should be valid");

    let denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect_err("missing historical record should deny after route lookup");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRetainedBridgeRecord
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::BridgeHistoricalEvaluation
    );
    assert_eq!(denial.counters().evidence_reference_count(), 3);
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(denial.counters().retained_bridge_binding_count(), 1);
    assert_eq!(denial.counters().missing_bridge_record_count(), 1);
    assert_eq!(denial.counters().bridge_record_unindexed_scan_count(), 0);
}

#[test]
fn causal_envelope_denies_external_authority_without_bridge_route_evidence() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "query-admission:external-only",
            ),
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "causal-anchor:external-only",
            ),
        )
        .expect("query admission summary should be valid"),
        vec![
            external_reference(
                BridgeCausalEvidenceOwner::Query,
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                        "query-observation:external-only",
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceReferenceIdentity::signal(
                    BridgeCausalEvidenceFamily::SignalInvalidation,
                    crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                        "signal-invalidation:external-only",
                    ),
                )
                .expect("signal reference identity should be valid"),
            ),
        ],
    )
    .expect("request should be valid before bridge authority assembly");

    let denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect_err("external evidence alone must not mint a bridge envelope");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRequiredBridgeRouteEvidence
    );
    assert_eq!(denial.family(), BridgeCausalEvidenceFamily::BridgeRoute);
    assert_eq!(denial.counters().evidence_reference_count(), 2);
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 0);
    assert_eq!(denial.counters().external_authority_reference_count(), 2);
    assert_eq!(denial.counters().bridge_record_unindexed_scan_count(), 0);
}

#[test]
fn causal_envelope_request_denies_missing_query_observation_anchor() {
    let denial = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "query-admission:missing-query-anchor",
            ),
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "causal-anchor:missing-query-anchor",
            ),
        )
        .expect("query admission summary should be valid"),
        vec![missing_bridge_reference(
            BridgeCausalEvidenceFamily::BridgeRoute,
            "route:missing-query-anchor",
        )],
    )
    .expect_err("bridge assembly request must carry a query observation anchor");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingQueryObservationAnchor
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::QueryObservation
    );
    assert_eq!(denial.supplied_owner(), BridgeCausalEvidenceOwner::Query);
    assert_eq!(denial.expected_owner(), BridgeCausalEvidenceOwner::Query);
    assert_eq!(denial.counters().bridge_record_unindexed_scan_count(), 0);
}

#[test]
fn causal_envelope_request_denies_multiple_query_observation_anchors() {
    let denial = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "query-admission:query-anchor-overclaim",
            ),
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "causal-anchor:query-anchor-overclaim",
            ),
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                        "query-observation:primary",
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            query_observation_reference(
                BridgeCausalEvidenceReferenceIdentity::query_observation(
                    crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                        "query-observation:overclaim",
                    ),
                )
                .expect("query observation reference identity should be valid"),
            ),
            missing_bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                "route:query-anchor-overclaim",
            ),
        ],
    )
    .expect_err("bridge assembly request must bind exactly one query observation anchor");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::QueryObservationAnchorOverclaim
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::QueryObservation
    );
    assert!(denial
        .reference_identity_for_reporting()
        .starts_with("forge.runtime.bridge.causal-envelope-identity.v1:"));
    assert_ne!(
        denial.reference_identity_for_reporting(),
        "query-observation-anchor-count:2"
    );
    assert_ne!(
        denial.reference_evidence_identity().as_str(),
        "query-observation-anchor-count:2"
    );
    assert_eq!(denial.counters().bridge_record_unindexed_scan_count(), 0);
}

#[test]
fn causal_reference_denies_owner_mismatch_before_envelope_assembly() {
    let denial = BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Signal,
        BridgeCausalEvidenceFamily::BridgeRoute,
        BridgeCausalEvidenceReferenceIdentity::runtime_bridge(
            BridgeCausalEvidenceFamily::BridgeRoute,
            crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                "route-owned-by-bridge",
            ),
        )
        .expect("bridge reference identity should be valid"),
    )
    .expect_err("owner mismatch should deny");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::EvidenceOwnerMismatch
    );
    assert_eq!(denial.supplied_owner(), BridgeCausalEvidenceOwner::Signal);
    assert_eq!(
        denial.expected_owner(),
        BridgeCausalEvidenceOwner::RuntimeBridge
    );
}

#[test]
fn causal_envelope_lookup_cost_ignores_unrelated_retained_routes() {
    for unrelated_routes in [0, 4, 12] {
        let runtime = runtime(BridgeRuntimePolicy::default());
        for index in 0..unrelated_routes {
            runtime
                .route(crate::truth_identity_fixtures::truth_commit_fixture(
                    format!("unrelated-causal-{index}"),
                ))
                .expect("unrelated route should succeed");
        }
        let routed = runtime
            .route(crate::truth_identity_fixtures::truth_commit_fixture(
                "commit-causal-target",
            ))
            .expect("target route should succeed");
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                    "query-admission:scale",
                ),
                crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                    "causal-anchor:scale",
                ),
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference(
                    BridgeCausalEvidenceReferenceIdentity::query_observation(
                        crate::facade::BridgeIdentityEvidence::from_bridge_owner_external_authority(
                            "query-observation:scale",
                        ),
                    )
                    .expect("query observation reference identity should be valid"),
                ),
                bridge_route_reference(routed.result().result_summary()),
            ],
        )
        .expect("request should be valid");

        let envelope = runtime
            .diagnostics()
            .assemble_causal_explanation_envelope(request)
            .expect("target route should bind");

        assert_eq!(
            runtime.diagnostics().route_records().len(),
            unrelated_routes + 1
        );
        assert_eq!(envelope.counters().bridge_retained_lookup_count(), 1);
        assert_eq!(envelope.counters().retained_bridge_binding_count(), 1);
        assert_eq!(envelope.counters().bridge_record_unindexed_scan_count(), 0);
    }
}
