use super::identity::basis_lifecycle_digest;

use super::{
    BasisAuthorityPosture, BasisOperationLaneRequest, BasisTenantSchemaPosture,
    NormalizedBasisFamily, NormalizedBasisIntent, NormalizedBasisSubject,
};

mod rules;

pub use rules::evaluate_basis_eligibility;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisEligibilityDisposition {
    Success,
    Advisory,
}

impl BasisEligibilityDisposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Advisory => "advisory",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeniedBasisCapabilityKind {
    Stale {
        family: NormalizedBasisFamily,
    },
    Inaccessible {
        family: NormalizedBasisFamily,
        authority: &'static str,
    },
    PolicyMasked {
        policy_scope: String,
    },
    TenantMismatched {
        tenant_scope: String,
    },
    SchemaIncompatible {
        schema_scope: String,
    },
    OperationIneligible {
        family: NormalizedBasisFamily,
        operation_lane: BasisOperationLaneRequest,
    },
    LowerRuntimeBindingMissing {
        authority: &'static str,
        family: NormalizedBasisFamily,
        operation_lane: BasisOperationLaneRequest,
    },
    LowerRuntimeBindingMismatch {
        authority: &'static str,
        expected: String,
        observed: String,
    },
    LowerRuntimeCapabilityUnsupported {
        authority: &'static str,
        family: NormalizedBasisFamily,
        operation_lane: BasisOperationLaneRequest,
    },
    HistoricalReplayUnsupported {
        family: NormalizedBasisFamily,
    },
    PreviewDrifted {
        family: NormalizedBasisFamily,
    },
    DurableOverclaim {
        family: NormalizedBasisFamily,
        operation_lane: BasisOperationLaneRequest,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisEligibilityCounters {
    consulted_row_count: usize,
    tenant_check_count: usize,
    policy_check_count: usize,
    schema_check_count: usize,
    lower_runtime_check_count: usize,
    denied_residue_count: usize,
}

impl BasisEligibilityCounters {
    pub fn consulted_row_count(&self) -> usize {
        self.consulted_row_count
    }

    pub fn tenant_check_count(&self) -> usize {
        self.tenant_check_count
    }

    pub fn policy_check_count(&self) -> usize {
        self.policy_check_count
    }

    pub fn schema_check_count(&self) -> usize {
        self.schema_check_count
    }

    pub fn lower_runtime_check_count(&self) -> usize {
        self.lower_runtime_check_count
    }

    pub fn denied_residue_count(&self) -> usize {
        self.denied_residue_count
    }

    pub(crate) fn for_intent(intent: &NormalizedBasisIntent, denied_residue_count: usize) -> Self {
        Self {
            consulted_row_count: 1,
            tenant_check_count: usize::from(intent.tenant_scope().is_some()),
            policy_check_count: usize::from(intent.policy_scope().is_some()),
            schema_check_count: usize::from(intent.schema_scope().is_some()),
            lower_runtime_check_count: 0,
            denied_residue_count,
        }
    }

    pub(crate) fn with_lower_runtime_check(mut self, denied_residue_count: usize) -> Self {
        self.lower_runtime_check_count += 1;
        self.denied_residue_count = denied_residue_count;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisEligibilityTrace {
    rule_label: &'static str,
    explanation: &'static str,
}

impl BasisEligibilityTrace {
    pub fn rule_label(&self) -> &'static str {
        self.rule_label
    }

    pub fn explanation(&self) -> &'static str {
        self.explanation
    }
}

pub(crate) fn denied_basis_capability_for_lane_mismatch(
    normalized_basis_intent_digest: &str,
    family: &NormalizedBasisFamily,
    operation_lane: &BasisOperationLaneRequest,
    mut counters: BasisEligibilityCounters,
    rule_label: &'static str,
    explanation: &'static str,
    failure_label: &'static str,
) -> DeniedBasisCapability {
    counters.denied_residue_count = 1;
    DeniedBasisCapability {
        normalized_basis_intent_digest: normalized_basis_intent_digest.to_string(),
        family: family.clone(),
        operation_lane: operation_lane.clone(),
        kind: DeniedBasisCapabilityKind::OperationIneligible {
            family: family.clone(),
            operation_lane: operation_lane.clone(),
        },
        trace: BasisEligibilityTrace {
            rule_label,
            explanation,
        },
        counters,
        failure_digest: basis_lifecycle_digest(
            "basis_capability_ineligible_denial_v1",
            [
                (
                    "normalized_basis_intent_digest",
                    normalized_basis_intent_digest.to_string(),
                ),
                ("failure", failure_label.to_string()),
            ],
        ),
    }
}

pub(crate) fn denied_basis_capability_for_lower_runtime_mismatch(
    normalized_basis_intent_digest: &str,
    family: &NormalizedBasisFamily,
    operation_lane: &BasisOperationLaneRequest,
    counters: BasisEligibilityCounters,
    authority: &'static str,
    expected: impl Into<String>,
    observed: impl Into<String>,
) -> DeniedBasisCapability {
    let expected = expected.into();
    let observed = observed.into();
    DeniedBasisCapability {
        normalized_basis_intent_digest: normalized_basis_intent_digest.to_string(),
        family: family.clone(),
        operation_lane: operation_lane.clone(),
        kind: DeniedBasisCapabilityKind::LowerRuntimeBindingMismatch {
            authority,
            expected: expected.clone(),
            observed: observed.clone(),
        },
        trace: BasisEligibilityTrace {
            rule_label: "lower_runtime_binding_mismatch",
            explanation:
                "lower-runtime authority evidence did not match the admitted query basis capability",
        },
        counters: counters.with_lower_runtime_check(1),
        failure_digest: basis_lifecycle_digest(
            "basis_lower_runtime_binding_mismatch_v1",
            [
                (
                    "normalized_basis_intent_digest",
                    normalized_basis_intent_digest.to_string(),
                ),
                ("authority", authority.to_string()),
                ("expected", expected.clone()),
                ("observed", observed.clone()),
                ("failure", "lower_runtime_binding_mismatch".to_string()),
            ],
        ),
    }
}

pub(crate) fn denied_basis_capability_for_lower_runtime_unsupported(
    normalized_basis_intent_digest: &str,
    family: &NormalizedBasisFamily,
    operation_lane: &BasisOperationLaneRequest,
    counters: BasisEligibilityCounters,
    authority: &'static str,
) -> DeniedBasisCapability {
    DeniedBasisCapability {
        normalized_basis_intent_digest: normalized_basis_intent_digest.to_string(),
        family: family.clone(),
        operation_lane: operation_lane.clone(),
        kind: DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported {
            authority,
            family: family.clone(),
            operation_lane: operation_lane.clone(),
        },
        trace: BasisEligibilityTrace {
            rule_label: "lower_runtime_capability_unsupported",
            explanation:
                "the admitted query basis capability does not have a supported lower-runtime binding path for this authority",
        },
        counters: counters.with_lower_runtime_check(1),
        failure_digest: basis_lifecycle_digest(
            "basis_lower_runtime_capability_unsupported_v1",
            [
                (
                    "normalized_basis_intent_digest",
                    normalized_basis_intent_digest.to_string(),
                ),
                ("authority", authority.to_string()),
                ("failure", "lower_runtime_capability_unsupported".to_string()),
            ],
        ),
    }
}

pub(crate) fn denied_basis_capability_for_scoped_use_requires_admitted_capability(
    normalized_basis_intent_digest: &str,
    family: &NormalizedBasisFamily,
    operation_lane: &BasisOperationLaneRequest,
    mut counters: BasisEligibilityCounters,
    scoped_label: &'static str,
) -> DeniedBasisCapability {
    counters.denied_residue_count = 1;
    DeniedBasisCapability {
        normalized_basis_intent_digest: normalized_basis_intent_digest.to_string(),
        family: family.clone(),
        operation_lane: operation_lane.clone(),
        kind: DeniedBasisCapabilityKind::OperationIneligible {
            family: family.clone(),
            operation_lane: operation_lane.clone(),
        },
        trace: BasisEligibilityTrace {
            rule_label: "scoped_use_requires_admitted_capability",
            explanation:
                "scoped-use construction for this lane requires an admitted capability rather than an advisory one",
        },
        counters,
        failure_digest: basis_lifecycle_digest(
            "basis_scoped_requires_admitted_capability_v1",
            [
                (
                    "normalized_basis_intent_digest",
                    normalized_basis_intent_digest.to_string(),
                ),
                ("scoped_label", scoped_label.to_string()),
                ("failure", "scoped_use_requires_admitted_capability".to_string()),
            ],
        ),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisEligibility {
    normalized_basis_intent_digest: String,
    family: NormalizedBasisFamily,
    authority_posture: BasisAuthorityPosture,
    normalized_subject: NormalizedBasisSubject,
    normalized_label: String,
    operation_lane: BasisOperationLaneRequest,
    tenant_schema_posture: BasisTenantSchemaPosture,
    disposition: BasisEligibilityDisposition,
    trace: BasisEligibilityTrace,
    counters: BasisEligibilityCounters,
    eligibility_digest: String,
}

impl BasisEligibility {
    pub fn normalized_basis_intent_digest(&self) -> &str {
        &self.normalized_basis_intent_digest
    }

    pub fn family(&self) -> &NormalizedBasisFamily {
        &self.family
    }

    pub fn authority_posture(&self) -> &BasisAuthorityPosture {
        &self.authority_posture
    }

    pub fn normalized_subject(&self) -> &NormalizedBasisSubject {
        &self.normalized_subject
    }

    pub fn normalized_label(&self) -> &str {
        &self.normalized_label
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn tenant_schema_posture(&self) -> &BasisTenantSchemaPosture {
        &self.tenant_schema_posture
    }

    pub fn disposition(&self) -> &BasisEligibilityDisposition {
        &self.disposition
    }

    pub fn trace(&self) -> &BasisEligibilityTrace {
        &self.trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeniedBasisCapability {
    normalized_basis_intent_digest: String,
    family: NormalizedBasisFamily,
    operation_lane: BasisOperationLaneRequest,
    kind: DeniedBasisCapabilityKind,
    trace: BasisEligibilityTrace,
    counters: BasisEligibilityCounters,
    failure_digest: String,
}

impl DeniedBasisCapability {
    pub fn normalized_basis_intent_digest(&self) -> &str {
        &self.normalized_basis_intent_digest
    }

    pub fn family(&self) -> &NormalizedBasisFamily {
        &self.family
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn kind(&self) -> &DeniedBasisCapabilityKind {
        &self.kind
    }

    pub fn trace(&self) -> &BasisEligibilityTrace {
        &self.trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}
