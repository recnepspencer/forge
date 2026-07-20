use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::lower_runtime_routing::support::support_posture_for_classification;

use super::{
    worth_query_lower_runtime_crossing_inventory, WorthQueryLowerRuntimeArtifactStrength,
    WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeBoundaryExecutionReceipt,
    WorthQueryLowerRuntimeCrossingClassification, WorthQueryLowerRuntimeReadmissionReceipt,
    WorthQueryLowerRuntimeRetainedEvidenceIdentity, WorthQueryLowerRuntimeRouteKind,
    WorthQueryLowerRuntimeRoutePlan, WorthQueryLowerRuntimeSeamKey,
    WorthQueryLowerRuntimeSupportPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeCostPosture {
    AuthorityReuse,
    QueryBoundaryAdapter,
    CompatibilityDebt,
    DeferredNeighbor,
    ForbiddenDuplicate,
}

impl WorthQueryLowerRuntimeCostPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityReuse => "authority-reuse",
            Self::QueryBoundaryAdapter => "query-boundary-adapter",
            Self::CompatibilityDebt => "compatibility-debt",
            Self::DeferredNeighbor => "deferred-neighbor",
            Self::ForbiddenDuplicate => "forbidden-duplicate",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeFailureTopology {
    RoutePlanningBoundary,
    ReadmissionHandoffBoundary,
}

impl WorthQueryLowerRuntimeFailureTopology {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoutePlanningBoundary => "route-planning-boundary",
            Self::ReadmissionHandoffBoundary => "readmission-handoff-boundary",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeBoundaryEnvelope {
    seam_key: WorthQueryLowerRuntimeSeamKey,
    capability_label: &'static str,
    crossing_classification: WorthQueryLowerRuntimeCrossingClassification,
    authority_owner: WorthQueryLowerRuntimeAuthorityOwner,
    route_kind: WorthQueryLowerRuntimeRouteKind,
    support_posture: WorthQueryLowerRuntimeSupportPosture,
    artifact_strength: WorthQueryLowerRuntimeArtifactStrength,
    request_identity: WorthQueryEvidenceIdentity,
    eligibility_identity: WorthQueryEvidenceIdentity,
    route_or_handoff_identity: WorthQueryEvidenceIdentity,
    boundary_execution_identity: WorthQueryEvidenceIdentity,
    retained_evidence_identity: WorthQueryLowerRuntimeRetainedEvidenceIdentity,
    route_authority_identity: WorthQueryEvidenceIdentity,
    route_evidence_identity: WorthQueryEvidenceIdentity,
    route_cost_posture: WorthQueryLowerRuntimeCostPosture,
    route_failure_topology: WorthQueryLowerRuntimeFailureTopology,
    envelope_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryLowerRuntimeBoundaryEnvelope {
    pub(crate) fn from_route_plan(
        seam_key: WorthQueryLowerRuntimeSeamKey,
        route_plan: &WorthQueryLowerRuntimeRoutePlan,
        boundary_execution_receipt: &WorthQueryLowerRuntimeBoundaryExecutionReceipt,
        retained_evidence_identity: &WorthQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        let row = crossing_row(seam_key);
        Self::new(
            row,
            route_plan.eligibility().request().request_identity(),
            route_plan.eligibility().eligibility_identity(),
            route_plan.route_identity(),
            boundary_execution_receipt.boundary_execution_identity(),
            retained_evidence_identity,
        )
    }

    pub(crate) fn from_route_plan_with_retained_evidence_identity(
        seam_key: WorthQueryLowerRuntimeSeamKey,
        route_plan: &WorthQueryLowerRuntimeRoutePlan,
        boundary_execution_receipt: &WorthQueryLowerRuntimeBoundaryExecutionReceipt,
        retained_evidence_identity: &WorthQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        Self::from_route_plan(
            seam_key,
            route_plan,
            boundary_execution_receipt,
            retained_evidence_identity,
        )
    }

    pub(crate) fn from_readmission_receipt(
        seam_key: WorthQueryLowerRuntimeSeamKey,
        readmission_receipt: &WorthQueryLowerRuntimeReadmissionReceipt,
        boundary_execution_receipt: &WorthQueryLowerRuntimeBoundaryExecutionReceipt,
    ) -> Self {
        let row = crossing_row(seam_key);
        Self::new(
            row,
            readmission_receipt
                .eligibility()
                .request()
                .request_identity(),
            readmission_receipt.eligibility().eligibility_identity(),
            readmission_receipt.handoff_identity(),
            boundary_execution_receipt.boundary_execution_identity(),
            readmission_receipt.retained_evidence_identity(),
        )
    }

    fn new(
        row: crate::lower_runtime_routing::WorthQueryLowerRuntimeCrossingRow,
        request_identity: &WorthQueryEvidenceIdentity,
        eligibility_identity: &WorthQueryEvidenceIdentity,
        route_or_handoff_identity: &WorthQueryEvidenceIdentity,
        boundary_execution_identity: &WorthQueryEvidenceIdentity,
        retained_evidence_identity: &WorthQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        let support_posture = support_posture_for_classification(row.classification());
        let route_cost_posture = cost_posture_for_classification(row.classification());
        let route_failure_topology = failure_topology_for_route_kind(row.route_kind());
        let route_authority_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryAuthority)
                .field_shape(WorthQueryEvidenceTag::new("seam"), row.seam_key().as_str())
                .field_shape(
                    WorthQueryEvidenceTag::new("owner"),
                    row.lower_runtime_owner().as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("classification"),
                    row.classification().as_str(),
                )
                .seal();
        let route_evidence_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("boundary"),
                    boundary_execution_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("retained"),
                    retained_evidence_identity.evidence_identity(),
                )
                .seal();
        let envelope_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEnvelope)
                .field_shape(WorthQueryEvidenceTag::new("seam"), row.seam_key().as_str())
                .field_shape(
                    WorthQueryEvidenceTag::new("capability"),
                    row.capability_label(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("classification"),
                    row.classification().as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("owner"),
                    row.lower_runtime_owner().as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("route_kind"),
                    row.route_kind().as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("support"),
                    support_posture.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("artifact"),
                    row.current_artifact_strength().as_str(),
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("request"), request_identity)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("eligibility"),
                    eligibility_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("route_or_handoff"),
                    route_or_handoff_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("boundary"),
                    boundary_execution_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("retained"),
                    retained_evidence_identity.evidence_identity(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("authority"),
                    &route_authority_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("evidence"),
                    &route_evidence_identity,
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("cost"),
                    route_cost_posture.as_str(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("failure"),
                    route_failure_topology.as_str(),
                )
                .seal();
        Self {
            seam_key: row.seam_key(),
            capability_label: row.capability_label(),
            crossing_classification: row.classification(),
            authority_owner: row.lower_runtime_owner(),
            route_kind: row.route_kind(),
            support_posture,
            artifact_strength: row.current_artifact_strength(),
            request_identity: request_identity.clone(),
            eligibility_identity: eligibility_identity.clone(),
            route_or_handoff_identity: route_or_handoff_identity.clone(),
            boundary_execution_identity: boundary_execution_identity.clone(),
            retained_evidence_identity: retained_evidence_identity.clone(),
            route_authority_identity,
            route_evidence_identity,
            route_cost_posture,
            route_failure_topology,
            envelope_identity,
        }
    }

    pub fn seam_key(&self) -> WorthQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn capability_label(&self) -> &'static str {
        self.capability_label
    }

    pub fn crossing_classification(&self) -> WorthQueryLowerRuntimeCrossingClassification {
        self.crossing_classification
    }

    pub fn authority_owner(&self) -> WorthQueryLowerRuntimeAuthorityOwner {
        self.authority_owner
    }

    pub fn route_kind(&self) -> WorthQueryLowerRuntimeRouteKind {
        self.route_kind
    }

    pub fn support_posture(&self) -> WorthQueryLowerRuntimeSupportPosture {
        self.support_posture
    }

    pub fn artifact_strength(&self) -> WorthQueryLowerRuntimeArtifactStrength {
        self.artifact_strength
    }

    pub fn request_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn eligibility_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.eligibility_identity
    }

    pub fn route_or_handoff_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.route_or_handoff_identity
    }

    pub fn boundary_execution_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.boundary_execution_identity
    }

    pub fn retained_evidence_identity(&self) -> &WorthQueryLowerRuntimeRetainedEvidenceIdentity {
        &self.retained_evidence_identity
    }

    pub fn route_authority_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.route_authority_identity
    }

    pub fn route_evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.route_evidence_identity
    }

    pub fn route_cost_posture(&self) -> WorthQueryLowerRuntimeCostPosture {
        self.route_cost_posture
    }

    pub fn route_failure_topology(&self) -> WorthQueryLowerRuntimeFailureTopology {
        self.route_failure_topology
    }

    pub fn envelope_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.envelope_identity
    }
}

