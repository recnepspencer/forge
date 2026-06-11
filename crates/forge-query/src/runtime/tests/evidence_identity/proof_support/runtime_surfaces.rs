use super::super::super::support::*;

pub(in super::super) fn compose_public_api_family_contract_identity(
    contract: &ForgeQueryRuntimePublicApiFamilyContract,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimePublicApiFamilyContract,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("family"),
        contract.family().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("status"),
        contract.status().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("teaching_posture"),
        contract.teaching_posture().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("owner_closure"),
        contract.owner_closure(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("extension_rule"),
        contract.extension_rule(),
    )
    .field_bool(
        crate::ForgeQueryEvidenceTag::new("parallel_api_forbidden"),
        contract.parallel_api_forbidden(),
    )
    .field_bool(
        crate::ForgeQueryEvidenceTag::new("admission_fail_closed"),
        contract.admission_fail_closed(),
    )
    .field_identity_sequence(
        crate::ForgeQueryEvidenceTag::new("authority_lanes"),
        contract.authority_lanes().iter().map(|lane| lane.as_str()),
    )
    .field_identity_sequence(
        crate::ForgeQueryEvidenceTag::new("evidence"),
        contract.evidence().iter().map(String::as_str),
    )
    .optional_value(
        crate::ForgeQueryEvidenceTag::new("reason"),
        contract.reason(),
    )
    .seal()
}

pub(in super::super) fn compose_public_api_contract_identity(
    contract: &ForgeQueryRuntimePublicApiContract,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimePublicApiContract,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("backend_posture"),
        contract.backend_posture().as_str(),
    )
    .field_identity_sequence(
        crate::ForgeQueryEvidenceTag::new("family_contract_digest"),
        contract.families().iter().map(|family| family.contract_digest()),
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("stable_family_count"),
        contract.stable_family_count().to_string(),
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("deferred_family_count"),
        contract.deferred_family_count().to_string(),
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("unsupported_family_count"),
        contract.unsupported_family_count().to_string(),
    )
    .seal()
}

pub(in super::super) fn compose_public_support_matrix_row_identity(
    row: &ForgeQueryRuntimePublicSupportMatrixRow,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimePublicSupportMatrixRow,
    )
    .field_shape(crate::ForgeQueryEvidenceTag::new("surface"), row.surface())
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("facade_family"),
        row.facade_family()
            .map(ForgeQueryRuntimeFacadeFamily::as_str)
            .unwrap_or("matrix-only"),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("status"),
        row.status().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("teaching_posture"),
        row.teaching_posture().as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("owner_milestone"),
        row.owner_milestone(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("extension_rule"),
        row.extension_rule(),
    )
    .field_bool(
        crate::ForgeQueryEvidenceTag::new("parallel_api_forbidden"),
        row.parallel_api_forbidden(),
    )
    .field_bool(
        crate::ForgeQueryEvidenceTag::new("admission_fail_closed"),
        row.admission_fail_closed(),
    )
    .optional_identity(
        crate::ForgeQueryEvidenceTag::new("support_contract_digest"),
        row.support_contract_digest(),
    )
    .seal()
}

pub(in super::super) fn compose_public_support_matrix_identity(
    matrix: &ForgeQueryRuntimePublicSupportMatrix,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimePublicSupportMatrix,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("backend_posture"),
        matrix.backend_posture().as_str(),
    )
    .field_usize(
        crate::ForgeQueryEvidenceTag::new("stable_row_count"),
        matrix.stable_row_count(),
    )
    .field_usize(
        crate::ForgeQueryEvidenceTag::new("deferred_row_count"),
        matrix.deferred_row_count(),
    )
    .field_usize(
        crate::ForgeQueryEvidenceTag::new("unsupported_row_count"),
        matrix.unsupported_row_count(),
    )
    .field_usize(
        crate::ForgeQueryEvidenceTag::new("fail_closed_row_count"),
        matrix.fail_closed_row_count(),
    )
    .field_usize(
        crate::ForgeQueryEvidenceTag::new("parallel_api_forbidden_row_count"),
        matrix.parallel_api_forbidden_row_count(),
    )
    .field_identity_sequence(
        crate::ForgeQueryEvidenceTag::new("row_digest"),
        matrix
            .rows()
            .iter()
            .map(|row| row.row_digest().as_str()),
    )
    .seal()
}

pub(in super::super) fn compose_state_snapshot_identity(
    snapshot: &ForgeQueryRuntimeStateSnapshot,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(crate::ForgeQueryEvidenceScope::RuntimeStateSnapshot)
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("kind"),
            snapshot.kind().as_str(),
        )
        .field_identity(
            crate::ForgeQueryEvidenceTag::new("basis_digest"),
            snapshot.basis_digest(),
        )
        .field_identity(
            crate::ForgeQueryEvidenceTag::new("result_shape_digest"),
            snapshot.result_shape_digest(),
        )
        .field_shape(
            crate::ForgeQueryEvidenceTag::new("authority_lane"),
            snapshot.authority_lane().as_str(),
        )
        .field_value(
            crate::ForgeQueryEvidenceTag::new("explanation"),
            snapshot.explanation(),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("ordinary_runtime_posture"),
            snapshot
                .ordinary_runtime_posture()
                .map(crate::ordinary_outcome::ForgeQueryOrdinaryRuntimePosture::posture_digest),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("async_result_state"),
            snapshot
                .async_result_state()
                .map(ForgeQueryRuntimeAsyncResultState::result_state_digest),
        )
        .optional_identity(
            crate::ForgeQueryEvidenceTag::new("remask_posture"),
            snapshot
                .remask_posture()
                .map(ForgeQueryRuntimeRemaskPosture::remask_digest),
        )
        .seal()
}

pub(in super::super) fn compose_runtime_public_api_transcript_identity(
    transcript: &ForgeQueryRuntimePublicApiTranscriptEvidence,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::RuntimePublicApiTranscriptEvidence,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("transcript_family"),
        transcript.transcript_family(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("support_contract_digest"),
        transcript.support_contract_digest(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("state_digest"),
        transcript.state_digest(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("live_surface_digest"),
        transcript.live_surface_digest(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("computed_surface_digest"),
        transcript.computed_surface_digest(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("effect_surface_digest"),
        transcript.effect_surface_digest(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("intent_receipt_digest"),
        transcript.intent_receipt_digest(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("inspection_digest"),
        transcript.inspection_digest(),
    )
    .field_identity_sequence(
        crate::ForgeQueryEvidenceTag::new("support_gated_neighbor_denial_digest"),
        transcript
            .support_gated_neighbor_denial_digests()
            .iter()
            .map(String::as_str),
    )
    .field_usize(
        crate::ForgeQueryEvidenceTag::new("delivery_residue_count"),
        transcript.delivery_residue_count(),
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("authority_lane_digest"),
        transcript.authority_lane_digest(),
    )
    .field_usize(
        crate::ForgeQueryEvidenceTag::new("meaningful_assertion_count"),
        transcript.meaningful_assertion_count(),
    )
    .seal()
}
