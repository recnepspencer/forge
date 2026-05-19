use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_crossing_inventory,
    inspect_lower_runtime_boundary, inspect_lower_runtime_closeout,
    summarize_lower_runtime_boundary, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeRoutePlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeRepresentativeSurface {
    requests: Vec<ForgeQueryLowerRuntimeCapabilityRequest>,
    eligibilities: Vec<ForgeQueryLowerRuntimeCapabilityEligibility>,
    route_plans: Vec<ForgeQueryLowerRuntimeRoutePlan>,
    boundary_receipts: Vec<ForgeQueryLowerRuntimeBoundaryExecutionReceipt>,
    envelopes: Vec<ForgeQueryLowerRuntimeBoundaryEnvelope>,
    dx_digest: String,
    golden_transcript_digest: String,
    query_digest: String,
    route_parity_digest: String,
}

impl ForgeQueryLowerRuntimeRepresentativeSurface {
    pub fn requests(&self) -> &[ForgeQueryLowerRuntimeCapabilityRequest] {
        &self.requests
    }

    pub fn eligibilities(&self) -> &[ForgeQueryLowerRuntimeCapabilityEligibility] {
        &self.eligibilities
    }

    pub fn route_plans(&self) -> &[ForgeQueryLowerRuntimeRoutePlan] {
        &self.route_plans
    }

    pub fn boundary_receipts(&self) -> &[ForgeQueryLowerRuntimeBoundaryExecutionReceipt] {
        &self.boundary_receipts
    }

    pub fn envelopes(&self) -> &[ForgeQueryLowerRuntimeBoundaryEnvelope] {
        &self.envelopes
    }

    pub fn dx_digest(&self) -> &str {
        &self.dx_digest
    }

    pub fn golden_transcript_digest(&self) -> &str {
        &self.golden_transcript_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn route_parity_digest(&self) -> &str {
        &self.route_parity_digest
    }
}

pub fn forge_query_lower_runtime_representative_surface(
) -> ForgeQueryLowerRuntimeRepresentativeSurface {
    let mut requests = Vec::new();
    let mut eligibilities = Vec::new();
    let mut route_plans = Vec::new();
    let mut boundary_receipts = Vec::new();
    let mut envelopes = Vec::new();

    for row in forge_query_lower_runtime_crossing_inventory().rows() {
        let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            row.seam_key(),
            row.route_kind(),
            row.lower_runtime_owner(),
            row.capability_label(),
            format!("{}-subject", row.seam_key().as_str()),
        );
        let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
            request.clone(),
            format!("{}-eligibility-detail", row.seam_key().as_str()),
        );
        match row.route_kind() {
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning => {
                let plan = ForgeQueryLowerRuntimeRoutePlan::new(
                    eligibility.clone(),
                    format!("{}-route", row.seam_key().as_str()),
                );
                let boundary = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
                    &plan,
                    format!("{}-evidence", row.seam_key().as_str()),
                );
                let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
                    row.seam_key(),
                    &plan,
                    &boundary,
                    &format!("{}-evidence", row.seam_key().as_str()),
                );
                route_plans.push(plan);
                boundary_receipts.push(boundary);
                envelopes.push(envelope);
            }
            ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff => {
                let handoff = ForgeQueryLowerRuntimeReadmissionReceipt::new(
                    eligibility.clone(),
                    format!("{}-evidence", row.seam_key().as_str()),
                );
                let boundary =
                    ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(
                        &handoff,
                    );
                let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
                    row.seam_key(),
                    &handoff,
                    &boundary,
                );
                boundary_receipts.push(boundary);
                envelopes.push(envelope);
            }
        }
        requests.push(request);
        eligibilities.push(eligibility);
    }

    let dx_digest = hash_parts(
        &envelopes
            .iter()
            .map(|envelope| {
                summarize_lower_runtime_boundary(envelope)
                    .summary_digest()
                    .to_string()
            })
            .collect::<Vec<_>>(),
    );
    let mut transcripts = envelopes
        .iter()
        .map(|envelope| {
            let inspection = inspect_lower_runtime_boundary(envelope);
            format!("{}|{}", inspection.headline(), inspection.detail())
        })
        .collect::<Vec<_>>();
    transcripts.extend(
        forge_query_lower_runtime_closeout_registry()
            .rows()
            .iter()
            .map(|row| {
                let inspection = inspect_lower_runtime_closeout(row);
                format!("{}|{}", inspection.headline(), inspection.detail())
            }),
    );
    let golden_transcript_digest = hash_parts(&transcripts);
    let query_digest = hash_parts(
        &requests
            .iter()
            .map(|request| request.subject_digest().to_string())
            .collect::<Vec<_>>(),
    );
    let route_parity_digest = hash_parts(&[
        normalized_parity_digest("compose-read", &envelopes),
        normalized_parity_digest("basis-readmission", &envelopes),
        hostile_parity_divergence_digest(&envelopes),
    ]);

    ForgeQueryLowerRuntimeRepresentativeSurface {
        requests,
        eligibilities,
        route_plans,
        boundary_receipts,
        envelopes,
        dx_digest,
        golden_transcript_digest,
        query_digest,
        route_parity_digest,
    }
}