fn crossing_row(
    seam_key: WorthQueryLowerRuntimeSeamKey,
) -> crate::lower_runtime_routing::WorthQueryLowerRuntimeCrossingRow {
    *worth_query_lower_runtime_crossing_inventory()
        .rows()
        .iter()
        .find(|row| row.seam_key() == seam_key)
        .expect("boundary envelope seam must exist in the crossing inventory")
}

fn cost_posture_for_classification(
    classification: WorthQueryLowerRuntimeCrossingClassification,
) -> WorthQueryLowerRuntimeCostPosture {
    match classification {
        WorthQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse => {
            WorthQueryLowerRuntimeCostPosture::AuthorityReuse
        }
        WorthQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter => {
            WorthQueryLowerRuntimeCostPosture::QueryBoundaryAdapter
        }
        WorthQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane => {
            WorthQueryLowerRuntimeCostPosture::CompatibilityDebt
        }
        WorthQueryLowerRuntimeCrossingClassification::DeferredNeighbor => {
            WorthQueryLowerRuntimeCostPosture::DeferredNeighbor
        }
        WorthQueryLowerRuntimeCrossingClassification::ForbiddenDuplicate => {
            WorthQueryLowerRuntimeCostPosture::ForbiddenDuplicate
        }
    }
}

