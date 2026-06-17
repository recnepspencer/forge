use forge_foundational::facade::CanonicalDerivedDigest;
use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationStep,
    ForgeQueryDeclarationEntryOrchestrationCostPosture,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationMaterializationGate,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStepDisposition,
};

use super::super::lower::forge_query_lower_declaration_entry_orchestration_on_handle;
use super::domain::{admitted_handle, AdmittedFamily, Input};

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
    assert_eq!(
        proof.plan().receipt_materialization_tier(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive
    );
    assert_eq!(
        proof.plan().envelope_materialization_tier(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive
    );
    assert_eq!(
        proof.plan().cost_posture(),
        ForgeQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyRich
    );
    assert_eq!(
        proof.plan().materialization_gate(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationGate::ExplicitRequestRequired
    );
    assert_eq!(
        proof.plan().foundational_evidence_profile(),
        FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness
    );
    assert!(proof.plan().descriptive_materialization_cost().is_some());
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
        stages[4].materialization_tier(),
        Some(ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive)
    );
    assert_eq!(
        stages[6].materialization_tier(),
        Some(ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive)
    );
    assert_eq!(
        stages[7].materialization_tier(),
        Some(ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive)
    );
    assert_eq!(
        proof.automation_boundary(),
        ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling
    );
    assert!(!proof.orchestration_digest().is_empty());
}

#[test]
fn canonical_lowering_tracks_publication_identity_across_exposure_levels() {
    let handle = admitted_handle("collaborative");
    let world_basis = handle.retained_world_basis();

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

    assert_ne!(
        ordinary.plan.orchestration_identity_digest(),
        checked.plan.orchestration_identity_digest()
    );
    assert_ne!(
        checked.plan.orchestration_identity_digest(),
        proof.plan.orchestration_identity_digest()
    );
    assert_ne!(
        ordinary.plan.orchestration_identity_digest(),
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
    assert_eq!(
        ordinary.plan.handle_identity_digest(),
        world_basis.handle_identity_for_reporting()
    );
    assert_eq!(
        ordinary.plan.operating_context_identity_digest(),
        world_basis.operating_context_identity_digest()
    );
    assert_eq!(
        ordinary.plan.foundational_evidence_profile(),
        FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics
    );
    assert_eq!(
        checked.plan.foundational_evidence_profile(),
        FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics
    );
    assert_eq!(
        proof.plan.foundational_evidence_profile(),
        FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness
    );
    assert_eq!(
        ordinary.plan.cost_posture(),
        ForgeQueryDeclarationEntryOrchestrationCostPosture::OrdinaryDefault
    );
    assert_eq!(
        checked.plan.cost_posture(),
        ForgeQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyLean
    );
    assert_eq!(
        proof.plan.cost_posture(),
        ForgeQueryDeclarationEntryOrchestrationCostPosture::ExplicitlyRich
    );
    assert_eq!(
        ordinary.plan.receipt_materialization_tier(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady
    );
    assert_eq!(
        checked.plan.receipt_materialization_tier(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean
    );
    assert_eq!(
        proof.plan.receipt_materialization_tier(),
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive
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
