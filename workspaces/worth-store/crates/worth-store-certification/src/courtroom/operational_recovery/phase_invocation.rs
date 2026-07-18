use sha2::{Digest, Sha256};
use worth_store_offline_verifier::OperationalTruthReport;
use worth_store_operations::{OperationalControlRecord, SelectedOperationalControlState};

use super::{S10OperationalScenarioKind, S10Phase};

mod record_artifacts;
use record_artifacts::{
    authorization_identity, backup_bundle_identity, backup_cut_identity, publication_identity,
    repair_execution_identity, repair_plan_identity, replica_identity, staged_workflow_identity,
};

pub(super) fn require_runtime_phase_artifact(
    kind: S10OperationalScenarioKind,
    phase: u8,
    records: &[OperationalControlRecord],
) -> Result<(), S10PhaseInvocationDenial> {
    match phase {
        5 => backup_cut_identity(records).map(drop),
        6 => backup_bundle_identity(records).map(drop),
        7 => authorization_identity(records).map(drop),
        8 => staged_workflow_identity(records, 1).map(drop),
        9 => staged_workflow_identity(records, 2).map(drop),
        10 => staged_workflow_identity(records, 3).map(drop),
        11 => repair_plan_identity(records).map(drop),
        12 => repair_execution_identity(records).map(drop),
        13 => publication_identity(records).map(drop),
        14 => replica_identity(kind, records).map(drop),
        _ => Err(S10PhaseInvocationDenial::EmptyProductionArtifact(S10Phase(
            phase,
        ))),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct S10ScenarioProductionEvidence<'a> {
    control: &'a SelectedOperationalControlState,
    truth: &'a OperationalTruthReport,
}

impl<'a> S10ScenarioProductionEvidence<'a> {
    pub const fn new(
        control: &'a SelectedOperationalControlState,
        truth: &'a OperationalTruthReport,
    ) -> Self {
        Self { control, truth }
    }

    pub(super) const fn control(self) -> &'a SelectedOperationalControlState {
        self.control
    }
    pub(super) fn control_records(self) -> &'a [OperationalControlRecord] {
        self.control.durable_records()
    }
    pub(super) const fn truth(self) -> &'a OperationalTruthReport {
        self.truth
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S10PhaseInvocationEvidence {
    phase: S10Phase,
    production_artifact_identity: [u8; 32],
    localization_members: Vec<[u8; 32]>,
}

impl S10PhaseInvocationEvidence {
    pub const fn phase(&self) -> S10Phase {
        self.phase
    }
    pub const fn production_artifact_identity(&self) -> [u8; 32] {
        self.production_artifact_identity
    }

    pub fn localization_members(&self) -> &[[u8; 32]] {
        &self.localization_members
    }
}

