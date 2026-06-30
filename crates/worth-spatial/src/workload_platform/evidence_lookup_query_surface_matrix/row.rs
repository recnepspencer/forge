use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyDeclaration, EvidenceLookupProjectionFactFamily,
};
use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_query_surface_contract::EvidenceLookupQuerySurfaceContract;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQuerySurfaceTouchpoint {
    FamilyCatalogQueryPosture,
    InputAdmissionQuerySupport,
    PlanSelectionQueryPosture,
    IndexProductQuerySupport,
    ExecutionReceiptQuerySupport,
    DiagnosticWitnessContract,
    PublicCloseoutProof,
}

impl EvidenceLookupQuerySurfaceTouchpoint {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FamilyCatalogQueryPosture => "family-catalog-query-posture",
            Self::InputAdmissionQuerySupport => "input-admission-query-support",
            Self::PlanSelectionQueryPosture => "plan-selection-query-posture",
            Self::IndexProductQuerySupport => "index-product-query-support",
            Self::ExecutionReceiptQuerySupport => "execution-receipt-query-support",
            Self::DiagnosticWitnessContract => "diagnostic-witness-contract",
            Self::PublicCloseoutProof => "public-closeout-proof",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQuerySurfaceMatrixRow {
    family_identity: String,
    stage: WorkloadEvidenceStage,
    touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    query_surface: EvidenceLookupQuerySurface,
    query_surface_type_name: Option<&'static str>,
    projection_fact_family: Option<EvidenceLookupProjectionFactFamily>,
    query_support_required: bool,
    proof_digest: Option<String>,
    declaration_digest: String,
    row_digest: String,
}

impl EvidenceLookupQuerySurfaceMatrixRow {
    pub(crate) fn from_family_stage_touchpoint_contract(
        family: &EvidenceLookupFamilyDeclaration,
        stage: WorkloadEvidenceStage,
        touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
        contract: Option<&EvidenceLookupQuerySurfaceContract>,
    ) -> Self {
        let query_support_required = family.query_posture().requires_query_evidence();
        let query_surface = contract
            .map(EvidenceLookupQuerySurfaceContract::query_surface)
            .unwrap_or(EvidenceLookupQuerySurface::NotQuery);
        let query_surface_type_name =
            contract.map(EvidenceLookupQuerySurfaceContract::query_surface_type_name);
        let projection_fact_family =
            contract.and_then(EvidenceLookupQuerySurfaceContract::projection_fact_family);
        let proof_digest = contract
            .map(EvidenceLookupQuerySurfaceContract::proof_digest)
            .map(str::to_string);
        let row_digest = row_digest(
            family.identity().as_str(),
            stage,
            touchpoint,
            query_surface,
            query_surface_type_name,
            projection_fact_family,
            query_support_required,
            proof_digest.as_deref(),
            family.declaration_digest(),
        );

        Self {
            family_identity: family.identity().as_str().to_string(),
            stage,
            touchpoint,
            query_surface,
            query_surface_type_name,
            projection_fact_family,
            query_support_required,
            proof_digest,
            declaration_digest: family.declaration_digest().to_string(),
            row_digest,
        }
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub const fn touchpoint(&self) -> EvidenceLookupQuerySurfaceTouchpoint {
        self.touchpoint
    }

    pub const fn query_surface(&self) -> EvidenceLookupQuerySurface {
        self.query_surface
    }

    pub const fn query_surface_type_name(&self) -> Option<&'static str> {
        self.query_surface_type_name
    }

    pub const fn projection_fact_family(&self) -> Option<EvidenceLookupProjectionFactFamily> {
        self.projection_fact_family
    }

    pub const fn query_support_required(&self) -> bool {
        self.query_support_required
    }

    pub fn proof_digest(&self) -> Option<&str> {
        self.proof_digest.as_deref()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
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
}

fn row_digest(
    family_identity: &str,
    stage: WorkloadEvidenceStage,
    touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    query_surface: EvidenceLookupQuerySurface,
    query_surface_type_name: Option<&'static str>,
    projection_fact_family: Option<EvidenceLookupProjectionFactFamily>,
    query_support_required: bool,
    proof_digest: Option<&str>,
    declaration_digest: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-query-surface-matrix-row:v1".to_string(),
            format!("family:{family_identity}"),
            format!("stage:{}", stage.human_name()),
            format!("touchpoint:{}", touchpoint.as_str()),
            format!("query-surface:{query_surface:?}"),
            format!(
                "query-surface-type:{}",
                query_surface_type_name.unwrap_or("not-query")
            ),
            format!(
                "projection-fact-family:{}",
                projection_fact_family
                    .map(EvidenceLookupProjectionFactFamily::as_str)
                    .unwrap_or("not-required")
            ),
            format!("query-support-required:{query_support_required}"),
            format!("proof-digest:{}", proof_digest.unwrap_or("not-required")),
            format!("declaration:{declaration_digest}"),
        ],
    )
}
