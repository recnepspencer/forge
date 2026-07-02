use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupRoutePacket {
    route_packet_digest: String,
    route_authority_digest: String,
    route_family_identity: String,
    right_route_family_identity: String,
    family_declaration_digest: String,
    stage: WorkloadEvidenceStage,
    stage_receipt_family_identity: String,
    right_stage_receipt_identity: String,
    selected_lookup_plan_digest: String,
    lookup_execution_receipt_digest: String,
    right_lookup_execution_receipt_digest: String,
    lookup_product_output_digest: String,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    selected_equivalence_family_identity: String,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    evidence_ledger_basis_digest: String,
    topology_support_digest: String,
    query_support_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    right_authority_stage_index_identity: String,
    lowering_raw_row_revisit_count: usize,
    lowering_right_receipt_revisit_count: usize,
    lowering_caller_owned_revisit_count: usize,
}

pub(crate) struct EvidenceLookupRoutePacketParts {
    pub(crate) route_authority_digest: String,
    pub(crate) route_family_identity: String,
    pub(crate) right_route_family_identity: String,
    pub(crate) family_declaration_digest: String,
    pub(crate) stage: WorkloadEvidenceStage,
    pub(crate) stage_receipt_family_identity: String,
    pub(crate) right_stage_receipt_identity: String,
    pub(crate) selected_lookup_plan_digest: String,
    pub(crate) lookup_execution_receipt_digest: String,
    pub(crate) right_lookup_execution_receipt_digest: String,
    pub(crate) lookup_product_output_digest: String,
    pub(crate) compiled_product_identity_digest: String,
    pub(crate) equivalence_policy_identity_digest: String,
    pub(crate) selected_equivalence_family_identity: String,
    pub(crate) selected_equivalence_basis_identity_digest: String,
    pub(crate) selected_compatibility_basis_identity_digest: String,
    pub(crate) selected_reuse_basis_identity_digest: String,
    pub(crate) evidence_ledger_basis_digest: String,
    pub(crate) topology_support_digest: String,
    pub(crate) query_support_digest: String,
    pub(crate) spatial_touch_digest: String,
    pub(crate) stage_receipt_digest: String,
    pub(crate) right_authority_stage_index_identity: String,
    pub(crate) lowering_raw_row_revisit_count: usize,
    pub(crate) lowering_right_receipt_revisit_count: usize,
    pub(crate) lowering_caller_owned_revisit_count: usize,
}

