use super::{runtime, BridgeRuntimePolicy};
use crate::facade::{
    BridgeCausalEnvelopeAssemblyRequest, BridgeCausalEnvelopeDenialKind,
    BridgeCausalEvidenceBinding, BridgeCausalEvidenceBindingClass, BridgeCausalEvidenceFamily,
    BridgeCausalEvidenceOwner, BridgeCausalEvidenceReference, BridgePreviewResidueClass,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeRequestKind, BridgeSignalBranchIdentity,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity, TruthBranchIdentity,
};

fn preview_declaration(suffix: &str) -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new(format!("preview:{suffix}")),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new(format!("binding:{suffix}")),
            TruthBranchIdentity::new(format!("truth:{suffix}")),
            BridgeSignalBranchIdentity::new(format!("signal:{suffix}")),
        ),
        format!("truth-view:{suffix}"),
        format!("source-capability:{suffix}"),
        format!("request-shape:{suffix}"),
        format!("artifact-schema:{suffix}"),
    )
}

fn bridge_reference(
    family: BridgeCausalEvidenceFamily,
    identity: &str,
) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(BridgeCausalEvidenceOwner::RuntimeBridge, family, identity)
        .expect("bridge reference should be valid")
}

fn query_observation_reference(identity: &str) -> BridgeCausalEvidenceReference {
    BridgeCausalEvidenceReference::new(
        BridgeCausalEvidenceOwner::Query,
        BridgeCausalEvidenceFamily::QueryObservation,
        identity,
    )
    .expect("query observation reference should be valid")
}

fn binding_for<'a>(
    bindings: &'a [BridgeCausalEvidenceBinding],
    family: BridgeCausalEvidenceFamily,
    reference_identity: &str,
) -> &'a BridgeCausalEvidenceBinding {
    bindings
        .iter()
        .find(|binding| {
            binding.owner() == BridgeCausalEvidenceOwner::RuntimeBridge
                && binding.family() == family
                && binding.reference_identity() == reference_identity
        })
        .expect("expected causal mapping binding should be present")
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
fn causal_envelope_maps_retained_preview_records_into_bridge_owned_bindings() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route("commit-causal-preview-mapping")
        .expect("route should succeed");

    let discard_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:causal-discard"),
            preview_declaration("causal-discard"),
        )
        .expect("discard preview should admit");
    let (discard_active, discard_execution) =
        runtime.activate_preview_session(discard_admitted, 3, 1, 2);
    let (_, discard_record) = runtime
        .discard_preview_session(
            discard_active,
            &discard_execution,
            vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ],
        )
        .expect("discard should close with zero authoritative residue");

    let promotion_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:causal-promotion"),
            preview_declaration("causal-promotion"),
        )
        .expect("promotion preview should admit");
    let (promotion_active, promotion_execution) =
        runtime.activate_preview_session(promotion_admitted, 2, 1, 1);
    let proof = promotion_active.promotion_admissibility_proof();
    let (_, promotion_record) = runtime
        .promote_preview_session(
            promotion_active,
            &promotion_execution,
            &proof,
            "commit-boundary:causal-promotion",
            "authoritative-artifact:causal-promotion",
        )
        .expect("promotion should succeed");

    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:preview-mapping",
            "causal-anchor:preview-mapping",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:preview-mapping"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgePreviewExecution,
                discard_execution.record_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgePreviewDiscard,
                discard_record.record_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgePreviewPromotion,
                promotion_record.record_identity().as_str(),
            ),
        ],
    )
    .expect("request should be valid");

    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect("preview mappings should assemble");

    assert_eq!(envelope.bindings().len(), 5);
    assert_eq!(envelope.counters().bridge_retained_lookup_count(), 4);
    assert_eq!(envelope.counters().retained_bridge_binding_count(), 4);
    assert_eq!(envelope.counters().lower_runtime_family_count(), 4);
    assert_eq!(envelope.counters().materialized_detail_count(), 5);
    assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);

    let execution_binding = binding_for(
        envelope.bindings(),
        BridgeCausalEvidenceFamily::BridgePreviewExecution,
        discard_execution.record_identity().as_str(),
    );
    assert_eq!(
        execution_binding.binding_class(),
        BridgeCausalEvidenceBindingClass::RetainedBridgeRecord
    );
    assert_eq!(
        execution_binding.retained_record_digest(),
        Some(
            digest(
                "bridge-causal-retained-preview-execution-record",
                &[
                    discard_execution.record_identity().as_str(),
                    discard_execution.preview_session_identity(),
                    discard_execution.preview_declaration_digest(),
                    discard_execution.branch_binding_digest(),
                    discard_execution.digest(),
                ],
            )
            .as_str()
        )
    );

    let discard_binding = binding_for(
        envelope.bindings(),
        BridgeCausalEvidenceFamily::BridgePreviewDiscard,
        discard_record.record_identity().as_str(),
    );
    assert_eq!(
        discard_binding.retained_record_digest(),
        Some(
            digest(
                "bridge-causal-retained-preview-discard-record",
                &[
                    discard_record.record_identity().as_str(),
                    discard_record.preview_session_identity(),
                    discard_record.preview_execution_record_identity().as_str(),
                    discard_record.residue_report().digest(),
                    discard_record.digest(),
                ],
            )
            .as_str()
        )
    );

    let promotion_binding = binding_for(
        envelope.bindings(),
        BridgeCausalEvidenceFamily::BridgePreviewPromotion,
        promotion_record.record_identity().as_str(),
    );
    assert_eq!(
        promotion_binding.retained_record_digest(),
        Some(
            digest(
                "bridge-causal-retained-preview-promotion-record",
                &[
                    promotion_record.record_identity().as_str(),
                    promotion_record.preview_session_identity(),
                    promotion_record
                        .preview_execution_record_identity()
                        .as_str(),
                    promotion_record.promotion_proof_digest(),
                    promotion_record.authoritative_commit_boundary_digest(),
                    promotion_record.authoritative_artifact_digest(),
                    promotion_record.digest(),
                ],
            )
            .as_str()
        )
    );
}

