use super::ForgeQueryGraphReadMaterializationPolicy;
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadMaterializationRequestError {
    MissingAsyncMaterializationDenial,
    SuggestedPostureMismatch,
}

impl ForgeQueryGraphReadMaterializationRequestError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingAsyncMaterializationDenial => "missing_async_materialization_denial",
            Self::SuggestedPostureMismatch => "suggested_posture_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadMaterializationRequest {
    digest: String,
    admission_digest: String,
    admission_denial_kind: ForgeQueryGraphReadAccessDenialKind,
    requirement_set_digest: String,
    cost_estimate_digest: String,
    estimated_touched_edges: usize,
    estimated_resident_bytes: usize,
    estimated_emitted_rows: usize,
    budget_digest: String,
    inventory_match_report_digest: String,
    read_graph_digest: String,
    policy: ForgeQueryGraphReadMaterializationPolicy,
}

impl ForgeQueryGraphReadMaterializationRequest {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn admission_denial_kind(&self) -> &ForgeQueryGraphReadAccessDenialKind {
        &self.admission_denial_kind
    }

    pub fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub fn cost_estimate_digest(&self) -> &str {
        &self.cost_estimate_digest
    }

    pub fn estimated_touched_edges(&self) -> usize {
        self.estimated_touched_edges
    }

    pub fn estimated_resident_bytes(&self) -> usize {
        self.estimated_resident_bytes
    }

    pub fn estimated_emitted_rows(&self) -> usize {
        self.estimated_emitted_rows
    }

    pub fn budget_digest(&self) -> &str {
        &self.budget_digest
    }

    pub fn inventory_match_report_digest(&self) -> &str {
        &self.inventory_match_report_digest
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn policy(&self) -> &ForgeQueryGraphReadMaterializationPolicy {
        &self.policy
    }

    pub fn from_required_admission(
        admission: &ForgeQueryGraphReadAccessAdmission,
        policy: ForgeQueryGraphReadMaterializationPolicy,
    ) -> Result<Self, ForgeQueryGraphReadMaterializationRequestError> {
        let denial = admission.denial().ok_or(
            ForgeQueryGraphReadMaterializationRequestError::MissingAsyncMaterializationDenial,
        )?;
        if denial.suggested_posture()
            != &ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired
        {
            return Err(ForgeQueryGraphReadMaterializationRequestError::SuggestedPostureMismatch);
        }
        if !matches!(
            denial.kind(),
            ForgeQueryGraphReadAccessDenialKind::BudgetExceeded
                | ForgeQueryGraphReadAccessDenialKind::RequiredAsyncMaterialization
        ) {
            return Err(
                ForgeQueryGraphReadMaterializationRequestError::MissingAsyncMaterializationDenial,
            );
        }
        Ok(Self::from_admission_parts(admission, policy))
    }

    fn from_admission_parts(
        admission: &ForgeQueryGraphReadAccessAdmission,
        policy: ForgeQueryGraphReadMaterializationPolicy,
    ) -> Self {
        let denial = admission
            .denial()
            .expect("materialization requests are built only after denial validation");
        let admission_denial_kind = denial.kind().clone();
        let admission_digest = admission.digest().to_string();
        let requirement_set_digest = admission.requirement_set().digest().as_str().to_string();
        let cost_estimate = admission.cost_estimate();
        let cost_estimate_digest = cost_estimate.digest().as_str().to_string();
        let estimated_touched_edges = cost_estimate.intrinsic().edge_touches();
        let estimated_resident_bytes = cost_estimate.supported().memory().total_bytes();
        let estimated_emitted_rows = cost_estimate.intrinsic().intermediate_set_size();
        let budget_digest = admission.budget_check().budget_digest().to_string();
        let inventory_match_report_digest = admission
            .graph_index_inventory_match_report()
            .digest()
            .to_string();
        let read_graph_digest = admission.requirement_set().read_graph_digest().to_string();
        let digest = hash_parts(&[
            "forge_query_graph_read_materialization_request_v1".to_string(),
            format!("admission:{admission_digest}"),
            format!("denial_kind:{}", admission_denial_kind.as_str()),
            format!("requirements:{requirement_set_digest}"),
            format!("estimate:{cost_estimate_digest}"),
            format!("estimated_touched_edges:{estimated_touched_edges}"),
            format!("estimated_resident_bytes:{estimated_resident_bytes}"),
            format!("estimated_emitted_rows:{estimated_emitted_rows}"),
            format!("budget:{budget_digest}"),
            format!("inventory_match:{inventory_match_report_digest}"),
            format!("read_graph:{read_graph_digest}"),
            format!("policy:{}", policy.digest()),
        ]);
        Self {
            digest,
            admission_digest,
            admission_denial_kind,
            requirement_set_digest,
            cost_estimate_digest,
            estimated_touched_edges,
            estimated_resident_bytes,
            estimated_emitted_rows,
            budget_digest,
            inventory_match_report_digest,
            read_graph_digest,
            policy,
        }
    }
}
