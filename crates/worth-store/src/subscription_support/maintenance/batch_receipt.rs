use super::super::SupportActionId;
use super::descriptor::SupportMaintenanceDescriptor;
use crate::{
    AdmittedMaintenanceDeclaration, MaintenanceAdmissionReceipt, MaintenanceBatch,
    MaintenanceBatchClass,
};

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
