use crate::{
    WorthServerBinaryPolicyDecision, WorthServerCacheabilityPolicy, WorthServerCanonicalFilename,
    WorthServerCompatibilityFileEnvelope, WorthServerFileMetadataReceipt,
    WorthServerFileMetadataTruthKind, WorthServerFileTransferDisposition,
    WorthServerFileTransferProvenance, WorthServerMetadataNormalizationReceipt,
};

pub(crate) fn project_metadata_read_envelope(
    direct_context: &crate::WorthServerDirectContextArtifact,
    operation_name: &str,
    truth_digest: &str,
    response_envelope: &crate::WorthServerResponseEnvelope,
    support_posture: &crate::WorthServerQuerySupportPosture,
    compatibility_cache_policy: &crate::WorthServerCompatibilityCachePolicy,
) -> WorthServerCompatibilityFileEnvelope {
    project_observed_envelope(
        direct_context,
        operation_name,
        WorthServerFileMetadataTruthKind::ObservedRead,
        truth_digest,
        response_envelope,
        support_posture,
        "metadata_read",
        WorthServerFileTransferDisposition::MetadataOnlyObservation,
        compatibility_cache_policy,
        None,
        0,
        false,
    )
}

pub(crate) fn project_metadata_inspection_envelope(
    direct_context: &crate::WorthServerDirectContextArtifact,
    operation_name: &str,
    truth_digest: &str,
    response_envelope: &crate::WorthServerResponseEnvelope,
    support_posture: &crate::WorthServerQuerySupportPosture,
    compatibility_cache_policy: &crate::WorthServerCompatibilityCachePolicy,
) -> WorthServerCompatibilityFileEnvelope {
    project_observed_envelope(
        direct_context,
        operation_name,
        WorthServerFileMetadataTruthKind::ObservedInspection,
        truth_digest,
        response_envelope,
        support_posture,
        "metadata_inspection",
        WorthServerFileTransferDisposition::MetadataOnlyObservation,
        compatibility_cache_policy,
        None,
        0,
        false,
    )
}

pub(crate) fn project_binary_egress_envelope(
    read: &crate::WorthServerCompatibilityRead,
    content_type: Option<String>,
    bytes_selected: u64,
    range_honored: bool,
    disposition: WorthServerFileTransferDisposition,
) -> WorthServerCompatibilityFileEnvelope {
    project_observed_envelope(
        read.direct_context(),
        read.operation_name(),
        WorthServerFileMetadataTruthKind::ObservedRead,
        read.read_result().receipt().result_digest(),
        read.response_envelope(),
        read.support_posture(),
        "binary_egress",
        disposition,
        read.cache_policy(),
        content_type,
        bytes_selected,
        range_honored,
    )
}

pub(crate) fn project_upload_envelope(
    session: &crate::WorthServerBinaryIngressSession,
    mutation: &crate::WorthServerCompatibilityMutation,
) -> WorthServerCompatibilityFileEnvelope {
    let canonical_filename = WorthServerCanonicalFilename::admit(
        session.operation_name(),
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        crate::WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
    )
    .expect("upload file identity should already be validated before truth linkage");
    let metadata_receipt = WorthServerFileMetadataReceipt::new(
        session.tenant_id(),
        session.workspace_digest(),
        session.branch_digest(),
        canonical_filename.canonical(),
        WorthServerFileMetadataTruthKind::CommittedMutation,
        mutation.mutation_result().result_digest(),
        mutation
            .envelope()
            .direct_context()
            .basis_digest()
            .map(str::to_string),
        mutation
            .envelope()
            .direct_context()
            .provenance()
            .artifact()
            .clone(),
    );
    let metadata_normalization_receipt = WorthServerMetadataNormalizationReceipt::from_manifest(
        metadata_receipt.metadata_identity(),
        canonical_filename.canonical(),
        session.upload().manifest().metadata_body(),
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        crate::WorthServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
    )
    .expect("upload metadata normalization should already be validated before truth linkage");
    let policy_decision = WorthServerBinaryPolicyDecision::new(
        metadata_receipt.metadata_identity(),
        metadata_receipt.tenant_id(),
        metadata_receipt.workspace_digest(),
        metadata_receipt.branch_digest(),
        metadata_receipt.operation_name(),
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        "upload_ingress",
        mutation.envelope().support_posture().canonical_label(),
        mutation.envelope().response_envelope().canonical_digest(),
        None,
    );
    let transfer_provenance = WorthServerFileTransferProvenance::new(
        metadata_receipt.metadata_identity(),
        metadata_receipt.tenant_id(),
        metadata_receipt.workspace_digest(),
        metadata_receipt.branch_digest(),
        metadata_receipt.operation_name(),
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        WorthServerFileTransferDisposition::VerifiedIngress,
        None,
        session
            .upload()
            .parts()
            .iter()
            .map(|part| part.authoritative_len())
            .sum(),
        false,
    );
    let cacheability_policy = WorthServerCacheabilityPolicy::scoped_private(
        "upload_ingress",
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        metadata_receipt.metadata_identity(),
        metadata_receipt.branch_digest(),
    );
    WorthServerCompatibilityFileEnvelope::new(
        metadata_receipt,
        canonical_filename,
        metadata_normalization_receipt,
        cacheability_policy,
        policy_decision,
        transfer_provenance,
    )
}

