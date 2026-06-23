use forge_query::facade::{
    CausalInspectionExplanationFamily, CausalInspectionMaterializationPolicy, CausalInspectionPlan,
    CausalInspectionRichness,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarDiagnosticEvidenceKind {
    PlanarReceipt,
    TopologyDeclaredSurface,
    QueryInspection,
    QueryCausalInspection,
    ProjectionConsumptionReceipt,
    BasisLifecycleReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticEvidence {
    kind: PlanarDiagnosticEvidenceKind,
    evidence_digest: String,
}

impl PlanarDiagnosticEvidence {
    pub fn new(kind: PlanarDiagnosticEvidenceKind, evidence_digest: impl Into<String>) -> Self {
        Self {
            kind,
            evidence_digest: evidence_digest.into(),
        }
    }

    pub fn kind(&self) -> PlanarDiagnosticEvidenceKind {
        self.kind
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticTopologyEvidence {
    declared_surface_digest: String,
}

impl PlanarDiagnosticTopologyEvidence {
    pub fn declared_surface(declared_surface_digest: impl Into<String>) -> Self {
        Self {
            declared_surface_digest: declared_surface_digest.into(),
        }
    }

    pub fn declared_surface_digest(&self) -> &str {
        &self.declared_surface_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticCausalEvidence {
    reference_digest: String,
    anchor_digest: String,
    reference_set_digest: String,
    request_digest: String,
    admission_digest: String,
    richness: CausalInspectionRichness,
    explanation_family: CausalInspectionExplanationFamily,
    materialization_policy: CausalInspectionMaterializationPolicy,
}

impl PlanarDiagnosticCausalEvidence {
    pub fn from_query_causal_inspection_plan(plan: &CausalInspectionPlan) -> Self {
        Self {
            reference_digest: format!(
                "query-causal-reference:{}:{}:{}",
                plan.anchor_for_reporting(),
                plan.reference_set_digest(),
                plan.admission_digest()
            ),
            anchor_digest: plan.anchor_for_reporting().to_string(),
            reference_set_digest: plan.reference_set_digest().to_string(),
            request_digest: plan.request_for_reporting().to_string(),
            admission_digest: plan.admission_digest().to_string(),
            richness: plan.requested_richness(),
            explanation_family: plan.explanation_family(),
            materialization_policy: plan.materialization_policy(),
        }
    }

    pub fn reference_digest(&self) -> &str {
        &self.reference_digest
    }

    pub fn anchor_digest(&self) -> &str {
        &self.anchor_digest
    }

    pub fn reference_set_digest(&self) -> &str {
        &self.reference_set_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn richness(&self) -> CausalInspectionRichness {
        self.richness
    }

    pub fn explanation_family(&self) -> CausalInspectionExplanationFamily {
        self.explanation_family
    }

    pub fn materialization_policy(&self) -> CausalInspectionMaterializationPolicy {
        self.materialization_policy
    }
}
