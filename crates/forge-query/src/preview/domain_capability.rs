use forge_runtime_bridge::facade::{
    BridgePreviewLifecycleStateKind, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity,
};

use crate::execution::ExecutionCounters;
use crate::identity::{hash_parts, CanonicalQueryDigest, ValidatedQueryDigest};
use crate::preview::{
    AdmittedPreviewWorkflowFoundation, PreviewBindingCounters, PreviewEvaluationClass,
    PreviewExecutionCounters, PreviewWorkflowFoundationArtifact, PreviewWorkflowFoundationError,
    PreviewWorkflowFoundationFailureClass, PreviewWorkflowFoundationRequest,
};

pub(crate) fn materialize_contributed_preview_workflow_foundation_artifact(
    binding_digest: String,
    canonical_query_digest: CanonicalQueryDigest,
    validated_query_digest: ValidatedQueryDigest,
    request_family: PreviewWorkflowFoundationRequest,
    preview_session_identity: BridgePreviewSessionIdentity,
    declaration_identity: BridgePreviewSessionDeclarationIdentity,
    declaration_digest: String,
    evaluation_class: PreviewEvaluationClass,
    shape_check_width: usize,
) -> PreviewWorkflowFoundationArtifact {
    let lifecycle_state_kind = BridgePreviewLifecycleStateKind::Active;
    let execution_record_identity = PreviewExecutionRecordIdentity::new(format!(
        "preview-execution-record:domain:{}",
        hash_parts(&[
            "forge_query_domain_preview_execution_record_v1".to_string(),
            format!("binding:{binding_digest}"),
            format!("canonical:{}", canonical_query_digest.as_str()),
            format!("validated:{}", validated_query_digest.as_str()),
            format!("request:{}", request_family.as_str()),
            format!("preview_session:{}", preview_session_identity.as_str()),
            format!("declaration:{}", declaration_identity.as_str()),
            format!("evaluation:{}", evaluation_class.as_str()),
        ])
    ));
    let digest = hash_parts(&[
        format!("binding:{binding_digest}"),
        format!("request:{}", request_family.as_str()),
        format!("preview_session:{}", preview_session_identity.as_str()),
        format!("declaration_identity:{}", declaration_identity.as_str()),
        format!("declaration_digest:{declaration_digest}"),
        format!("lifecycle:{lifecycle_state_kind:?}"),
        format!("execution_record:{}", execution_record_identity.as_str()),
        format!("evaluation_class:{}", evaluation_class.as_str()),
        format!("shape_check_width:{shape_check_width}"),
    ]);

    PreviewWorkflowFoundationArtifact {
        digest,
        binding_digest,
        canonical_query_digest,
        validated_query_digest,
        request_family,
        preview_session_identity,
        declaration_identity,
        declaration_digest,
        lifecycle_state_kind,
        evaluation_class,
        execution_record_identity,
        shape_check_width,
    }
}

pub(crate) fn admit_contributed_preview_workflow_foundation(
    artifact: PreviewWorkflowFoundationArtifact,
) -> Result<AdmittedPreviewWorkflowFoundation, PreviewWorkflowFoundationError> {
    match artifact.request_family() {
        PreviewWorkflowFoundationRequest::CompareBasisPair => Ok(AdmittedPreviewWorkflowFoundation {
            artifact,
            counters: PreviewExecutionCounters {
                binding_counters: PreviewBindingCounters::default(),
                execution_counters: ExecutionCounters::default(),
                preview_execution_envelope_count: 0,
                preview_execution_count: 0,
                preview_promotable_execution_count: 0,
                preview_read_only_execution_count: 0,
                preview_comparison_eligibility_proof_count: 0,
                preview_comparison_shape_check_width: 0,
                preview_workflow_foundation_admission_count: 1,
                preview_workflow_foundation_denial_count: 0,
                preview_workflow_foundation_artifact_lookup_count: 1,
                preview_work_avoided_by_explicit_basis_count: 1,
            },
        }),
        PreviewWorkflowFoundationRequest::DeferredMutationWriteback => {
            Err(PreviewWorkflowFoundationError {
                failure_class:
                    PreviewWorkflowFoundationFailureClass::OutOfScopeWorkflowFoundationRequest,
                message:
                    "preview workflow foundation requests that imply mutation or writeback authority remain out of scope in milestone 5.2",
                counters: PreviewExecutionCounters {
                    binding_counters: PreviewBindingCounters::default(),
                    execution_counters: ExecutionCounters::default(),
                    preview_execution_envelope_count: 0,
                    preview_execution_count: 0,
                    preview_promotable_execution_count: 0,
                    preview_read_only_execution_count: 0,
                    preview_comparison_eligibility_proof_count: 0,
                    preview_comparison_shape_check_width: 0,
                    preview_workflow_foundation_admission_count: 0,
                    preview_workflow_foundation_denial_count: 1,
                    preview_workflow_foundation_artifact_lookup_count: 0,
                    preview_work_avoided_by_explicit_basis_count: 0,
                },
            })
        }
    }
}
