use crate::application::{
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    ForgeQueryDeclarationEntryOrchestrationProduct, ForgeQueryDeclarationEntryOrchestrationStage,
};

use super::super::lower::{
    forge_query_lower_declaration_entry_orchestration_on_handle,
    forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle,
};
use super::domain::{
    admitted_handle, AspectRichFamily, AuthorityRichFamily, ConflictingAspectFamily, Input,
};

#[test]
fn orchestration_policy_exposes_semantic_publication_by_lane() {
    let handle = admitted_handle("collaborative");

    let ordinary = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AspectRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );
    let checked = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AspectRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
    );
    let proof = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AspectRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
    );

    assert_eq!(
        ordinary.plan.foundational_aspect_publication().present(),
        &["selection.active_edge".to_string()]
    );
    assert_eq!(
        ordinary.plan.aspect_coverage_basis(),
        ForgeQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage
    );
    assert_eq!(
        ordinary.plan.receipt_aspect_publication().present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string()
        ]
    );
    assert_eq!(
        ordinary.plan.envelope_aspect_publication().present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string()
        ]
    );

    assert_eq!(
        checked.plan.receipt_aspect_publication().present(),
        &["selection.active_edge".to_string()]
    );
    assert_eq!(
        proof.plan.foundational_aspect_publication().present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string(),
            "selection.material_edit".to_string()
        ]
    );
    assert_eq!(
        proof.plan.envelope_aspect_publication().masked(),
        &["selection.private_authority".to_string()]
    );
    assert_eq!(
        proof.plan.materialization_tier(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive
    );
}

#[test]
fn orchestration_identity_tracks_aspect_publication_inputs() {
    let handle = admitted_handle("collaborative");

    let aspect_rich = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AspectRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );
    let plain = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<super::domain::AdmittedFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );

    assert_ne!(
        aspect_rich.plan.orchestration_identity_digest(),
        plain.plan.orchestration_identity_digest()
    );
}

#[test]
fn orchestration_identity_changes_when_lane_publication_changes() {
    let handle = admitted_handle("collaborative");

    let ordinary = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AspectRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );
    let checked = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AspectRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
    );

    assert_ne!(
        ordinary.plan.orchestration_identity_digest(),
        checked.plan.orchestration_identity_digest()
    );
}

#[test]
fn progressed_product_plans_carry_reviewed_aspect_coverage_basis() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .declare_review_and_progress(Input::<AspectRichFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let lowered =
        forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
            &handle,
            progressed,
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
            ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
            ForgeQueryDeclarationEntryOrchestrationProduct::RoutePlan,
            None,
        );

    assert_eq!(
        lowered.plan.aspect_coverage_basis(),
        ForgeQueryDeclarationAspectCoverageBasis::ReviewedRetainedCoverage
    );
}

#[test]
fn declared_family_coverage_keeps_conflicting_publication_out_of_visible_surfaces() {
    let handle = admitted_handle("collaborative");

    let proof = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<ConflictingAspectFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
    );

    assert_eq!(
        proof.plan.aspect_coverage_basis(),
        ForgeQueryDeclarationAspectCoverageBasis::DeclaredFamilyCoverage
    );
    assert_eq!(
        proof.plan.foundational_aspect_publication().present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string()
        ]
    );
    assert_eq!(
        proof.plan.foundational_aspect_publication().masked(),
        &[
            "selection.material_edit".to_string(),
            "selection.private_authority".to_string()
        ]
    );
}

#[test]
fn product_orchestration_receipts_and_envelopes_match_planned_crossing_publication() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .declare_review_and_progress(Input::<AspectRichFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let receipt_lowered =
        forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
            &handle,
            progressed.clone(),
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
            ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
            ForgeQueryDeclarationEntryOrchestrationProduct::Receipt,
            None,
        );
    let envelope_lowered =
        forge_query_lower_declaration_entry_product_orchestration_from_progressed_on_handle(
            &handle,
            progressed,
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
            ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
            ForgeQueryDeclarationEntryOrchestrationProduct::Envelope,
            None,
        );

    let receipt = match receipt_lowered.checked {
        super::super::lower::ForgeQueryDeclarationEntryProductChecked::Receipt(
            crate::application::ForgeQueryDeclarationReceiptChecked::Issued(receipt),
        ) => receipt,
        _ => panic!("receipt product should issue"),
    };
    let envelope = match envelope_lowered.checked {
        super::super::lower::ForgeQueryDeclarationEntryProductChecked::Envelope(
            crate::application::ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope),
        ) => envelope,
        _ => panic!("envelope product should issue"),
    };

    assert_eq!(
        receipt.aspect_publication(),
        receipt_lowered.plan.receipt_aspect_publication()
    );
    assert_eq!(
        envelope.aspect_publication(),
        envelope_lowered.plan.envelope_aspect_publication()
    );
    assert_eq!(envelope.aspect_publication(), receipt.aspect_publication());
}

#[test]
fn envelope_proof_keeps_envelope_ceiling_while_exposing_authority_summaries() {
    let handle = admitted_handle("collaborative");
    let progressed = handle
        .declare_review_and_progress(Input::<AuthorityRichFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("progression should admit"));

    let proof = handle.orchestrate_envelope_from_progressed_proof(progressed);

    assert_eq!(
        proof.plan().product(),
        ForgeQueryDeclarationEntryOrchestrationProduct::Envelope
    );
    assert_eq!(
        proof
            .step_records()
            .last()
            .expect("proof should end at envelope")
            .stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert_eq!(
        proof.relational_authority_summary(),
        proof.plan().relational_authority_summary()
    );
    assert_eq!(
        proof.bridge_authority_summary(),
        proof.plan().bridge_authority_summary()
    );
    assert_eq!(
        proof.signal_authority_summary(),
        proof.plan().signal_authority_summary()
    );
    assert_eq!(
        proof.relational_authority_summary().aspect_coverage_basis(),
        ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage
    );
    assert_eq!(
        proof.bridge_authority_summary().mapping_fit(),
        ForgeQueryDeclarationAspectFit::MissingRequired
    );
    assert_eq!(
        proof
            .signal_authority_summary()
            .produced_aspects()
            .required(),
        &["signal.preview_patch".to_string()]
    );
}

#[test]
fn authority_summaries_change_plan_identity_when_retained_meaning_changes() {
    let handle = admitted_handle("collaborative");
    let authority_rich = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AuthorityRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );
    let plain = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AspectRichFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );

    assert_ne!(
        authority_rich.plan.orchestration_identity_digest(),
        plain.plan.orchestration_identity_digest()
    );
    assert_ne!(
        authority_rich.plan.bridge_authority_summary(),
        plain.plan.bridge_authority_summary()
    );
    assert_ne!(
        authority_rich.plan.signal_authority_summary(),
        plain.plan.signal_authority_summary()
    );
}
