use crate::identity::hash_parts;
use crate::lower_runtime_routing::support::support_posture_for_classification;

use super::{
    forge_query_lower_runtime_crossing_inventory, ForgeQueryLowerRuntimeArtifactStrength,
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    ForgeQueryLowerRuntimeCrossingClassification, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeRoutePlan,
    ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSupportPosture,
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
    request_digest: String,
    eligibility_digest: String,
    route_or_handoff_digest: String,
    boundary_execution_digest: String,
    retained_evidence_digest: String,
    route_authority_digest: String,
    route_evidence_digest: String,
    route_cost_posture: ForgeQueryLowerRuntimeCostPosture,
    route_failure_topology: ForgeQueryLowerRuntimeFailureTopology,
    envelope_digest: String,
}

impl ForgeQueryLowerRuntimeBoundaryEnvelope {
    pub(crate) fn from_route_plan(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        route_plan: &ForgeQueryLowerRuntimeRoutePlan,
        boundary_execution_receipt: &ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
        retained_evidence_digest: &str,
    ) -> Self {
        let row = crossing_row(seam_key);
        Self::new(
            row,
            route_plan.eligibility().request().request_digest(),
            route_plan.eligibility().eligibility_digest(),
            route_plan.route_digest(),
            boundary_execution_receipt.boundary_execution_digest(),
            retained_evidence_digest,
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
            readmission_receipt.eligibility().request().request_digest(),
            readmission_receipt.eligibility().eligibility_digest(),
            readmission_receipt.handoff_digest(),
            boundary_execution_receipt.boundary_execution_digest(),
            readmission_receipt.retained_evidence_digest(),
        )
    }

    fn new(
        row: crate::lower_runtime_routing::ForgeQueryLowerRuntimeCrossingRow,
        request_digest: &str,
        eligibility_digest: &str,
        route_or_handoff_digest: &str,
        boundary_execution_digest: &str,
        retained_evidence_digest: &str,
    ) -> Self {
        let support_posture = support_posture_for_classification(row.classification());
        let route_cost_posture = cost_posture_for_classification(row.classification());
        let route_failure_topology = failure_topology_for_route_kind(row.route_kind());
        let route_authority_digest = hash_parts(&[
            "lower_runtime_boundary_authority_v1".to_string(),
            format!("seam:{}", row.seam_key().as_str()),
            format!("owner:{}", row.lower_runtime_owner().as_str()),
            format!("classification:{}", row.classification().as_str()),
        ]);
        let route_evidence_digest = hash_parts(&[
            "lower_runtime_boundary_evidence_v1".to_string(),
            format!("boundary:{boundary_execution_digest}"),
            format!("retained:{retained_evidence_digest}"),
        ]);
        let envelope_digest = hash_parts(&[
            "lower_runtime_boundary_envelope_v1".to_string(),
            format!("seam:{}", row.seam_key().as_str()),
            format!("capability:{}", row.capability_label()),
            format!("classification:{}", row.classification().as_str()),
            format!("owner:{}", row.lower_runtime_owner().as_str()),
            format!("route_kind:{}", row.route_kind().as_str()),
            format!("support:{}", support_posture.as_str()),
            format!("artifact:{}", row.current_artifact_strength().as_str()),
            format!("request:{request_digest}"),
            format!("eligibility:{eligibility_digest}"),
            format!("route_or_handoff:{route_or_handoff_digest}"),
            format!("boundary:{boundary_execution_digest}"),
            format!("retained:{retained_evidence_digest}"),
            format!("authority:{route_authority_digest}"),
            format!("evidence:{route_evidence_digest}"),
            format!("cost:{}", route_cost_posture.as_str()),
            format!("failure:{}", route_failure_topology.as_str()),
        ]);
        Self {
            seam_key: row.seam_key(),
            capability_label: row.capability_label(),
            crossing_classification: row.classification(),
            authority_owner: row.lower_runtime_owner(),
            route_kind: row.route_kind(),
            support_posture,
            artifact_strength: row.current_artifact_strength(),
            request_digest: request_digest.to_string(),
            eligibility_digest: eligibility_digest.to_string(),
            route_or_handoff_digest: route_or_handoff_digest.to_string(),
            boundary_execution_digest: boundary_execution_digest.to_string(),
            retained_evidence_digest: retained_evidence_digest.to_string(),
            route_authority_digest,
            route_evidence_digest,
            route_cost_posture,
            route_failure_topology,
            envelope_digest,
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

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn eligibility_digest(&self) -> &str {
        &self.eligibility_digest
    }

    pub fn route_or_handoff_digest(&self) -> &str {
        &self.route_or_handoff_digest
    }

    pub fn boundary_execution_digest(&self) -> &str {
        &self.boundary_execution_digest
    }

    pub fn retained_evidence_digest(&self) -> &str {
        &self.retained_evidence_digest
    }

    pub fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub fn route_evidence_digest(&self) -> &str {
        &self.route_evidence_digest
    }

    pub fn route_cost_posture(&self) -> ForgeQueryLowerRuntimeCostPosture {
        self.route_cost_posture
    }

    pub fn route_failure_topology(&self) -> ForgeQueryLowerRuntimeFailureTopology {
        self.route_failure_topology
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
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
    use crate::lower_runtime_routing::{
        forge_query_lower_runtime_support_matrix, ForgeQueryLowerRuntimeCapabilityEligibility,
        ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRoutePlan,
    };

    #[test]
    fn route_plan_envelope_preserves_inventory_authority_and_support() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Signal,
            "frontier-evidence-intake",
            "subject-1",
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request, "detail-1");
        let plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "signal-frontier");
        let boundary =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(&plan, "evidence-1");
        let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
            &plan,
            &boundary,
            "evidence-1",
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
            "subject-2",
        );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request, "detail-2");
        let readmission = ForgeQueryLowerRuntimeReadmissionReceipt::new(eligibility, "evidence-2");
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
