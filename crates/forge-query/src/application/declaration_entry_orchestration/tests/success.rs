use forge_foundational::facade::CanonicalDerivedDigest;

use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationChecked, ForgeQueryDeclarationEntryOrchestrationStage,
};

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
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            digest_text(envelope.envelope_digest())
        }
        _ => panic!("checked orchestration should envelope"),
    };
    let proof_digest = match proof.outcome() {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
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
    assert!(stages.iter().all(|record| record.is_reached()));
    assert_eq!(
        stages.last().expect("last stage should exist").stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
    assert!(!proof.orchestration_digest().is_empty());
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
