use super::super::{
    classification_error, stable_digest, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalBasis, SubscriptionSupportRole,
};
use super::decision::{
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMaintenanceDecisionKind,
    SupportMaintenanceWorkKind,
};
use crate::failure::StoreError;
use crate::{
    MaintenanceDeclaration, MaintenanceDeclarationId, MaintenanceWorkDescriptor,
    RebuildMaintenanceDeclaration, SnapshotRefreshMaintenanceDeclaration,
};
use serde::Serialize;

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
    pub(super) fn from_basis(
        basis: &SubscriptionSupportOperationalBasis,
        decision: &SubscriptionSupportMaintenanceDecision,
    ) -> Result<Self, StoreError> {
        validate_support_maintenance_basis(basis, decision)?;
        let work_kind = decision.work_kind();
        let maintenance_key = derive_support_maintenance_key(basis, work_kind)?;
        let declaration = build_support_maintenance_declaration(basis, work_kind, &maintenance_key);
        let descriptor = build_support_maintenance_descriptor(&declaration, decision);
        let descriptor_digest =
            derive_support_maintenance_descriptor_digest(&declaration, &descriptor, decision)?;
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

    pub(super) fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub(super) fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub(super) fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub(super) fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
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

fn validate_support_maintenance_basis(
    basis: &SubscriptionSupportOperationalBasis,
    decision: &SubscriptionSupportMaintenanceDecision,
) -> Result<(), StoreError> {
    if let Some(retained_basis_digest) = decision.retained_basis_digest() {
        if retained_basis_digest != basis.basis_digest() {
            return Err(classification_error(
                "subscription-support rebuild descriptors require retained basis evidence matching the support basis",
            ));
        }
    }
    Ok(())
}

fn derive_support_maintenance_key(
    basis: &SubscriptionSupportOperationalBasis,
    work_kind: SupportMaintenanceWorkKind,
) -> Result<String, StoreError> {
    stable_digest(&(
        basis.family_id(),
        basis.family_kind(),
        basis.support_role(),
        basis.artifact_id(),
        work_kind,
        basis.basis_digest(),
    ))
}

fn build_support_maintenance_declaration(
    basis: &SubscriptionSupportOperationalBasis,
    work_kind: SupportMaintenanceWorkKind,
    maintenance_key: &str,
) -> MaintenanceDeclaration {
    let declaration_id =
        MaintenanceDeclarationId::new(format!("subscription-support:{maintenance_key}"));
    let family_label = basis.family_id().as_str().to_string();
    let artifact_label = basis.artifact_id().as_str().to_string();
    match work_kind {
        SupportMaintenanceWorkKind::Rebuild | SupportMaintenanceWorkKind::DegradationRecovery => {
            MaintenanceDeclaration::rebuild(
                declaration_id,
                RebuildMaintenanceDeclaration::new(
                    basis.basis_digest(),
                    family_label,
                    artifact_label.clone(),
                    Some(artifact_label),
                ),
            )
        }
        SupportMaintenanceWorkKind::Refresh => MaintenanceDeclaration::snapshot_refresh(
            declaration_id,
            SnapshotRefreshMaintenanceDeclaration::new(
                family_label,
                basis.artifact_id().as_str(),
                "subscription-support-refresh",
            ),
        ),
        SupportMaintenanceWorkKind::CompatibilityMigration => {
            MaintenanceDeclaration::derived_family_rebuild(
                declaration_id,
                crate::DerivedFamilyRebuildMaintenanceDeclaration::new(
                    basis.basis_digest(),
                    family_label,
                    artifact_label,
                ),
            )
        }
    }
}

fn build_support_maintenance_descriptor(
    declaration: &MaintenanceDeclaration,
    decision: &SubscriptionSupportMaintenanceDecision,
) -> MaintenanceWorkDescriptor {
    declaration
        .work_descriptor()
        .with_recovered_from_restart(matches!(
            decision.kind(),
            SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
        ))
}

fn derive_support_maintenance_descriptor_digest(
    declaration: &MaintenanceDeclaration,
    descriptor: &MaintenanceWorkDescriptor,
    decision: &SubscriptionSupportMaintenanceDecision,
) -> Result<String, StoreError> {
    stable_digest(&(&declaration, &descriptor, decision.kind()))
}
