use crate::identity::hash_parts;

use super::counters::BasisEligibilityCounters;
use super::lanes::BasisOperationLane;
use super::taxonomy::{
    BasisAuthorityPosture, BasisEligibilityDenialCause, BasisFamily, BasisLifecyclePosture,
    BasisScopePosture, BasisVisibilityPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NormalizedBasisCoordinates {
    pub(super) family: BasisFamily,
    pub(super) authority: BasisAuthorityPosture,
    pub(super) scope: BasisScopePosture,
    pub(super) visibility: BasisVisibilityPosture,
    pub(super) lifecycle: BasisLifecyclePosture,
    pub(super) operation_lane: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct NormalizedBasisAuthorityBindings {
    pub(super) policy_digest: Option<String>,
    pub(super) tenant_schema_digest: Option<String>,
    pub(super) lower_runtime_binding_digest: Option<String>,
}

impl NormalizedBasisAuthorityBindings {
    pub(super) fn lower_runtime(lower_runtime_binding_digest: Option<String>) -> Self {
        Self {
            lower_runtime_binding_digest,
            ..Self::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NormalizedBasisAdmissionOutcome {
    Admitted,
    Denied(BasisEligibilityDenialCause),
}

impl NormalizedBasisAdmissionOutcome {
    fn denial_cause(self) -> Option<BasisEligibilityDenialCause> {
        match self {
            Self::Admitted => None,
            Self::Denied(cause) => Some(cause),
        }
    }

    fn digest_label(self) -> &'static str {
        match self {
            Self::Admitted => "none",
            Self::Denied(cause) => cause.as_str(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NormalizedBasisIntentInput {
    pub(super) coordinates: NormalizedBasisCoordinates,
    pub(super) authority_bindings: NormalizedBasisAuthorityBindings,
    pub(super) admission: NormalizedBasisAdmissionOutcome,
    pub(super) source_path: String,
}

impl NormalizedBasisIntentInput {
    pub(super) fn with_admission(
        coordinates: NormalizedBasisCoordinates,
        authority_bindings: NormalizedBasisAuthorityBindings,
        admission: NormalizedBasisAdmissionOutcome,
        source_path: &'static str,
    ) -> Self {
        Self {
            coordinates,
            authority_bindings,
            admission,
            source_path: source_path.into(),
        }
    }

    pub(super) fn admitted(
        coordinates: NormalizedBasisCoordinates,
        authority_bindings: NormalizedBasisAuthorityBindings,
        source_path: &'static str,
    ) -> Self {
        Self::with_admission(
            coordinates,
            authority_bindings,
            NormalizedBasisAdmissionOutcome::Admitted,
            source_path,
        )
    }

    pub(super) fn denied(
        coordinates: NormalizedBasisCoordinates,
        authority_bindings: NormalizedBasisAuthorityBindings,
        cause: BasisEligibilityDenialCause,
        source_path: &'static str,
    ) -> Self {
        Self::with_admission(
            coordinates,
            authority_bindings,
            NormalizedBasisAdmissionOutcome::Denied(cause),
            source_path,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedBasisIntent {
    coordinates: NormalizedBasisCoordinates,
    authority_bindings: NormalizedBasisAuthorityBindings,
    admission: NormalizedBasisAdmissionOutcome,
    source_path: String,
    normalized_digest: String,
    counters: BasisEligibilityCounters,
}

impl NormalizedBasisIntent {
    pub(crate) fn new(input: NormalizedBasisIntentInput) -> Self {
        let normalized_digest = normalized_basis_digest(&input);
        Self {
            coordinates: input.coordinates,
            authority_bindings: input.authority_bindings,
            admission: input.admission,
            source_path: input.source_path,
            normalized_digest,
            counters: BasisEligibilityCounters::normalized(1),
        }
    }

    pub fn family(&self) -> BasisFamily {
        self.coordinates.family
    }

    pub(crate) fn authority(&self) -> BasisAuthorityPosture {
        self.coordinates.authority
    }

    pub(crate) fn scope(&self) -> BasisScopePosture {
        self.coordinates.scope
    }

    pub fn operation_lane(&self) -> &str {
        &self.coordinates.operation_lane
    }

    pub fn normalized_digest(&self) -> &str {
        &self.normalized_digest
    }

    pub fn lower_runtime_binding_digest(&self) -> Option<&str> {
        self.authority_bindings
            .lower_runtime_binding_digest
            .as_deref()
    }

    pub(crate) fn policy_digest(&self) -> Option<&str> {
        self.authority_bindings.policy_digest.as_deref()
    }

    pub(crate) fn tenant_schema_digest(&self) -> Option<&str> {
        self.authority_bindings.tenant_schema_digest.as_deref()
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub(crate) fn visibility(&self) -> BasisVisibilityPosture {
        self.coordinates.visibility
    }

    pub(crate) fn lifecycle(&self) -> BasisLifecyclePosture {
        self.coordinates.lifecycle
    }

    pub(crate) fn eligibility_denial_cause(&self) -> Option<BasisEligibilityDenialCause> {
        self.admission.denial_cause()
    }

    pub(crate) fn capability_digest<L: BasisOperationLane>(&self) -> String {
        hash_parts(&[
            "admitted_basis_capability_v1".to_string(),
            format!("normalized:{}", self.normalized_digest),
            format!("lane:{}", L::lane_name()),
        ])
    }
}

fn normalized_basis_digest(input: &NormalizedBasisIntentInput) -> String {
    let coordinates = &input.coordinates;
    let bindings = &input.authority_bindings;
    hash_parts(&[
        format!("family:{}", coordinates.family.as_str()),
        format!("authority:{}", coordinates.authority.as_str()),
        format!("scope:{}", coordinates.scope.as_str()),
        format!("visibility:{}", coordinates.visibility.as_str()),
        format!("lifecycle:{}", coordinates.lifecycle.as_str()),
        format!("lane:{}", coordinates.operation_lane),
        format!(
            "policy:{}",
            bindings.policy_digest.as_deref().unwrap_or("none")
        ),
        format!(
            "tenant_schema:{}",
            bindings.tenant_schema_digest.as_deref().unwrap_or("none")
        ),
        format!("eligibility_denial:{}", input.admission.digest_label()),
        format!(
            "lower_runtime:{}",
            bindings
                .lower_runtime_binding_digest
                .as_deref()
                .unwrap_or("none")
        ),
    ])
}