impl EvidenceLookupRoutePacket {
    pub(crate) fn from_parts(parts: EvidenceLookupRoutePacketParts) -> Self {
        let route_packet_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-route-packet:v1".to_string(),
                format!("route-authority:{}", parts.route_authority_digest),
                format!("route-family:{}", parts.route_family_identity),
                format!("right-route-family:{}", parts.right_route_family_identity),
                format!("family-declaration:{}", parts.family_declaration_digest),
                format!("stage:{}", parts.stage.human_name()),
                format!(
                    "stage-receipt-family:{}",
                    parts.stage_receipt_family_identity
                ),
                format!("selected-plan:{}", parts.selected_lookup_plan_digest),
                format!(
                    "execution-receipt:{}",
                    parts.lookup_execution_receipt_digest
                ),
                format!("right-stage-receipt:{}", parts.right_stage_receipt_identity),
                format!(
                    "right-execution-receipt:{}",
                    parts.right_lookup_execution_receipt_digest
                ),
                format!("lookup-output:{}", parts.lookup_product_output_digest),
                format!(
                    "compiled-product:{}",
                    parts.compiled_product_identity_digest
                ),
                format!(
                    "equivalence-policy:{}",
                    parts.equivalence_policy_identity_digest
                ),
                format!(
                    "selected-family:{}",
                    parts.selected_equivalence_family_identity
                ),
                format!(
                    "selected-equivalence-basis:{}",
                    parts.selected_equivalence_basis_identity_digest
                ),
                format!(
                    "selected-compatibility-basis:{}",
                    parts.selected_compatibility_basis_identity_digest
                ),
                format!(
                    "selected-reuse-basis:{}",
                    parts.selected_reuse_basis_identity_digest
                ),
                format!("ledger-basis:{}", parts.evidence_ledger_basis_digest),
                format!("topology-support:{}", parts.topology_support_digest),
                format!("query-support:{}", parts.query_support_digest),
                format!("spatial-touch:{}", parts.spatial_touch_digest),
                format!("stage-receipt:{}", parts.stage_receipt_digest),
                format!(
                    "right-authority-stage-index:{}",
                    parts.right_authority_stage_index_identity
                ),
                format!(
                    "lowering-raw-row-revisit:{}",
                    parts.lowering_raw_row_revisit_count
                ),
                format!(
                    "lowering-right-receipt-revisit:{}",
                    parts.lowering_right_receipt_revisit_count
                ),
                format!(
                    "lowering-caller-owned-revisit:{}",
                    parts.lowering_caller_owned_revisit_count
                ),
            ],
        );
        Self {
            route_packet_digest,
            route_authority_digest: parts.route_authority_digest,
            route_family_identity: parts.route_family_identity,
            right_route_family_identity: parts.right_route_family_identity,
            family_declaration_digest: parts.family_declaration_digest,
            stage: parts.stage,
            stage_receipt_family_identity: parts.stage_receipt_family_identity,
            right_stage_receipt_identity: parts.right_stage_receipt_identity,
            selected_lookup_plan_digest: parts.selected_lookup_plan_digest,
            lookup_execution_receipt_digest: parts.lookup_execution_receipt_digest,
            right_lookup_execution_receipt_digest: parts.right_lookup_execution_receipt_digest,
            lookup_product_output_digest: parts.lookup_product_output_digest,
            compiled_product_identity_digest: parts.compiled_product_identity_digest,
            equivalence_policy_identity_digest: parts.equivalence_policy_identity_digest,
            selected_equivalence_family_identity: parts.selected_equivalence_family_identity,
            selected_equivalence_basis_identity_digest: parts
                .selected_equivalence_basis_identity_digest,
            selected_compatibility_basis_identity_digest: parts
                .selected_compatibility_basis_identity_digest,
            selected_reuse_basis_identity_digest: parts.selected_reuse_basis_identity_digest,
            evidence_ledger_basis_digest: parts.evidence_ledger_basis_digest,
            topology_support_digest: parts.topology_support_digest,
            query_support_digest: parts.query_support_digest,
            spatial_touch_digest: parts.spatial_touch_digest,
            stage_receipt_digest: parts.stage_receipt_digest,
            right_authority_stage_index_identity: parts.right_authority_stage_index_identity,
            lowering_raw_row_revisit_count: parts.lowering_raw_row_revisit_count,
            lowering_right_receipt_revisit_count: parts.lowering_right_receipt_revisit_count,
            lowering_caller_owned_revisit_count: parts.lowering_caller_owned_revisit_count,
        }
    }

    pub fn route_packet_digest(&self) -> &str {
        &self.route_packet_digest
    }
    pub fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }
    pub fn route_family_identity(&self) -> &str {
        &self.route_family_identity
    }
    pub fn right_route_family_identity(&self) -> &str {
        &self.right_route_family_identity
    }
    pub fn family_declaration_digest(&self) -> &str {
        &self.family_declaration_digest
    }
    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }
    pub fn stage_receipt_family_identity(&self) -> &str {
        &self.stage_receipt_family_identity
    }
    pub fn right_stage_receipt_identity(&self) -> &str {
        &self.right_stage_receipt_identity
    }
    pub fn selected_lookup_plan_digest(&self) -> &str {
        &self.selected_lookup_plan_digest
    }
    pub fn lookup_execution_receipt_digest(&self) -> &str {
        &self.lookup_execution_receipt_digest
    }
    pub fn right_lookup_execution_receipt_digest(&self) -> &str {
        &self.right_lookup_execution_receipt_digest
    }
    pub fn lookup_product_output_digest(&self) -> &str {
        &self.lookup_product_output_digest
    }
    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }
    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }
    pub fn selected_equivalence_family_identity(&self) -> &str {
        &self.selected_equivalence_family_identity
    }
    pub fn selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.selected_equivalence_basis_identity_digest
    }
    pub fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }
    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }
    pub fn evidence_ledger_basis_digest(&self) -> &str {
        &self.evidence_ledger_basis_digest
    }
    pub fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }
    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }
    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }
    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }
    pub fn right_authority_stage_index_identity(&self) -> &str {
        &self.right_authority_stage_index_identity
    }
    pub const fn lowering_raw_row_revisit_count(&self) -> usize {
        self.lowering_raw_row_revisit_count
    }
    pub const fn lowering_right_receipt_revisit_count(&self) -> usize {
        self.lowering_right_receipt_revisit_count
    }
    pub const fn lowering_caller_owned_revisit_count(&self) -> usize {
        self.lowering_caller_owned_revisit_count
    }
}
