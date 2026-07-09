use crate::identity::hash_parts;

use super::counters::BasisEligibilityCounters;
use super::lanes::BasisOperationLane;
use super::taxonomy::{
    BasisAuthorityPosture, BasisEligibilityDenialCause, BasisFamily, BasisIntentDenialKind,
    BasisLifecyclePosture, BasisScopePosture, BasisVisibilityPosture, DeniedBasisCapabilityKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedBasisIntent {
    family: BasisFamily,
    authority: BasisAuthorityPosture,
    scope: BasisScopePosture,
    visibility: BasisVisibilityPosture,
    lifecycle: BasisLifecyclePosture,
    operation_lane: String,
    policy_digest: Option<String>,
    tenant_schema_digest: Option<String>,
    eligibility_denial_cause: Option<BasisEligibilityDenialCause>,
    lower_runtime_binding_digest: Option<String>,
    source_path: String,
    normalized_digest: String,
    counters: BasisEligibilityCounters,
}

impl NormalizedBasisIntent {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        family: BasisFamily,
        authority: BasisAuthorityPosture,
        scope: BasisScopePosture,
        visibility: BasisVisibilityPosture,
        lifecycle: BasisLifecyclePosture,
        operation_lane: impl Into<String>,
        policy_digest: Option<String>,
        tenant_schema_digest: Option<String>,
        lower_runtime_binding_digest: Option<String>,
        source_path: impl Into<String>,
    ) -> Self {
        Self::new_with_denial_cause(
            family,
            authority,
            scope,
            visibility,
            lifecycle,
            operation_lane,
            policy_digest,
            tenant_schema_digest,
            None,
            lower_runtime_binding_digest,
            source_path,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new_with_denial_cause(
        family: BasisFamily,
        authority: BasisAuthorityPosture,
        scope: BasisScopePosture,
        visibility: BasisVisibilityPosture,
        lifecycle: BasisLifecyclePosture,
        operation_lane: impl Into<String>,
        policy_digest: Option<String>,
        tenant_schema_digest: Option<String>,
        eligibility_denial_cause: Option<BasisEligibilityDenialCause>,
        lower_runtime_binding_digest: Option<String>,
        source_path: impl Into<String>,
    ) -> Self {
        let operation_lane = operation_lane.into();
        let source_path = source_path.into();
        let normalized_digest = hash_parts(&[
            format!("family:{}", family.as_str()),
            format!("authority:{}", authority.as_str()),
            format!("scope:{}", scope.as_str()),
            format!("visibility:{}", visibility.as_str()),
            format!("lifecycle:{}", lifecycle.as_str()),
            format!("lane:{operation_lane}"),
            format!("policy:{}", policy_digest.as_deref().unwrap_or("none")),
            format!(
                "tenant_schema:{}",
                tenant_schema_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "eligibility_denial:{}",
                eligibility_denial_cause
                    .map(|cause| cause.as_str())
                    .unwrap_or("none")
            ),
            format!(
                "lower_runtime:{}",
                lower_runtime_binding_digest.as_deref().unwrap_or("none")
            ),
        ]);
        Self {
            family,
            authority,
            scope,
            visibility,
            lifecycle,
            operation_lane,
            policy_digest,
            tenant_schema_digest,
            eligibility_denial_cause,
            lower_runtime_binding_digest,
            source_path,
            normalized_digest,
            counters: BasisEligibilityCounters::normalized(1),
        }
    }

    pub fn family(&self) -> BasisFamily {
        self.family
    }

    pub(crate) fn authority(&self) -> BasisAuthorityPosture {
        self.authority
    }

    pub fn operation_lane(&self) -> &str {
        &self.operation_lane
    }

    pub fn normalized_digest(&self) -> &str {
        &self.normalized_digest
    }

    pub fn lower_runtime_binding_digest(&self) -> Option<&str> {
        self.lower_runtime_binding_digest.as_deref()
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub(crate) fn visibility(&self) -> BasisVisibilityPosture {
        self.visibility
    }

    pub(crate) fn lifecycle(&self) -> BasisLifecyclePosture {
        self.lifecycle
    }

    pub(crate) fn eligibility_denial_cause(&self) -> Option<BasisEligibilityDenialCause> {
        self.eligibility_denial_cause
    }

    pub(crate) fn capability_digest<L: BasisOperationLane>(&self) -> String {
        hash_parts(&[
            "admitted_basis_capability_v1".to_string(),
            format!("normalized:{}", self.normalized_digest),
            format!("lane:{}", L::lane_name()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisIntentDenial {
    denial_kind: BasisIntentDenialKind,
    message: &'static str,
    counters: BasisEligibilityCounters,
}

impl BasisIntentDenial {
    pub(crate) fn new(denial_kind: BasisIntentDenialKind, message: &'static str) -> Self {
        Self {
            denial_kind,
            message,
            counters: BasisEligibilityCounters::rejected(1),
        }
    }

    pub fn denial_kind(&self) -> BasisIntentDenialKind {
        self.denial_kind
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeniedBasisCapability {
    denial_kind: DeniedBasisCapabilityKind,
    decision_trace: BasisEligibilityDecisionTrace,
    counters: BasisEligibilityCounters,
}

impl DeniedBasisCapability {
    pub(crate) fn new(
        denial_kind: DeniedBasisCapabilityKind,
        normalized: &NormalizedBasisIntent,
        message: &'static str,
        counters: BasisEligibilityCounters,
    ) -> Self {
        Self {
            denial_kind,
            decision_trace: BasisEligibilityDecisionTrace::new(normalized, "violation", message),
            counters,
        }
    }

    pub fn denial_kind(&self) -> DeniedBasisCapabilityKind {
        self.denial_kind
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub(crate) fn new_readmission(
        denial_kind: DeniedBasisCapabilityKind,
        decision_trace: BasisEligibilityDecisionTrace,
        counters: BasisEligibilityCounters,
    ) -> Self {
        Self {
            denial_kind,
            decision_trace,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisEligibilityDecisionTrace {
    normalized_digest: String,
    outcome: String,
    message: &'static str,
    trace_digest: String,
}

impl BasisEligibilityDecisionTrace {
    pub(crate) fn new(
        normalized: &NormalizedBasisIntent,
        outcome: impl Into<String>,
        message: &'static str,
    ) -> Self {
        let outcome = outcome.into();
        let trace_digest = hash_parts(&[
            format!("normalized:{}", normalized.normalized_digest()),
            format!("outcome:{outcome}"),
            format!("message:{message}"),
        ]);
        Self {
            normalized_digest: normalized.normalized_digest().to_string(),
            outcome,
            message,
            trace_digest,
        }
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }

    pub(crate) fn new_lower_runtime_readmission(
        scoped_basis_digest: &str,
        evidence_digest: &str,
        outcome: impl Into<String>,
        message: &'static str,
    ) -> Self {
        let outcome = outcome.into();
        let trace_digest = hash_parts(&[
            format!("scoped_basis:{scoped_basis_digest}"),
            format!("lower_runtime_evidence:{evidence_digest}"),
            format!("outcome:{outcome}"),
            format!("message:{message}"),
        ]);
        Self {
            normalized_digest: scoped_basis_digest.to_string(),
            outcome,
            message,
            trace_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisEligibility<L: BasisOperationLane> {
    normalized: NormalizedBasisIntent,
    lane: L,
    decision_trace: BasisEligibilityDecisionTrace,
    counters: BasisEligibilityCounters,
}

impl<L: BasisOperationLane> BasisEligibility<L> {
    pub(crate) fn new(normalized: NormalizedBasisIntent, lane: L) -> Self {
        let decision_trace =
            BasisEligibilityDecisionTrace::new(&normalized, "success", "basis lane admitted");
        Self {
            normalized,
            lane,
            decision_trace,
            counters: BasisEligibilityCounters::eligibility(0, 0, 0, 0),
        }
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryBasisEligibility<L: BasisOperationLane> {
    normalized: NormalizedBasisIntent,
    lane: L,
    decision_trace: BasisEligibilityDecisionTrace,
}

impl<L: BasisOperationLane> AdvisoryBasisEligibility<L> {
    pub(crate) fn new(normalized: NormalizedBasisIntent, lane: L) -> Self {
        let decision_trace =
            BasisEligibilityDecisionTrace::new(&normalized, "advisory", "basis lane is advisory");
        Self {
            normalized,
            lane,
            decision_trace,
        }
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }

    pub fn authoring_digest(&self) -> String {
        hash_parts(&[
            "advisory_basis_authoring_v1".to_string(),
            format!("normalized:{}", self.normalized.normalized_digest()),
            format!("lane:{}", L::lane_name()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredBasisEligibility<L: BasisOperationLane> {
    normalized: NormalizedBasisIntent,
    lane: L,
    denial_kind: DeniedBasisCapabilityKind,
    decision_trace: BasisEligibilityDecisionTrace,
    counters: BasisEligibilityCounters,
}

impl<L: BasisOperationLane> DeferredBasisEligibility<L> {
    pub(crate) fn new(
        normalized: NormalizedBasisIntent,
        lane: L,
        denial_kind: DeniedBasisCapabilityKind,
        message: &'static str,
    ) -> Self {
        let decision_trace = BasisEligibilityDecisionTrace::new(&normalized, "deferred", message);
        Self {
            normalized,
            lane,
            denial_kind,
            decision_trace,
            counters: BasisEligibilityCounters::eligibility(0, 0, 1, 0),
        }
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }

    pub fn denial_kind(&self) -> DeniedBasisCapabilityKind {
        self.denial_kind
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub fn authoring_digest(&self) -> String {
        hash_parts(&[
            "deferred_basis_authoring_v1".to_string(),
            format!("normalized:{}", self.normalized.normalized_digest()),
            format!("lane:{}", L::lane_name()),
            format!("denial_kind:{}", self.denial_kind.as_str()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedBasisCapability<L: BasisOperationLane> {
    normalized: NormalizedBasisIntent,
    lane: L,
    capability_digest: String,
}

impl<L: BasisOperationLane> AdmittedBasisCapability<L> {
    pub(crate) fn new(eligibility: BasisEligibility<L>) -> Self {
        let capability_digest = eligibility.normalized.capability_digest::<L>();
        Self {
            normalized: eligibility.normalized,
            lane: eligibility.lane,
            capability_digest,
        }
    }

    pub fn capability_digest(&self) -> &str {
        &self.capability_digest
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }
}
