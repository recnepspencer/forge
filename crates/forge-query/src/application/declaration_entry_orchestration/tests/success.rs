use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationStep,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStepDisposition,
    ForgeQueryDeclarationEntryOrchestrationVerbCeiling,
    ForgeQueryDeclarationEntryOrchestrationVerbFamily,
    ForgeQueryDeclarationEntryOrchestrationVerbInventory,
};

use super::super::lower::forge_query_lower_declaration_entry_orchestration_on_handle;
use super::super::sequencing::ForgeQueryDeclarationEntryOrchestrationAutomationParityReceipt;
use super::domain::{admitted_handle, AdmittedFamily, DeferredRouteFamily, Input};
use super::explicit_paths::{explicit_deferred_route_path_parity, explicit_success_path_parity};

#[test]
fn ordinary_checked_and_proof_surfaces_converge_on_one_envelope() {
    let handle = admitted_handle("collaborative");

    let ordinary = handle
        .orchestrate_declaration_entry(Input::<AdmittedFamily>::new("edge:42"))
        .unwrap_or_else(|_| panic!("ordinary orchestration should envelope"));
    let checked =
        handle.orchestrate_declaration_entry_checked(Input::<AdmittedFamily>::new("edge:42"));
    let proof = handle.orchestrate_declaration_entry_proof(Input::<AdmittedFamily>::new("edge:42"));

    let checked_digest = match checked {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope) => {
            digest_text(envelope.envelope_digest())
        }
        _ => panic!("checked orchestration should envelope"),
    };
    let proof_digest = match proof.outcome() {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope) => {
            digest_text(envelope.envelope_digest())
        }
        _ => panic!("proof orchestration should envelope"),
    };

    assert_eq!(digest_text(ordinary.envelope_digest()), checked_digest);
    assert_eq!(digest_text(ordinary.envelope_digest()), proof_digest);
}

#[test]
fn proof_surface_records_the_full_envelope_ceiling_sequence() {
    let handle = admitted_handle("collaborative");
    let proof = handle.orchestrate_declaration_entry_proof(Input::<AdmittedFamily>::new("edge:42"));
    let stages = proof.stage_records();

    assert_eq!(stages.len(), 8);
    assert_eq!(
        proof.plan().ceiling_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert_eq!(
        proof.plan().automation_boundary(),
        ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling
    );
    assert_eq!(
        proof.plan().automation_steps(),
        &[
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::AdmittedHandle,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::CanonicalDeclaration,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Legality,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Progression,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::FoundationalEvidence,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::RoutePlan,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Receipt,
            ForgeQueryDeclarationEntryOrchestrationAutomationStep::Envelope,
        ]
    );
    assert!(proof.plan().explicit_caller_handoff_steps().is_empty());
    assert!(stages[..7].iter().all(|record| record.is_reached()));
    assert_eq!(
        stages
            .iter()
            .map(|record| record.automation_step())
            .collect::<Vec<_>>(),
        proof.plan().automation_steps().to_vec()
    );
    assert_eq!(
        stages
            .last()
            .expect("last stage should exist")
            .disposition(),
        ForgeQueryDeclarationEntryOrchestrationStepDisposition::TerminalSuccess
    );
    assert_eq!(
        stages.last().expect("last stage should exist").stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert_eq!(
        proof.automation_boundary(),
        ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling
    );
    assert!(!proof.orchestration_digest().is_empty());
}

#[test]
fn canonical_lowering_preserves_plan_and_outcome_identity_across_exposure_levels() {
    let handle = admitted_handle("collaborative");

    let ordinary = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AdmittedFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::OrdinaryEnvelopeOnly,
    );
    let checked = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AdmittedFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
    );
    let proof = forge_query_lower_declaration_entry_orchestration_on_handle(
        &handle,
        Input::<AdmittedFamily>::new("edge:42"),
        ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::ProofVisibleTranscript,
    );

    assert_eq!(
        ordinary.plan.orchestration_identity_digest(),
        checked.plan.orchestration_identity_digest()
    );
    assert_eq!(
        checked.plan.orchestration_identity_digest(),
        proof.plan.orchestration_identity_digest()
    );
    assert_eq!(
        ordinary.outcome.outcome_identity_digest(),
        checked.outcome.outcome_identity_digest()
    );
    assert_eq!(
        checked.outcome.outcome_identity_digest(),
        proof.outcome.outcome_identity_digest()
    );
}

