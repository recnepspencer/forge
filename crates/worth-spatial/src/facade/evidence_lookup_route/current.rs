use std::sync::OnceLock;

use super::packet::EvidenceLookupRoutePacketParts;
use super::{
    current_evidence_lookup_route_source, EvidenceLookupRouteAdmissionError,
    EvidenceLookupRoutePacket,
};

pub fn current_evidence_lookup_route_packet(
) -> Result<EvidenceLookupRoutePacket, EvidenceLookupRouteAdmissionError> {
    static CACHE: OnceLock<EvidenceLookupRoutePacket> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let source = current_evidence_lookup_route_source()?;
    let family = source.left_family();
    let right_family = source.right_family();
    let boundary = source.left_boundary();
    let right_boundary = source.right_boundary();
    let handoff = boundary.workload_handoff();
    let index_product = boundary.index_product();
    let lowering_evidence = source.lowering_evidence();

    let packet = EvidenceLookupRoutePacket::from_parts(EvidenceLookupRoutePacketParts {
        route_authority_digest: source.route_authority_digest().to_string(),
        route_family_identity: family.identity().as_str().to_string(),
        right_route_family_identity: right_family.identity().as_str().to_string(),
        family_declaration_digest: family.declaration_digest().to_string(),
        stage: boundary.selected_plan().stage(),
        stage_receipt_family_identity: family
            .stage_applicability()
            .stage_receipt_family_identity()
            .digest()
            .to_string(),
        right_stage_receipt_identity: right_boundary
            .workload_handoff()
            .stage_receipt_identity()
            .to_string(),
        selected_lookup_plan_digest: handoff.selected_lookup_plan_digest().to_string(),
        lookup_execution_receipt_digest: handoff.lookup_execution_receipt_digest().to_string(),
        right_lookup_execution_receipt_digest: right_boundary
            .workload_handoff()
            .lookup_execution_receipt_digest()
            .to_string(),
        lookup_product_output_digest: handoff.lookup_product_output_digest().to_string(),
        compiled_product_identity_digest: index_product
            .compiled_product_identity_digest()
            .to_string(),
        equivalence_policy_identity_digest: index_product
            .equivalence_policy_identity_digest()
            .to_string(),
        selected_equivalence_family_identity: index_product
            .selected_equivalence_family_identity()
            .as_str()
            .to_string(),
        selected_equivalence_basis_identity_digest: index_product
            .selected_equivalence_basis_identity_digest()
            .to_string(),
        selected_compatibility_basis_identity_digest: index_product
            .selected_compatibility_basis_identity_digest()
            .to_string(),
        selected_reuse_basis_identity_digest: index_product
            .selected_reuse_basis_identity_digest()
            .to_string(),
        evidence_ledger_basis_digest: index_product.evidence_ledger_basis_digest().to_string(),
        topology_support_digest: index_product.topology_support_digest().to_string(),
        query_support_digest: index_product.query_support_digest().to_string(),
        spatial_touch_digest: boundary.authority().digest().as_str().to_string(),
        stage_receipt_digest: index_product.stage_receipt_digest().to_string(),
        right_authority_stage_index_identity: right_boundary
            .authority()
            .stage_index_identity()
            .to_string(),
        lowering_raw_row_revisit_count: lowering_evidence.raw_row_revisit_count(),
        lowering_right_receipt_revisit_count: lowering_evidence.right_receipt_revisit_count(),
        lowering_caller_owned_revisit_count: lowering_evidence.caller_owned_revisit_count(),
    });
    let _ = CACHE.set(packet.clone());
    Ok(packet)
}