fn normalized_parity_digest(
    label: &str,
    envelopes: &[ForgeQueryLowerRuntimeBoundaryEnvelope],
) -> String {
    let selected = match label {
        "compose-read" => envelopes
            .iter()
            .filter(|row| {
                matches!(
                    row.seam_key().as_str(),
                    "compose-read" | "execute-read-family"
                )
            })
            .collect::<Vec<_>>(),
        _ => envelopes
            .iter()
            .filter(|row| {
                matches!(
                    row.seam_key().as_str(),
                    "basis-readmission-from-truth-view-evidence"
                        | "basis-readmission-from-subscription-evidence"
                )
            })
            .collect::<Vec<_>>(),
    };
    hash_parts(
        &selected
            .iter()
            .map(|envelope| {
                format!(
                    "{}|{}|{}|{}|{}|{}|{}",
                    envelope.authority_owner().as_str(),
                    envelope.route_kind().as_str(),
                    envelope.support_posture().as_str(),
                    envelope.route_cost_posture().as_str(),
                    envelope.route_failure_topology().as_str(),
                    envelope.artifact_strength().as_str(),
                    envelope.crossing_classification().as_str()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn hostile_parity_divergence_digest(
    envelopes: &[ForgeQueryLowerRuntimeBoundaryEnvelope],
) -> String {
    let readmission = envelopes
        .iter()
        .find(|row| row.seam_key().as_str() == "live-view-schema-admission")
        .expect("live-view schema admission seam should be present");
    let routing = envelopes
        .iter()
        .find(|row| row.seam_key().as_str() == "signal-invalidation-routing")
        .expect("signal invalidation routing seam should be present");

    hash_parts(&[
        "hostile-route-divergence".to_string(),
        format!(
            "{}|{}|{}|{}",
            readmission.authority_owner().as_str(),
            readmission.route_kind().as_str(),
            readmission.artifact_strength().as_str(),
            readmission.route_failure_topology().as_str(),
        ),
        format!(
            "{}|{}|{}|{}",
            routing.authority_owner().as_str(),
            routing.route_kind().as_str(),
            routing.artifact_strength().as_str(),
            routing.route_failure_topology().as_str(),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representative_surface_covers_every_crossing_row_once() {
        let surface = forge_query_lower_runtime_representative_surface();
        let crossing_count = forge_query_lower_runtime_crossing_inventory().rows().len();

        assert_eq!(surface.requests().len(), crossing_count);
        assert_eq!(surface.eligibilities().len(), crossing_count);
        assert_eq!(surface.boundary_receipts().len(), crossing_count);
        assert_eq!(surface.envelopes().len(), crossing_count);
        assert!(!surface.route_parity_digest().is_empty());
    }
}
