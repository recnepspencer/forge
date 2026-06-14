use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::live::{LivePromotionDescriptor, LiveQueryFamily};
use crate::view_shape_live::LiveViewShapeFamily;

use super::construction_source::QuerySubscriptionConstructionSource;
use super::dimensions::QuerySubscriptionAdmissionDimensions;
use super::evidence_identities::{
    diagnostic_source_identity, live_delivery_intent_projection_identity, live_relevance_identity,
};
use super::future_selection::QuerySubscriptionFutureSelection;
use super::posture::QuerySubscriptionBasisPosture;
use super::relationship_proof::QuerySubscriptionRelationshipProofPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveQueryAdmissionArtifact {
    pub(super) live_family: LiveQueryFamily,
    pub(super) query_digest: String,
    pub(super) plan_digest: String,
    pub(super) query_identity: ForgeQueryEvidenceIdentity,
    pub(super) plan_identity: ForgeQueryEvidenceIdentity,
    pub(super) collection_digest: Option<String>,
    pub(super) view_family: Option<LiveViewShapeFamily>,
    pub(super) basis_posture: QuerySubscriptionBasisPosture,
    pub(super) future_selection: QuerySubscriptionFutureSelection,
    pub(super) policy_digest: Option<String>,
    pub(super) tenant_digest: Option<String>,
    pub(super) relationship_proof_digest: Option<String>,
    pub(super) relationship_proof_posture: QuerySubscriptionRelationshipProofPosture,
    pub(super) relevance_identity: ForgeQueryEvidenceIdentity,
    pub(super) delivery_intent_identity: ForgeQueryEvidenceIdentity,
    pub(super) diagnostic_source_identity: ForgeQueryEvidenceIdentity,
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
        let query_identity = descriptor.query_digest().evidence_identity().clone();
        let plan_identity = descriptor.plan_digest().evidence_identity().clone();
        let query_digest = query_identity.as_str().to_string();
        let plan_digest = plan_identity.as_str().to_string();
        let relevance_identity = live_relevance_identity(
            descriptor.family(),
            &query_identity,
            &plan_identity,
        );
        let delivery_intent_identity =
            live_delivery_intent_projection_identity(descriptor.family());
        let mut artifact = Self {
            live_family: descriptor.family().clone(),
            query_digest,
            plan_digest,
            query_identity,
            plan_identity,
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
            relevance_identity,
            delivery_intent_identity,
            diagnostic_source_identity: ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
            )
            .seal(),
            authorized_projection_width: dimensions.authorized_projection_width,
            ordering_width: dimensions.ordering_width,
            grouping_width: dimensions.grouping_width,
            relation_scope_width: dimensions.relation_scope_width,
            view_shape_metadata_width: dimensions.view_shape_metadata_width,
            construction_source,
        };
        artifact.diagnostic_source_identity = diagnostic_source_identity(&artifact);
        artifact
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

    pub fn query_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_identity
    }

    pub(super) fn plan_digest(&self) -> &str {
        &self.plan_digest
    }

    pub(super) fn plan_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.plan_identity
    }

    pub(super) fn collection_digest(&self) -> Option<&str> {
        self.collection_digest.as_deref()
    }

    pub(super) fn ordering_width(&self) -> usize {
        self.ordering_width
    }

    pub(super) fn grouping_width(&self) -> usize {
        self.grouping_width
    }

    pub(super) fn relation_scope_width(&self) -> usize {
        self.relation_scope_width
    }

    pub(super) fn relationship_proof_posture(&self) -> &QuerySubscriptionRelationshipProofPosture {
        &self.relationship_proof_posture
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

    pub fn relevance_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.relevance_identity
    }

    pub fn relevance_for_reporting(&self) -> &str {
        self.relevance_identity.as_str()
    }

    pub fn delivery_intent_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.delivery_intent_identity
    }

    pub fn delivery_intent_for_reporting(&self) -> &str {
        self.delivery_intent_identity.as_str()
    }

    pub fn diagnostic_source_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.diagnostic_source_identity
    }

    pub fn diagnostic_source_for_reporting(&self) -> &str {
        self.diagnostic_source_identity.as_str()
    }

    pub fn authorized_projection_width(&self) -> usize {
        self.authorized_projection_width
    }

    pub fn view_shape_metadata_width(&self) -> usize {
        self.view_shape_metadata_width
    }
}
