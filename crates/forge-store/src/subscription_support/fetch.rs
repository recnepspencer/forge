use super::{
    SubscriptionSupportArtifactId, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportStoredRecordSet,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportFetchRequest {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    artifact_id: SubscriptionSupportArtifactId,
}

impl SubscriptionSupportFetchRequest {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        artifact_id: SubscriptionSupportArtifactId,
    ) -> Self {
        Self {
            family_id,
            family_kind,
            artifact_id,
        }
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportFetchReport {
    lookup_key_count: u64,
    rows_read: u64,
    global_scan_count: u64,
    access_structure_debt: bool,
}

impl SubscriptionSupportFetchReport {
    pub(crate) fn direct_lookup(rows_read: u64) -> Self {
        Self {
            lookup_key_count: 1,
            rows_read,
            global_scan_count: 0,
            access_structure_debt: false,
        }
    }

    pub fn lookup_key_count(&self) -> u64 {
        self.lookup_key_count
    }

    pub fn rows_read(&self) -> u64 {
        self.rows_read
    }

    pub fn global_scan_count(&self) -> u64 {
        self.global_scan_count
    }

    pub fn access_structure_debt(&self) -> bool {
        self.access_structure_debt
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FetchedSubscriptionSupportArtifact {
    record_set: SubscriptionSupportStoredRecordSet,
    fetch_report: SubscriptionSupportFetchReport,
}

impl FetchedSubscriptionSupportArtifact {
    pub(crate) fn new(
        record_set: SubscriptionSupportStoredRecordSet,
        fetch_report: SubscriptionSupportFetchReport,
    ) -> Self {
        Self {
            record_set,
            fetch_report,
        }
    }

    pub fn record_set(&self) -> &SubscriptionSupportStoredRecordSet {
        &self.record_set
    }

    pub fn fetch_report(&self) -> &SubscriptionSupportFetchReport {
        &self.fetch_report
    }
}