fn project_observed_envelope(
    direct_context: &crate::WorthServerDirectContextArtifact,
    operation_name: &str,
    truth_kind: WorthServerFileMetadataTruthKind,
    truth_digest: &str,
    response_envelope: &crate::WorthServerResponseEnvelope,
    support_posture: &crate::WorthServerQuerySupportPosture,
    surface_kind: &str,
    transfer_disposition: WorthServerFileTransferDisposition,
    compatibility_cache_policy: &crate::WorthServerCompatibilityCachePolicy,
    content_type: Option<String>,
    bytes_selected: u64,
    range_honored: bool,
) -> WorthServerCompatibilityFileEnvelope {
    let canonical_filename = WorthServerCanonicalFilename::admit(
        operation_name,
        response_envelope.diagnostics_profile(),
        crate::WorthServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
    )
    .expect("observed file identity should already be validated before projection");
    let metadata_receipt = WorthServerFileMetadataReceipt::new(
        direct_context.workspace_target().tenant_id(),
        direct_context.workspace_digest(),
        direct_context.branch_digest(),
        canonical_filename.canonical(),
        truth_kind,
        truth_digest,
        direct_context.basis_digest().map(str::to_string),
        direct_context.provenance().artifact().clone(),
    );
    let metadata_normalization_receipt = WorthServerMetadataNormalizationReceipt::observed(
        metadata_receipt.metadata_identity(),
        canonical_filename.canonical(),
        response_envelope.diagnostics_profile(),
    );
    let policy_decision = WorthServerBinaryPolicyDecision::new(
        metadata_receipt.metadata_identity(),
        metadata_receipt.tenant_id(),
        metadata_receipt.workspace_digest(),
        metadata_receipt.branch_digest(),
        metadata_receipt.operation_name(),
        response_envelope.diagnostics_profile(),
        surface_kind,
        support_posture.canonical_label(),
        response_envelope.canonical_digest(),
        None,
    );
    let transfer_provenance = WorthServerFileTransferProvenance::new(
        metadata_receipt.metadata_identity(),
        metadata_receipt.tenant_id(),
        metadata_receipt.workspace_digest(),
        metadata_receipt.branch_digest(),
        metadata_receipt.operation_name(),
        response_envelope.diagnostics_profile(),
        transfer_disposition,
        content_type,
        bytes_selected,
        range_honored,
    );
    let cacheability_policy = WorthServerCacheabilityPolicy::from_compatibility_policy(
        surface_kind,
        response_envelope.diagnostics_profile(),
        metadata_receipt.metadata_identity(),
        metadata_receipt.branch_digest(),
        compatibility_cache_policy,
        false,
    );
    WorthServerCompatibilityFileEnvelope::new(
        metadata_receipt,
        canonical_filename,
        metadata_normalization_receipt,
        cacheability_policy,
        policy_decision,
        transfer_provenance,
    )
}