pub(super) fn derive_phase_invocations(
    kind: S10OperationalScenarioKind,
    production: S10ScenarioProductionEvidence<'_>,
    later_phase_identities: [[u8; 32]; 5],
) -> Result<Vec<S10PhaseInvocationEvidence>, S10PhaseInvocationDenial> {
    validate_selected_history(production)?;
    let records = production.control_records();
    let truth = production.truth();
    let mut phases = vec![
        invocation(
            2,
            selected_control_identity(production.control()),
            selected_control_members(production.control()),
        )?,
        invocation(3, truth.source_inspection_identity(), Vec::new())?,
        invocation(4, truth.truth_evidence_identity(), Vec::new())?,
        record_invocation(5, backup_cut_identity(records)?)?,
        record_invocation(6, backup_bundle_identity(records)?)?,
        record_invocation(7, authorization_identity(records)?)?,
        record_invocation(8, staged_workflow_identity(records, 1)?)?,
        record_invocation(9, staged_workflow_identity(records, 2)?)?,
    ];
    if kind != S10OperationalScenarioKind::SplitBrainPromotion {
        phases.push(record_invocation(
            10,
            staged_workflow_identity(records, 3)?,
        )?);
    }
    if kind != S10OperationalScenarioKind::BurningPrimary {
        phases.push(record_invocation(11, repair_plan_identity(records)?)?);
        phases.push(record_invocation(12, repair_execution_identity(records)?)?);
    }
    phases.push(record_invocation(13, publication_identity(records)?)?);
    if kind != S10OperationalScenarioKind::AuthorityRepairRollback {
        phases.push(record_invocation(14, replica_identity(kind, records)?)?);
    }
    for (phase, identity) in (15_u8..=19).zip(later_phase_identities) {
        phases.push(invocation(phase, identity, Vec::new())?);
    }
    phases.sort_by_key(|evidence| evidence.phase.number());
    Ok(phases)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10PhaseInvocationDenial {
    EmptyProductionArtifact(S10Phase),
    SelectedControlHistoryEmpty,
    SelectedControlRecordCountMismatch,
    ForeignControlAuthority,
    MissingBackupCut,
    MissingBackupBundleLifecycle,
    MissingAuthorization,
    MissingStagedWorkflow(u8),
    MissingRepairPlan,
    MissingRepairExecution,
    MissingPublicationLifecycle,
    MissingReplicaLifecycle,
}

fn invocation(
    phase: u8,
    production_artifact_identity: [u8; 32],
    localization_members: Vec<[u8; 32]>,
) -> Result<S10PhaseInvocationEvidence, S10PhaseInvocationDenial> {
    let phase = S10Phase(phase);
    if production_artifact_identity == [0; 32] {
        return Err(S10PhaseInvocationDenial::EmptyProductionArtifact(phase));
    }
    Ok(S10PhaseInvocationEvidence {
        phase,
        production_artifact_identity,
        localization_members,
    })
}

fn record_invocation(
    phase: u8,
    artifacts: record_artifacts::PhaseRecordArtifacts,
) -> Result<S10PhaseInvocationEvidence, S10PhaseInvocationDenial> {
    let identity = artifacts.identity();
    invocation(phase, identity, artifacts.into_localization_members())
}

fn validate_selected_history(
    production: S10ScenarioProductionEvidence<'_>,
) -> Result<(), S10PhaseInvocationDenial> {
    let history = production.control().history_summary();
    if history.record_count() == 0 {
        return Err(S10PhaseInvocationDenial::SelectedControlHistoryEmpty);
    }
    if history.record_count() != production.control_records().len() as u64 {
        return Err(S10PhaseInvocationDenial::SelectedControlRecordCountMismatch);
    }
    let authority = production
        .control()
        .selected_generation()
        .authority_identity();
    if production
        .control_records()
        .iter()
        .any(|record| record.authority_identity() != authority)
    {
        return Err(S10PhaseInvocationDenial::ForeignControlAuthority);
    }
    Ok(())
}

fn selected_control_identity(control: &SelectedOperationalControlState) -> [u8; 32] {
    let selected = control.selected_generation();
    let history = control.history_summary();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-selected-control-evidence-v1");
    digest.update(selected.authority_identity().fingerprint());
    digest.update(selected.media_identity_fingerprint());
    digest.update(selected.generation().get().to_be_bytes());
    digest.update(selected.prefix_digest());
    digest.update(history.record_count().to_be_bytes());
    digest.update(history.completed_backups().to_be_bytes());
    digest.update(history.abandoned_backups().to_be_bytes());
    digest.finalize().into()
}

fn selected_control_members(control: &SelectedOperationalControlState) -> Vec<[u8; 32]> {
    let selected = control.selected_generation();
    vec![
        selected.authority_identity().fingerprint(),
        selected.media_identity_fingerprint(),
        selected.prefix_digest(),
        control_generation_identity(selected.generation().get()),
    ]
}

pub(super) fn control_generation_identity(generation: u64) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-control-generation-identity-v1");
    digest.update(generation.to_be_bytes());
    digest.finalize().into()
}
