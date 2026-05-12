use super::*;
use crate::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenialKind,
    BridgeCausalEvidenceBinding, BridgeCausalEvidenceBindingClass, BridgeCausalEvidenceFamily,
    BridgeCausalEvidenceOwner, BridgeCausalEvidenceReference, BridgeTruthViewEvaluationRequest,
};

mod admission_summary;
mod external_authority;
mod mapping;
mod receipt;
mod retained_mapping;
mod retained_mapping_bulk;
mod retained_mapping_edges;
mod retained_mapping_stream_history;
mod retained_mapping_support;

fn bridge_reference(
    family: BridgeCausalEvidenceFamily,
    identity: &str,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
        .expect("bridge reference should be valid")
}

fn external_reference(
    owner: BridgeCausalEvidenceOwner,
    family: BridgeCausalEvidenceFamily,
    identity: &str,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(owner, family, identity)
        .expect("external reference should be valid")
}

fn query_observation_reference(identity: &str) -> BridgeCausalEvidenceReference {
    external_reference(
        BridgeCausalEvidenceOwner::Query,
        BridgeCausalEvidenceFamily::QueryObservation,
        identity,
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
                && binding.reference_identity() == reference_identity
        })
        .expect("expected causal evidence binding should be present")
}

fn expected_retained_route_digest(
    route_identity: &str,
    invalidation_identity: &str,
    source_commit: &str,
    planning_summary_digest: &str,
    lowering_summary_digest: &str,
) -> String {
    digest(
        "bridge-causal-retained-route-record",
        &[
            route_identity,
            invalidation_identity,
            source_commit,
            planning_summary_digest,
            lowering_summary_digest,
        ],
    )
}

fn expected_retained_historical_digest(
    record_identity: &str,
    decision_log_identity: &str,
    snapshot_identity: &str,
) -> String {
    digest(
        "bridge-causal-retained-historical-record",
        &[record_identity, decision_log_identity, snapshot_identity],
    )
}

fn digest(label: &str, parts: &[&str]) -> String {
    use sha2::{Digest, Sha256};

    let mut canonical = String::from(label);
    for part in parts {
        canonical.push('|');
        canonical.push_str(part);
    }
    let digest = Sha256::digest(canonical.as_bytes());
    format!("{label}:sha256:{digest:x}")
}

#[test]
fn causal_envelope_denies_missing_retained_bridge_record_without_scan_fallback() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing",
            "causal-anchor:missing",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:missing-route"),
            bridge_reference(
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
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_missing_historical_record_preserves_prior_lookup_counters() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route("commit-causal-before-missing-history")
        .expect("route should succeed");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing-history",
            "causal-anchor:missing-history",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:missing-history"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
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
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_denies_external_authority_without_bridge_route_evidence() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:external-only",
            "causal-anchor:external-only",
        )
        .expect("query admission summary should be valid"),
        vec![
            external_reference(
                BridgeCausalEvidenceOwner::Query,
                BridgeCausalEvidenceFamily::QueryObservation,
                "query-observation:external-only",
            ),
            external_reference(
                BridgeCausalEvidenceOwner::Signal,
                BridgeCausalEvidenceFamily::SignalInvalidation,
                "signal-invalidation:external-only",
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
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_request_denies_missing_query_observation_anchor() {
    let denial = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing-query-anchor",
            "causal-anchor:missing-query-anchor",
        )
        .expect("query admission summary should be valid"),
        vec![bridge_reference(
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
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_request_denies_multiple_query_observation_anchors() {
    let denial = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:query-anchor-overclaim",
            "causal-anchor:query-anchor-overclaim",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:primary"),
            query_observation_reference("query-observation:overclaim"),
            bridge_reference(
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
    assert_eq!(
        denial.reference_identity(),
        "query-observation-anchor-count:2"
    );
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_reference_denies_owner_mismatch_before_envelope_assembly() {
    let denial = BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Signal,
        BridgeCausalEvidenceFamily::BridgeRoute,
        "route-owned-by-bridge",
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
                .route(format!("unrelated-causal-{index}"))
                .expect("unrelated route should succeed");
        }
        let routed = runtime
            .route("commit-causal-target")
            .expect("target route should succeed");
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                "query-admission:scale",
                "causal-anchor:scale",
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference("query-observation:scale"),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.result().result_summary().route_identity().as_str(),
                ),
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
        assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
    }
}
