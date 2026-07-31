use super::WorthQueryGraphReadMaterializationPolicy;
use crate::identity::hash_parts;
use crate::runtime::{
    WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessAdmissionPosture,
    WorthQueryGraphReadAccessDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadMaterializationRequestError {
    MissingAsyncMaterializationDenial,
    SuggestedPostureMismatch,
}

impl WorthQueryGraphReadMaterializationRequestError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MissingAsyncMaterializationDenial => "missing_async_materialization_denial",
            Self::SuggestedPostureMismatch => "suggested_posture_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializationRequest {
    digest: String,
    admission_digest: String,
    admission_denial_kind: WorthQueryGraphReadAccessDenialKind,
    requirement_set_digest: String,
    cost_estimate_digest: String,
    estimated_touched_edges: usize,
    estimated_resident_bytes: usize,
    estimated_emitted_rows: usize,
    budget_digest: String,
    inventory_match_report_digest: String,
    read_graph_digest: String,
    policy: WorthQueryGraphReadMaterializationPolicy,
}

impl WorthQueryGraphReadMaterializationRequest {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn admission_denial_kind(&self) -> &WorthQueryGraphReadAccessDenialKind {
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

    pub fn policy(&self) -> &WorthQueryGraphReadMaterializationPolicy {
        &self.policy
    }

    pub fn from_required_admission(
        admission: &WorthQueryGraphReadAccessAdmission,
        policy: WorthQueryGraphReadMaterializationPolicy,
    ) -> Result<Self, WorthQueryGraphReadMaterializationRequestError> {
        let denial = admission.denial().ok_or(
            WorthQueryGraphReadMaterializationRequestError::MissingAsyncMaterializationDenial,
        )?;
        if denial.suggested_posture()
            != &WorthQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired
        {
            return Err(WorthQueryGraphReadMaterializationRequestError::SuggestedPostureMismatch);
        }
        if !matches!(
            denial.kind(),
            WorthQueryGraphReadAccessDenialKind::BudgetExceeded
                | WorthQueryGraphReadAccessDenialKind::RequiredAsyncMaterialization
        ) {
            return Err(
                WorthQueryGraphReadMaterializationRequestError::MissingAsyncMaterializationDenial,
            );
        }
        Ok(Self::from_admission_parts(admission, policy))
    }

    fn from_admission_parts(
        admission: &WorthQueryGraphReadAccessAdmission,
        policy: WorthQueryGraphReadMaterializationPolicy,
    ) -> Self {
        let denial = admission
            .denial()
            .expect("materialization requests are built only after denial validation");
        let admission_denial_kind = denial.kind().clone();
        let admission_digest = admission.digest().to_string();
        let requirement_set_digest = admission.requirement_set().digest().render_support_hex();
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
        let read_graph_digest = admission.requirement_set().read_graph_digest().render_hex();
        let digest = hash_parts(&[
            "worth_query_graph_read_materialization_request_v1".to_string(),
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
