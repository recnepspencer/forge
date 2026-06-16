use super::{ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeCloseoutRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundarySummary {
    headline: String,
    summary_digest: String,
}

impl ForgeQueryLowerRuntimeBoundarySummary {
    pub(crate) fn new(headline: impl Into<String>, summary_digest: impl Into<String>) -> Self {
        Self {
            headline: headline.into(),
            summary_digest: summary_digest.into(),
        }
    }

    pub fn headline(&self) -> &str {
        &self.headline
    }

    pub fn summary_digest(&self) -> &str {
        &self.summary_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeRoutingInspection {
    headline: String,
    detail: String,
    inspection_digest: String,
}

impl ForgeQueryLowerRuntimeRoutingInspection {
    pub(crate) fn new(
        headline: impl Into<String>,
        detail: impl Into<String>,
        inspection_digest: impl Into<String>,
    ) -> Self {
        Self {
            headline: headline.into(),
            detail: detail.into(),
            inspection_digest: inspection_digest.into(),
        }
    }

    pub fn headline(&self) -> &str {
        &self.headline
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

pub fn summarize_lower_runtime_boundary(
    envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
) -> ForgeQueryLowerRuntimeBoundarySummary {
    ForgeQueryLowerRuntimeBoundarySummary::new(
        format!(
            "{} [{} / {} / {}]",
            envelope.capability_label(),
            envelope.authority_owner().as_str(),
            envelope.route_kind().as_str(),
            envelope.support_posture().as_str(),
        ),
        envelope.envelope_identity().terminal_projection_for_reporting(),
    )
}

pub fn inspect_lower_runtime_boundary(
    envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
) -> ForgeQueryLowerRuntimeRoutingInspection {
    ForgeQueryLowerRuntimeRoutingInspection::new(
        summarize_lower_runtime_boundary(envelope)
            .headline()
            .to_owned(),
        format!(
            "cost={} failure={} retained={} route={} evidence={}",
            envelope.route_cost_posture().as_str(),
            envelope.route_failure_topology().as_str(),
            envelope.retained_evidence_identity().as_str(),
            envelope.route_authority_identity().terminal_projection_for_reporting(),
            envelope.route_evidence_identity().terminal_projection_for_reporting(),
        ),
        envelope.envelope_identity().terminal_projection_for_reporting(),
    )
}

pub fn inspect_lower_runtime_closeout(
    row: &ForgeQueryLowerRuntimeCloseoutRow,
) -> ForgeQueryLowerRuntimeRoutingInspection {
    ForgeQueryLowerRuntimeRoutingInspection::new(
        format!("{} [{}]", row.capability_label(), row.posture().as_str()),
        format!(
            "owner={} route={} target={} closeout={} certification={}",
            row.owner().as_str(),
            row.route_kind().as_str(),
            row.closeout_target(),
            row.required_closeout(),
            row.certification_row(),
        ),
        format!(
            "{}::{}::{}",
            row.seam_key().as_str(),
            row.posture().as_str(),
            row.certification_row(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lower_runtime_routing::{
        forge_query_lower_runtime_closeout_registry, ForgeQueryLowerRuntimeAuthorityOwner,
        ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
        ForgeQueryLowerRuntimeCapabilityEligibility, ForgeQueryLowerRuntimeCapabilityRequest,
        ForgeQueryLowerRuntimeReadmissionReceipt, ForgeQueryLowerRuntimeRouteKind,
        ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSubjectIdentity,
    };

    #[test]
    fn common_path_summary_reuses_envelope_digest() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "live-view-schema-admission",
            ForgeQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_value(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("test_subject"),
                    "subject-3",
                )
                .seal(),
        );
        let detail_identity = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_value(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("test_detail"),
            "detail-3",
        )
        .seal();
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );
        let retained_evidence =
            crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
                "dx-test",
                &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                    crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_value(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                    "evidence-3",
                )
                .seal(),
            );
        let readmission =
            ForgeQueryLowerRuntimeReadmissionReceipt::new(eligibility, &retained_evidence);
        let boundary =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&readmission);
        let envelope = crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
            ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
            &readmission,
            &boundary,
        );
        let summary = summarize_lower_runtime_boundary(&envelope);

        assert_eq!(
            summary.summary_digest(),
            envelope.envelope_identity().terminal_projection_for_reporting()
        );
        assert_eq!(
            summary.headline(),
            "Live view schema admission [query / readmission-handoff / admitted]"
        );
    }

    #[test]
    fn advanced_boundary_inspection_reuses_envelope_digest() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            "signal-invalidation-routing",
            ForgeQueryLowerRuntimeSubjectIdentity::compose("test-subject")
                .field_value(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("test_subject"),
                    "subject-4",
                )
                .seal(),
        );
        let detail_identity = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
            crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
        )
        .field_value(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("test_detail"),
            "detail-4",
        )
        .seal();
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                request,
                &detail_identity,
            );
        let route = crate::lower_runtime_routing::ForgeQueryLowerRuntimeRoutePlan::new(
            eligibility,
            crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "lower-runtime-dx-route",
                &detail_identity,
            ),
        );
        let retained_evidence =
            crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
                "dx-route-test",
                &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                    crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
                )
                .field_value(
                    crate::evidence_identity::ForgeQueryEvidenceTag::new("test_retained"),
                    "evidence-4",
                )
                .seal(),
            );
        let boundary = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &route,
            &retained_evidence,
        );
        let envelope =
            crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
                ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
                &route,
                &boundary,
                &retained_evidence,
            );
        let inspection = inspect_lower_runtime_boundary(&envelope);

        assert_eq!(
            inspection.inspection_digest(),
            envelope.envelope_identity().terminal_projection_for_reporting()
        );
        assert_eq!(
            inspection.headline(),
            "Signal invalidation routing [query / route-planning / admitted]"
        );
        assert_eq!(
            inspection.detail(),
            format!(
                "cost={} failure={} retained={} route={} evidence={}",
                envelope.route_cost_posture().as_str(),
                envelope.route_failure_topology().as_str(),
                envelope.retained_evidence_identity().as_str(),
                envelope.route_authority_identity().terminal_projection_for_reporting(),
                envelope.route_evidence_identity().terminal_projection_for_reporting(),
            )
        );
    }

    #[test]
    fn deferred_closeout_inspection_names_milestone_and_certification_row() {
        let row = forge_query_lower_runtime_closeout_registry()
            .rows()
            .iter()
            .find(|row| {
                row.seam_key() == ForgeQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor
            })
            .expect("store-backed deferred row should exist");
        let inspection = inspect_lower_runtime_closeout(row);

        assert_eq!(
            inspection.headline(),
            "Store-backed route parity [deferred-neighbor]"
        );
        assert_eq!(
            inspection.detail(),
            "owner=store route=route-planning target=later store-backed route parity milestone closeout=fail deferred until forge-store owns the route-parity contract and replay evidence certification=deferred-store-route-parity"
        );
    }

    #[test]
    fn seam_elimination_closeout_inspection_names_eliminated_posture() {
        let row = forge_query_lower_runtime_closeout_registry()
            .rows()
            .iter()
            .find(|row| row.seam_key() == ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule)
            .expect("runtime intent elimination row should exist");
        let inspection = inspect_lower_runtime_closeout(row);

        assert_eq!(
            inspection.headline(),
            "Runtime intent module seam [seam-eliminated]"
        );
        assert_eq!(
            inspection.detail(),
            "owner=query route=route-planning target=phase-2 backend intent authority boundary ownership closeout=runtime/intent/mod.rs must remain free of direct lower-runtime authority imports certification=phase-2-runtime-intent-elimination"
        );
    }
}
