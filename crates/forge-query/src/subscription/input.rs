use crate::identity::hash_parts;
use crate::live::{LivePromotionDescriptor, LiveQueryFamily};
use crate::view_shape_live::LiveViewShapeFamily;

use super::construction_source::QuerySubscriptionConstructionSource;
use super::dimensions::QuerySubscriptionAdmissionDimensions;
use super::future_selection::QuerySubscriptionFutureSelection;
use super::posture::QuerySubscriptionBasisPosture;
use super::relationship_proof::QuerySubscriptionRelationshipProofPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveQueryAdmissionArtifact {
    pub(super) live_family: LiveQueryFamily,
    pub(super) query_digest: String,
    pub(super) plan_digest: String,
    pub(super) collection_digest: Option<String>,
    pub(super) view_family: Option<LiveViewShapeFamily>,
    pub(super) basis_posture: QuerySubscriptionBasisPosture,
    pub(super) future_selection: QuerySubscriptionFutureSelection,
    pub(super) policy_digest: Option<String>,
    pub(super) tenant_digest: Option<String>,
    pub(super) relationship_proof_digest: Option<String>,
    pub(super) relationship_proof_posture: QuerySubscriptionRelationshipProofPosture,
    pub(super) relevance_digest: String,
    pub(super) delivery_intent_digest: String,
    pub(super) authorized_projection_width: usize,
    pub(super) ordering_width: usize,
    pub(super) grouping_width: usize,
    pub(super) relation_scope_width: usize,
    pub(super) view_shape_metadata_width: usize,
    pub(super) construction_source: QuerySubscriptionConstructionSource,
}

impl LiveQueryAdmissionArtifact {
    pub fn from_live_promotion(
        descriptor: &LivePromotionDescriptor,
        basis_posture: QuerySubscriptionBasisPosture,
        dimensions: QuerySubscriptionAdmissionDimensions,
    ) -> Self {
        Self::from_live_promotion_with_future_selection(
            descriptor,
            basis_posture,
            QuerySubscriptionFutureSelection::ordinary(),
            dimensions,
        )
    }

    pub fn from_live_promotion_with_future_selection(
        descriptor: &LivePromotionDescriptor,
        basis_posture: QuerySubscriptionBasisPosture,
        future_selection: QuerySubscriptionFutureSelection,
        dimensions: QuerySubscriptionAdmissionDimensions,
    ) -> Self {
        Self::from_promotion_parts(
            descriptor,
            basis_posture,
            None,
            dimensions,
            QuerySubscriptionConstructionSource::FacadeLive,
            future_selection,
            None,
            None,
            None,
        )
    }

    pub fn from_live_promotion_with_view(
        descriptor: &LivePromotionDescriptor,
        basis_posture: QuerySubscriptionBasisPosture,
        view_family: LiveViewShapeFamily,
        dimensions: QuerySubscriptionAdmissionDimensions,
    ) -> Self {
        Self::from_live_promotion_with_view_and_future_selection(
            descriptor,
            basis_posture,
            view_family,
            QuerySubscriptionFutureSelection::ordinary(),
            dimensions,
        )
    }

    pub fn from_live_promotion_with_view_and_future_selection(
        descriptor: &LivePromotionDescriptor,
        basis_posture: QuerySubscriptionBasisPosture,
        view_family: LiveViewShapeFamily,
        future_selection: QuerySubscriptionFutureSelection,
        dimensions: QuerySubscriptionAdmissionDimensions,
    ) -> Self {
        Self::from_promotion_parts(
            descriptor,
            basis_posture,
            Some(view_family),
            dimensions,
            QuerySubscriptionConstructionSource::FacadeLive,
            future_selection,
            None,
            None,
            None,
        )
    }

