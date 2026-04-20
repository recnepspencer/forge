use forge_relational::facade::history::CommitId;
use serde::Serialize;

use crate::{bulk::BulkPlanKind, RecoveryDecisionClass};

use super::super::{DurableMutationIdentity, DurableRecoveryOutcome, RecoverySourceKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BulkRecoverySummary {
    total_chunks: usize,
    ingest_chunks: usize,
    transform_chunks: usize,
    resume_ready: usize,
    already_published: usize,
    discardable_intent: usize,
    needs_rebuild: usize,
    needs_quarantine: usize,
    published_authoritative_truth: usize,
    hosted_runtime_canonical_result: usize,
    intent_only: usize,
    requires_rebuild: usize,
    requires_quarantine: usize,
    maintenance_residue: usize,
}

impl BulkRecoverySummary {
    pub(crate) fn from_bulk_chunks(chunks: &[RecoveredBulkChunk]) -> Self {
        let mut summary = Self {
            total_chunks: 0,
            ingest_chunks: 0,
            transform_chunks: 0,
            resume_ready: 0,
            already_published: 0,
            discardable_intent: 0,
            needs_rebuild: 0,
            needs_quarantine: 0,
            published_authoritative_truth: 0,
            hosted_runtime_canonical_result: 0,
            intent_only: 0,
            requires_rebuild: 0,
            requires_quarantine: 0,
            maintenance_residue: 0,
        };

        for chunk in chunks {
            summary.total_chunks += 1;
            match chunk.plan_kind() {
                BulkPlanKind::Ingest => summary.ingest_chunks += 1,
                BulkPlanKind::Transform => summary.transform_chunks += 1,
            }
            match chunk.disposition() {
                BulkRecoveryDisposition::ResumeReady => summary.resume_ready += 1,
                BulkRecoveryDisposition::AlreadyPublished => summary.already_published += 1,
                BulkRecoveryDisposition::DiscardableIntent => summary.discardable_intent += 1,
                BulkRecoveryDisposition::NeedsRebuild => summary.needs_rebuild += 1,
                BulkRecoveryDisposition::NeedsQuarantine => summary.needs_quarantine += 1,
            }
            match chunk.source_kind() {
                RecoverySourceKind::PublishedAuthoritativeTruth => {
                    summary.published_authoritative_truth += 1
                }
                RecoverySourceKind::HostedRuntimeCanonicalResult => {
                    summary.hosted_runtime_canonical_result += 1
                }
                RecoverySourceKind::IntentOnly => summary.intent_only += 1,
                RecoverySourceKind::RequiresRebuild => summary.requires_rebuild += 1,
                RecoverySourceKind::RequiresQuarantine => summary.requires_quarantine += 1,
                RecoverySourceKind::MaintenanceResidue => summary.maintenance_residue += 1,
            }
        }

        summary
    }

    pub fn total_chunks(&self) -> usize { self.total_chunks }
    pub fn ingest_chunks(&self) -> usize { self.ingest_chunks }
    pub fn transform_chunks(&self) -> usize { self.transform_chunks }
    pub fn resume_ready(&self) -> usize { self.resume_ready }
    pub fn already_published(&self) -> usize { self.already_published }
    pub fn discardable_intent(&self) -> usize { self.discardable_intent }
    pub fn needs_rebuild(&self) -> usize { self.needs_rebuild }
    pub fn needs_quarantine(&self) -> usize { self.needs_quarantine }
    pub fn published_authoritative_truth(&self) -> usize { self.published_authoritative_truth }
    pub fn hosted_runtime_canonical_result(&self) -> usize { self.hosted_runtime_canonical_result }
    pub fn intent_only(&self) -> usize { self.intent_only }
    pub fn requires_rebuild(&self) -> usize { self.requires_rebuild }
    pub fn requires_quarantine(&self) -> usize { self.requires_quarantine }
    pub fn maintenance_residue(&self) -> usize { self.maintenance_residue }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoveredBulkChunk {
    durable_mutation_id: crate::DurableMutationId,
    plan_kind: BulkPlanKind,
    program_id: String,
    plan_id: String,
    chunk_ordinal: u64,
    disposition: BulkRecoveryDisposition,
    source_kind: RecoverySourceKind,
    decision: Option<RecoveryDecisionClass>,
    commit_id: Option<CommitId>,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeEligibleRecoveredBulkChunk {
    recovered: RecoveredBulkChunk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BulkRecoveryDisposition {
    ResumeReady,
    AlreadyPublished,
    DiscardableIntent,
    NeedsRebuild,
    NeedsQuarantine,
}

impl RecoveredBulkChunk {
    pub(crate) fn collect_from_outcome(outcome: &DurableRecoveryOutcome) -> Vec<Self> {
        outcome
            .source_reports
            .iter()
            .filter_map(|report| {
                let DurableMutationIdentity::BulkChunk {
                    plan_kind,
                    program_id,
                    plan_id,
                    chunk_ordinal,
                } = report.mutation_identity()
                else {
                    return None;
                };
                let decision = outcome
                    .decisions
                    .iter()
                    .find(|decision| decision.durable_mutation_id == report.durable_mutation_id());
                Some(Self {
                    durable_mutation_id: report.durable_mutation_id(),
                    plan_kind: *plan_kind,
                    program_id: program_id.clone(),
                    plan_id: plan_id.clone(),
                    chunk_ordinal: *chunk_ordinal,
                    disposition: disposition_for_source_kind(report.source_kind()),
                    source_kind: report.source_kind(),
                    decision: decision.map(|decision| decision.decision),
                    commit_id: decision.and_then(|decision| decision.commit_id),
                    reason: report.reason().to_string(),
                })
            })
            .collect()
    }

    pub fn durable_mutation_id(&self) -> crate::DurableMutationId { self.durable_mutation_id }
    pub fn plan_kind(&self) -> BulkPlanKind { self.plan_kind }
    pub fn program_id(&self) -> &str { &self.program_id }
    pub fn plan_id(&self) -> &str { &self.plan_id }
    pub fn chunk_ordinal(&self) -> u64 { self.chunk_ordinal }
    pub fn disposition(&self) -> BulkRecoveryDisposition { self.disposition }
    pub fn source_kind(&self) -> RecoverySourceKind { self.source_kind }
    pub fn decision(&self) -> Option<RecoveryDecisionClass> { self.decision }
    pub fn commit_id(&self) -> Option<CommitId> { self.commit_id }
    pub fn reason(&self) -> &str { &self.reason }

    pub fn admit_resume(&self) -> Result<ResumeEligibleRecoveredBulkChunk, crate::StoreError> {
        match self.disposition() {
            BulkRecoveryDisposition::ResumeReady | BulkRecoveryDisposition::AlreadyPublished => {
                Ok(ResumeEligibleRecoveredBulkChunk {
                    recovered: self.clone(),
                })
            }
            BulkRecoveryDisposition::DiscardableIntent => Err(crate::StoreError::new(
                crate::StoreErrorKind::BulkResumeBoundaryAmbiguous,
                format!(
                    "bulk chunk {} for program `{}` was only admitted as discardable intent and cannot be resumed",
                    self.chunk_ordinal(),
                    self.program_id()
                ),
            )),
            BulkRecoveryDisposition::NeedsRebuild => Err(crate::StoreError::new(
                crate::StoreErrorKind::BulkResumeBoundaryAmbiguous,
                format!(
                    "bulk chunk {} for program `{}` requires rebuild before resume",
                    self.chunk_ordinal(),
                    self.program_id()
                ),
            )),
            BulkRecoveryDisposition::NeedsQuarantine => Err(crate::StoreError::new(
                crate::StoreErrorKind::BulkResumeBoundaryAmbiguous,
                format!(
                    "bulk chunk {} for program `{}` requires quarantine before resume",
                    self.chunk_ordinal(),
                    self.program_id()
                ),
            )),
        }
    }
}

impl ResumeEligibleRecoveredBulkChunk {
    pub fn recovered(&self) -> &RecoveredBulkChunk {
        &self.recovered
    }
}

fn disposition_for_source_kind(source_kind: RecoverySourceKind) -> BulkRecoveryDisposition {
    match source_kind {
        RecoverySourceKind::PublishedAuthoritativeTruth => {
            BulkRecoveryDisposition::AlreadyPublished
        }
        RecoverySourceKind::HostedRuntimeCanonicalResult => BulkRecoveryDisposition::ResumeReady,
        RecoverySourceKind::IntentOnly => BulkRecoveryDisposition::DiscardableIntent,
        RecoverySourceKind::RequiresRebuild => BulkRecoveryDisposition::NeedsRebuild,
        RecoverySourceKind::RequiresQuarantine | RecoverySourceKind::MaintenanceResidue => {
            BulkRecoveryDisposition::NeedsQuarantine
        }
    }
}