fn failure_topology_for_route_kind(
    route_kind: WorthQueryLowerRuntimeRouteKind,
) -> WorthQueryLowerRuntimeFailureTopology {
    match route_kind {
        WorthQueryLowerRuntimeRouteKind::RoutePlanning => {
            WorthQueryLowerRuntimeFailureTopology::RoutePlanningBoundary
        }
        WorthQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
            WorthQueryLowerRuntimeFailureTopology::ReadmissionHandoffBoundary
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_identity::WorthQueryEvidenceIdentity;
    use crate::lower_runtime_routing::{
        worth_query_lower_runtime_support_matrix, WorthQueryLowerRuntimeCapabilityEligibility,
        WorthQueryLowerRuntimeCapabilityRequest, WorthQueryLowerRuntimeRoutePlan,
        WorthQueryLowerRuntimeRouteSubjectIdentity, WorthQueryLowerRuntimeSubjectIdentity,
    };

    #[test]
    fn route_plan_envelope_preserves_inventory_authority_and_support() {
        let request = WorthQueryLowerRuntimeCapabilityRequest::new(
            WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
            WorthQueryLowerRuntimeRouteKind::RoutePlanning,
            WorthQueryLowerRuntimeAuthorityOwner::Signal,
            "frontier-evidence-intake",
            WorthQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_value(WorthQueryEvidenceTag::new("test_subject"), "subject-1")
                .seal(),
        );
        let detail_identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_value(WorthQueryEvidenceTag::new("test_detail"), "detail-1")
        .seal();
        let eligibility =
            WorthQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );
        let plan = WorthQueryLowerRuntimeRoutePlan::new(
            eligibility,
            WorthQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "test-route",
                &detail_identity,
            ),
        );
        let retained_evidence =
            crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
                "envelope-route-test",
                &crate::evidence_identity::WorthQueryEvidenceIdentity::compose(
                    crate::evidence_identity::WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_value(
                    crate::evidence_identity::WorthQueryEvidenceTag::new("test_retained"),
                    "evidence-1",
                )
                .seal(),
            );
        let boundary = WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &plan,
            &retained_evidence,
        );
        let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
            &plan,
            &boundary,
            &retained_evidence,
        );
        let support_matrix = worth_query_lower_runtime_support_matrix();
        let support = support_matrix
            .support_for(WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake)
            .expect("frontier support row must exist");

        assert_eq!(
            envelope.authority_owner(),
            WorthQueryLowerRuntimeAuthorityOwner::Signal
        );
        assert_eq!(envelope.support_posture(), support.posture());
        assert_eq!(
            envelope.route_cost_posture(),
            WorthQueryLowerRuntimeCostPosture::QueryBoundaryAdapter
        );
    }

    #[test]
    fn readmission_envelope_keeps_handoff_failure_topology_distinct() {
        let request = WorthQueryLowerRuntimeCapabilityRequest::new(
            WorthQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            WorthQueryLowerRuntimeRouteKind::ReadmissionHandoff,
            WorthQueryLowerRuntimeAuthorityOwner::Query,
            "live-view-schema-admission",
            WorthQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_value(WorthQueryEvidenceTag::new("test_subject"), "subject-2")
                .seal(),
        );
        let detail_identity = WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_value(WorthQueryEvidenceTag::new("test_detail"), "detail-2")
        .seal();
        let eligibility =
            WorthQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );
        let retained_evidence =
            crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
                "envelope-test",
                &crate::evidence_identity::WorthQueryEvidenceIdentity::compose(
                    crate::evidence_identity::WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_value(
                    crate::evidence_identity::WorthQueryEvidenceTag::new("test_retained"),
                    "evidence-2",
                )
                .seal(),
            );
        let readmission =
            WorthQueryLowerRuntimeReadmissionReceipt::new(eligibility, &retained_evidence);
        let boundary =
            WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&readmission);
        let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
            WorthQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            &readmission,
            &boundary,
        );

        assert_eq!(
            envelope.route_failure_topology(),
            WorthQueryLowerRuntimeFailureTopology::ReadmissionHandoffBoundary
        );
        assert_eq!(
            envelope.route_kind(),
            WorthQueryLowerRuntimeRouteKind::ReadmissionHandoff
        );
    }
}
