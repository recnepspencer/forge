use std::collections::{BTreeMap, BTreeSet};

use crate::physical_runtime::{PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition};

use super::PhysicalWorkCourtroomFinding;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct SignalRequestLineage {
    pub request: u64,
    pub generation: u64,
    pub branch: u64,
    pub restore_epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct SignalAttemptLineage {
    pub request: u64,
    pub generation: u64,
    pub branch: u64,
    pub restore_epoch: u64,
    pub attempt: u64,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct CausalAttempt {
    pub operation: u64,
    pub signal: SignalAttemptLineage,
    pub predecessor: Option<SignalRequestLineage>,
    pub backend_operation: Option<u64>,
    pub fate: PhysicalWorkEffectFate,
    pub recovery: PhysicalWorkRecoveryDisposition,
}

#[derive(Default)]
pub(super) struct CausalValidation {
    signal_attempts: BTreeMap<SignalAttemptLineage, u64>,
    backend_operations: BTreeSet<u64>,
    latest_by_operation: BTreeMap<u64, CausalAttempt>,
}

impl CausalValidation {
    pub(super) fn observe(
        &mut self,
        attempt: CausalAttempt,
        findings: &mut Vec<PhysicalWorkCourtroomFinding>,
    ) {
        if self
            .signal_attempts
            .insert(attempt.signal, attempt.operation)
            .is_some()
        {
            findings.push(PhysicalWorkCourtroomFinding::DuplicateSignalAttemptIdentity);
        }
        if let Some(backend) = attempt.backend_operation {
            if !self.backend_operations.insert(backend) {
                findings.push(PhysicalWorkCourtroomFinding::DuplicateBackendOperationIdentity);
            }
        }
        if let Some(previous) = self.latest_by_operation.insert(attempt.operation, attempt) {
            if !legal_retry_successor(previous, attempt) {
                findings.push(PhysicalWorkCourtroomFinding::InvalidRetryCausalChain);
            }
        }
    }
}

fn legal_retry_successor(previous: CausalAttempt, current: CausalAttempt) -> bool {
    current.predecessor.is_some_and(|predecessor| {
        predecessor.request == previous.signal.request
            && predecessor.generation == previous.signal.generation
    }) && current.signal.request != previous.signal.request
        && current.signal.generation == previous.signal.generation
        && current.signal.attempt == previous.signal.attempt.saturating_add(1)
        && current.signal.attempt > previous.signal.attempt
        && previous.fate == PhysicalWorkEffectFate::ProvenNoEffect
        && matches!(
            previous.recovery,
            PhysicalWorkRecoveryDisposition::NoEffect | PhysicalWorkRecoveryDisposition::RetryExact
        )
}

#[cfg(test)]
mod tests {
    use crate::physical_runtime::{PhysicalWorkEffectFate, PhysicalWorkRecoveryDisposition};

    use super::{
        CausalAttempt, CausalValidation, PhysicalWorkCourtroomFinding, SignalAttemptLineage,
        SignalRequestLineage,
    };

    #[test]
    fn proven_no_effect_attempt_can_precede_one_exact_retry() {
        let mut validation = CausalValidation::default();
        let mut findings = Vec::new();
        validation.observe(
            attempt(
                7,
                1,
                Some(101),
                PhysicalWorkEffectFate::ProvenNoEffect,
                PhysicalWorkRecoveryDisposition::RetryExact,
            ),
            &mut findings,
        );
        validation.observe(
            attempt(
                7,
                2,
                Some(102),
                PhysicalWorkEffectFate::WriteCompleted,
                PhysicalWorkRecoveryDisposition::ContinueSettlement,
            ),
            &mut findings,
        );
        assert!(findings.is_empty());
    }

    #[test]
    fn duplicate_signal_attempt_and_backend_receipt_are_both_rejected() {
        let mut validation = CausalValidation::default();
        let mut findings = Vec::new();
        let first = attempt(
            7,
            1,
            Some(101),
            PhysicalWorkEffectFate::ProvenNoEffect,
            PhysicalWorkRecoveryDisposition::RetryExact,
        );
        validation.observe(first, &mut findings);
        validation.observe(
            CausalAttempt {
                operation: 8,
                ..first
            },
            &mut findings,
        );
        assert!(findings.contains(&PhysicalWorkCourtroomFinding::DuplicateSignalAttemptIdentity));
        assert!(findings.contains(&PhysicalWorkCourtroomFinding::DuplicateBackendOperationIdentity));
    }

    #[test]
    fn effectful_attempt_cannot_be_followed_under_the_same_operation() {
        let mut validation = CausalValidation::default();
        let mut findings = Vec::new();
        validation.observe(
            attempt(
                7,
                1,
                Some(101),
                PhysicalWorkEffectFate::WriteCompleted,
                PhysicalWorkRecoveryDisposition::ContinueSettlement,
            ),
            &mut findings,
        );
        validation.observe(
            attempt(
                7,
                2,
                Some(102),
                PhysicalWorkEffectFate::WriteCompleted,
                PhysicalWorkRecoveryDisposition::ContinueSettlement,
            ),
            &mut findings,
        );
        assert!(findings.contains(&PhysicalWorkCourtroomFinding::InvalidRetryCausalChain));
    }

    #[test]
    fn retry_without_the_exact_predecessor_request_is_rejected() {
        let mut validation = CausalValidation::default();
        let mut findings = Vec::new();
        validation.observe(
            attempt(
                7,
                1,
                Some(101),
                PhysicalWorkEffectFate::ProvenNoEffect,
                PhysicalWorkRecoveryDisposition::RetryExact,
            ),
            &mut findings,
        );
        let mut retry = attempt(
            7,
            2,
            Some(102),
            PhysicalWorkEffectFate::WriteCompleted,
            PhysicalWorkRecoveryDisposition::ContinueSettlement,
        );
        retry.predecessor.as_mut().unwrap().request = 900;
        validation.observe(retry, &mut findings);
        assert!(findings.contains(&PhysicalWorkCourtroomFinding::InvalidRetryCausalChain));
    }

    fn attempt(
        operation: u64,
        signal_attempt: u64,
        backend_operation: Option<u64>,
        fate: PhysicalWorkEffectFate,
        recovery: PhysicalWorkRecoveryDisposition,
    ) -> CausalAttempt {
        CausalAttempt {
            operation,
            signal: SignalAttemptLineage {
                request: 40 + signal_attempt,
                generation: 3,
                branch: 1,
                restore_epoch: 0,
                attempt: signal_attempt,
            },
            predecessor: (signal_attempt > 1).then_some(SignalRequestLineage {
                request: 39 + signal_attempt,
                generation: 3,
                branch: 1,
                restore_epoch: 0,
            }),
            backend_operation,
            fate,
            recovery,
        }
    }
}
