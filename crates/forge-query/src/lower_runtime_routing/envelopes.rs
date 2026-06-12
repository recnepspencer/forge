use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::lower_runtime_routing::support::support_posture_for_classification;

use super::{
    forge_query_lower_runtime_crossing_inventory, ForgeQueryLowerRuntimeArtifactStrength,
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    ForgeQueryLowerRuntimeCrossingClassification, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRetainedEvidenceIdentity, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
    ForgeQueryLowerRuntimeSupportPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeCostPosture {
    AuthorityReuse,
    QueryBoundaryAdapter,
    CompatibilityDebt,
    DeferredNeighbor,
    ForbiddenDuplicate,
}

impl ForgeQueryLowerRuntimeCostPosture {
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
pub enum ForgeQueryLowerRuntimeFailureTopology {
    RoutePlanningBoundary,
    ReadmissionHandoffBoundary,
}

impl ForgeQueryLowerRuntimeFailureTopology {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoutePlanningBoundary => "route-planning-boundary",
            Self::ReadmissionHandoffBoundary => "readmission-handoff-boundary",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundaryEnvelope {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &'static str,
    crossing_classification: ForgeQueryLowerRuntimeCrossingClassification,
    authority_owner: ForgeQueryLowerRuntimeAuthorityOwner,
    route_kind: ForgeQueryLowerRuntimeRouteKind,
    support_posture: ForgeQueryLowerRuntimeSupportPosture,
    artifact_strength: ForgeQueryLowerRuntimeArtifactStrength,
    request_identity: ForgeQueryEvidenceIdentity,
    eligibility_identity: ForgeQueryEvidenceIdentity,
    route_or_handoff_identity: ForgeQueryEvidenceIdentity,
    boundary_execution_identity: ForgeQueryEvidenceIdentity,
    retained_evidence_identity: ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    route_authority_identity: ForgeQueryEvidenceIdentity,
    route_evidence_identity: ForgeQueryEvidenceIdentity,
    route_cost_posture: ForgeQueryLowerRuntimeCostPosture,
    route_failure_topology: ForgeQueryLowerRuntimeFailureTopology,
    envelope_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeBoundaryEnvelope {
    pub(crate) fn from_route_plan(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        route_plan: &ForgeQueryLowerRuntimeRoutePlan,
        boundary_execution_receipt: &ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
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
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        route_plan: &ForgeQueryLowerRuntimeRoutePlan,
        boundary_execution_receipt: &ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        Self::from_route_plan(
            seam_key,
            route_plan,
            boundary_execution_receipt,
            retained_evidence_identity,
        )
    }

    pub(crate) fn from_readmission_receipt(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        readmission_receipt: &ForgeQueryLowerRuntimeReadmissionReceipt,
        boundary_execution_receipt: &ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
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
        row: crate::lower_runtime_routing::ForgeQueryLowerRuntimeCrossingRow,
        request_identity: &ForgeQueryEvidenceIdentity,
        eligibility_identity: &ForgeQueryEvidenceIdentity,
        route_or_handoff_identity: &ForgeQueryEvidenceIdentity,
        boundary_execution_identity: &ForgeQueryEvidenceIdentity,
        retained_evidence_identity: &ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ) -> Self {
        let support_posture = support_posture_for_classification(row.classification());
        let route_cost_posture = cost_posture_for_classification(row.classification());
        let route_failure_topology = failure_topology_for_route_kind(row.route_kind());
        let route_authority_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryAuthority)
                .field_shape(ForgeQueryEvidenceTag::new("seam"), row.seam_key().as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("owner"),
                    row.lower_runtime_owner().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("classification"),
                    row.classification().as_str(),
                )
                .seal();
        let route_evidence_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("boundary"),
                    boundary_execution_identity,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("retained"),
                    retained_evidence_identity.evidence_identity(),
                )
                .seal();
        let envelope_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEnvelope)
                .field_shape(ForgeQueryEvidenceTag::new("seam"), row.seam_key().as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("capability"),
                    row.capability_label(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("classification"),
                    row.classification().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("owner"),
                    row.lower_runtime_owner().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("route_kind"),
                    row.route_kind().as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("support"),
                    support_posture.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("artifact"),
                    row.current_artifact_strength().as_str(),
                )
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
                    ForgeQueryEvidenceTag::new("boundary"),
                    boundary_execution_identity,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("retained"),
                    retained_evidence_identity.evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("authority"),
                    &route_authority_identity,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("evidence"),
                    &route_evidence_identity,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("cost"),
                    route_cost_posture.as_str(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("failure"),
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

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn capability_label(&self) -> &'static str {
        self.capability_label
    }

    pub fn crossing_classification(&self) -> ForgeQueryLowerRuntimeCrossingClassification {
        self.crossing_classification
    }

    pub fn authority_owner(&self) -> ForgeQueryLowerRuntimeAuthorityOwner {
        self.authority_owner
    }

    pub fn route_kind(&self) -> ForgeQueryLowerRuntimeRouteKind {
        self.route_kind
    }

    pub fn support_posture(&self) -> ForgeQueryLowerRuntimeSupportPosture {
        self.support_posture
    }

    pub fn artifact_strength(&self) -> ForgeQueryLowerRuntimeArtifactStrength {
        self.artifact_strength
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

    pub fn boundary_execution_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.boundary_execution_identity
    }

    pub fn retained_evidence_identity(&self) -> &ForgeQueryLowerRuntimeRetainedEvidenceIdentity {
        &self.retained_evidence_identity
    }

    pub fn route_authority_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.route_authority_identity
    }

    pub fn route_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.route_evidence_identity
    }

    pub fn route_cost_posture(&self) -> ForgeQueryLowerRuntimeCostPosture {
        self.route_cost_posture
    }

    pub fn route_failure_topology(&self) -> ForgeQueryLowerRuntimeFailureTopology {
        self.route_failure_topology
    }

    pub fn envelope_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.envelope_identity
    }
}

