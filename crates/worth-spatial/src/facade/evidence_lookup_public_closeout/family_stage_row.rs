use crate::spatial_compiled_product_family::SpatialCompiledProductLoweredIdentity;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyDeclaration;
use crate::workload_platform::evidence_lookup_stage_cutover::EvidenceLookupCoveredStageCutoverProof;
use crate::workload_platform::selected_equivalence_family::SpatialSelectedEquivalenceFamilyIdentity;
use schema::facade::platform::authority::compiled_product_semantic_graph::{
    CompiledProductEquivalencePolicyIdentity, CompiledProductIdentity,
};
use topology::facade::TopologyQueryBackedConsumerFamilyRow;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPublicCloseoutDisposition {
    ReceiptProof {
        selected_lookup_plan_digest: String,
        lookup_execution_receipt_digest: String,
        lookup_product_output_digest: String,
    },
    NonOrdinaryResidue {
        reason: String,
        removal_trigger: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseoutFamilyStageRow {
    pub(super) family_identity: String,
    pub(super) family_declaration_digest: String,
    pub(super) stage: WorkloadEvidenceStage,
    pub(super) stage_receipt_family_identity: String,
    pub(super) spatial_compiled_product_identity: Option<CompiledProductIdentity>,
    pub(super) spatial_equivalence_policy_identity:
        Option<CompiledProductEquivalencePolicyIdentity>,
    pub(super) spatial_selected_equivalence_family_identity:
        Option<SpatialSelectedEquivalenceFamilyIdentity>,
    pub(super) spatial_touch_digest: Option<String>,
    pub(super) topology_query_backed_cutover_digest: Option<String>,
    pub(super) topology_read_family_row_digest: Option<String>,
    pub(super) topology_input_summary: String,
    pub(super) query_import_evidence_digest: Option<String>,
    pub(super) query_surface_row_digest: String,
    pub(super) disposition: EvidenceLookupPublicCloseoutDisposition,
    pub(super) row_digest: String,
}

impl EvidenceLookupPublicCloseoutFamilyStageRow {
    pub(crate) fn from_receipt_proof(
        family: &EvidenceLookupFamilyDeclaration,
        stage: WorkloadEvidenceStage,
        query_surface_row_digest: &str,
        proof: &EvidenceLookupCoveredStageCutoverProof,
        lowered_identity: &SpatialCompiledProductLoweredIdentity,
    ) -> Self {
        let row = Self {
            family_identity: family.identity().as_str().to_string(),
            family_declaration_digest: family.declaration_digest().to_string(),
            stage,
            stage_receipt_family_identity: family
                .stage_applicability()
                .stage_receipt_family_identity()
                .digest()
                .to_string(),
            spatial_compiled_product_identity: Some(
                lowered_identity.compiled_product_identity().clone(),
            ),
            spatial_equivalence_policy_identity: Some(
                lowered_identity.equivalence_policy_identity().clone(),
            ),
            spatial_selected_equivalence_family_identity: Some(
                proof.selected_equivalence_family_identity_kind(),
            ),
            spatial_touch_digest: Some(proof.spatial_touch_digest().to_string()),
            topology_query_backed_cutover_digest: None,
            topology_read_family_row_digest: None,
            topology_input_summary: topology_input_summary(family),
            query_import_evidence_digest: family
                .query_posture()
                .imported_evidence_digest()
                .map(str::to_string),
            query_surface_row_digest: query_surface_row_digest.to_string(),
            disposition: EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
                selected_lookup_plan_digest: proof.selected_lookup_plan_digest().to_string(),
                lookup_execution_receipt_digest: proof
                    .lookup_execution_receipt_digest()
                    .to_string(),
                lookup_product_output_digest: proof.lookup_product_output_digest().to_string(),
            },
            row_digest: String::new(),
        };
        row.with_row_digest()
    }

    pub(crate) fn from_receipt_proof_with_topology_read_receipt(
        family: &EvidenceLookupFamilyDeclaration,
        stage: WorkloadEvidenceStage,
        query_surface_row_digest: &str,
        proof: &EvidenceLookupCoveredStageCutoverProof,
        lowered_identity: &SpatialCompiledProductLoweredIdentity,
        topology_query_backed_cutover_digest: &str,
        topology_read_family_row: &TopologyQueryBackedConsumerFamilyRow,
    ) -> Self {
        let row = Self {
            family_identity: family.identity().as_str().to_string(),
            family_declaration_digest: family.declaration_digest().to_string(),
            stage,
            stage_receipt_family_identity: family
                .stage_applicability()
                .stage_receipt_family_identity()
                .digest()
                .to_string(),
            spatial_compiled_product_identity: Some(
                lowered_identity.compiled_product_identity().clone(),
            ),
            spatial_equivalence_policy_identity: Some(
                lowered_identity.equivalence_policy_identity().clone(),
            ),
            spatial_selected_equivalence_family_identity: Some(
                proof.selected_equivalence_family_identity_kind(),
            ),
            spatial_touch_digest: Some(proof.spatial_touch_digest().to_string()),
            topology_query_backed_cutover_digest: Some(
                topology_query_backed_cutover_digest.to_string(),
            ),
            topology_read_family_row_digest: Some(
                topology_read_family_row.row_digest().to_string(),
            ),
            topology_input_summary: topology_input_summary(family),
            query_import_evidence_digest: family
                .query_posture()
                .imported_evidence_digest()
                .map(str::to_string),
            query_surface_row_digest: query_surface_row_digest.to_string(),
            disposition: EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
                selected_lookup_plan_digest: proof.selected_lookup_plan_digest().to_string(),
                lookup_execution_receipt_digest: proof
                    .lookup_execution_receipt_digest()
                    .to_string(),
                lookup_product_output_digest: proof.lookup_product_output_digest().to_string(),
            },
            row_digest: String::new(),
        };
        row.with_row_digest()
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
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
    pub fn spatial_touch_digest(&self) -> Option<&str> {
        self.spatial_touch_digest.as_deref()
    }
    pub fn spatial_compiled_product_identity_digest(&self) -> Option<&str> {
        self.spatial_compiled_product_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }
    pub fn spatial_equivalence_policy_identity_digest(&self) -> Option<&str> {
        self.spatial_equivalence_policy_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }
    pub fn spatial_selected_equivalence_family_identity(&self) -> Option<&str> {
        self.spatial_selected_equivalence_family_identity
            .as_ref()
            .map(|identity| identity.as_str())
    }
    pub fn topology_input_summary(&self) -> &str {
        &self.topology_input_summary
    }
    pub fn topology_query_backed_cutover_digest(&self) -> Option<&str> {
        self.topology_query_backed_cutover_digest.as_deref()
    }
    pub fn topology_read_family_row_digest(&self) -> Option<&str> {
        self.topology_read_family_row_digest.as_deref()
    }
    pub fn query_import_evidence_digest(&self) -> Option<&str> {
        self.query_import_evidence_digest.as_deref()
    }
    pub fn query_surface_row_digest(&self) -> &str {
        &self.query_surface_row_digest
    }
    pub const fn disposition(&self) -> &EvidenceLookupPublicCloseoutDisposition {
        &self.disposition
    }
    pub fn selected_lookup_plan_digest(&self) -> Option<&str> {
        match &self.disposition {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
                selected_lookup_plan_digest,
                ..
            } => Some(selected_lookup_plan_digest),
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => None,
        }
    }
    pub fn lookup_execution_receipt_digest(&self) -> Option<&str> {
        match &self.disposition {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
                lookup_execution_receipt_digest,
                ..
            } => Some(lookup_execution_receipt_digest),
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => None,
        }
    }
    pub fn lookup_product_output_digest(&self) -> Option<&str> {
        match &self.disposition {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
                lookup_product_output_digest,
                ..
            } => Some(lookup_product_output_digest),
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => None,
        }
    }
    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    fn with_row_digest(mut self) -> Self {
        self.row_digest = row_digest(&self);
        self
    }
}

