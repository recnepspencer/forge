use crate::{
    failure::{StoreError, StoreErrorKind},
    FetchedSubscriptionSupportArtifact, SubscriptionSupportCatalog,
    SubscriptionSupportClassificationReport, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportFamilyKind, SubscriptionSupportFetchReport,
    SubscriptionSupportRestartReconstructionReport,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportRestartShard,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest,
    SubscriptionSupportStoredRecordSet,
};
use std::collections::BTreeMap;

use super::super::{StateBackedStoreBackend, StatePersistence};

pub(super) struct RestartShardAdmission {
    family_kind: SubscriptionSupportFamilyKind,
    shard_key: String,
    prefix: String,
    upper: String,
    max_support_rows: u64,
}

pub(super) fn admit_restart_shard(
    request: &SubscriptionSupportRestartReconstructionRequest,
) -> Result<RestartShardAdmission, StoreError> {
    if SubscriptionSupportCatalog::first_ship()
        .density_for(request.shard().family_kind())
        .is_none()
    {
        return Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportClassificationViolation,
            "subscription-support restart reconstruction requires an admitted catalog family",
        ));
    }
    let family_id = request.shard().family_id().as_str();
    let prefix = format!("{family_id}\u{1f}");
    let upper = format!("{family_id}\u{1f}\u{10ffff}");
    Ok(RestartShardAdmission {
        family_kind: request.shard().family_kind(),
        shard_key: request.shard().shard_key(),
        prefix,
        upper,
        max_support_rows: request.max_support_rows(),
    })
}

pub(super) struct BoundedRestartShardRecords {
    pub(super) records: Vec<SubscriptionSupportStoredRecordSet>,
    pub(super) support_rows_read: u64,
}

pub(super) fn load_bounded_restart_shard_records(
    record_sets: &BTreeMap<String, SubscriptionSupportStoredRecordSet>,
    admission: &RestartShardAdmission,
) -> Result<BoundedRestartShardRecords, StoreError> {
    let records = record_sets
        .range(admission.prefix.clone()..admission.upper.clone())
        .filter(|(storage_key, _)| storage_key.starts_with(&admission.prefix))
        .filter(|(_, record_set)| record_set.restart_shard() == Some(admission.shard_key.as_str()))
        .map(|(_, record_set)| record_set.clone())
        .collect::<Vec<_>>();
    if records.len() as u64 > admission.max_support_rows {
        return Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportClassificationViolation,
            "subscription-support restart shard exceeded its bounded reconstruction row budget",
        ));
    }
    Ok(BoundedRestartShardRecords {
        support_rows_read: records.len() as u64,
        records,
    })
}

pub(super) fn classify_restart_shard_records<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    bounded_records: BoundedRestartShardRecords,
    admission: &RestartShardAdmission,
) -> Result<Vec<SubscriptionSupportClassificationReport>, StoreError> {
    bounded_records
        .records
        .into_iter()
        .map(|record_set| classify_restart_record(backend, record_set, admission))
        .collect()
}

fn classify_restart_record<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    record_set: SubscriptionSupportStoredRecordSet,
    admission: &RestartShardAdmission,
) -> Result<SubscriptionSupportClassificationReport, StoreError> {
    if record_set.family_kind() != admission.family_kind {
        return Err(StoreError::new(
            StoreErrorKind::SubscriptionSupportClassificationViolation,
            "subscription-support restart shard rejected cross-family-kind record reuse",
        ));
    }
    let fetched = FetchedSubscriptionSupportArtifact::new(
        record_set.clone(),
        SubscriptionSupportFetchReport::direct_lookup(1),
    );
    let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 0, true)?;
    let plan =
        super::resume_plans::restart_plan_for_record(&record_set, admission.shard_key.clone())?;
    backend.classify_subscription_support_resume(SubscriptionSupportResumeRequest::new(
        fetched, evidence, plan,
    ))
}

pub(super) fn publish_restart_reconstruction(
    counter_snapshot: &mut SubscriptionSupportCounterSnapshot,
    shard: SubscriptionSupportRestartShard,
    reports: Vec<SubscriptionSupportClassificationReport>,
    support_rows_read: u64,
) -> SubscriptionSupportRestartReconstructionReport {
    counter_snapshot.record_restart_reconstruction(1);
    SubscriptionSupportRestartReconstructionReport::new(shard, reports, support_rows_read)
}
