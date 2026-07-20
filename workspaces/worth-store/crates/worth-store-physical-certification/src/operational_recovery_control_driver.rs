use std::io::{Error, ErrorKind};

use worth_store_operations::{
    OperationalControlAppendDenial, OperationalControlRecord, OperationalControlStore,
    OperationalControlStorePort,
};
use worth_store_physical_backend::{
    ControlMediaFault, ControlRecoveryObjectHandle, PhysicalControlAppendReceipt,
};

use crate::{
    OperationalRecoveryControlTransitionKind, OperationalRecoveryProductionDriver,
    OperationalRecoveryYieldpoint,
};

/// A transparent production control-store decorator used by the S.4.5 driver.
/// It forwards every real append to the ordinary durable owner and records the
/// exact before/after cutpoints. It cannot construct or modify control records.
pub struct DrivenOperationalControlStore<'store, 'driver> {
    store: &'store OperationalControlStore,
    driver: &'driver OperationalRecoveryProductionDriver,
}

impl<'store, 'driver> DrivenOperationalControlStore<'store, 'driver> {
    pub fn new(
        store: &'store OperationalControlStore,
        driver: &'driver OperationalRecoveryProductionDriver,
    ) -> Self {
        if let Ok(observation) = store.session_observation() {
            driver.observe_control_session(observation);
        }
        Self { store, driver }
    }

    fn drive_append(
        &self,
        kind: OperationalRecoveryControlTransitionKind,
        record: &OperationalControlRecord,
        persist: impl FnOnce() -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial>,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        self.driver.observe_operation(record.operation_id());
        let before_point = OperationalRecoveryYieldpoint::BeforeDurableControlTransition(kind);
        let after_point = OperationalRecoveryYieldpoint::AfterDurableControlTransition(kind);
        let before_observation = self
            .store
            .session_observation()
            .map_err(OperationalControlAppendDenial::Media)?;
        let interrupted_before = self.driver.record_control_yieldpoint(before_point);
        self.driver
            .crash_at_control_cut_if_scheduled(before_point, before_observation);
        if interrupted_before {
            return Err(interrupted_append());
        }
        let receipt = persist()?;
        self.driver.observe_durable_control_transition(record);
        let interrupted_after = self.driver.record_control_yieldpoint(after_point);
        self.driver
            .crash_at_control_cut_if_scheduled(after_point, before_observation);
        if let Ok(observation) = self.store.session_observation() {
            self.driver.observe_control_session(observation);
        }
        if interrupted_after {
            Err(interrupted_append())
        } else {
            Ok(receipt)
        }
    }
}

impl OperationalControlStorePort for DrivenOperationalControlStore<'_, '_> {
    fn publish_recovery_object(
        &self,
        content: &[u8],
    ) -> Result<ControlRecoveryObjectHandle, OperationalControlAppendDenial> {
        self.store.publish_recovery_object(content)
    }

    fn append(
        &self,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        let Some(kind) = OperationalRecoveryControlTransitionKind::from_record(record.kind())
        else {
            return self.store.append(record);
        };
        self.drive_append(kind, record, || self.store.append(record))
    }

    fn compare_exchange_authorization_consumption(
        &self,
        expected: Option<worth_store_authority::ControlStoreGeneration>,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        let kind = OperationalRecoveryControlTransitionKind::AuthorizationConsumption;
        if !kind.matches(record.kind()) {
            return Err(OperationalControlAppendDenial::Media(
                ControlMediaFault::DerivedTransitionIndexCorrupt,
            ));
        }
        self.drive_append(kind, record, || {
            self.store
                .compare_exchange_authorization_consumption(expected, record)
        })
    }
}

fn interrupted_append() -> OperationalControlAppendDenial {
    OperationalControlAppendDenial::Media(ControlMediaFault::Io(Error::new(
        ErrorKind::Interrupted,
        "S.10 deterministic control transition interruption",
    )))
}