#[test]
fn explicit_and_orchestrated_success_paths_produce_matching_parity_receipt() {
    let handle = admitted_handle("collaborative");
    let explicit = explicit_success_path_parity(&handle);
    let orchestrated =
        handle.orchestrate_declaration_entry_checked(Input::<AdmittedFamily>::new("edge:42"));
    let parity = ForgeQueryDeclarationEntryOrchestrationAutomationParityReceipt::new(
        explicit.outcome_identity_digest().to_string(),
        orchestrated.outcome_identity_digest(),
        explicit.stop_stage(),
        orchestrated.stop_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        orchestrated.stop_stage(),
    );

    assert_eq!(
        parity.explicit_outcome_identity_digest(),
        parity.orchestrated_outcome_identity_digest()
    );
    assert_eq!(
        parity.explicit_stop_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert_eq!(
        parity.orchestrated_stop_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert_eq!(
        parity.explicit_farthest_crossed_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert_eq!(
        parity.orchestrated_farthest_crossed_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert!(parity.parity_holds());
}

#[test]
fn explicit_and_orchestrated_deferred_route_paths_agree_on_receipt_stop() {
    let handle = admitted_handle("collaborative");
    let explicit = explicit_deferred_route_path_parity(&handle);
    let orchestrated =
        handle.orchestrate_declaration_entry_checked(Input::<DeferredRouteFamily>::new("edge:42"));
    let parity = ForgeQueryDeclarationEntryOrchestrationAutomationParityReceipt::new(
        explicit.outcome_identity_digest().to_string(),
        orchestrated.outcome_identity_digest(),
        explicit.stop_stage(),
        orchestrated.stop_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
    );

    assert_eq!(
        parity.explicit_stop_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued
    );
    assert_eq!(
        parity.orchestrated_stop_stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued
    );
    assert!(parity.parity_holds());
}

#[test]
fn grammar_inventory_freezes_the_generic_trio_and_envelope_ceiling() {
    let inventory = ForgeQueryDeclarationEntryOrchestrationVerbInventory::current();
    let verbs = inventory.verbs();

    assert_eq!(verbs.len(), 3);
    assert_eq!(
        verbs
            .iter()
            .map(|verb| verb.public_name())
            .collect::<Vec<_>>(),
        vec![
            "orchestrate_declaration_entry",
            "orchestrate_declaration_entry_checked",
            "orchestrate_declaration_entry_proof",
        ]
    );
    assert!(verbs.iter().all(|verb| {
        verb.family() == ForgeQueryDeclarationEntryOrchestrationVerbFamily::GenericDeclarationEntry
    }));
    assert!(verbs.iter().all(|verb| {
        verb.ceiling() == ForgeQueryDeclarationEntryOrchestrationVerbCeiling::Envelope
    }));
    assert_eq!(
        verbs
            .iter()
            .map(|verb| verb.canonical_base_name())
            .collect::<Vec<_>>(),
        vec![
            "orchestrate_declaration_entry",
            "orchestrate_declaration_entry",
            "orchestrate_declaration_entry",
        ]
    );
    assert_eq!(
        verbs
            .iter()
            .map(|verb| verb.exposure_level())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::Ordinary,
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
            ForgeQueryDeclarationEntryOrchestrationExposureLevel::ProofVisible,
        ]
    );
}

fn digest_text(digest: &CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
