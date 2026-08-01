use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;

use crate::domain_computation::WorthQueryDirectExecutionResourceAttempt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedDirectRunAdmissionFailureKind {
    QueryAuthority,
    RelationalBasis,
    BridgePlanning,
    InstalledStepContract,
    BridgeExecutionBasis,
    ManagedAuthorityJoin,
}

pub struct WorthQueryManagedDirectRunAdmissionFailure {
    kind: WorthQueryManagedDirectRunAdmissionFailureKind,
    detail: Arc<str>,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    retained_relational_basis: Option<RelationalExecutionBasisLease>,
}

impl WorthQueryManagedDirectRunAdmissionFailure {
    pub(super) fn new(
        kind: WorthQueryManagedDirectRunAdmissionFailureKind,
        detail: impl Into<Arc<str>>,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            resource_attempt,
            retained_relational_basis: None,
        }
    }

    pub(super) fn with_retained_basis(
        kind: WorthQueryManagedDirectRunAdmissionFailureKind,
        detail: impl Into<Arc<str>>,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        retained_relational_basis: RelationalExecutionBasisLease,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            resource_attempt,
            retained_relational_basis: Some(retained_relational_basis),
        }
    }

    pub fn kind(&self) -> WorthQueryManagedDirectRunAdmissionFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn into_resource_attempt(self) -> WorthQueryDirectExecutionResourceAttempt {
        self.resource_attempt
    }

    pub(in crate::domain_computation) fn into_retained_resources(
        self,
    ) -> Option<(
        WorthQueryDirectExecutionResourceAttempt,
        RelationalExecutionBasisLease,
    )> {
        self.retained_relational_basis
            .map(|basis| (self.resource_attempt, basis))
    }
}

impl std::fmt::Debug for WorthQueryManagedDirectRunAdmissionFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryManagedDirectRunAdmissionFailure")
            .field("kind", &self.kind)
            .field("detail", &self.detail)
            .field(
                "resource_attempt",
                &self.resource_attempt.resources().identity(),
            )
            .finish()
    }
}
