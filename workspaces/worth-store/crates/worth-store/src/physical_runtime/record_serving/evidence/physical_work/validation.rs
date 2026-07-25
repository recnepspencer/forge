use std::collections::BTreeSet;

use crate::physical_runtime::{PhysicalWorkIdentity, PhysicalWorkShutdownObservation};

use super::{
    super::super::c6_handoff::C6PhysicalWorkHandoffIdentity, PhysicalWorkArtifactBinding,
    PhysicalWorkCourtroomFinding, PhysicalWorkMutantLocalization, PhysicalWorkOracleEvidence,
};

pub(super) fn validate_execution(
    expected: C6PhysicalWorkHandoffIdentity,
    records: &[crate::physical_runtime::PhysicalWorkCausalRecord],
    causal_overflow: u64,
    terminal: &PhysicalWorkShutdownObservation,
    artifacts: &[PhysicalWorkArtifactBinding],
    oracle: &PhysicalWorkOracleEvidence,
    mutants: &[PhysicalWorkMutantLocalization],
) -> Vec<PhysicalWorkCourtroomFinding> {
    let mut findings = Vec::new();
    if records.is_empty() {
        findings.push(PhysicalWorkCourtroomFinding::MissingCausalRecord);
    }
    if causal_overflow != 0 {
        findings.push(PhysicalWorkCourtroomFinding::CausalEvidenceOverflow);
    }
    validate_shutdown(terminal, &mut findings);
    validate_artifacts(artifacts, &mut findings);
    if !oracle.accepted() {
        findings.push(PhysicalWorkCourtroomFinding::OracleRejected);
    }
    if mutants.is_empty() {
        findings.push(PhysicalWorkCourtroomFinding::MissingMutantLocalization);
    } else if mutants.iter().any(|mutant| !mutant.killed()) {
        findings.push(PhysicalWorkCourtroomFinding::MutantSurvived);
    }
    for record in records {
        validate_identity(expected, record.identity(), &mut findings);
    }
    for observation in terminal.terminal() {
        validate_identity(expected, observation.identity(), &mut findings);
    }
    findings
}

pub(super) fn validate_identity(
    expected: C6PhysicalWorkHandoffIdentity,
    actual: PhysicalWorkIdentity,
    findings: &mut Vec<PhysicalWorkCourtroomFinding>,
) {
    if actual.store() != expected.store() {
        findings.push(PhysicalWorkCourtroomFinding::ForeignStoreIdentity);
    }
    if actual.runtime() != expected.runtime() {
        findings.push(PhysicalWorkCourtroomFinding::ForeignRuntimeIdentity);
    }
    if actual.generation().lifecycle() != expected.generation() {
        findings.push(PhysicalWorkCourtroomFinding::ForeignLifecycleGeneration);
    }
}

fn validate_shutdown(
    terminal: &PhysicalWorkShutdownObservation,
    findings: &mut Vec<PhysicalWorkCourtroomFinding>,
) {
    if terminal.residual() != 0 {
        findings.push(PhysicalWorkCourtroomFinding::ShutdownResidual);
    }
    if terminal.unaccounted_terminal() != 0 {
        findings.push(PhysicalWorkCourtroomFinding::ShutdownOvercount);
    }
    if terminal.drain().evidence_overflow() != 0 {
        findings.push(PhysicalWorkCourtroomFinding::DrainEvidenceOverflow);
    }
    if !terminal.drain().residual().is_empty() {
        findings.push(PhysicalWorkCourtroomFinding::DrainResidual);
    }
}

fn validate_artifacts(
    artifacts: &[PhysicalWorkArtifactBinding],
    findings: &mut Vec<PhysicalWorkCourtroomFinding>,
) {
    if artifacts.is_empty() {
        findings.push(PhysicalWorkCourtroomFinding::MissingArtifactManifest);
        return;
    }
    let mut paths = BTreeSet::new();
    if artifacts
        .iter()
        .any(|artifact| !paths.insert(artifact.path()))
    {
        findings.push(PhysicalWorkCourtroomFinding::DuplicateArtifactPath);
    }
}
