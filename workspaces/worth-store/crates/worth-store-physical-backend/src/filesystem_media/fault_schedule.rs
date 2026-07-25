use std::sync::Arc;

use super::{MediaOperationRole, MediaPauseGate};

#[derive(Debug, Clone)]
pub enum MediaFaultDirective {
    FailBefore {
        kind: std::io::ErrorKind,
        raw_os_error: Option<i32>,
    },
    AllowPrefix {
        bytes: u64,
    },
    AllowPrefixThenPause {
        bytes: u64,
        gate: MediaPauseGate,
    },
    IndeterminateAfterEffect,
    FailBarrier {
        kind: std::io::ErrorKind,
        raw_os_error: Option<i32>,
    },
    PauseBefore(MediaPauseGate),
    PauseAfter(MediaPauseGate),
    PanicAfter,
    InterruptReplacementObservation,
}

#[derive(Debug, Clone)]
pub struct MediaFaultRule {
    pub(super) role: MediaOperationRole,
    pub(super) ordinal: u64,
    pub(super) directive: MediaFaultDirective,
    pub(super) owner: Option<super::MediaOwnerIdentity>,
    pub(super) store: Option<worth_store_physical_format::store_namespace::StableStoreIdentity>,
    pub(super) operation: Option<super::MediaOperationIdentity>,
    pub(super) identified_operation: Option<bool>,
    pub(super) identified_operation_ordinal: Option<u64>,
    pub(super) runtime_incarnation: Option<u64>,
    pub(super) any_ordinal_after_activation: bool,
    pub(super) activation: Option<super::fault_activation::CertificationMediaFaultActivation>,
}

impl MediaFaultRule {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(crate) fn for_certification(
        role: MediaOperationRole,
        ordinal: u64,
        directive: MediaFaultDirective,
    ) -> Self {
        Self {
            role,
            ordinal,
            directive,
            owner: None,
            store: None,
            operation: None,
            identified_operation: None,
            identified_operation_ordinal: None,
            runtime_incarnation: None,
            any_ordinal_after_activation: false,
            activation: None,
        }
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_owner(mut self, owner: super::MediaOwnerIdentity) -> Self {
        self.owner = Some(owner);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_store(
        mut self,
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    ) -> Self {
        self.store = Some(store);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_operation(mut self, operation: super::MediaOperationIdentity) -> Self {
        self.operation = Some(operation);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_identified_operation(mut self) -> Self {
        self.identified_operation = Some(true);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_identified_operation_ordinal(mut self) -> Self {
        self.identified_operation = Some(true);
        self.identified_operation_ordinal = Some(self.ordinal);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_runtime_incarnation(mut self, runtime_incarnation: u64) -> Self {
        self.runtime_incarnation = Some(runtime_incarnation);
        self
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn for_next_identified_operation_after_activation(
        mut self,
        activation: super::fault_activation::CertificationMediaFaultActivation,
    ) -> Self {
        self.identified_operation = Some(true);
        self.identified_operation_ordinal = None;
        self.any_ordinal_after_activation = true;
        self.activation = Some(activation);
        self
    }

    pub(super) fn matches(&self, context: super::MediaOperationContext) -> bool {
        let ordinal_matches = self.any_ordinal_after_activation
            || self.identified_operation_ordinal.map_or_else(
                || self.ordinal == context.role_ordinal(),
                |ordinal| context.identified_operation_ordinal() == Some(ordinal),
            );
        let structural_match = self.role == context.role()
            && ordinal_matches
            && self
                .owner
                .is_none_or(|owner| context.owner() == Some(owner))
            && self
                .store
                .is_none_or(|store| context.store() == Some(store))
            && self
                .operation
                .is_none_or(|operation| context.operation() == Some(operation))
            && self
                .identified_operation
                .is_none_or(|identified| context.operation().is_some() == identified)
            && self
                .runtime_incarnation
                .is_none_or(|runtime| context.runtime_incarnation() == Some(runtime));
        structural_match
            && self.activation.as_ref().is_none_or(
                super::fault_activation::CertificationMediaFaultActivation::consume_if_armed,
            )
    }
}

#[derive(Debug, Clone, Default)]
pub struct MediaFaultSchedule {
    pub(super) rules: Arc<[MediaFaultRule]>,
    pub(super) lease_release_pause: Option<MediaPauseGate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFaultScheduleDenial {
    ZeroOrdinal,
    DuplicateSemanticMatch,
    DirectiveRoleMismatch,
}

impl MediaFaultSchedule {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub(crate) fn for_certification(
        rules: Vec<MediaFaultRule>,
    ) -> Result<Self, MediaFaultScheduleDenial> {
        for (index, rule) in rules.iter().enumerate() {
            if rule.ordinal == 0 {
                return Err(MediaFaultScheduleDenial::ZeroOrdinal);
            }
            if !super::fault_schedule_validation::directive_matches_role(rule.role, &rule.directive)
            {
                return Err(MediaFaultScheduleDenial::DirectiveRoleMismatch);
            }
            if rules[..index].iter().any(|prior| {
                prior.role == rule.role
                    && same_ordinal_selector(prior, rule)
                    && prior.owner == rule.owner
                    && prior.store == rule.store
                    && prior.operation == rule.operation
                    && prior.identified_operation == rule.identified_operation
                    && prior.identified_operation_ordinal == rule.identified_operation_ordinal
                    && prior.runtime_incarnation == rule.runtime_incarnation
                    && prior.any_ordinal_after_activation == rule.any_ordinal_after_activation
                    && same_activation(prior.activation.as_ref(), rule.activation.as_ref())
            }) {
                return Err(MediaFaultScheduleDenial::DuplicateSemanticMatch);
            }
        }
        Ok(Self {
            rules: rules.into(),
            lease_release_pause: None,
        })
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn pause_before_lease_release(mut self, gate: MediaPauseGate) -> Self {
        self.lease_release_pause = Some(gate);
        self
    }
}

fn same_ordinal_selector(left: &MediaFaultRule, right: &MediaFaultRule) -> bool {
    if left.any_ordinal_after_activation || right.any_ordinal_after_activation {
        left.any_ordinal_after_activation == right.any_ordinal_after_activation
    } else {
        left.ordinal == right.ordinal
    }
}

fn same_activation(
    left: Option<&super::fault_activation::CertificationMediaFaultActivation>,
    right: Option<&super::fault_activation::CertificationMediaFaultActivation>,
) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => left.same_activation(right),
        _ => false,
    }
}