#[test]
fn causal_envelope_denies_missing_preview_mapping_after_required_route_evidence() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let routed = runtime
        .route("commit-causal-missing-preview")
        .expect("route should succeed");
    let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:missing-preview",
            "causal-anchor:missing-preview",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:missing-preview"),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgeRoute,
                routed.result().result_summary().route_identity().as_str(),
            ),
            bridge_reference(
                BridgeCausalEvidenceFamily::BridgePreviewPromotion,
                "missing-preview-promotion-record",
            ),
        ],
    )
    .expect("request should be valid");

    let denial = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(request)
        .expect_err("missing preview mapping should deny");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::MissingRetainedBridgeRecord
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::BridgePreviewPromotion
    );
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 2);
    assert_eq!(denial.counters().retained_bridge_binding_count(), 1);
    assert_eq!(denial.counters().missing_bridge_record_count(), 1);
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_request_denies_duplicate_preview_references_before_mapping() {
    let duplicate_reference = bridge_reference(
        BridgeCausalEvidenceFamily::BridgePreviewExecution,
        "preview-execution:duplicate-reference",
    );

    let denial = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
        crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
            "query-admission:duplicate-preview",
            "causal-anchor:duplicate-preview",
        )
        .expect("query admission summary should be valid"),
        vec![
            query_observation_reference("query-observation:duplicate-preview"),
            duplicate_reference.clone(),
            duplicate_reference,
        ],
    )
    .expect_err("duplicate preview references should deny before mapping");

    assert_eq!(
        denial.kind(),
        BridgeCausalEnvelopeDenialKind::DuplicateEvidenceReference
    );
    assert_eq!(
        denial.family(),
        BridgeCausalEvidenceFamily::BridgePreviewExecution
    );
    assert_eq!(
        denial.reference_identity(),
        "preview-execution:duplicate-reference"
    );
    assert_eq!(denial.counters().bridge_retained_lookup_count(), 0);
    assert_eq!(denial.counters().bridge_record_scan_fallback_count(), 0);
}

#[test]
fn causal_envelope_preview_mapping_cost_ignores_unrelated_preview_records() {
    let mut envelope_identities = Vec::new();

    for unrelated_previews in [0, 3, 9] {
        let runtime = runtime(BridgeRuntimePolicy::default());
        for index in 0..unrelated_previews {
            let admitted = runtime
                .admit_preview_session(
                    BridgePreviewSessionIdentity::new(format!("preview-session:noise-{index}")),
                    preview_declaration(&format!("noise-{index}")),
                )
                .expect("unrelated preview should admit");
            runtime.activate_preview_session(admitted, 1, 0, 0);
        }
        let routed = runtime
            .route("commit-causal-preview-scale")
            .expect("route should succeed");
        let admitted = runtime
            .admit_preview_session(
                BridgePreviewSessionIdentity::new("preview-session:causal-scale"),
                preview_declaration("causal-scale"),
            )
            .expect("target preview should admit");
        let (_, execution_record) = runtime.activate_preview_session(admitted, 2, 1, 1);
        let request = BridgeCausalEnvelopeAssemblyRequest::from_query_admission(
            crate::facade::BridgeCausalInspectionAdmissionSummary::admitted(
                "query-admission:preview-scale",
                "causal-anchor:preview-scale",
            )
            .expect("query admission summary should be valid"),
            vec![
                query_observation_reference("query-observation:preview-scale"),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgeRoute,
                    routed.result().result_summary().route_identity().as_str(),
                ),
                bridge_reference(
                    BridgeCausalEvidenceFamily::BridgePreviewExecution,
                    execution_record.record_identity().as_str(),
                ),
            ],
        )
        .expect("request should be valid");

        let envelope = runtime
            .diagnostics()
            .assemble_causal_explanation_envelope(request)
            .expect("target preview should bind");

        assert_eq!(
            runtime.diagnostics().preview_execution_records().len(),
            unrelated_previews + 1
        );
        assert_eq!(envelope.counters().bridge_retained_lookup_count(), 2);
        assert_eq!(envelope.counters().retained_bridge_binding_count(), 2);
        assert_eq!(envelope.counters().external_authority_reference_count(), 1);
        assert_eq!(envelope.counters().bridge_record_scan_fallback_count(), 0);
        envelope_identities.push(envelope.identity().identity_digest().to_string());
    }

    assert_eq!(envelope_identities[0], envelope_identities[1]);
    assert_eq!(envelope_identities[1], envelope_identities[2]);
}
