use worth_relational::facade::bridge::RuntimeBridgeRelationalSource;
use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::RuntimeBridge;

use super::admission::{validate_direct_run_head, validate_direct_run_lower};
use super::lower_admission::{
    admit_managed_lower_execution_basis, admit_managed_lower_execution_basis_from_retained,
    WorthQueryManagedLowerAdmissionFailureKind, WorthQueryManagedLowerBinding,
};
use super::{
    WorthQueryAdmittedDirectRun, WorthQueryManagedDirectRunAdmissionFailure,
    WorthQueryManagedDirectRunAdmissionFailureKind, WorthQueryManagedTruthReadRequest,
};
use crate::domain_computation::{
    WorthQueryDirectExecutionResourceAttempt, WorthQueryExecutionBoundOperationAuthority,
    WorthQueryExecutionRuntime,
};

pub struct WorthQueryManagedRunAdmission<'runtime> {
    pub(super) query: &'runtime WorthQueryExecutionRuntime,
    pub(super) bridge: &'runtime RuntimeBridge,
    pub(super) relational: &'runtime RuntimeBridgeRelationalSource,
}

impl WorthQueryExecutionRuntime {
    pub fn managed_run_admission<'runtime>(
        &'runtime self,
        bridge: &'runtime RuntimeBridge,
        relational: &'runtime RuntimeBridgeRelationalSource,
    ) -> WorthQueryManagedRunAdmission<'runtime> {
        WorthQueryManagedRunAdmission {
            query: self,
            bridge,
            relational,
        }
    }
}

