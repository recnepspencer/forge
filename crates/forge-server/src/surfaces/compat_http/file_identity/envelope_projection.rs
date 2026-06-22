use crate::{
    ForgeServerBinaryPolicyDecision, ForgeServerCacheabilityPolicy, ForgeServerCanonicalFilename,
    ForgeServerCompatibilityFileEnvelope, ForgeServerFileMetadataReceipt,
    ForgeServerFileMetadataTruthKind, ForgeServerFileTransferDisposition,
    ForgeServerFileTransferProvenance, ForgeServerMetadataNormalizationReceipt,
};

pub(crate) fn project_metadata_read_envelope(
    direct_context: &crate::ForgeServerDirectContextArtifact,
    operation_name: &str,
    truth_digest: &str,
    response_envelope: &crate::ForgeServerResponseEnvelope,
    support_posture: &crate::ForgeServerQuerySupportPosture,
    compatibility_cache_policy: &crate::ForgeServerCompatibilityCachePolicy,
) -> ForgeServerCompatibilityFileEnvelope {
    project_observed_envelope(
        direct_context,
        operation_name,
        ForgeServerFileMetadataTruthKind::ObservedRead,
        truth_digest,
        response_envelope,
        support_posture,
        "metadata_read",
        ForgeServerFileTransferDisposition::MetadataOnlyObservation,
        compatibility_cache_policy,
        None,
        0,
        false,
    )
}

pub(crate) fn project_metadata_inspection_envelope(
    direct_context: &crate::ForgeServerDirectContextArtifact,
    operation_name: &str,
    truth_digest: &str,
    response_envelope: &crate::ForgeServerResponseEnvelope,
    support_posture: &crate::ForgeServerQuerySupportPosture,
    compatibility_cache_policy: &crate::ForgeServerCompatibilityCachePolicy,
) -> ForgeServerCompatibilityFileEnvelope {
    project_observed_envelope(
        direct_context,
        operation_name,
        ForgeServerFileMetadataTruthKind::ObservedInspection,
        truth_digest,
        response_envelope,
        support_posture,
        "metadata_inspection",
        ForgeServerFileTransferDisposition::MetadataOnlyObservation,
        compatibility_cache_policy,
        None,
        0,
        false,
    )
}

pub(crate) fn project_binary_egress_envelope(
    read: &crate::ForgeServerCompatibilityRead,
    content_type: Option<String>,
    bytes_selected: u64,
    range_honored: bool,
    disposition: ForgeServerFileTransferDisposition,
) -> ForgeServerCompatibilityFileEnvelope {
    project_observed_envelope(
        read.direct_context(),
        read.operation_name(),
        ForgeServerFileMetadataTruthKind::ObservedRead,
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
    session: &crate::ForgeServerBinaryIngressSession,
    mutation: &crate::ForgeServerCompatibilityMutation,
) -> ForgeServerCompatibilityFileEnvelope {
    let canonical_filename = ForgeServerCanonicalFilename::admit(
        session.operation_name(),
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        crate::ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
    )
    .expect("upload file identity should already be validated before truth linkage");
    let metadata_receipt = ForgeServerFileMetadataReceipt::new(
        session.tenant_id(),
        session.workspace_digest(),
        session.branch_digest(),
        canonical_filename.canonical(),
        ForgeServerFileMetadataTruthKind::CommittedMutation,
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
    let metadata_normalization_receipt = ForgeServerMetadataNormalizationReceipt::from_manifest(
        metadata_receipt.metadata_identity(),
        canonical_filename.canonical(),
        session.upload().manifest().metadata_body(),
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        crate::ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
    )
    .expect("upload metadata normalization should already be validated before truth linkage");
    let policy_decision = ForgeServerBinaryPolicyDecision::new(
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
    let transfer_provenance = ForgeServerFileTransferProvenance::new(
        metadata_receipt.metadata_identity(),
        metadata_receipt.tenant_id(),
        metadata_receipt.workspace_digest(),
        metadata_receipt.branch_digest(),
        metadata_receipt.operation_name(),
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        ForgeServerFileTransferDisposition::VerifiedIngress,
        None,
        session
            .upload()
            .parts()
            .iter()
            .map(|part| part.authoritative_len())
            .sum(),
        false,
    );
    let cacheability_policy = ForgeServerCacheabilityPolicy::scoped_private(
        "upload_ingress",
        mutation
            .envelope()
            .response_envelope()
            .diagnostics_profile(),
        metadata_receipt.metadata_identity(),
        metadata_receipt.branch_digest(),
    );
    ForgeServerCompatibilityFileEnvelope::new(
        metadata_receipt,
        canonical_filename,
        metadata_normalization_receipt,
        cacheability_policy,
        policy_decision,
        transfer_provenance,
    )
}

fn project_observed_envelope(
    direct_context: &crate::ForgeServerDirectContextArtifact,
    operation_name: &str,
    truth_kind: ForgeServerFileMetadataTruthKind,
    truth_digest: &str,
    response_envelope: &crate::ForgeServerResponseEnvelope,
    support_posture: &crate::ForgeServerQuerySupportPosture,
    surface_kind: &str,
    transfer_disposition: ForgeServerFileTransferDisposition,
    compatibility_cache_policy: &crate::ForgeServerCompatibilityCachePolicy,
    content_type: Option<String>,
    bytes_selected: u64,
    range_honored: bool,
) -> ForgeServerCompatibilityFileEnvelope {
    let canonical_filename = ForgeServerCanonicalFilename::admit(
        operation_name,
        response_envelope.diagnostics_profile(),
        crate::ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid,
    )
    .expect("observed file identity should already be validated before projection");
    let metadata_receipt = ForgeServerFileMetadataReceipt::new(
        direct_context.workspace_target().tenant_id(),
        direct_context.workspace_digest(),
        direct_context.branch_digest(),
        canonical_filename.canonical(),
        truth_kind,
        truth_digest,
        direct_context.basis_digest().map(str::to_string),
        direct_context.provenance().artifact().clone(),
    );
    let metadata_normalization_receipt = ForgeServerMetadataNormalizationReceipt::observed(
        metadata_receipt.metadata_identity(),
        canonical_filename.canonical(),
        response_envelope.diagnostics_profile(),
    );
    let policy_decision = ForgeServerBinaryPolicyDecision::new(
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
    let transfer_provenance = ForgeServerFileTransferProvenance::new(
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
    let cacheability_policy = ForgeServerCacheabilityPolicy::from_compatibility_policy(
        surface_kind,
        response_envelope.diagnostics_profile(),
        metadata_receipt.metadata_identity(),
        metadata_receipt.branch_digest(),
        compatibility_cache_policy,
        false,
    );
    ForgeServerCompatibilityFileEnvelope::new(
        metadata_receipt,
        canonical_filename,
        metadata_normalization_receipt,
        cacheability_policy,
        policy_decision,
        transfer_provenance,
    )
}
