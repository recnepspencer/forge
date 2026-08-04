use std::collections::{BTreeMap, BTreeSet};

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityElevationRule, ErasedApplicationCapabilityContract,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryElevationLifecycleOperationRole {
    Request,
    Approve,
    Revoke,
    CompleteReview,
}

pub(super) struct WorthQueryInstalledElevationLifecycleOperation {
    capability_identity: [u8; 32],
    role: WorthQueryElevationLifecycleOperationRole,
}

#[derive(Default)]
pub(super) struct WorthQueryInstalledElevationLifecycleRegistry {
    operations: BTreeMap<
        String,
        BTreeMap<String, BTreeMap<String, WorthQueryInstalledElevationLifecycleOperation>>,
    >,
    executable_operations: BTreeSet<(String, String)>,
}

impl WorthQueryInstalledElevationLifecycleRegistry {
    pub(super) fn install(
        &mut self,
        capability_identity: [u8; 32],
        contract: &ErasedApplicationCapabilityContract,
    ) -> Result<(), ()> {
        let ApplicationCapabilityElevationRule::Governed(elevation) = contract.elevation() else {
            return Ok(());
        };
        let lifecycle = elevation.lifecycle();
        for (role, operation) in [
            (
                WorthQueryElevationLifecycleOperationRole::Request,
                lifecycle.request(),
            ),
            (
                WorthQueryElevationLifecycleOperationRole::Approve,
                lifecycle.approve(),
            ),
            (
                WorthQueryElevationLifecycleOperationRole::Revoke,
                lifecycle.revoke(),
            ),
            (
                WorthQueryElevationLifecycleOperationRole::CompleteReview,
                lifecycle.complete_review(),
            ),
        ] {
            if !self.executable_operations.insert((
                operation.operation().to_owned(),
                operation.input_type().to_owned(),
            )) {
                return Err(());
            }
            if self
                .operations
                .entry(operation.operation().to_owned())
                .or_default()
                .entry(operation.input_type().to_owned())
                .or_default()
                .insert(
                    operation.operation_type().to_owned(),
                    WorthQueryInstalledElevationLifecycleOperation {
                        capability_identity,
                        role,
                    },
                )
                .is_some()
            {
                return Err(());
            }
        }
        Ok(())
    }

    pub(super) fn operation<Operation, Input>(
        &self,
        operation: &str,
    ) -> Result<Option<([u8; 32], WorthQueryElevationLifecycleOperationRole)>, ()> {
        let Some(inputs) = self.operations.get(operation) else {
            return Ok(None);
        };
        let Some(markers) = inputs.get(std::any::type_name::<Input>()) else {
            return Ok(None);
        };
        let installed = markers.get(std::any::type_name::<Operation>()).ok_or(())?;
        Ok(Some((installed.capability_identity, installed.role)))
    }

    pub(super) fn len(&self) -> usize {
        self.executable_operations.len()
    }
}
