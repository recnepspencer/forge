use sha2::{Digest, Sha256};

use super::{
    assemble_operational_audit_records, AuditCompletenessReceipt, OperationalAuditRecord,
    OperationalAuditTransitionKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalEvidenceExportDenial {
    InvalidCanonicalRecords,
    CompletenessBindingMismatch,
}

/// One typed row in the canonical operational evidence export.
///
/// Rendering this row for a terminal or transport is deliberately left to a
/// one-way projection boundary. The ordinary operational lane retains the
/// semantic values used to derive the export identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalEvidenceExportRow {
    sequence: u64,
    transition_id: String,
    transition_kind: OperationalAuditTransitionKind,
    record_identity: [u8; 32],
}

impl OperationalEvidenceExportRow {
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn transition_id(&self) -> &str {
        &self.transition_id
    }

    pub const fn transition_kind(&self) -> OperationalAuditTransitionKind {
        self.transition_kind
    }

    pub const fn record_identity(&self) -> [u8; 32] {
        self.record_identity
    }

    fn bind_identity(&self, hasher: &mut Sha256) {
        hasher.update(self.sequence.to_le_bytes());
        bind_bytes(hasher, self.transition_id.as_bytes());
        hasher.update([transition_kind_tag(self.transition_kind)]);
        hasher.update(self.record_identity);
    }
}

/// A complete, canonical, typed export of operational audit truth.
///
/// It has no readmission path into authorization or current authority. JSON,
/// log, and other human-facing representations must consume these rows at an
/// explicitly terminal projection boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalEvidenceExport {
    operation_identity: [u8; 32],
    completeness_terminal_identity: [u8; 32],
    export_identity: [u8; 32],
    rows: Vec<OperationalEvidenceExportRow>,
}

impl OperationalEvidenceExport {
    pub fn from_complete_audit(
        completeness: &AuditCompletenessReceipt,
        deliveries: &[OperationalAuditRecord],
    ) -> Result<Self, OperationalEvidenceExportDenial> {
        let assembled = assemble_operational_audit_records(deliveries.iter().cloned())
            .map_err(|_| OperationalEvidenceExportDenial::InvalidCanonicalRecords)?;
        let records = assembled
            .iter()
            .filter(|record| record.operation_id() == completeness.operation_id())
            .collect::<Vec<_>>();
        if records.len() as u64 != completeness.transition_count()
            || records.last().map(|record| record.record_identity())
                != Some(completeness.terminal_record_identity())
        {
            return Err(OperationalEvidenceExportDenial::CompletenessBindingMismatch);
        }

        let rows = records
            .into_iter()
            .map(|record| OperationalEvidenceExportRow {
                sequence: record.sequence().get(),
                transition_id: record.transition_id().as_str().to_owned(),
                transition_kind: record.transition_kind(),
                record_identity: record.record_identity(),
            })
            .collect::<Vec<_>>();
        let operation_identity = completeness.operation_id().stable_fingerprint();
        let mut hasher = Sha256::new();
        hasher.update(b"worth-store-operational-evidence-export-v2");
        hasher.update(operation_identity);
        hasher.update(completeness.terminal_record_identity());
        hasher.update((rows.len() as u64).to_le_bytes());
        for row in &rows {
            row.bind_identity(&mut hasher);
        }
        let export_identity = hasher.finalize().into();

        Ok(Self {
            operation_identity,
            completeness_terminal_identity: completeness.terminal_record_identity(),
            export_identity,
            rows,
        })
    }

    pub const fn operation_identity(&self) -> [u8; 32] {
        self.operation_identity
    }

    pub const fn completeness_terminal_identity(&self) -> [u8; 32] {
        self.completeness_terminal_identity
    }

    pub const fn export_identity(&self) -> [u8; 32] {
        self.export_identity
    }

    pub fn rows(&self) -> &[OperationalEvidenceExportRow] {
        &self.rows
    }
}

fn bind_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

const fn transition_kind_tag(kind: OperationalAuditTransitionKind) -> u8 {
    use OperationalAuditTransitionKind as Kind;
    match kind {
        Kind::WorkflowOpened => 1,
        Kind::SourceLeasePersisted => 2,
        Kind::MaterializationOpened => 3,
        Kind::MaterializationRecorded => 4,
        Kind::IndependentVerificationRecorded => 5,
        Kind::Abandoned => 6,
        Kind::AuthorizationConsumed => 7,
        Kind::OwnerExecutionOpened => 8,
        Kind::OwnerEffectStarted => 9,
        Kind::OwnerReceiptPersisted => 10,
        Kind::DispositionRecorded => 11,
        Kind::StagingCompleted => 12,
        Kind::ReplicaBootstrapTransferRecorded => 17,
        Kind::ReplicaPromotionFenceRecorded => 18,
        Kind::ReplicaPromotionRecorded => 19,
        Kind::ReplicaBootstrapCompleted => 20,
        Kind::ReplicaPromotionPublished => 21,
        Kind::ReplicaPromotionReadmitted => 22,
        Kind::OldPrimaryRejoinPlanned => 23,
        Kind::OldPrimaryRejoinCompleted => 24,
    }
}
