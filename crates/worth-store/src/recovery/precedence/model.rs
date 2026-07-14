use crate::{
    failure::StoreError,
    publication::{DurableRecoveryPublicationObservation, PublicationClassification},
    wal::DurableMutationId,
};
use worth_relational::facade::{history::CommitId, replay::CanonicalCommitEnvelope};
use serde::Serialize;

use super::super::DurableMutationIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecoverySourceKind {
    PublishedAuthoritativeTruth,
    HostedRuntimeCanonicalResult,
    IntentOnly,
    RequiresRebuild,
    RequiresQuarantine,
    MaintenanceResidue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoverySourceReport {
    pub(crate) durable_mutation_id: DurableMutationId,
    pub(crate) mutation_identity: DurableMutationIdentity,
    pub(crate) source_kind: RecoverySourceKind,
    pub(crate) publication_classification: PublicationClassification,
    pub(crate) reason: String,
}

impl RecoverySourceReport {
    pub fn durable_mutation_id(&self) -> DurableMutationId {
        self.durable_mutation_id
    }
    pub fn source_kind(&self) -> RecoverySourceKind {
        self.source_kind
    }
    pub fn mutation_identity(&self) -> &DurableMutationIdentity {
        &self.mutation_identity
    }
    pub fn publication_classification(&self) -> PublicationClassification {
        self.publication_classification
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RecoverySourceSet {
    pub(crate) durable_mutation_id: DurableMutationId,
    pub(crate) mutation_identity: DurableMutationIdentity,
    pub(crate) observation: DurableRecoveryPublicationObservation,
}

impl RecoverySourceSet {
    pub(crate) fn durable_mutation_id(&self) -> DurableMutationId {
        self.durable_mutation_id
    }
    pub(crate) fn observation(&self) -> &DurableRecoveryPublicationObservation {
        &self.observation
    }
    pub(crate) fn mutation_identity(&self) -> &DurableMutationIdentity {
        &self.mutation_identity
    }
}

pub(crate) struct RecoverySourceSelection {
    pub(crate) source_kind: RecoverySourceKind,
    pub(crate) commit_id: Option<CommitId>,
    pub(crate) canonical_envelope: Option<CanonicalCommitEnvelope>,
    pub(crate) acknowledgment_eligible: bool,
    pub(crate) report: RecoverySourceReport,
}

impl RecoverySourceSelection {
    pub(crate) fn source_kind(&self) -> RecoverySourceKind {
        self.source_kind
    }
    pub(crate) fn commit_id(&self) -> Option<CommitId> {
        self.commit_id
    }
    pub(crate) fn canonical_envelope(&self) -> Option<&CanonicalCommitEnvelope> {
        self.canonical_envelope.as_ref()
    }
    pub(crate) fn acknowledgment_eligible(&self) -> bool {
        self.acknowledgment_eligible
    }
    pub(crate) fn report(&self) -> &RecoverySourceReport {
        &self.report
    }
}

#[allow(dead_code)]
fn _assert_store_error_usage(_: StoreError) {}
