use std::sync::Arc;

use crate::branch::{RelationalBranchBasisDescriptor, RelationalBranchIdentity};
use crate::mvcc::RelationalBranchObservation;

use super::{RelationalBasisRetentionReason, RelationalExternalRetentionTerminalAccounting};

/// Explicit external owner obligation for one admitted component basis.
///
/// This is separate from cloning an admitted basis. It is owner-issued and
/// must be consumed by release or dropped; it exposes no read or mutation
/// capability itself.
#[derive(Debug)]
pub struct RelationalComponentBasisRetentionLease {
    observation: Option<RelationalBranchObservation>,
    terminal_accounting: Arc<RelationalExternalRetentionTerminalAccounting>,
}

impl RelationalComponentBasisRetentionLease {
    pub(crate) fn new(
        observation: RelationalBranchObservation,
        terminal_accounting: Arc<RelationalExternalRetentionTerminalAccounting>,
    ) -> Self {
        Self {
            observation: Some(observation),
            terminal_accounting,
        }
    }

    pub fn identity(&self) -> &RelationalBranchIdentity {
        self.observation
            .as_ref()
            .expect("live retention lease carries its observation")
            .identity()
    }

    pub fn descriptor(&self) -> &RelationalBranchBasisDescriptor {
        self.observation
            .as_ref()
            .expect("live retention lease carries its observation")
            .descriptor()
    }

    pub const fn retention_reason(&self) -> RelationalBasisRetentionReason {
        RelationalBasisRetentionReason::ExternalComponentBasis
    }

    pub fn release(mut self) -> RelationalComponentBasisRetentionReleaseReceipt {
        let observation = self
            .observation
            .take()
            .expect("retention lease can be released only once");
        self.terminal_accounting.record_explicit_release();
        RelationalComponentBasisRetentionReleaseReceipt {
            descriptor: observation.descriptor().clone(),
        }
    }
}

impl Drop for RelationalComponentBasisRetentionLease {
    fn drop(&mut self) {
        if self.observation.take().is_some() {
            self.terminal_accounting.record_dropped_release();
        }
    }
}

/// A denied release that preserves the still-live external obligation.
#[derive(Debug)]
pub struct RelationalComponentBasisRetentionReleaseDenial {
    denial: crate::branch::RelationalBranchBasisDenial,
    lease: RelationalComponentBasisRetentionLease,
}

impl RelationalComponentBasisRetentionReleaseDenial {
    pub(crate) fn new(
        denial: crate::branch::RelationalBranchBasisDenial,
        lease: RelationalComponentBasisRetentionLease,
    ) -> Self {
        Self { denial, lease }
    }

    pub fn denial(&self) -> &crate::branch::RelationalBranchBasisDenial {
        &self.denial
    }

    pub fn lease(&self) -> &RelationalComponentBasisRetentionLease {
        &self.lease
    }

    pub fn into_lease(self) -> RelationalComponentBasisRetentionLease {
        self.lease
    }
}

/// Evidence that one explicit external retention obligation was consumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalComponentBasisRetentionReleaseReceipt {
    descriptor: RelationalBranchBasisDescriptor,
}

impl RelationalComponentBasisRetentionReleaseReceipt {
    pub fn descriptor(&self) -> &RelationalBranchBasisDescriptor {
        &self.descriptor
    }
}
