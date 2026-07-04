use crate::facade::replay_undo_semantic_graph::{
    current_boolean_event_ledger_spatial_boundary, current_projection_receipt_spatial_boundary,
    CurrentReplayUndoSpatialBoundary,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyDeclaration,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::admission::EvidenceLookupRouteAdmissionError;

const LEFT_ROUTE_FAMILY_IDENTITY: &str = "spatial-touch.boolean.event-ledger-evidence.v1";
const RIGHT_ROUTE_FAMILY_IDENTITY: &str =
    "spatial-touch.boolean.projection-consumption-evidence.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupRouteLoweringEvidence {
    raw_row_revisit_count: usize,
    right_receipt_revisit_count: usize,
    caller_owned_revisit_count: usize,
}

impl EvidenceLookupRouteLoweringEvidence {
    fn from_consumed_boundaries(
        left_boundary: &CurrentReplayUndoSpatialBoundary,
        right_boundary: &CurrentReplayUndoSpatialBoundary,
    ) -> Self {
        let left_counters = left_boundary.workload_handoff().counters();
        let right_counters = right_boundary.workload_handoff().counters();

        Self {
            raw_row_revisit_count: left_counters.raw_row_scan_count()
                + right_counters.raw_row_scan_count(),
            right_receipt_revisit_count: left_counters.broad_receipt_scan_count()
                + right_counters.broad_receipt_scan_count(),
            caller_owned_revisit_count: left_counters.caller_owned_scan_count()
                + right_counters.caller_owned_scan_count(),
        }
    }

    pub(crate) const fn raw_row_revisit_count(&self) -> usize {
        self.raw_row_revisit_count
    }

    pub(crate) const fn right_receipt_revisit_count(&self) -> usize {
        self.right_receipt_revisit_count
    }

    pub(crate) const fn caller_owned_revisit_count(&self) -> usize {
        self.caller_owned_revisit_count
    }
}

pub(crate) struct CurrentEvidenceLookupRouteSource {
    left_family: EvidenceLookupFamilyDeclaration,
    right_family: EvidenceLookupFamilyDeclaration,
    left_boundary: CurrentReplayUndoSpatialBoundary,
    right_boundary: CurrentReplayUndoSpatialBoundary,
    route_authority_digest: String,
    lowering_evidence: EvidenceLookupRouteLoweringEvidence,
}

impl CurrentEvidenceLookupRouteSource {
    pub(crate) fn left_family(&self) -> &EvidenceLookupFamilyDeclaration {
        &self.left_family
    }

    pub(crate) fn left_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.left_boundary
    }

    pub(crate) fn right_family(&self) -> &EvidenceLookupFamilyDeclaration {
        &self.right_family
    }

    pub(crate) fn right_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.right_boundary
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub(crate) fn lowering_evidence(&self) -> &EvidenceLookupRouteLoweringEvidence {
        &self.lowering_evidence
    }
}

pub(crate) fn current_evidence_lookup_route_source(
) -> Result<CurrentEvidenceLookupRouteSource, EvidenceLookupRouteAdmissionError> {
    let left_boundary = current_boolean_event_ledger_spatial_boundary().map_err(|error| {
        EvidenceLookupRouteAdmissionError::current_route_unavailable(format!(
            "current evidence lookup route requires the boolean-event-ledger boundary: {}",
            error.detail()
        ))
    })?;
    let right_boundary = current_projection_receipt_spatial_boundary().map_err(|error| {
        EvidenceLookupRouteAdmissionError::current_route_unavailable(format!(
            "current evidence lookup route requires the projection-receipt boundary: {}",
            error.detail()
        ))
    })?;
    let catalog = current_evidence_lookup_family_catalog().map_err(|error| {
        EvidenceLookupRouteAdmissionError::current_route_unavailable(format!(
            "current evidence lookup route requires family catalog: {:?}",
            error.kind()
        ))
    })?;
    let left_family = catalog
        .family_by_identity(LEFT_ROUTE_FAMILY_IDENTITY)
        .cloned()
        .ok_or_else(|| {
            EvidenceLookupRouteAdmissionError::current_route_unavailable(format!(
                "current evidence lookup route is missing left route family `{LEFT_ROUTE_FAMILY_IDENTITY}`"
            ))
        })?;
    let right_family = catalog
        .family_by_identity(RIGHT_ROUTE_FAMILY_IDENTITY)
        .cloned()
        .ok_or_else(|| {
            EvidenceLookupRouteAdmissionError::current_route_unavailable(format!(
                "current evidence lookup route is missing right route family `{RIGHT_ROUTE_FAMILY_IDENTITY}`"
            ))
        })?;

    let route_authority_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-route-authority:v1".to_string(),
            format!("left-family:{}", left_family.identity().as_str()),
            format!(
                "left-stage:{}",
                left_boundary.workload_handoff().stage_receipt_identity()
            ),
            format!(
                "left-lookup:{}",
                left_boundary
                    .workload_handoff()
                    .lookup_execution_receipt_digest()
            ),
            format!(
                "left-authority:{}",
                left_boundary.authority().stage_index_identity()
            ),
            format!("right-family:{}", right_family.identity().as_str()),
            format!(
                "right-stage:{}",
                right_boundary.workload_handoff().stage_receipt_identity()
            ),
            format!(
                "right-lookup:{}",
                right_boundary
                    .workload_handoff()
                    .lookup_execution_receipt_digest()
            ),
            format!(
                "right-authority:{}",
                right_boundary.authority().stage_index_identity()
            ),
        ],
    );
    let lowering_evidence = EvidenceLookupRouteLoweringEvidence::from_consumed_boundaries(
        &left_boundary,
        &right_boundary,
    );

    Ok(CurrentEvidenceLookupRouteSource {
        left_family,
        right_family,
        left_boundary,
        right_boundary,
        route_authority_digest,
        lowering_evidence,
    })
}