impl WorthQueryManagedRunAdmission<'_> {
    pub fn admit_direct(
        &self,
        operation: &WorthQueryExecutionBoundOperationAuthority,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        request: WorthQueryManagedTruthReadRequest,
    ) -> Result<WorthQueryAdmittedDirectRun, WorthQueryManagedDirectRunAdmissionFailure> {
        let counters = match validate_direct_run_head(self.query, operation, &resource_attempt) {
            Ok(counters) => counters,
            Err(denial) => {
                return Err(WorthQueryManagedDirectRunAdmissionFailure::new(
                    WorthQueryManagedDirectRunAdmissionFailureKind::QueryAuthority,
                    denial.detail(),
                    resource_attempt,
                ));
            }
        };
        let lower = match admit_managed_lower_execution_basis(
            self.bridge,
            self.relational,
            WorthQueryManagedLowerBinding::new(
                operation.binding_identity(),
                resource_attempt.attempt_identity().as_str(),
                resource_attempt.resources().envelope(),
            ),
            request,
        ) {
            Ok(lower) => lower,
            Err(failure) => {
                let kind = match failure.kind {
                    WorthQueryManagedLowerAdmissionFailureKind::BridgeSourceProfile => {
                        WorthQueryManagedDirectRunAdmissionFailureKind::ManagedAuthorityJoin
                    }
                    WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis => {
                        WorthQueryManagedDirectRunAdmissionFailureKind::RelationalBasis
                    }
                    WorthQueryManagedLowerAdmissionFailureKind::BridgePlanning => {
                        WorthQueryManagedDirectRunAdmissionFailureKind::BridgePlanning
                    }
                    WorthQueryManagedLowerAdmissionFailureKind::InstalledStepContract => {
                        WorthQueryManagedDirectRunAdmissionFailureKind::InstalledStepContract
                    }
                    WorthQueryManagedLowerAdmissionFailureKind::BridgeExecutionBasis => {
                        WorthQueryManagedDirectRunAdmissionFailureKind::BridgeExecutionBasis
                    }
                };
                return Err(WorthQueryManagedDirectRunAdmissionFailure::new(
                    kind,
                    failure.detail,
                    resource_attempt,
                ));
            }
        };
        let counters = match validate_direct_run_lower(
            operation,
            &resource_attempt,
            &lower.bridge,
            &lower.relational,
            counters,
        ) {
            Ok(counters) => counters,
            Err(denial) => {
                return Err(WorthQueryManagedDirectRunAdmissionFailure::new(
                    WorthQueryManagedDirectRunAdmissionFailureKind::ManagedAuthorityJoin,
                    denial.detail(),
                    resource_attempt,
                ));
            }
        };
        Ok(WorthQueryAdmittedDirectRun::new(
            operation,
            resource_attempt,
            lower.bridge,
            lower.relational,
            counters,
        ))
    }

    pub(in crate::domain_computation) fn admit_direct_with_retained_basis(
        &self,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        request: WorthQueryManagedTruthReadRequest,
        relational_basis: RelationalExecutionBasisLease,
    ) -> Result<WorthQueryAdmittedDirectRun, WorthQueryManagedDirectRunAdmissionFailure> {
        let operation = resource_attempt.retain_binding_authority();
        let counters = match validate_direct_run_head(self.query, &operation, &resource_attempt) {
            Ok(counters) => counters,
            Err(denial) => {
                return Err(
                    WorthQueryManagedDirectRunAdmissionFailure::with_retained_basis(
                        WorthQueryManagedDirectRunAdmissionFailureKind::QueryAuthority,
                        denial.detail(),
                        resource_attempt,
                        relational_basis,
                    ),
                );
            }
        };
        let lower = match admit_managed_lower_execution_basis_from_retained(
            self.bridge,
            self.relational,
            WorthQueryManagedLowerBinding::new(
                operation.binding_identity(),
                resource_attempt.attempt_identity().as_str(),
                resource_attempt.resources().envelope(),
            ),
            request,
            relational_basis,
        ) {
            Ok(lower) => lower,
            Err(failure) => {
                let (kind, detail, retained_basis) = failure.into_parts();
                let retained_basis = retained_basis.expect(
                    "retained-basis managed admission returns the exact lease on every denial",
                );
                return Err(
                    WorthQueryManagedDirectRunAdmissionFailure::with_retained_basis(
                        map_lower_failure_kind(kind),
                        detail,
                        resource_attempt,
                        retained_basis,
                    ),
                );
            }
        };
        let counters = match validate_direct_run_lower(
            &operation,
            &resource_attempt,
            &lower.bridge,
            &lower.relational,
            counters,
        ) {
            Ok(counters) => counters,
            Err(denial) => {
                return Err(
                    WorthQueryManagedDirectRunAdmissionFailure::with_retained_basis(
                        WorthQueryManagedDirectRunAdmissionFailureKind::ManagedAuthorityJoin,
                        denial.detail(),
                        resource_attempt,
                        lower.relational,
                    ),
                );
            }
        };
        Ok(WorthQueryAdmittedDirectRun::new(
            &operation,
            resource_attempt,
            lower.bridge,
            lower.relational,
            counters,
        ))
    }
}

fn map_lower_failure_kind(
    kind: WorthQueryManagedLowerAdmissionFailureKind,
) -> WorthQueryManagedDirectRunAdmissionFailureKind {
    match kind {
        WorthQueryManagedLowerAdmissionFailureKind::BridgeSourceProfile => {
            WorthQueryManagedDirectRunAdmissionFailureKind::ManagedAuthorityJoin
        }
        WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis => {
            WorthQueryManagedDirectRunAdmissionFailureKind::RelationalBasis
        }
        WorthQueryManagedLowerAdmissionFailureKind::BridgePlanning => {
            WorthQueryManagedDirectRunAdmissionFailureKind::BridgePlanning
        }
        WorthQueryManagedLowerAdmissionFailureKind::InstalledStepContract => {
            WorthQueryManagedDirectRunAdmissionFailureKind::InstalledStepContract
        }
        WorthQueryManagedLowerAdmissionFailureKind::BridgeExecutionBasis => {
            WorthQueryManagedDirectRunAdmissionFailureKind::BridgeExecutionBasis
        }
    }
}
