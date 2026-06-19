#![allow(dead_code)]

use forge_proof::TransitionOutcome;
use forge_server::{
    ForgeServer, ForgeServerAdmittedDirectDeclaration, ForgeServerForgeNativeSession,
    ForgeServerOperatorEvidenceRecord, ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

use crate::forge_native_runtime::forge_native_session_input_builder;

pub(crate) fn forge_native_session(server: &ForgeServer) -> ForgeServerForgeNativeSession {
    match server.forge_native().session(
        forge_native_session_input_builder()
            .build()
            .expect("session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected successful forge-native session, got {other:?}"),
    }
}

pub(crate) fn admitted_named_read(
    session: &ForgeServerForgeNativeSession,
    operation_name: &str,
) -> ForgeServerAdmittedDirectDeclaration {
    session
        .declarations()
        .read(forge_server::ForgeServerDirectDeclaration::named_read(
            operation_name,
        ))
        .expect("direct declaration should prepare")
        .admit()
        .expect("direct declaration should admit")
}

pub(crate) fn family_contract_digest(posture: &ForgeServerQuerySupportPosture) -> &str {
    match posture {
        ForgeServerQuerySupportPosture::ProductIndependent { label } => label,
        ForgeServerQuerySupportPosture::QueryReadSupported { family_contract }
        | ForgeServerQuerySupportPosture::DirectReadSupported { family_contract }
        | ForgeServerQuerySupportPosture::DirectStateSupported { family_contract }
        | ForgeServerQuerySupportPosture::DirectInspectionSupported { family_contract }
        | ForgeServerQuerySupportPosture::DirectProjectionSupported { family_contract }
        | ForgeServerQuerySupportPosture::DirectMutationSupported { family_contract }
        | ForgeServerQuerySupportPosture::QueryMutationSupported { family_contract } => {
            family_contract.contract_digest()
        }
        ForgeServerQuerySupportPosture::DownstreamDeliverySupported {
            family_contract, ..
        }
        | ForgeServerQuerySupportPosture::RuntimeBackedResumeSupported {
            family_contract, ..
        }
        | ForgeServerQuerySupportPosture::DurableResumeSupported {
            family_contract, ..
        } => family_contract.contract_digest(),
    }
}

pub(crate) fn response_provenance_digest(response: &ForgeServerResponseEnvelope) -> String {
    let provenance = response.provenance();
    provenance_visibility_summary(
        provenance.locality(),
        provenance.freshness_posture(),
        provenance.source_basis().kind(),
        provenance.authority_path().is_some(),
        provenance.strategy_basis().is_some(),
        provenance.profile_basis().is_some(),
        provenance.comparison_basis().is_some(),
        provenance.canonical_digest_basis().is_some(),
        provenance.support_context_attachments().len(),
    )
}

pub(crate) fn direct_provenance_digest(
    provenance: &forge_server::ForgeServerDirectProvenance,
) -> String {
    provenance_visibility_summary(
        provenance.locality(),
        provenance.freshness_posture(),
        provenance.source_basis_kind(),
        provenance.has_authority_path(),
        provenance.has_strategy_basis(),
        provenance.has_profile_basis(),
        provenance.has_comparison_basis(),
        provenance.has_canonical_digest_basis(),
        provenance.support_context_attachment_count(),
    )
}

fn provenance_visibility_summary(
    locality: impl std::fmt::Debug,
    freshness_posture: impl std::fmt::Debug,
    source_basis_kind: impl std::fmt::Debug,
    has_authority_path: bool,
    has_strategy_basis: bool,
    has_profile_basis: bool,
    has_comparison_basis: bool,
    has_canonical_digest_basis: bool,
    support_context_attachment_count: usize,
) -> String {
    format!(
        "locality={:?};freshness={:?};source={:?};authority={};strategy={};profile={};comparison={};canonical={};support_contexts={}",
        locality,
        freshness_posture,
        source_basis_kind,
        has_authority_path,
        has_strategy_basis,
        has_profile_basis,
        has_comparison_basis,
        has_canonical_digest_basis,
        support_context_attachment_count,
    )
}

pub(crate) fn operator_evidence_record(
    server: &ForgeServer,
    response: ForgeServerResponseEnvelope,
) -> ForgeServerOperatorEvidenceRecord {
    server
        .operator_evidence()
        .record(
            forge_server::ForgeServerEvidenceInput::response_envelope(response),
            forge_server::ForgeServerEvidenceTransform::operator_default(),
        )
        .expect("operator evidence record should materialize")
}
