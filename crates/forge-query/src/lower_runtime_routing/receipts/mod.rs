use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::{ForgeQueryLowerRuntimeCapabilityEligibility, ForgeQueryLowerRuntimeRoutePlan};

pub(crate) fn forge_query_lower_runtime_retained_evidence_identity(
    retained_family: impl AsRef<str>,
    retained_evidence: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryLowerRuntimeRetainedEvidenceIdentity {
    ForgeQueryLowerRuntimeRetainedEvidenceIdentity::new(
        forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_shape(
                ForgeQueryEvidenceTag::new("retained_family"),
                retained_family,
            )
            .field_evidence_identity(ForgeQueryEvidenceTag::new("retained"), retained_evidence)
            .seal(),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeRetainedEvidenceIdentity {
    evidence_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeRetainedEvidenceIdentity {
    pub(crate) fn new(evidence_identity: ForgeQueryEvidenceIdentity) -> Self {
        Self { evidence_identity }
    }

    pub(crate) fn from_evidence_identity(
        retained_family: impl AsRef<str>,
        evidence_identity: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        forge_query_lower_runtime_retained_evidence_identity(retained_family, evidence_identity)
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }

    pub fn as_str(&self) -> &str {
        self.evidence_identity.as_ref()
    }
}

impl AsRef<str> for ForgeQueryLowerRuntimeRetainedEvidenceIdentity {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeReadmissionReceipt {
    eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
    retained_evidence_identity: ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    handoff_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeReadmissionReceipt {
    pub(crate) fn new(
        eligibility: ForgeQueryLowerRuntimeCapabilityEligibility,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        let handoff_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeReadmissionReceipt)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("eligibility"),
                    eligibility.eligibility_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("retained_evidence"),
                    retained_evidence_identity.evidence_identity(),
                )
                .seal();
        Self {
            eligibility,
            retained_evidence_identity: retained_evidence_identity.clone(),
            handoff_identity,
        }
    }

    pub fn eligibility(&self) -> &ForgeQueryLowerRuntimeCapabilityEligibility {
        &self.eligibility
    }

    pub fn retained_evidence_identity(&self) -> &ForgeQueryLowerRuntimeRetainedEvidenceIdentity {
        &self.retained_evidence_identity
    }

    pub fn handoff_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.handoff_identity
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
    request_identity: ForgeQueryEvidenceIdentity,
    eligibility_identity: ForgeQueryEvidenceIdentity,
    route_or_handoff_identity: ForgeQueryEvidenceIdentity,
    retained_evidence_identity: ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    boundary_execution_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
    pub(crate) fn from_route_plan(
        plan: &ForgeQueryLowerRuntimeRoutePlan,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        Self::new(
            ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan,
            plan.eligibility().request().request_identity(),
            plan.eligibility().eligibility_identity(),
            plan.route_identity(),
            retained_evidence_identity,
        )
    }

    pub(crate) fn from_route_plan_with_retained_evidence_identity(
        plan: &ForgeQueryLowerRuntimeRoutePlan,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        Self::from_route_plan(plan, retained_evidence_identity)
    }

    pub(crate) fn from_readmission_receipt(
        receipt: &ForgeQueryLowerRuntimeReadmissionReceipt,
    ) -> Self {
        Self::new(
            ForgeQueryLowerRuntimeBoundaryExecutionKind::ReadmissionHandoff,
            receipt.eligibility().request().request_identity(),
            receipt.eligibility().eligibility_identity(),
            receipt.handoff_identity(),
            receipt.retained_evidence_identity(),
        )
    }

    fn new(
        kind: ForgeQueryLowerRuntimeBoundaryExecutionKind,
        request_identity: &ForgeQueryEvidenceIdentity,
        eligibility_identity: &ForgeQueryEvidenceIdentity,
        route_or_handoff_identity: &ForgeQueryEvidenceIdentity,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        let boundary_execution_identity = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryExecutionReceipt,
        )
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("request"), request_identity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("eligibility"),
            eligibility_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("route_or_handoff"),
            route_or_handoff_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("retained_evidence"),
            retained_evidence_identity.evidence_identity(),
        )
        .seal();
        Self {
            kind,
            request_identity: request_identity.clone(),
            eligibility_identity: eligibility_identity.clone(),
            route_or_handoff_identity: route_or_handoff_identity.clone(),
            retained_evidence_identity: retained_evidence_identity.clone(),
            boundary_execution_identity,
        }
    }

    pub fn kind(&self) -> ForgeQueryLowerRuntimeBoundaryExecutionKind {
        self.kind
    }

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn eligibility_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.eligibility_identity
    }

    pub fn route_or_handoff_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.route_or_handoff_identity
    }

    pub fn retained_evidence_identity(&self) -> &ForgeQueryLowerRuntimeRetainedEvidenceIdentity {
        &self.retained_evidence_identity
    }

    pub fn boundary_execution_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.boundary_execution_identity
    }

    pub fn drift_from_route_plan(
        &self,
        plan: &ForgeQueryLowerRuntimeRoutePlan,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Option<String> {
        if self.kind != ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan {
            return Some("boundary execution kind drifted from route-plan".to_string());
        }
        if self.request_identity != *plan.eligibility().request().request_identity() {
            return Some(
                "boundary execution request digest drifted from the route plan".to_string(),
            );
        }
        if self.eligibility_identity != *plan.eligibility().eligibility_identity() {
            return Some(
                "boundary execution eligibility digest drifted from the route plan".to_string(),
            );
        }
        if self.route_or_handoff_identity != *plan.route_identity() {
            return Some("boundary execution route digest drifted from the route plan".to_string());
        }
        if self.retained_evidence_identity != *retained_evidence_identity {
            return Some(
                "boundary execution retained evidence drifted from the routed evidence".to_string(),
            );
        }
        None
    }

    pub fn drift_from_route_plan_with_retained_evidence_identity(
        &self,
        plan: &ForgeQueryLowerRuntimeRoutePlan,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Option<String> {
        self.drift_from_route_plan(plan, retained_evidence_identity)
    }

    pub fn drift_from_readmission_receipt(
        &self,
        receipt: &ForgeQueryLowerRuntimeReadmissionReceipt,
    ) -> Option<String> {
        if self.kind != ForgeQueryLowerRuntimeBoundaryExecutionKind::ReadmissionHandoff {
            return Some("boundary execution kind drifted from readmission-handoff".to_string());
        }
        if self.request_identity != *receipt.eligibility().request().request_identity() {
            return Some(
                "boundary execution request digest drifted from the readmission receipt"
                    .to_string(),
            );
        }
        if self.eligibility_identity != *receipt.eligibility().eligibility_identity() {
            return Some(
                "boundary execution eligibility digest drifted from the readmission receipt"
                    .to_string(),
            );
        }
        if self.route_or_handoff_identity != *receipt.handoff_identity() {
            return Some(
                "boundary execution handoff digest drifted from the readmission receipt"
                    .to_string(),
            );
        }
        if self.retained_evidence_identity != *receipt.retained_evidence_identity() {
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
        ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeRouteSubjectIdentity,
        ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSubjectIdentity,
    };

    #[test]
    fn route_plan_boundary_receipt_reuses_request_and_eligibility_digests() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "write-authority",
            ForgeQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_identity(ForgeQueryEvidenceTag::new("test_subject"), "subject-1")
                .seal(),
        );
        let detail_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_identity(ForgeQueryEvidenceTag::new("test_detail"), "detail-1")
        .seal();
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );
        let plan = ForgeQueryLowerRuntimeRoutePlan::new(
            eligibility,
            ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "test-route",
                &detail_identity,
            ),
        );
        let retained_evidence = forge_query_lower_runtime_retained_evidence_identity(
            "receipt-test",
            &ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_identity(ForgeQueryEvidenceTag::new("test_retained"), "receipt-1")
            .seal(),
        );
        let boundary = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &plan,
            &retained_evidence,
        );

        assert_eq!(
            boundary.request_identity(),
            plan.eligibility().request().request_identity()
        );
        assert_eq!(
            boundary.eligibility_identity(),
            plan.eligibility().eligibility_identity()
        );
        assert_eq!(
            boundary.kind(),
            ForgeQueryLowerRuntimeBoundaryExecutionKind::RoutePlan
        );
    }
}
