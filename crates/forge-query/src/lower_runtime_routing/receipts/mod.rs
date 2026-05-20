use crate::identity::hash_parts;

use super::{ForgeQueryLowerRuntimeCapabilityEligibility, ForgeQueryLowerRuntimeRoutePlan};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeReadmissionReceipt {
    eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
    retained_evidence_digest: String,
    handoff_digest: String,
}

impl ForgeQueryLowerRuntimeReadmissionReceipt {
    pub(crate) fn new(
        eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
        retained_evidence_digest: impl Into<String>,
    ) -> Self {
        let retained_evidence_digest = retained_evidence_digest.into();
        let handoff_digest = hash_parts(&[
            "lower_runtime_readmission_receipt_v1".to_string(),
            format!("eligibility:{}", eligibility.eligibility_digest()),
            format!("retained_evidence:{retained_evidence_digest}"),
        ]);
        Self {
            eligibility,
            retained_evidence_digest,
            handoff_digest,
        }
    }

    pub fn eligibility(&self) -> &ForgeQueryLowerRuntimeCapabilityEligibility {
        &self.eligibility
    }

    pub fn retained_evidence_digest(&self) -> &str {
        &self.retained_evidence_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeBoundaryExecutionKind {
    RoutePlan,
    ReadmissionHandoff,
}

impl ForgeQueryLowerRuntimeBoundaryExecutionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoutePlan => "route-plan",
            Self::ReadmissionHandoff => "readmission-handoff",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
    kind: ForgeQueryLowerRuntimeBoundaryExecutionKind,
    request_digest: String,
    eligibility_digest: String,
    route_or_handoff_digest: String,
    retained_evidence_digest: String,
    boundary_execution_digest: String,
}

impl ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
    pub(crate) fn from_route_plan(
        plan: &ForgeQueryLowerRuntimeRoutePlan,
        retained_evidence_digest: impl Into<String>,
    ) -> Self {
        let retained_evidence_digest = retained_evidence_digest.into();
        Self::new(
            ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan,
            plan.eligibility().request().request_digest(),
            plan.eligibility().eligibility_digest(),
            plan.route_digest(),
            retained_evidence_digest,
        )
    }

    pub(crate) fn from_readmission_receipt(
        receipt: &ForgeQueryLowerRuntimeReadmissionReceipt,
    ) -> Self {
        Self::new(
            ForgeQueryLowerRuntimeBoundaryExecutionKind::ReadmissionHandoff,
            receipt.eligibility().request().request_digest(),
            receipt.eligibility().eligibility_digest(),
            receipt.handoff_digest(),
            receipt.retained_evidence_digest(),
        )
    }

    fn new(
        kind: ForgeQueryLowerRuntimeBoundaryExecutionKind,
        request_digest: impl Into<String>,
        eligibility_digest: impl Into<String>,
        route_or_handoff_digest: impl Into<String>,
        retained_evidence_digest: impl Into<String>,
    ) -> Self {
        let request_digest = request_digest.into();
        let eligibility_digest = eligibility_digest.into();
        let route_or_handoff_digest = route_or_handoff_digest.into();
        let retained_evidence_digest = retained_evidence_digest.into();
        let boundary_execution_digest = hash_parts(&[
            "lower_runtime_boundary_execution_receipt_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("request:{request_digest}"),
            format!("eligibility:{eligibility_digest}"),
            format!("route_or_handoff:{route_or_handoff_digest}"),
            format!("retained_evidence:{retained_evidence_digest}"),
        ]);
        Self {
            kind,
            request_digest,
            eligibility_digest,
            route_or_handoff_digest,
            retained_evidence_digest,
            boundary_execution_digest,
        }
    }

    pub fn kind(&self) -> ForgeQueryLowerRuntimeBoundaryExecutionKind {
        self.kind
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn route_or_handoff_digest(&self) -> &str {
        &self.route_or_handoff_digest
    }

    pub fn retained_evidence_digest(&self) -> &str {
        &self.retained_evidence_digest
    }

    pub fn boundary_execution_digest(&self) -> &str {
        &self.boundary_execution_digest
    }

    pub fn drift_from_route_plan(
        &self,
        plan: &ForgeQueryLowerRuntimeRoutePlan,
        retained_evidence_digest: &str,
    ) -> Option<String> {
        if self.kind != ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan {
            return Some("boundary execution kind drifted from route-plan".to_string());
        }
        if self.request_digest != plan.eligibility().request().request_digest() {
            return Some(
                "boundary execution request digest drifted from the route plan".to_string(),
            );
        }
        if self.eligibility_digest != plan.eligibility().eligibility_digest() {
            return Some(
                "boundary execution eligibility digest drifted from the route plan".to_string(),
            );
        }
        if self.route_or_handoff_digest != plan.route_digest() {
            return Some("boundary execution route digest drifted from the route plan".to_string());
        }
        if self.retained_evidence_digest != retained_evidence_digest {
            return Some(
                "boundary execution retained evidence drifted from the routed evidence".to_string(),
            );
        }
        None
    }

    pub fn drift_from_readmission_receipt(
        &self,
        receipt: &ForgeQueryLowerRuntimeReadmissionReceipt,
    ) -> Option<String> {
        if self.kind != ForgeQueryLowerRuntimeBoundaryExecutionKind::ReadmissionHandoff {
            return Some("boundary execution kind drifted from readmission-handoff".to_string());
        }
        if self.request_digest != receipt.eligibility().request().request_digest() {
            return Some(
                "boundary execution request digest drifted from the readmission receipt"
                    .to_string(),
            );
        }
        if self.eligibility_digest != receipt.eligibility().eligibility_digest() {
            return Some(
                "boundary execution eligibility digest drifted from the readmission receipt"
                    .to_string(),
            );
        }
        if self.route_or_handoff_digest != receipt.handoff_digest() {
            return Some(
                "boundary execution handoff digest drifted from the readmission receipt"
                    .to_string(),
            );
        }
        if self.retained_evidence_digest != receipt.retained_evidence_digest() {
            return Some(
                "boundary execution retained evidence drifted from the readmission receipt"
                    .to_string(),
            );
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeCapabilityRequest,
        ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
    };

    #[test]
    fn route_plan_boundary_receipt_reuses_request_and_eligibility_digests() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "write-authority",
            "subject-1",
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request, "detail-1");
        let plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "mutation-write");
        let boundary =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(&plan, "receipt-1");

        assert_eq!(
            boundary.request_digest(),
            plan.eligibility().request().request_digest()
        );
        assert_eq!(
            boundary.eligibility_digest(),
            plan.eligibility().eligibility_digest()
        );
        assert_eq!(
            boundary.kind(),
            ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan
        );
    }
}
