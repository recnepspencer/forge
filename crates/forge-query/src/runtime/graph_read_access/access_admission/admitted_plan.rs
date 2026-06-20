use super::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessPlanExplanation,
};
use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryEphemeralGraphIndexPlan, ForgeQueryGraphIndexInventoryMatchReport,
    ForgeQueryGraphReadStreamingPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadAccessPlan {
    digest: String,
    admission: ForgeQueryGraphReadAccessAdmission,
    execution_strategy: String,
    ephemeral_index_plan: Option<ForgeQueryEphemeralGraphIndexPlan>,
    streaming_plan: Option<ForgeQueryGraphReadStreamingPlan>,
}

impl ForgeQueryAdmittedGraphReadAccessPlan {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn admission(&self) -> &ForgeQueryGraphReadAccessAdmission {
        &self.admission
    }

    pub fn posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        self.admission.posture()
    }

    pub fn execution_strategy(&self) -> &str {
        &self.execution_strategy
    }

    pub fn ephemeral_index_plan(&self) -> Option<&ForgeQueryEphemeralGraphIndexPlan> {
        self.ephemeral_index_plan.as_ref()
    }

    pub fn streaming_plan(&self) -> Option<&ForgeQueryGraphReadStreamingPlan> {
        self.streaming_plan.as_ref()
    }

    pub fn graph_index_support(&self) -> &ForgeQueryGraphIndexInventoryMatchReport {
        self.admission.graph_index_inventory_match_report()
    }

    pub fn explanation(&self) -> ForgeQueryGraphReadAccessPlanExplanation {
        ForgeQueryGraphReadAccessPlanExplanation::from_admitted_plan(self)
    }

    pub(crate) fn from_admission(admission: ForgeQueryGraphReadAccessAdmission) -> Option<Self> {
        if !admission.is_admitted() {
            return None;
        }
        let execution_strategy = match admission.posture() {
            ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed => {
                "inline-indexed-read-execution"
            }
            ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex => {
                "bounded-ephemeral-index-read-execution"
            }
            ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming => {
                "paged-streaming-frontier-read-execution"
            }
            ForgeQueryGraphReadAccessAdmissionPosture::PagedStreamingRequired
            | ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired
            | ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired
            | ForgeQueryGraphReadAccessAdmissionPosture::StoreBackedCapabilityRequired
            | ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired
            | ForgeQueryGraphReadAccessAdmissionPosture::Denied => return None,
        }
        .to_string();
        let ephemeral_index_plan = ForgeQueryEphemeralGraphIndexPlan::from_admission(&admission);
        let streaming_plan = ForgeQueryGraphReadStreamingPlan::from_admission(&admission);
        let digest = hash_parts(&[
            "forge_query_admitted_graph_read_access_plan_v1".to_string(),
            format!("admission:{}", admission.digest()),
            format!("posture:{}", admission.posture().as_str()),
            format!("strategy:{execution_strategy}"),
            format!(
                "ephemeral_index_plan:{}",
                ephemeral_index_plan
                    .as_ref()
                    .map(ForgeQueryEphemeralGraphIndexPlan::digest)
                    .unwrap_or("none")
            ),
            format!(
                "streaming_plan:{}",
                streaming_plan
                    .as_ref()
                    .map(ForgeQueryGraphReadStreamingPlan::digest)
                    .unwrap_or("none")
            ),
        ]);
        Some(Self {
            digest,
            admission,
            execution_strategy,
            ephemeral_index_plan,
            streaming_plan,
        })
    }
}
