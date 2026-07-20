use sha2::{Digest, Sha256};
use worth_store_operations::OperationalControlRecord;

pub(in crate::courtroom::operational_recovery::phase_invocation) struct PhaseRecordArtifacts {
    pub(super) identity: [u8; 32],
    pub(super) localization_members: Vec<[u8; 32]>,
}

impl PhaseRecordArtifacts {
    pub(in crate::courtroom::operational_recovery::phase_invocation) const fn identity(
        &self,
    ) -> [u8; 32] {
        self.identity
    }

    pub(in crate::courtroom::operational_recovery::phase_invocation) fn into_localization_members(
        self,
    ) -> Vec<[u8; 32]> {
        self.localization_members
    }
}

pub(super) fn operation_with(
    records: &[OperationalControlRecord],
    accepts: impl Fn(&[&OperationalControlRecord]) -> bool,
) -> Option<String> {
    let mut operations = records
        .iter()
        .map(|record| record.operation_id().as_str())
        .collect::<Vec<_>>();
    operations.sort_unstable();
    operations.dedup();
    operations.into_iter().find_map(|operation| {
        let operation_records = records
            .iter()
            .filter(|record| record.operation_id().as_str() == operation)
            .collect::<Vec<_>>();
        accepts(&operation_records).then(|| operation.to_owned())
    })
}

pub(super) fn operation_record_identity(
    records: &[OperationalControlRecord],
    operation: &str,
    domain: &[u8],
    include: impl Fn(&OperationalControlRecord) -> bool,
) -> PhaseRecordArtifacts {
    record_set_identity(records, domain, |record| {
        record.operation_id().as_str() == operation && include(record)
    })
    .expect("the selected operation has required records")
}

pub(super) fn record_set_identity(
    records: &[OperationalControlRecord],
    domain: &[u8],
    include: impl Fn(&OperationalControlRecord) -> bool,
) -> Option<PhaseRecordArtifacts> {
    let identities = matching_record_fingerprints(records, include);
    if identities.is_empty() {
        return None;
    }
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-phase-production-artifacts-v2");
    digest.update((domain.len() as u64).to_be_bytes());
    digest.update(domain);
    digest.update((identities.len() as u64).to_be_bytes());
    for identity in &identities {
        digest.update(identity);
    }
    Some(PhaseRecordArtifacts {
        identity: digest.finalize().into(),
        localization_members: identities,
    })
}

pub(super) fn matching_record_fingerprints(
    records: &[OperationalControlRecord],
    include: impl Fn(&OperationalControlRecord) -> bool,
) -> Vec<[u8; 32]> {
    let mut identities = records
        .iter()
        .filter(|record| include(record))
        .map(OperationalControlRecord::stable_fingerprint)
        .collect::<Vec<_>>();
    identities.sort_unstable();
    identities
}