    fn from_promotion_parts(
        descriptor: &LivePromotionDescriptor,
        basis_posture: QuerySubscriptionBasisPosture,
        view_family: Option<LiveViewShapeFamily>,
        dimensions: QuerySubscriptionAdmissionDimensions,
        construction_source: QuerySubscriptionConstructionSource,
        future_selection: QuerySubscriptionFutureSelection,
        policy_digest: Option<String>,
        tenant_digest: Option<String>,
        relationship_proof_digest: Option<String>,
    ) -> Self {
        let relationship_proof_posture = relationship_proof_digest
            .as_ref()
            .map(|_| QuerySubscriptionRelationshipProofPosture::Admitted)
            .unwrap_or(QuerySubscriptionRelationshipProofPosture::NotRequired);
        Self {
            live_family: descriptor.family().clone(),
            query_digest: descriptor.query_digest().as_str().to_string(),
            plan_digest: descriptor.plan_digest().as_str().to_string(),
            collection_digest: descriptor
                .collection_digest()
                .map(|digest| digest.as_str().to_string()),
            view_family,
            basis_posture,
            future_selection,
            policy_digest,
            tenant_digest,
            relationship_proof_digest,
            relationship_proof_posture,
            relevance_digest: hash_parts(&[
                "live_relevance".to_string(),
                descriptor.family().as_str().to_string(),
                descriptor.query_digest().as_str().to_string(),
                descriptor.plan_digest().as_str().to_string(),
            ]),
            delivery_intent_digest: hash_parts(&[
                "live_delivery_intent".to_string(),
                descriptor.family().as_str().to_string(),
            ]),
            authorized_projection_width: dimensions.authorized_projection_width,
            ordering_width: dimensions.ordering_width,
            grouping_width: dimensions.grouping_width,
            relation_scope_width: dimensions.relation_scope_width,
            view_shape_metadata_width: dimensions.view_shape_metadata_width,
            construction_source,
        }
    }

    pub fn live_family(&self) -> &LiveQueryFamily {
        &self.live_family
    }

    pub fn view_family(&self) -> Option<LiveViewShapeFamily> {
        self.view_family
    }

    pub fn basis_posture(&self) -> &QuerySubscriptionBasisPosture {
        &self.basis_posture
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn construction_source(&self) -> &QuerySubscriptionConstructionSource {
        &self.construction_source
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn policy_digest(&self) -> Option<&str> {
        self.policy_digest.as_deref()
    }

    pub fn tenant_digest(&self) -> Option<&str> {
        self.tenant_digest.as_deref()
    }

    pub fn relationship_proof_digest(&self) -> Option<&str> {
        self.relationship_proof_digest.as_deref()
    }

    pub fn authorized_projection_width(&self) -> usize {
        self.authorized_projection_width
    }

    pub fn view_shape_metadata_width(&self) -> usize {
        self.view_shape_metadata_width
    }

    pub(super) fn diagnostic_source_digest(&self) -> String {
        hash_parts(&[
            "query_subscription_live_admission_source_v1".to_string(),
            format!("live_family:{}", self.live_family.as_str()),
            format!("query:{}", self.query_digest),
            format!("plan:{}", self.plan_digest),
            format!(
                "collection:{}",
                self.collection_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "view_family:{}",
                self.view_family
                    .map(|family| family.as_str())
                    .unwrap_or("none")
            ),
            format!("basis:{}", self.basis_posture.as_str()),
            format!(
                "future_selection:{}",
                self.future_selection.projection_digest()
            ),
            format!("policy:{}", self.policy_digest.as_deref().unwrap_or("none")),
            format!("tenant:{}", self.tenant_digest.as_deref().unwrap_or("none")),
            format!(
                "relationship_proof:{}",
                self.relationship_proof_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "relationship_proof_posture:{}",
                self.relationship_proof_posture.as_str()
            ),
            format!("relevance:{}", self.relevance_digest),
            format!("delivery:{}", self.delivery_intent_digest),
            format!("projection_width:{}", self.authorized_projection_width),
            format!("ordering_width:{}", self.ordering_width),
            format!("grouping_width:{}", self.grouping_width),
            format!("relation_scope_width:{}", self.relation_scope_width),
            format!("metadata_width:{}", self.view_shape_metadata_width),
            format!("source:{}", self.construction_source.as_str()),
        ])
    }
}
