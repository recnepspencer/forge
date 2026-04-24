use super::{
    classification_error, cost_surface_for_program_path, publication_error, stable_digest,
    CompletedSupportProgramAction, SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface, SubscriptionSupportRole,
    SupportActionId, SupportAffectedSetDigest, SupportAllocationScope, SupportProgramDensityClass,
    SupportProgramPathPlan,
};
use crate::failure::StoreError;
use crate::{
    AdmittedMaintenanceDeclaration, MaintenanceAdmissionReceipt, MaintenanceBatch,
    MaintenanceBatchClass, MaintenanceDeclaration, MaintenanceDeclarationId, MaintenanceWorkClass,
    MaintenanceWorkDescriptor, RebuildMaintenanceDeclaration,
    SnapshotRefreshMaintenanceDeclaration,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceAffectedSet {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    affected_set_digest: SupportAffectedSetDigest,
    affected_bases: Vec<SubscriptionSupportOperationalBasis>,
}

impl SupportMaintenanceAffectedSet {
    pub(crate) fn from_maintenance_bases(
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
    ) -> Result<Self, StoreError> {
        let Some(first) = affected_bases.first() else {
            return Err(classification_error(
                "subscription-support maintenance affected sets must not be empty",
            ));
        };
        if first.action_origin() != SubscriptionSupportActionOrigin::Maintenance {
            return Err(classification_error(
                "subscription-support maintenance affected sets require maintenance-origin bases",
            ));
        }
        for basis in &affected_bases {
            if basis.action_origin() != SubscriptionSupportActionOrigin::Maintenance {
                return Err(classification_error(
                    "subscription-support maintenance affected sets cannot mix action origins",
                ));
            }
            if basis.family_id() != first.family_id()
                || basis.family_kind() != first.family_kind()
                || basis.support_role() != first.support_role()
            {
                return Err(classification_error(
                    "subscription-support maintenance affected sets must be family-local",
                ));
            }
        }
        Ok(Self {
            family_id: first.family_id().clone(),
            family_kind: first.family_kind(),
            support_role: first.support_role(),
            affected_set_digest: SupportAffectedSetDigest::from_bases(&affected_bases)?,
            affected_bases,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn affected_count(&self) -> u64 {
        self.affected_bases.len() as u64
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub(crate) fn primary_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.affected_bases[0]
    }

    pub(crate) fn affected_bases(&self) -> &[SubscriptionSupportOperationalBasis] {
        &self.affected_bases
    }

    pub(crate) fn descriptors_for(
        &self,
        decision: &SubscriptionSupportMaintenanceDecision,
    ) -> Result<(Vec<SupportMaintenanceDescriptor>, u64), StoreError> {
        let mut descriptors_by_key = BTreeMap::new();
        let mut duplicate_count = 0;
        for basis in &self.affected_bases {
            let descriptor = SupportMaintenanceDescriptor::from_basis(basis, decision)?;
            let key = descriptor.maintenance_key().to_string();
            if descriptors_by_key.insert(key, descriptor).is_some() {
                duplicate_count += 1;
            }
        }
        Ok((descriptors_by_key.into_values().collect(), duplicate_count))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportMaintenanceWorkKind {
    Rebuild,
    Refresh,
    CompatibilityMigration,
    DegradationRecovery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMaintenanceDecision {
    evidence: SubscriptionSupportMaintenanceDecisionEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
enum SubscriptionSupportMaintenanceDecisionEvidence {
    RebuildDescriptorAdmitted {
        retained_basis_digest: String,
    },
    RefreshDescriptorAdmitted {
        refresh_reason: String,
    },
    CompatibilityMigrationDescriptorAdmitted {
        migration_digest: String,
    },
    DegradationRecoveryDescriptorAdmitted {
        recovery_reason: String,
    },
    InterruptedRestartRecovered {
        recovered_work_kind: SupportMaintenanceWorkKind,
        restart_recovery_digest: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportMaintenanceDecision {
    pub(crate) fn rebuild_descriptor_admitted(
        retained_basis_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::RebuildDescriptorAdmitted {
                retained_basis_digest: require_non_empty(
                    "retained rebuild basis",
                    retained_basis_digest,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn refresh_descriptor_admitted(
        refresh_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::RefreshDescriptorAdmitted {
                refresh_reason: require_non_empty("refresh reason", refresh_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn compatibility_migration_descriptor_admitted(
        migration_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::CompatibilityMigrationDescriptorAdmitted {
                migration_digest: require_non_empty("compatibility migration", migration_digest)?,
            }
            .into(),
        )
    }

    pub(crate) fn degradation_recovery_descriptor_admitted(
        recovery_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::DegradationRecoveryDescriptorAdmitted {
                recovery_reason: require_non_empty("degradation recovery", recovery_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn interrupted_restart_recovered(
        recovered_work_kind: SupportMaintenanceWorkKind,
        restart_recovery_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportMaintenanceDecisionEvidence::InterruptedRestartRecovered {
                recovered_work_kind,
                restart_recovery_digest: require_non_empty(
                    "interrupted maintenance restart recovery",
                    restart_recovery_digest,
                )?,
            }
            .into(),
        )
    }

    pub fn kind(&self) -> SubscriptionSupportMaintenanceDecisionKind {
        match &self.evidence {
            SubscriptionSupportMaintenanceDecisionEvidence::RebuildDescriptorAdmitted { .. } => {
                SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted
            }
            SubscriptionSupportMaintenanceDecisionEvidence::RefreshDescriptorAdmitted { .. } => {
                SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted
            }
            SubscriptionSupportMaintenanceDecisionEvidence::CompatibilityMigrationDescriptorAdmitted {
                ..
            } => SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted,
            SubscriptionSupportMaintenanceDecisionEvidence::DegradationRecoveryDescriptorAdmitted {
                ..
            } => SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted,
            SubscriptionSupportMaintenanceDecisionEvidence::InterruptedRestartRecovered { .. } => {
                SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
            }
        }
    }

    pub fn work_kind(&self) -> SupportMaintenanceWorkKind {
        match self.kind() {
            SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted => {
                SupportMaintenanceWorkKind::Rebuild
            }
            SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted => {
                SupportMaintenanceWorkKind::Refresh
            }
            SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted => {
                SupportMaintenanceWorkKind::CompatibilityMigration
            }
            SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted => {
                SupportMaintenanceWorkKind::DegradationRecovery
            }
            SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered => {
                match &self.evidence {
                    SubscriptionSupportMaintenanceDecisionEvidence::InterruptedRestartRecovered {
                        recovered_work_kind,
                        ..
                    } => *recovered_work_kind,
                    _ => SupportMaintenanceWorkKind::Rebuild,
                }
            }
        }
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match self.work_kind() {
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
        }
    }

    fn retained_basis_digest(&self) -> Option<&str> {
        match &self.evidence {
            SubscriptionSupportMaintenanceDecisionEvidence::RebuildDescriptorAdmitted {
                retained_basis_digest,
            } => Some(retained_basis_digest),
            _ => None,
        }
    }
}

impl From<SubscriptionSupportMaintenanceDecisionEvidence>
    for SubscriptionSupportMaintenanceDecision
{
    fn from(evidence: SubscriptionSupportMaintenanceDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportMaintenanceDecisionKind {
    RebuildDescriptorAdmitted,
    RefreshDescriptorAdmitted,
    CompatibilityMigrationDescriptorAdmitted,
    DegradationRecoveryDescriptorAdmitted,
    InterruptedRestartRecovered,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceDescriptor {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    portability_digest: String,
    work_kind: SupportMaintenanceWorkKind,
    maintenance_key: String,
    retained_basis_digest: Option<String>,
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
    descriptor_digest: String,
}

impl SupportMaintenanceDescriptor {
    fn from_basis(
        basis: &SubscriptionSupportOperationalBasis,
        decision: &SubscriptionSupportMaintenanceDecision,
    ) -> Result<Self, StoreError> {
        if let Some(retained_basis_digest) = decision.retained_basis_digest() {
            if retained_basis_digest != basis.basis_digest() {
                return Err(classification_error(
                    "subscription-support rebuild descriptors require retained basis evidence matching the support basis",
                ));
            }
        }
        let work_kind = decision.work_kind();
        let maintenance_key = stable_digest(&(
            basis.family_id(),
            basis.family_kind(),
            basis.support_role(),
            basis.artifact_id(),
            work_kind,
            basis.basis_digest(),
        ))?;
        let declaration_id =
            MaintenanceDeclarationId::new(format!("subscription-support:{maintenance_key}"));
        let family_label = basis.family_id().as_str().to_string();
        let artifact_label = basis.artifact_id().as_str().to_string();
        let declaration = match work_kind {
            SupportMaintenanceWorkKind::Rebuild
            | SupportMaintenanceWorkKind::DegradationRecovery => MaintenanceDeclaration::rebuild(
                declaration_id.clone(),
                RebuildMaintenanceDeclaration::new(
                    basis.basis_digest(),
                    family_label.clone(),
                    artifact_label.clone(),
                    Some(artifact_label.clone()),
                ),
            ),
            SupportMaintenanceWorkKind::Refresh => MaintenanceDeclaration::snapshot_refresh(
                declaration_id.clone(),
                SnapshotRefreshMaintenanceDeclaration::new(
                    family_label.clone(),
                    basis.artifact_id().as_str(),
                    "subscription-support-refresh",
                ),
            ),
            SupportMaintenanceWorkKind::CompatibilityMigration => {
                MaintenanceDeclaration::derived_family_rebuild(
                    declaration_id.clone(),
                    crate::DerivedFamilyRebuildMaintenanceDeclaration::new(
                        basis.basis_digest(),
                        family_label.clone(),
                        artifact_label.clone(),
                    ),
                )
            }
        };
        let descriptor = declaration
            .work_descriptor()
            .with_recovered_from_restart(matches!(
                decision.kind(),
                SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
            ));
        let descriptor_digest = stable_digest(&(&declaration, &descriptor, decision.kind()))?;
        Ok(Self {
            family_id: basis.family_id().clone(),
            family_kind: basis.family_kind(),
            support_role: basis.support_role(),
            artifact_id: basis.artifact_id().clone(),
            basis_digest: basis.basis_digest().to_string(),
            cursor_digest: basis.cursor_digest().to_string(),
            checkpoint_digest: basis.checkpoint_digest().to_string(),
            compatibility_digest: basis.compatibility_digest().to_string(),
            portability_digest: basis.portability_digest().to_string(),
            work_kind,
            maintenance_key,
            retained_basis_digest: decision.retained_basis_digest().map(str::to_string),
            declaration,
            descriptor,
            descriptor_digest,
        })
    }

    pub fn work_kind(&self) -> SupportMaintenanceWorkKind {
        self.work_kind
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

    pub fn maintenance_key(&self) -> &str {
        &self.maintenance_key
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceAdmissionWitness {
    maintenance_key: String,
    declaration_id: String,
    descriptor_digest: String,
    maintenance_work_class: MaintenanceWorkClass,
}

impl SupportMaintenanceAdmissionWitness {
    pub(crate) fn new(descriptor: &SupportMaintenanceDescriptor) -> Self {
        Self {
            maintenance_key: descriptor.maintenance_key().to_string(),
            declaration_id: descriptor
                .descriptor()
                .declaration_id()
                .as_str()
                .to_string(),
            descriptor_digest: descriptor.descriptor_digest().to_string(),
            maintenance_work_class: descriptor.descriptor().work_class(),
        }
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportMaintenanceAffectedSet,
    path_plan: SupportProgramPathPlan,
    descriptors: Vec<SupportMaintenanceDescriptor>,
    maintenance_receipt: MaintenanceAdmissionReceipt,
    coalesced_duplicate_count: u64,
    decision: SubscriptionSupportMaintenanceDecision,
}

impl SupportMaintenanceBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportMaintenanceAffectedSet,
        path_plan: SupportProgramPathPlan,
        descriptors: Vec<SupportMaintenanceDescriptor>,
        maintenance_receipt: MaintenanceAdmissionReceipt,
        coalesced_duplicate_count: u64,
        decision: SubscriptionSupportMaintenanceDecision,
    ) -> Result<Self, StoreError> {
        if path_plan.path_class() != super::SupportPathClass::MaintenanceExecution {
            return Err(classification_error(
                "subscription-support maintenance plans require maintenance-execution paths",
            ));
        }
        if path_plan.density_class() != SupportProgramDensityClass::MaintenanceKeyBatch {
            return Err(classification_error(
                "subscription-support maintenance plans require maintenance-key density",
            ));
        }
        if path_plan.allocation_scope() != SupportAllocationScope::FamilyLocalBatch {
            return Err(classification_error(
                "subscription-support maintenance plans require family-local allocation",
            ));
        }
        if path_plan.batch_width() != affected_set.affected_count() {
            return Err(classification_error(
                "subscription-support maintenance plan width must match affected-set breadth",
            ));
        }
        if descriptors.is_empty() {
            return Err(classification_error(
                "subscription-support maintenance plans require at least one descriptor",
            ));
        }
        if descriptors.len() as u64 + coalesced_duplicate_count != affected_set.affected_count() {
            return Err(classification_error(
                "subscription-support maintenance coalescing must account for every affected entry",
            ));
        }
        if !maintenance_receipt.rejections().is_empty() {
            return Err(classification_error(
                "subscription-support maintenance plans require fully admitted maintenance receipts",
            ));
        }
        if maintenance_receipt.batch_summary().batch_class()
            != MaintenanceBatchClass::SubscriptionSupport
        {
            return Err(classification_error(
                "subscription-support maintenance plans require subscription-support maintenance batch class",
            ));
        }
        if maintenance_receipt.admitted_declarations().len() != descriptors.len() {
            return Err(classification_error(
                "subscription-support maintenance receipts must admit every unique descriptor",
            ));
        }
        let admitted_by_declaration = maintenance_receipt
            .admitted_declarations()
            .iter()
            .map(|admitted| (admitted.declaration().id().clone(), admitted))
            .collect::<BTreeMap<_, _>>();
        for descriptor in &descriptors {
            let declaration_id = descriptor.descriptor().declaration_id();
            let admitted = admitted_by_declaration
                .get(declaration_id)
                .copied()
                .ok_or_else(|| {
                    classification_error(
                        "subscription-support maintenance receipt is missing an admitted declaration for a descriptor",
                    )
                })?;
            let expected_admitted_descriptor = if matches!(
                decision.kind(),
                SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
            ) {
                descriptor
                    .descriptor()
                    .clone()
                    .with_recovered_from_restart(false)
            } else {
                descriptor.descriptor().clone()
            };
            if admitted.descriptor() != &expected_admitted_descriptor
                || admitted.declaration() != descriptor.declaration()
            {
                return Err(classification_error(
                    "subscription-support maintenance receipt drifted from the admitted descriptor",
                ));
            }
        }
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        })
    }

    pub fn affected_set(&self) -> &SupportMaintenanceAffectedSet {
        &self.affected_set
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn descriptors(&self) -> &[SupportMaintenanceDescriptor] {
        &self.descriptors
    }

    pub fn maintenance_receipt(&self) -> &MaintenanceAdmissionReceipt {
        &self.maintenance_receipt
    }

    pub fn coalesced_duplicate_count(&self) -> u64 {
        self.coalesced_duplicate_count
    }

    pub fn decision(&self) -> &SubscriptionSupportMaintenanceDecision {
        &self.decision
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportMaintenanceAffectedSet,
        SupportProgramPathPlan,
        Vec<SupportMaintenanceDescriptor>,
        MaintenanceAdmissionReceipt,
        u64,
        SubscriptionSupportMaintenanceDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.descriptors,
            self.maintenance_receipt,
            self.coalesced_duplicate_count,
            self.decision,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceDebtSummary {
    action_id: SupportActionId,
    affected_set_digest: SupportAffectedSetDigest,
    work_kind: SupportMaintenanceWorkKind,
    verdict: SubscriptionSupportOperationalVerdict,
    delay_reason: String,
    descriptor_count: u64,
    coalesced_duplicate_count: u64,
}

impl SupportMaintenanceDebtSummary {
    fn new(
        action_id: &SupportActionId,
        affected_set: &SupportMaintenanceAffectedSet,
        decision: &SubscriptionSupportMaintenanceDecision,
        descriptor_count: u64,
        coalesced_duplicate_count: u64,
        delay_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        if descriptor_count == 0 {
            return Err(classification_error(
                "subscription-support maintenance debt summaries require admitted descriptors",
            ));
        }
        Ok(Self {
            action_id: action_id.clone(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            work_kind: decision.work_kind(),
            verdict: decision.verdict(),
            delay_reason: require_non_empty("delay reason", delay_reason)?,
            descriptor_count,
            coalesced_duplicate_count,
        })
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn work_kind(&self) -> SupportMaintenanceWorkKind {
        self.work_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn delay_reason(&self) -> &str {
        &self.delay_reason
    }

    pub fn descriptor_count(&self) -> u64 {
        self.descriptor_count
    }

    pub fn coalesced_duplicate_count(&self) -> u64 {
        self.coalesced_duplicate_count
    }
}

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
    fn from_descriptor(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &SupportMaintenanceAffectedSet,
        descriptor: &SupportMaintenanceDescriptor,
        decision_kind: SubscriptionSupportMaintenanceDecisionKind,
    ) -> Result<Self, StoreError> {
        let record_key = stable_digest(&(
            descriptor.family_id.as_str(),
            descriptor.family_kind,
            descriptor.support_role,
            descriptor.artifact_id.as_str(),
            completed_action.envelope().action_id().as_str(),
            descriptor.descriptor().declaration_id().as_str(),
            descriptor.descriptor_digest(),
        ))?;
        Ok(Self {
            record_key,
            family_id: descriptor.family_id.clone(),
            family_kind: descriptor.family_kind,
            support_role: descriptor.support_role,
            artifact_id: descriptor.artifact_id.clone(),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceParticipationRecord {
    action_id: SupportActionId,
    affected_set_digest: SupportAffectedSetDigest,
    decision_kind: SubscriptionSupportMaintenanceDecisionKind,
    verdict: SubscriptionSupportOperationalVerdict,
    descriptor_count: u64,
    coalesced_duplicate_count: u64,
}

impl SupportMaintenanceParticipationRecord {
    fn new(
        completed_action: &CompletedSupportProgramAction,
        affected_set: &SupportMaintenanceAffectedSet,
        decision_kind: SubscriptionSupportMaintenanceDecisionKind,
        descriptor_count: u64,
        coalesced_duplicate_count: u64,
    ) -> Result<Self, StoreError> {
        if completed_action.envelope().action_origin()
            != SubscriptionSupportActionOrigin::Maintenance
        {
            return Err(classification_error(
                "subscription-support maintenance participation record action origin drift",
            ));
        }
        Ok(Self {
            action_id: completed_action.envelope().action_id().clone(),
            affected_set_digest: affected_set.affected_set_digest().clone(),
            decision_kind,
            verdict: completed_action.envelope().verdict(),
            descriptor_count,
            coalesced_duplicate_count,
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn decision_kind(&self) -> SubscriptionSupportMaintenanceDecisionKind {
        self.decision_kind
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.verdict
    }

    pub fn descriptor_count(&self) -> u64 {
        self.descriptor_count
    }

    pub fn coalesced_duplicate_count(&self) -> u64 {
        self.coalesced_duplicate_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMaintenanceReport {
    completed_action: CompletedSupportProgramAction,
    participation_record: SupportMaintenanceParticipationRecord,
    admissions: Vec<SupportMaintenanceAdmissionWitness>,
    descriptor_records: Vec<SupportMaintenanceDescriptorRecord>,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportMaintenanceReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        affected_set: SupportMaintenanceAffectedSet,
        descriptors: Vec<SupportMaintenanceDescriptor>,
        maintenance_receipt: &MaintenanceAdmissionReceipt,
        coalesced_duplicate_count: u64,
        decision: &SubscriptionSupportMaintenanceDecision,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        let participation_record = SupportMaintenanceParticipationRecord::new(
            &completed_action,
            &affected_set,
            decision.kind(),
            descriptors.len() as u64,
            coalesced_duplicate_count,
        )?;
        let admissions = descriptors
            .iter()
            .map(SupportMaintenanceAdmissionWitness::new)
            .collect();
        let admitted_by_declaration = maintenance_receipt
            .admitted_declarations()
            .iter()
            .map(|admitted| (admitted.declaration().id().clone(), admitted))
            .collect::<BTreeMap<_, _>>();
        let descriptor_records = descriptors
            .iter()
            .map(|descriptor| {
                let declaration_id = descriptor.descriptor().declaration_id();
                admitted_by_declaration
                    .get(declaration_id)
                    .ok_or_else(|| {
                        classification_error(
                            "subscription-support maintenance report lost its admitted maintenance declaration",
                        )
                    })?;
                SupportMaintenanceDescriptorRecord::from_descriptor(
                    &completed_action,
                    &affected_set,
                    descriptor,
                    decision.kind(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            completed_action,
            participation_record,
            admissions,
            descriptor_records,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::MaintenanceParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn participation_record(&self) -> &SupportMaintenanceParticipationRecord {
        &self.participation_record
    }

    pub fn admissions(&self) -> &[SupportMaintenanceAdmissionWitness] {
        &self.admissions
    }

    pub fn descriptor_records(&self) -> &[SupportMaintenanceDescriptorRecord] {
        &self.descriptor_records
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMaintenanceDebtReport {
    debt_summary: SupportMaintenanceDebtSummary,
    admissions: Vec<SupportMaintenanceAdmissionWitness>,
    translation_bases: Vec<SubscriptionSupportOperationalBasis>,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportMaintenanceDebtReport {
    pub(crate) fn new(
        plan: &SupportMaintenanceBatchPlan,
        delay_reason: impl Into<String>,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        if path_plan.path_class() != super::SupportPathClass::OperatorReporting {
            return Err(classification_error(
                "subscription-support maintenance debt reports require operator-reporting paths",
            ));
        }
        if path_plan.density_class() != SupportProgramDensityClass::MaintenanceKeyBatch {
            return Err(classification_error(
                "subscription-support maintenance debt reports require maintenance-key density",
            ));
        }
        if path_plan.allocation_scope() != SupportAllocationScope::OperatorReport {
            return Err(classification_error(
                "subscription-support maintenance debt reports require operator-report allocation",
            ));
        }
        if path_plan.batch_width() != plan.affected_set().affected_count() {
            return Err(classification_error(
                "subscription-support maintenance debt reports must preserve affected-set breadth",
            ));
        }
        Ok(Self {
            debt_summary: SupportMaintenanceDebtSummary::new(
                plan.action_id(),
                plan.affected_set(),
                plan.decision(),
                plan.descriptors().len() as u64,
                plan.coalesced_duplicate_count(),
                delay_reason,
            )?,
            admissions: plan
                .descriptors()
                .iter()
                .map(SupportMaintenanceAdmissionWitness::new)
                .collect(),
            translation_bases: plan.affected_set().affected_bases().to_vec(),
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::MaintenanceParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn debt_summary(&self) -> &SupportMaintenanceDebtSummary {
        &self.debt_summary
    }

    pub fn admissions(&self) -> &[SupportMaintenanceAdmissionWitness] {
        &self.admissions
    }

    pub fn translation_bases(&self) -> &[SubscriptionSupportOperationalBasis] {
        &self.translation_bases
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}

pub(crate) fn support_maintenance_batch(
    action_id: &SupportActionId,
    descriptors: &[SupportMaintenanceDescriptor],
) -> MaintenanceBatch {
    MaintenanceBatch::new(
        format!("subscription-support:{}", action_id.as_str()),
        MaintenanceBatchClass::SubscriptionSupport,
        descriptors
            .iter()
            .map(|descriptor| descriptor.declaration().clone())
            .collect(),
    )
}

pub(crate) fn synthetic_support_maintenance_receipt(
    batch: &MaintenanceBatch,
    descriptors: &[SupportMaintenanceDescriptor],
) -> MaintenanceAdmissionReceipt {
    MaintenanceAdmissionReceipt::new(
        batch.summary(),
        descriptors
            .iter()
            .map(|descriptor| {
                AdmittedMaintenanceDeclaration::new(
                    descriptor.declaration().clone(),
                    descriptor.declaration().work_descriptor(),
                )
            })
            .collect(),
        Vec::new(),
    )
}

fn require_non_empty(label: &'static str, value: impl Into<String>) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support maintenance {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}
