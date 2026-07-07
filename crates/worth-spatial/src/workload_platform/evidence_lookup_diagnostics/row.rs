use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupEvidenceClassSet;
use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanQueryPosture, EvidenceLookupPlanTopologyPosture,
};
use crate::workload_platform::evidence_lookup_query_surface_contract::{
    EvidenceLookupQuerySurfaceContract, EvidenceLookupQuerySurfaceContractProvenance,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::advisory_reason::EvidenceLookupDiagnosticAdvisoryReason;
use super::denial_reason::EvidenceLookupDiagnosticDenialReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupDiagnosticQuerySurfaceProvenance {
    SupportAdmission,
    SupportPinning,
    ProjectionConsumption,
    LowerRuntimeBoundaryEnvelope,
    TypedArtifactIdentity,
    ConsumerKitProof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupDiagnosticWitness {
    Success,
    Advisory(EvidenceLookupDiagnosticAdvisoryReason),
    Denied(EvidenceLookupDiagnosticDenialReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupDiagnosticRow {
    family_identity: String,
    family_declaration_digest: String,
    stage: WorkloadEvidenceStage,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    selected_plan_digest: String,
    selected_plan_row_digest: String,
    execution_receipt_digest: String,
    evidence_classes: EvidenceLookupEvidenceClassSet,
    topology_posture: EvidenceLookupPlanTopologyPosture,
    query_posture: EvidenceLookupPlanQueryPosture,
    query_surface_contract: Option<EvidenceLookupQuerySurfaceContract>,
    witness: EvidenceLookupDiagnosticWitness,
    row_digest: String,
}

pub(crate) struct EvidenceLookupDiagnosticRowParts {
    pub(crate) family_identity: String,
    pub(crate) family_declaration_digest: String,
    pub(crate) stage: WorkloadEvidenceStage,
    pub(crate) spatial_touch_digest: String,
    pub(crate) stage_receipt_digest: String,
    pub(crate) selected_plan_digest: String,
    pub(crate) selected_plan_row_digest: String,
    pub(crate) execution_receipt_digest: String,
    pub(crate) evidence_classes: EvidenceLookupEvidenceClassSet,
    pub(crate) topology_posture: EvidenceLookupPlanTopologyPosture,
    pub(crate) query_posture: EvidenceLookupPlanQueryPosture,
    pub(crate) query_surface_contract: Option<EvidenceLookupQuerySurfaceContract>,
    pub(crate) witness: EvidenceLookupDiagnosticWitness,
}

impl EvidenceLookupDiagnosticRow {
    pub(crate) fn from_parts(parts: EvidenceLookupDiagnosticRowParts) -> Self {
        let row_digest = row_digest(&parts);
        Self {
            family_identity: parts.family_identity,
            family_declaration_digest: parts.family_declaration_digest,
            stage: parts.stage,
            spatial_touch_digest: parts.spatial_touch_digest,
            stage_receipt_digest: parts.stage_receipt_digest,
            selected_plan_digest: parts.selected_plan_digest,
            selected_plan_row_digest: parts.selected_plan_row_digest,
            execution_receipt_digest: parts.execution_receipt_digest,
            evidence_classes: parts.evidence_classes,
            topology_posture: parts.topology_posture,
            query_posture: parts.query_posture,
            query_surface_contract: parts.query_surface_contract,
            witness: parts.witness,
            row_digest,
        }
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

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn selected_plan_row_digest(&self) -> &str {
        &self.selected_plan_row_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub const fn evidence_classes(&self) -> &EvidenceLookupEvidenceClassSet {
        &self.evidence_classes
    }

    pub const fn topology_posture(&self) -> &EvidenceLookupPlanTopologyPosture {
        &self.topology_posture
    }

    pub const fn query_posture(&self) -> &EvidenceLookupPlanQueryPosture {
        &self.query_posture
    }

    pub const fn witness(&self) -> EvidenceLookupDiagnosticWitness {
        self.witness
    }

    pub fn query_surface(&self) -> EvidenceLookupQuerySurface {
        self.query_surface_contract
            .as_ref()
            .map(EvidenceLookupQuerySurfaceContract::query_surface)
            .unwrap_or(EvidenceLookupQuerySurface::NotQuery)
    }

    pub fn query_surface_type_name(&self) -> Option<&'static str> {
        self.query_surface_contract
            .as_ref()
            .map(EvidenceLookupQuerySurfaceContract::query_surface_type_name)
    }

    pub fn query_surface_provenance(
        &self,
    ) -> Option<EvidenceLookupDiagnosticQuerySurfaceProvenance> {
        self.query_surface_contract
            .as_ref()
            .map(|contract| match contract.provenance() {
                EvidenceLookupQuerySurfaceContractProvenance::SupportAdmission => {
                    EvidenceLookupDiagnosticQuerySurfaceProvenance::SupportAdmission
                }
                EvidenceLookupQuerySurfaceContractProvenance::SupportPinning => {
                    EvidenceLookupDiagnosticQuerySurfaceProvenance::SupportPinning
                }
                EvidenceLookupQuerySurfaceContractProvenance::ProjectionConsumption => {
                    EvidenceLookupDiagnosticQuerySurfaceProvenance::ProjectionConsumption
                }
                EvidenceLookupQuerySurfaceContractProvenance::LowerRuntimeBoundaryEnvelope => {
                    EvidenceLookupDiagnosticQuerySurfaceProvenance::LowerRuntimeBoundaryEnvelope
                }
                EvidenceLookupQuerySurfaceContractProvenance::TypedArtifactIdentity => {
                    EvidenceLookupDiagnosticQuerySurfaceProvenance::TypedArtifactIdentity
                }
                EvidenceLookupQuerySurfaceContractProvenance::ConsumerKitProof => {
                    EvidenceLookupDiagnosticQuerySurfaceProvenance::ConsumerKitProof
                }
            })
    }

    pub fn query_proof_digest(&self) -> Option<&str> {
        self.query_surface_contract
            .as_ref()
            .map(EvidenceLookupQuerySurfaceContract::proof_digest)
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub const fn claims_lookup_execution_authority(&self) -> bool {
        false
    }

    pub const fn claims_query_descriptor_authority(&self) -> bool {
        false
    }

    pub(crate) const fn query_surface_contract(
        &self,
    ) -> Option<&EvidenceLookupQuerySurfaceContract> {
        self.query_surface_contract.as_ref()
    }
}

fn row_digest(parts: &EvidenceLookupDiagnosticRowParts) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-diagnostic-row:v1".to_string(),
            format!("family:{}", parts.family_identity),
            format!("declaration:{}", parts.family_declaration_digest),
            format!("stage:{}", parts.stage.human_name()),
            format!("spatial-touch:{}", parts.spatial_touch_digest),
            format!("stage-receipt:{}", parts.stage_receipt_digest),
            format!("selected-plan:{}", parts.selected_plan_digest),
            format!("selected-plan-row:{}", parts.selected_plan_row_digest),
            format!("execution-receipt:{}", parts.execution_receipt_digest),
            parts.topology_posture.digest_part(),
            parts.query_posture.digest_part(),
            format!("witness:{:?}", parts.witness),
            format!(
                "query-proof:{}",
                parts
                    .query_surface_contract
                    .as_ref()
                    .map(EvidenceLookupQuerySurfaceContract::proof_digest)
                    .unwrap_or("not-query")
            ),
        ],
    )
}
