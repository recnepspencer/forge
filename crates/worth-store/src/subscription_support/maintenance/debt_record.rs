use super::super::{
    publication_error, stable_digest, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole, SupportActionId,
    SupportAffectedSetDigest,
};
use super::batch_plan::SupportMaintenanceBatchPlan;
use super::debt_report::SubscriptionSupportMaintenanceDebtReport;
use super::decision::SupportMaintenanceWorkKind;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportMaintenanceDebtRecord {
    record_key: String,
    action_id: SupportActionId,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    work_kind: SupportMaintenanceWorkKind,
    verdict: SubscriptionSupportOperationalVerdict,
    delay_reason: String,
    descriptor_count: u64,
    coalesced_duplicate_count: u64,
}

impl Serialize for SupportMaintenanceDebtRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PersistedSupportMaintenanceDebtRecord::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SupportMaintenanceDebtRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedSupportMaintenanceDebtRecord::deserialize(deserializer)?;
        Self::try_from(persisted).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedSupportMaintenanceDebtRecord {
    record_key: String,
    action_id: String,
    family_id: String,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: String,
    work_kind: String,
    verdict: String,
    delay_reason: String,
    descriptor_count: u64,
    coalesced_duplicate_count: u64,
}

impl SupportMaintenanceDebtRecord {
    pub(crate) fn from_plan_and_report(
        plan: &SupportMaintenanceBatchPlan,
        report: &SubscriptionSupportMaintenanceDebtReport,
    ) -> Result<Self, StoreError> {
        let record_key = stable_digest(&(
            plan.action_id().as_str(),
            plan.affected_set().family_id().as_str(),
            plan.affected_set().support_role(),
            report.debt_summary().affected_set_digest().as_str(),
            report.debt_summary().delay_reason(),
            report.debt_summary().work_kind(),
        ))?;
        Ok(Self {
            record_key,
            action_id: plan.action_id().clone(),
            family_id: plan.affected_set().family_id().clone(),
            family_kind: plan.affected_set().family_kind(),
            support_role: plan.affected_set().support_role(),
            affected_set_digest: report.debt_summary().affected_set_digest().clone(),
            work_kind: report.debt_summary().work_kind(),
            verdict: report.debt_summary().verdict(),
            delay_reason: report.debt_summary().delay_reason().to_string(),
            descriptor_count: report.debt_summary().descriptor_count(),
            coalesced_duplicate_count: report.debt_summary().coalesced_duplicate_count(),
        })
    }

    pub fn record_key(&self) -> &str {
        &self.record_key
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub(crate) fn validate(&self) -> Result<(), StoreError> {
        let expected_record_key = stable_digest(&(
            self.action_id.as_str(),
            self.family_id.as_str(),
            self.support_role,
            self.affected_set_digest.as_str(),
            self.delay_reason.as_str(),
            self.work_kind,
        ))?;
        if self.record_key != expected_record_key {
            return Err(publication_error(
                "subscription-support maintenance debt record key drifted from its debt identity",
            ));
        }
        if self.descriptor_count == 0 {
            return Err(publication_error(
                "subscription-support maintenance debt records require admitted descriptors",
            ));
        }
        if self.delay_reason.trim().is_empty() {
            return Err(publication_error(
                "subscription-support maintenance debt records require a non-empty delay reason",
            ));
        }
        let expected_verdict = match self.work_kind {
            SupportMaintenanceWorkKind::Rebuild => {
                SubscriptionSupportOperationalVerdict::RebuildRequired
            }
            SupportMaintenanceWorkKind::Refresh
            | SupportMaintenanceWorkKind::CompatibilityMigration => {
                SubscriptionSupportOperationalVerdict::ExactResumePreserved
            }
            SupportMaintenanceWorkKind::DegradationRecovery => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
        };
        if self.verdict != expected_verdict {
            return Err(publication_error(
                "subscription-support maintenance debt record verdict drifted from work-kind posture",
            ));
        }
        Ok(())
    }
}

impl From<&SupportMaintenanceDebtRecord> for PersistedSupportMaintenanceDebtRecord {
    fn from(record: &SupportMaintenanceDebtRecord) -> Self {
        Self {
            record_key: record.record_key.clone(),
            action_id: record.action_id.as_str().to_string(),
            family_id: record.family_id.as_str().to_string(),
            family_kind: record.family_kind,
            support_role: record.support_role,
            affected_set_digest: record.affected_set_digest.as_str().to_string(),
            work_kind: format!("{:?}", record.work_kind),
            verdict: format!("{:?}", record.verdict),
            delay_reason: record.delay_reason.clone(),
            descriptor_count: record.descriptor_count,
            coalesced_duplicate_count: record.coalesced_duplicate_count,
        }
    }
}

impl TryFrom<PersistedSupportMaintenanceDebtRecord> for SupportMaintenanceDebtRecord {
    type Error = String;

    fn try_from(record: PersistedSupportMaintenanceDebtRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            record_key: require_non_empty("debt record key", record.record_key)
                .map_err(|error| error.to_string())?,
            action_id: SupportActionId::new(record.action_id).map_err(|error| error.to_string())?,
            family_id: SubscriptionSupportFamilyId::new(record.family_id)
                .map_err(|error| error.to_string())?,
            family_kind: record.family_kind,
            support_role: record.support_role,
            affected_set_digest: SupportAffectedSetDigest::from_persisted(
                record.affected_set_digest,
            )
            .map_err(|error| error.to_string())?,
            work_kind: parse_persisted_maintenance_work_kind(&record.work_kind)?,
            verdict: parse_persisted_operational_verdict(&record.verdict)?,
            delay_reason: require_non_empty("delay reason", record.delay_reason)
                .map_err(|error| error.to_string())?,
            descriptor_count: record.descriptor_count,
            coalesced_duplicate_count: record.coalesced_duplicate_count,
        })
    }
}

fn parse_persisted_maintenance_work_kind(
    value: &str,
) -> Result<SupportMaintenanceWorkKind, String> {
    match value {
        "Rebuild" => Ok(SupportMaintenanceWorkKind::Rebuild),
        "Refresh" => Ok(SupportMaintenanceWorkKind::Refresh),
        "CompatibilityMigration" => Ok(SupportMaintenanceWorkKind::CompatibilityMigration),
        "DegradationRecovery" => Ok(SupportMaintenanceWorkKind::DegradationRecovery),
        other => Err(format!(
            "unknown subscription-support maintenance work kind `{other}`"
        )),
    }
}

fn parse_persisted_operational_verdict(
    value: &str,
) -> Result<SubscriptionSupportOperationalVerdict, String> {
    match value {
        "ExactResumePreserved" => Ok(SubscriptionSupportOperationalVerdict::ExactResumePreserved),
        "DegradedResumePreserved" => {
            Ok(SubscriptionSupportOperationalVerdict::DegradedResumePreserved)
        }
        "RebuildRequired" => Ok(SubscriptionSupportOperationalVerdict::RebuildRequired),
        "NotResumable" => Ok(SubscriptionSupportOperationalVerdict::NotResumable),
        "RejectedByPolicy" => Ok(SubscriptionSupportOperationalVerdict::RejectedByPolicy),
        other => Err(format!(
            "unknown subscription-support operational verdict `{other}`"
        )),
    }
}