fn crossing_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
) -> crate::lower_runtime_routing::ForgeQueryLowerRuntimeCrossingRow {
    *forge_query_lower_runtime_crossing_inventory()
        .rows()
        .iter()
        .find(|row| row.seam_key() == seam_key)
        .expect("boundary envelope seam must exist in the crossing inventory")
}

fn cost_posture_for_classification(
    classification: ForgeQueryLowerRuntimeCrossingClassification,
) -> ForgeQueryLowerRuntimeCostPosture {
    match classification {
        ForgeQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse => {
            ForgeQueryLowerRuntimeCostPosture::AuthorityReuse
        }
        ForgeQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter => {
            ForgeQueryLowerRuntimeCostPosture::QueryBoundaryAdapter
        }
        ForgeQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane => {
            ForgeQueryLowerRuntimeCostPosture::CompatibilityDebt
        }
        ForgeQueryLowerRuntimeCrossingClassification::DeferredNeighbor => {
            ForgeQueryLowerRuntimeCostPosture::DeferredNeighbor
        }
        ForgeQueryLowerRuntimeCrossingClassification::ForbiddenDuplicate => {
            ForgeQueryLowerRuntimeCostPosture::ForbiddenDuplicate
        }
    }
}

fn failure_topology_for_route_kind(
    route_kind: ForgeQueryLowerRuntimeRouteKind,
) -> ForgeQueryLowerRuntimeFailureTopology {
    match route_kind {
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning => {
            ForgeQueryLowerRuntimeFailureTopology::RoutePlanningBoundary
        }
        ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
            ForgeQueryLowerRuntimeFailureTopology::ReadmissionHandoffBoundary
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_identity::ForgeQueryEvidenceIdentity;
    use crate::lower_runtime_routing::{
        forge_query_lower_runtime_support_matrix, ForgeQueryLowerRuntimeCapabilityEligibility,
        ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRoutePlan,
        ForgeQueryLowerRuntimeRouteSubjectIdentity, ForgeQueryLowerRuntimeSubjectIdentity,
    };

    #[test]
    fn route_plan_envelope_preserves_inventory_authority_and_support() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Signal,
            "frontier-evidence-intake",
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
        let retained_evidence =
            crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
                "envelope-route-test",
                &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                    crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_identity(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                    "evidence-1",
                )
                .seal(),
            );
        let boundary = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &plan,
            &retained_evidence,
        );
        let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
            &plan,
            &boundary,
            &retained_evidence,
        );
        let support_matrix = forge_query_lower_runtime_support_matrix();
        let support = support_matrix
            .support_for(ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake)
            .expect("frontier support row must exist");

        assert_eq!(
            envelope.authority_owner(),
            ForgeQueryLowerRuntimeAuthorityOwner::Signal
        );
        assert_eq!(envelope.support_posture(), support.posture());
        assert_eq!(
            envelope.route_cost_posture(),
            ForgeQueryLowerRuntimeCostPosture::QueryBoundaryAdapter
        );
    }

    #[test]
    fn readmission_envelope_keeps_handoff_failure_topology_distinct() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "live-view-schema-admission",
            ForgeQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_identity(ForgeQueryEvidenceTag::new("test_subject"), "subject-2")
                .seal(),
        );
        let detail_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_identity(ForgeQueryEvidenceTag::new("test_detail"), "detail-2")
        .seal();
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );
        let retained_evidence =
            crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
                "envelope-test",
                &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                    crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_identity(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                    "evidence-2",
                )
                .seal(),
            );
        let readmission =
            ForgeQueryLowerRuntimeReadmissionReceipt::new(eligibility, &retained_evidence);
        let boundary =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&readmission);
        let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
            ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            &readmission,
            &boundary,
        );

        assert_eq!(
            envelope.route_failure_topology(),
            ForgeQueryLowerRuntimeFailureTopology::ReadmissionHandoffBoundary
        );
        assert_eq!(
            envelope.route_kind(),
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff
        );
    }
}
