use super::super::{
    publication_error, stable_digest, CompletedSupportProgramAction, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole, SupportActionId,
    SupportAffectedSetDigest,
};
use super::decision::{SubscriptionSupportMaintenanceDecisionKind, SupportMaintenanceWorkKind};
use super::descriptor::SupportMaintenanceDescriptor;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use crate::{MaintenanceDeclaration, MaintenanceWorkClass, MaintenanceWorkDescriptor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportMaintenanceDescriptorRecord {
    record_key: String,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    work_kind: SupportMaintenanceWorkKind,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    portability_digest: String,
    action_id: SupportActionId,
    affected_set_digest: SupportAffectedSetDigest,
    decision_kind: SubscriptionSupportMaintenanceDecisionKind,
    verdict: SubscriptionSupportOperationalVerdict,
    maintenance_key: String,
    declaration_id: String,
    descriptor_digest: String,
    maintenance_work_class: MaintenanceWorkClass,
    recovered_from_restart: bool,
}

impl Serialize for SupportMaintenanceDescriptorRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PersistedSupportMaintenanceDescriptorRecord::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SupportMaintenanceDescriptorRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedSupportMaintenanceDescriptorRecord::deserialize(deserializer)?;
        Self::try_from(persisted).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedSupportMaintenanceDescriptorRecord {
    record_key: String,
    family_id: String,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: String,
    work_kind: String,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    portability_digest: String,
    action_id: String,
    affected_set_digest: String,
    decision_kind: String,
    verdict: String,
    maintenance_key: String,
    declaration_id: String,
    descriptor_digest: String,
    maintenance_work_class: MaintenanceWorkClass,
    recovered_from_restart: bool,
}

impl SupportMaintenanceDescriptorRecord {
    pub(super) fn from_descriptor(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &super::affected_set::SupportMaintenanceAffectedSet,
        descriptor: &SupportMaintenanceDescriptor,
        decision_kind: SubscriptionSupportMaintenanceDecisionKind,
    ) -> Result<Self, StoreError> {
        let record_key = stable_digest(&(
            descriptor.family_id().as_str(),
            descriptor.family_kind(),
            descriptor.support_role(),
            descriptor.artifact_id().as_str(),
            completed_action.envelope().action_id().as_str(),
            descriptor.descriptor().declaration_id().as_str(),
            descriptor.descriptor_digest(),
        ))?;
        Ok(Self {
            record_key,
            family_id: descriptor.family_id().clone(),
            family_kind: descriptor.family_kind(),
            support_role: descriptor.support_role(),
            artifact_id: descriptor.artifact_id().clone(),
            work_kind: descriptor.work_kind(),
            basis_digest: descriptor.basis_digest().to_string(),
            cursor_digest: descriptor.cursor_digest().to_string(),
            checkpoint_digest: descriptor.checkpoint_digest().to_string(),
            compatibility_digest: descriptor.compatibility_digest().to_string(),
            portability_digest: descriptor.portability_digest().to_string(),
            action_id: completed_action.envelope().action_id().clone(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            decision_kind,
            verdict: completed_action.envelope().verdict(),
            maintenance_key: descriptor.maintenance_key().to_string(),
            declaration_id: descriptor
                .descriptor()
                .declaration_id()
                .as_str()
                .to_string(),
            descriptor_digest: descriptor.descriptor_digest().to_string(),
            maintenance_work_class: descriptor.descriptor().work_class(),
            recovered_from_restart: matches!(
                decision_kind,
                SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
            ),
        })
    }

    pub fn record_key(&self) -> &str {
        &self.record_key
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn maintenance_key(&self) -> &str {
        &self.maintenance_key
    }

    pub fn declaration_id(&self) -> &str {
        &self.declaration_id
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub fn maintenance_work_class(&self) -> MaintenanceWorkClass {
        self.maintenance_work_class
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn cursor_digest(&self) -> &str {
        &self.cursor_digest
    }

    pub fn checkpoint_digest(&self) -> &str {
        &self.checkpoint_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn portability_digest(&self) -> &str {
        &self.portability_digest
    }

    pub(crate) fn verify_persisted_descriptor(
        &self,
        declaration: &MaintenanceDeclaration,
        descriptor: &MaintenanceWorkDescriptor,
    ) -> Result<(), StoreError> {
        if descriptor.declaration_id().as_str() != self.declaration_id
            || descriptor.work_class() != self.maintenance_work_class
        {
            return Err(publication_error(
                "subscription-support maintenance descriptor record drifted from the persisted maintenance work descriptor",
            ));
        }
        let expected_maintenance_key = stable_digest(&(
            &self.family_id,
            self.family_kind,
            self.support_role,
            &self.artifact_id,
            self.work_kind(),
            self.basis_digest.as_str(),
        ))?;
        if expected_maintenance_key != self.maintenance_key {
            return Err(publication_error(
                "subscription-support maintenance descriptor record maintenance key drifted from persisted support basis",
            ));
        }
        let expected_descriptor_digest =
            stable_digest(&(declaration, descriptor, self.decision_kind))?;
        if expected_descriptor_digest != self.descriptor_digest {
            return Err(publication_error(
                "subscription-support maintenance descriptor record digest drifted from persisted maintenance declaration",
            ));
        }
        Ok(())
    }

    fn work_kind(&self) -> SupportMaintenanceWorkKind {
        self.work_kind
    }
}

impl From<&SupportMaintenanceDescriptorRecord> for PersistedSupportMaintenanceDescriptorRecord {
    fn from(record: &SupportMaintenanceDescriptorRecord) -> Self {
        Self {
            record_key: record.record_key.clone(),
            family_id: record.family_id.as_str().to_string(),
            family_kind: record.family_kind,
            support_role: record.support_role,
            artifact_id: record.artifact_id.as_str().to_string(),
            work_kind: format!("{:?}", record.work_kind),
            basis_digest: record.basis_digest.clone(),
            cursor_digest: record.cursor_digest.clone(),
            checkpoint_digest: record.checkpoint_digest.clone(),
            compatibility_digest: record.compatibility_digest.clone(),
            portability_digest: record.portability_digest.clone(),
            action_id: record.action_id.as_str().to_string(),
            affected_set_digest: record.affected_set_digest.as_str().to_string(),
            decision_kind: format!("{:?}", record.decision_kind),
            verdict: format!("{:?}", record.verdict),
            maintenance_key: record.maintenance_key.clone(),
            declaration_id: record.declaration_id.clone(),
            descriptor_digest: record.descriptor_digest.clone(),
            maintenance_work_class: record.maintenance_work_class,
            recovered_from_restart: record.recovered_from_restart,
        }
    }
}

impl TryFrom<PersistedSupportMaintenanceDescriptorRecord> for SupportMaintenanceDescriptorRecord {
    type Error = String;

    fn try_from(record: PersistedSupportMaintenanceDescriptorRecord) -> Result<Self, Self::Error> {
        let decision_kind = parse_persisted_maintenance_decision_kind(&record.decision_kind)?;
        let verdict = parse_persisted_operational_verdict(&record.verdict)?;
        Ok(Self {
            record_key: require_non_empty("descriptor record key", record.record_key)
                .map_err(|error| error.to_string())?,
            family_id: SubscriptionSupportFamilyId::new(record.family_id)
                .map_err(|error| error.to_string())?,
            family_kind: record.family_kind,
            support_role: record.support_role,
            artifact_id: SubscriptionSupportArtifactId(record.artifact_id),
            work_kind: parse_persisted_maintenance_work_kind(&record.work_kind)?,
            basis_digest: require_non_empty("basis digest", record.basis_digest)
                .map_err(|error| error.to_string())?,
            cursor_digest: require_non_empty("cursor digest", record.cursor_digest)
                .map_err(|error| error.to_string())?,
            checkpoint_digest: require_non_empty("checkpoint digest", record.checkpoint_digest)
                .map_err(|error| error.to_string())?,
            compatibility_digest: require_non_empty(
                "compatibility digest",
                record.compatibility_digest,
            )
            .map_err(|error| error.to_string())?,
            portability_digest: require_non_empty("portability digest", record.portability_digest)
                .map_err(|error| error.to_string())?,
            action_id: SupportActionId::new(record.action_id).map_err(|error| error.to_string())?,
            affected_set_digest: SupportAffectedSetDigest::from_persisted(
                record.affected_set_digest,
            )
            .map_err(|error| error.to_string())?,
            decision_kind,
            verdict,
            maintenance_key: require_non_empty("maintenance key", record.maintenance_key)
                .map_err(|error| error.to_string())?,
            declaration_id: require_non_empty("declaration id", record.declaration_id)
                .map_err(|error| error.to_string())?,
            descriptor_digest: require_non_empty("descriptor digest", record.descriptor_digest)
                .map_err(|error| error.to_string())?,
            maintenance_work_class: record.maintenance_work_class,
            recovered_from_restart: record.recovered_from_restart,
        })
    }
}

fn parse_persisted_maintenance_decision_kind(
    value: &str,
) -> Result<SubscriptionSupportMaintenanceDecisionKind, String> {
    match value {
        "RebuildDescriptorAdmitted" => {
            Ok(SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted)
        }
        "RefreshDescriptorAdmitted" => {
            Ok(SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted)
        }
        "CompatibilityMigrationDescriptorAdmitted" => Ok(
            SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted,
        ),
        "DegradationRecoveryDescriptorAdmitted" => {
            Ok(SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted)
        }
        "InterruptedRestartRecovered" => {
            Ok(SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered)
        }
        other => Err(format!(
            "unknown subscription-support maintenance decision kind `{other}`"
        )),
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
