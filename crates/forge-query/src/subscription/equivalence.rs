use crate::identity::hash_parts;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

use super::family::QuerySubscriptionFamily;
use super::future_selection::QuerySubscriptionFutureSelection;
use super::input::LiveQueryAdmissionArtifact;
use super::posture::{
    QuerySubscriptionBasisPosture, QuerySubscriptionBridgePosture, QuerySubscriptionCostPosture,
};
use super::selection::FamilyClassification;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QuerySubscriptionMeaningDigest(String);

impl QuerySubscriptionMeaningDigest {
    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionEquivalenceBasis {
    digest: QuerySubscriptionMeaningDigest,
    digest_part_count: usize,
    family: QuerySubscriptionFamily,
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    future_selection: QuerySubscriptionFutureSelection,
    cost_posture: QuerySubscriptionCostPosture,
    basis_posture: QuerySubscriptionBasisPosture,
    bridge_posture: QuerySubscriptionBridgePosture,
}

impl QuerySubscriptionEquivalenceBasis {
    pub(super) fn new(
        input: &LiveQueryAdmissionArtifact,
        classification: &FamilyClassification,
    ) -> Self {
        let mut parts = vec![
            "query_subscription_equivalence_v1".to_string(),
            format!("family:{}", classification.family.as_str()),
            format!("live_family:{}", input.live_family.as_str()),
            format!(
                "view_family:{}",
                input
                    .view_family
                    .map(|family| family.as_str())
                    .unwrap_or("none")
            ),
            format!(
                "future_selection:{}",
                input.future_selection.projection_digest()
            ),
            format!("basis:{}", input.basis_posture.as_str()),
            format!("cost:{}", classification.cost_posture.as_str()),
            format!("bridge:{}", classification.bridge_posture.as_str()),
            format!("query:{}", input.query_digest),
            format!("plan:{}", input.plan_digest),
            format!(
                "collection:{}",
                input.collection_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "policy:{}",
                input.policy_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "tenant:{}",
                input.tenant_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "relationship_proof:{}",
                input.relationship_proof_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "relationship_proof_posture:{}",
                input.relationship_proof_posture.as_str()
            ),
            format!("relevance:{}", input.relevance_digest),
            format!("delivery_intent:{}", input.delivery_intent_digest),
            format!("authorized_width:{}", input.authorized_projection_width),
            format!("ordering_width:{}", input.ordering_width),
            format!("grouping_width:{}", input.grouping_width),
            format!("relation_scope_width:{}", input.relation_scope_width),
            format!("view_metadata_width:{}", input.view_shape_metadata_width),
        ];
        parts.sort();
        let digest = QuerySubscriptionMeaningDigest::from_parts(&parts);
        Self {
            digest,
            digest_part_count: parts.len(),
            family: classification.family.clone(),
            live_family: input.live_family.clone(),
            view_family: input.view_family,
            future_selection: input.future_selection.clone(),
            cost_posture: classification.cost_posture.clone(),
            basis_posture: input.basis_posture.clone(),
            bridge_posture: classification.bridge_posture.clone(),
        }
    }

    pub fn digest(&self) -> &QuerySubscriptionMeaningDigest {
        &self.digest
    }

    pub fn digest_part_count(&self) -> usize {
        self.digest_part_count
    }

    pub fn family(&self) -> &QuerySubscriptionFamily {
        &self.family
    }

    pub fn live_family(&self) -> &LiveQueryFamily {
        &self.live_family
    }

    pub fn view_family(&self) -> Option<LiveViewShapeFamily> {
        self.view_family
    }

    pub fn cost_posture(&self) -> &QuerySubscriptionCostPosture {
        &self.cost_posture
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_posture(&self) -> &QuerySubscriptionBasisPosture {
        &self.basis_posture
    }

    pub fn bridge_posture(&self) -> &QuerySubscriptionBridgePosture {
        &self.bridge_posture
    }
}
