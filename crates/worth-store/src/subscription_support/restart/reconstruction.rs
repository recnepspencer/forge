use super::super::{
    classification_error, SubscriptionSupportClassificationReport, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRestartShard {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
}

impl SubscriptionSupportRestartShard {
    pub fn for_family(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
    ) -> Self {
        Self {
            family_id,
            family_kind,
        }
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub(crate) fn shard_key(&self) -> String {
        format!("subscription-support-restart:{}", self.family_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRestartReconstructionRequest {
    shard: SubscriptionSupportRestartShard,
    max_support_rows: u64,
}

impl SubscriptionSupportRestartReconstructionRequest {
    pub fn new(
        shard: SubscriptionSupportRestartShard,
        max_support_rows: u64,
    ) -> Result<Self, StoreError> {
        if max_support_rows == 0 {
            return Err(classification_error(
                "subscription-support restart reconstruction requires a non-zero row bound",
            ));
        }
        Ok(Self {
            shard,
            max_support_rows,
        })
    }

    pub(crate) fn shard(&self) -> &SubscriptionSupportRestartShard {
        &self.shard
    }

    pub(crate) fn max_support_rows(&self) -> u64 {
        self.max_support_rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportRestartReconstructionReport {
    shard: SubscriptionSupportRestartShard,
    reports: Vec<SubscriptionSupportClassificationReport>,
    support_rows_read: u64,
    restart_shards_touched: u64,
    global_scan_count: u64,
}

impl SubscriptionSupportRestartReconstructionReport {
    pub(crate) fn new(
        shard: SubscriptionSupportRestartShard,
        reports: Vec<SubscriptionSupportClassificationReport>,
        support_rows_read: u64,
    ) -> Self {
        Self {
            shard,
            reports,
            support_rows_read,
            restart_shards_touched: 1,
            global_scan_count: 0,
        }
    }

    pub fn shard(&self) -> &SubscriptionSupportRestartShard {
        &self.shard
    }

    pub fn reports(&self) -> &[SubscriptionSupportClassificationReport] {
        &self.reports
    }

    pub fn support_rows_read(&self) -> u64 {
        self.support_rows_read
    }

    pub fn restart_shards_touched(&self) -> u64 {
        self.restart_shards_touched
    }

    pub fn global_scan_count(&self) -> u64 {
        self.global_scan_count
    }
}