fn topology_input_summary(family: &EvidenceLookupFamilyDeclaration) -> String {
    match family.topology_input_posture().required_family_identity() {
        Some(family_identity) => format!(
            "{:?}:{family_identity}",
            family.topology_input_posture().state()
        ),
        None => format!("{:?}:none", family.topology_input_posture().state()),
    }
}

fn row_digest(row: &EvidenceLookupPublicCloseoutFamilyStageRow) -> String {
    let disposition_part = match &row.disposition {
        EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
            selected_lookup_plan_digest,
            lookup_execution_receipt_digest,
            lookup_product_output_digest,
        } => format!("receipt-proof:{selected_lookup_plan_digest}:{lookup_execution_receipt_digest}:{lookup_product_output_digest}"),
        EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { reason, removal_trigger } => {
            format!("residue:{reason}:{removal_trigger}")
        }
    };
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-public-closeout-family-stage-row:v2".to_string(),
            row.family_identity.clone(),
            row.family_declaration_digest.clone(),
            row.stage.human_name().to_string(),
            row.stage_receipt_family_identity.clone(),
            row.spatial_compiled_product_identity
                .as_ref()
                .map(|identity| identity.identity_digest().to_string())
                .unwrap_or_else(|| "no-spatial-compiled-product-proof".to_string()),
            row.spatial_equivalence_policy_identity
                .as_ref()
                .map(|identity| identity.identity_digest().to_string())
                .unwrap_or_else(|| "no-spatial-equivalence-proof".to_string()),
            row.spatial_selected_equivalence_family_identity
                .as_ref()
                .map(|identity| identity.as_str().to_string())
                .unwrap_or_else(|| "no-spatial-selected-family".to_string()),
            row.spatial_touch_digest
                .clone()
                .unwrap_or_else(|| "no-spatial-touch-proof".to_string()),
            row.topology_query_backed_cutover_digest
                .clone()
                .unwrap_or_else(|| "no-topology-query-backed-cutover".to_string()),
            row.topology_read_family_row_digest
                .clone()
                .unwrap_or_else(|| "no-topology-read-family-row".to_string()),
            row.topology_input_summary.clone(),
            row.query_import_evidence_digest
                .clone()
                .unwrap_or_else(|| "no-query-import".to_string()),
            row.query_surface_row_digest.clone(),
            disposition_part,
        ],
    )
}
