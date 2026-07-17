use std::cell::Cell;

use worth_store_physical_backend::{
    ControlMediaFault, ControlRecoveryObjectHandle, PhysicalControlAppendReceipt,
};

use crate::{
    OperationalControlAppendDenial, OperationalControlRecord, OperationalControlStorePort,
};

pub(super) struct FailAfterSuccessfulControlAppends<'a> {
    delegate: &'a dyn OperationalControlStorePort,
    successful_appends_before_failure: usize,
    append_calls: Cell<usize>,
}

pub(super) struct LoseSuccessfulControlAppendReceipt<'a> {
    delegate: &'a dyn OperationalControlStorePort,
    lost_call: usize,
    append_calls: Cell<usize>,
}

impl<'a> LoseSuccessfulControlAppendReceipt<'a> {
    pub(super) fn new(delegate: &'a dyn OperationalControlStorePort, lost_call: usize) -> Self {
        Self {
            delegate,
            lost_call,
            append_calls: Cell::new(0),
        }
    }
}

impl<'a> FailAfterSuccessfulControlAppends<'a> {
    pub(super) fn new(
        delegate: &'a dyn OperationalControlStorePort,
        successful_appends_before_failure: usize,
    ) -> Self {
        Self {
            delegate,
            successful_appends_before_failure,
            append_calls: Cell::new(0),
        }
    }

    fn admit_append(&self) -> Result<(), OperationalControlAppendDenial> {
        let call = self.append_calls.get();
        self.append_calls.set(call + 1);
        if call < self.successful_appends_before_failure {
            return Ok(());
        }
        Err(injected_append_failure())
    }
}

impl OperationalControlStorePort for FailAfterSuccessfulControlAppends<'_> {
    fn publish_recovery_object(
        &self,
        content: &[u8],
    ) -> Result<ControlRecoveryObjectHandle, OperationalControlAppendDenial> {
        self.delegate.publish_recovery_object(content)
    }

    fn append(
        &self,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        self.admit_append()?;
        self.delegate.append(record)
    }

    fn compare_exchange_authorization_consumption(
        &self,
        expected: Option<worth_store_authority::ControlStoreGeneration>,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        self.admit_append()?;
        self.delegate
            .compare_exchange_authorization_consumption(expected, record)
    }
}

impl OperationalControlStorePort for LoseSuccessfulControlAppendReceipt<'_> {
    fn publish_recovery_object(
        &self,
        content: &[u8],
    ) -> Result<ControlRecoveryObjectHandle, OperationalControlAppendDenial> {
        self.delegate.publish_recovery_object(content)
    }

    fn append(
        &self,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        let call = self.append_calls.get();
        self.append_calls.set(call + 1);
        let receipt = self.delegate.append(record)?;
        if call == self.lost_call {
            return Err(OperationalControlAppendDenial::Media(
                ControlMediaFault::Io(std::io::Error::other(
                    "injected loss of a successful durable append receipt",
                )),
            ));
        }
        Ok(receipt)
    }

    fn compare_exchange_authorization_consumption(
        &self,
        expected: Option<worth_store_authority::ControlStoreGeneration>,
        record: &OperationalControlRecord,
    ) -> Result<PhysicalControlAppendReceipt, OperationalControlAppendDenial> {
        let call = self.append_calls.get();
        self.append_calls.set(call + 1);
        let receipt = self
            .delegate
            .compare_exchange_authorization_consumption(expected, record)?;
        if call == self.lost_call {
            return Err(lost_append_receipt());
        }
        Ok(receipt)
    }
}

fn injected_append_failure() -> OperationalControlAppendDenial {
    OperationalControlAppendDenial::Media(ControlMediaFault::Io(std::io::Error::other(
        "injected control-append media failure",
    )))
}

fn lost_append_receipt() -> OperationalControlAppendDenial {
    OperationalControlAppendDenial::Media(ControlMediaFault::Io(std::io::Error::other(
        "injected loss of a successful durable append receipt",
    )))
}
