use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::evidence_lookup_route::EvidenceLookupRoutePacket;

pub(crate) struct SpatialRouteProjectionMarkers {
    evidence_lookup_public_closeout_digest: String,
    evidence_lookup_family_coverage_digest: String,
    evidence_lookup_query_surface_matrix_digest: String,
    evidence_lookup_query_consumer_kit_digest: String,
    evidence_lookup_query_boundary_support_digest: String,
}

impl SpatialRouteProjectionMarkers {
    pub(crate) fn from_route_packet(route_packet: &EvidenceLookupRoutePacket) -> Self {
        Self {
            evidence_lookup_public_closeout_digest: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-kernel:evidence-lookup-route-public-closeout-marker:v1".to_string(),
                    format!("route-packet:{}", route_packet.route_packet_digest()),
                    format!("route-authority:{}", route_packet.route_authority_digest()),
                    format!(
                        "selected-plan:{}",
                        route_packet.selected_lookup_plan_digest()
                    ),
                    format!(
                        "right-route-family:{}",
                        route_packet.right_route_family_identity()
                    ),
                    format!(
                        "compiled-product:{}",
                        route_packet.compiled_product_identity_digest()
                    ),
                    format!(
                        "equivalence-policy:{}",
                        route_packet.equivalence_policy_identity_digest()
                    ),
                    format!(
                        "selected-family:{}",
                        route_packet.selected_equivalence_family_identity()
                    ),
                    format!(
                        "selected-reuse-basis:{}",
                        route_packet.selected_reuse_basis_identity_digest()
                    ),
                ],
            ),
            evidence_lookup_family_coverage_digest: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-kernel:evidence-lookup-route-family-coverage-marker:v1".to_string(),
                    format!("route-family:{}", route_packet.route_family_identity()),
                    format!(
                        "right-route-family:{}",
                        route_packet.right_route_family_identity()
                    ),
                    format!(
                        "stage-receipt-family:{}",
                        route_packet.stage_receipt_family_identity()
                    ),
                    format!("stage:{}", route_packet.stage().human_name()),
                    format!(
                        "right-stage-receipt:{}",
                        route_packet.right_stage_receipt_identity()
                    ),
                    format!("spatial-touch:{}", route_packet.spatial_touch_digest()),
                    format!("stage-receipt:{}", route_packet.stage_receipt_digest()),
                ],
            ),
            evidence_lookup_query_surface_matrix_digest: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-kernel:evidence-lookup-route-query-surface-matrix-marker:v1".to_string(),
                    format!("route-packet:{}", route_packet.route_packet_digest()),
                    format!(
                        "lookup-output:{}",
                        route_packet.lookup_product_output_digest()
                    ),
                    format!(
                        "execution-receipt:{}",
                        route_packet.lookup_execution_receipt_digest()
                    ),
                    format!(
                        "right-execution-receipt:{}",
                        route_packet.right_lookup_execution_receipt_digest()
                    ),
                    format!("query-support:{}", route_packet.query_support_digest()),
                    format!(
                        "topology-support:{}",
                        route_packet.topology_support_digest()
                    ),
                    format!(
                        "right-authority-stage-index:{}",
                        route_packet.right_authority_stage_index_identity()
                    ),
                ],
            ),
            evidence_lookup_query_consumer_kit_digest: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-kernel:evidence-lookup-route-query-consumer-kit-marker:v1".to_string(),
                    format!(
                        "compiled-product:{}",
                        route_packet.compiled_product_identity_digest()
                    ),
                    format!(
                        "equivalence-policy:{}",
                        route_packet.equivalence_policy_identity_digest()
                    ),
                    format!(
                        "selected-equivalence-basis:{}",
                        route_packet.selected_equivalence_basis_identity_digest()
                    ),
                    format!(
                        "selected-compatibility-basis:{}",
                        route_packet.selected_compatibility_basis_identity_digest()
                    ),
                    format!(
                        "selected-reuse-basis:{}",
                        route_packet.selected_reuse_basis_identity_digest()
                    ),
                    format!(
                        "lowering-raw-row-revisit:{}",
                        route_packet.lowering_raw_row_revisit_count()
                    ),
                    format!(
                        "lowering-right-receipt-revisit:{}",
                        route_packet.lowering_right_receipt_revisit_count()
                    ),
                    format!(
                        "lowering-caller-owned-revisit:{}",
                        route_packet.lowering_caller_owned_revisit_count()
                    ),
                ],
            ),
            evidence_lookup_query_boundary_support_digest: route_packet
                .query_support_digest()
                .to_string(),
        }
    }

    pub(crate) fn evidence_lookup_public_closeout_digest(&self) -> &str {
        &self.evidence_lookup_public_closeout_digest
    }

    pub(crate) fn evidence_lookup_family_coverage_digest(&self) -> &str {
        &self.evidence_lookup_family_coverage_digest
    }

    pub(crate) fn evidence_lookup_query_surface_matrix_digest(&self) -> &str {
        &self.evidence_lookup_query_surface_matrix_digest
    }

    pub(crate) fn evidence_lookup_query_consumer_kit_digest(&self) -> &str {
        &self.evidence_lookup_query_consumer_kit_digest
    }

    pub(crate) fn evidence_lookup_query_boundary_support_digest(&self) -> &str {
        &self.evidence_lookup_query_boundary_support_digest
    }
}
