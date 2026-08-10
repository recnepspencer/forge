use crate::{
    failure::{StoreError, StoreErrorKind},
    SubscriptionSupportMaintenanceBatchRequest, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportMaintenanceReport, SubscriptionSupportMissingSupportRecoveryRequest,
    SubscriptionSupportOperationalBasis, SupportAllocationScope, SupportPathClass,
    SupportProgramDensityClass, SupportProgramPathPolicy,
};

use super::super::{StateBackedStoreBackend, StatePersistence};

pub(super) fn execute_missing_support_rebuild_maintenance<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    request: &SubscriptionSupportMissingSupportRecoveryRequest,
) -> Result<SubscriptionSupportMaintenanceReport, StoreError> {
    let maintenance_admission = request.rebuild_maintenance_admission().ok_or_else(|| {
        StoreError::new(
            crate::failure::StoreErrorKind::SubscriptionSupportClassificationViolation,
            "subscription-support rebuild-required missing recovery requires maintenance admission planning",
        )
    })?;
    let retained_basis_digest = maintenance_admission
        .retained_rebuild_basis_digest()
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support rebuild-required missing recovery lost retained basis planning",
            )
        })?;
    let basis = SubscriptionSupportOperationalBasis::new(
        request.family_id().clone(),
        request.family_kind(),
        request.support_role(),
        request.missing_artifact_id().clone(),
        request.basis_digest(),
        request.cursor_digest(),
        request.checkpoint_digest(),
        request.compatibility_digest(),
        request.portability_digest(),
        crate::SubscriptionSupportActionOrigin::Maintenance,
    )?;
    let plan = backend.admit_subscription_support_maintenance_batch(
        SubscriptionSupportMaintenanceBatchRequest {
            action_id: maintenance_admission.action_id().clone(),
            affected_bases: vec![basis],
            decision: SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                retained_basis_digest,
            )?,
            path: SupportProgramPathPolicy {
                path_class: SupportPathClass::MaintenanceExecution,
                density_class: SupportProgramDensityClass::MaintenanceKeyBatch,
                allocation_scope: SupportAllocationScope::FamilyLocalBatch,
                budget: maintenance_admission.breadth_budget().clone(),
                payload_header_bytes: maintenance_admission.payload_header_bytes(),
            },
        },
    )?;
    backend.publish_subscription_support_maintenance_consequence(plan)
}
