use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

use super::construction_source::QuerySubscriptionConstructionSource;
use super::future_selection::QuerySubscriptionFutureSelection;
use super::input::LiveQueryAdmissionArtifact;
use super::posture::QuerySubscriptionBasisPosture;
use super::relationship_proof::QuerySubscriptionRelationshipProofPosture;

impl LiveQueryAdmissionArtifact {
    pub(crate) fn for_test(
        live_family: LiveQueryFamily,
        view_family: Option<LiveViewShapeFamily>,
        construction_source: QuerySubscriptionConstructionSource,
    ) -> Self {
        Self::for_test_with_basis(
            live_family,
            view_family,
            construction_source,
            QuerySubscriptionBasisPosture::CurrentHead,
        )
    }

    pub(crate) fn for_test_with_basis(
        live_family: LiveQueryFamily,
        view_family: Option<LiveViewShapeFamily>,
        construction_source: QuerySubscriptionConstructionSource,
        basis_posture: QuerySubscriptionBasisPosture,
    ) -> Self {
        Self::for_test_with_context(
            live_family,
            view_family,
            construction_source,
            basis_posture,
            QuerySubscriptionFutureSelection::ordinary(),
            Some("policy".to_string()),
            Some("tenant".to_string()),
            Some("relationship-proof".to_string()),
            QuerySubscriptionRelationshipProofPosture::Admitted,
        )
    }

    pub(crate) fn for_test_with_context(
        live_family: LiveQueryFamily,
        view_family: Option<LiveViewShapeFamily>,
        construction_source: QuerySubscriptionConstructionSource,
        basis_posture: QuerySubscriptionBasisPosture,
        future_selection: QuerySubscriptionFutureSelection,
        policy_digest: Option<String>,
        tenant_digest: Option<String>,
        relationship_proof_digest: Option<String>,
        relationship_proof_posture: QuerySubscriptionRelationshipProofPosture,
    ) -> Self {
        let (
            authorized_projection_width,
            ordering_width,
            grouping_width,
            relation_scope_width,
            view_shape_metadata_width,
        ) = match (live_family.clone(), view_family) {
            (_, Some(LiveViewShapeFamily::Table)) => (2, 1, 0, 0, 0),
            (_, Some(LiveViewShapeFamily::KanbanGrouped)) => (2, 1, 1, 0, 1),
            (_, Some(LiveViewShapeFamily::Detail)) => (2, 0, 0, 0, 0),
            (_, Some(LiveViewShapeFamily::InspectorDetailObserved))
            | (_, Some(LiveViewShapeFamily::InspectorDetailFocused)) => (2, 0, 0, 0, 1),
            (LiveQueryFamily::Detail, None) => (2, 0, 0, 0, 0),
            (LiveQueryFamily::OrderedCollection, None) => (2, 1, 0, 0, 0),
            (LiveQueryFamily::BoundedMaterialization, None) => (2, 1, 0, 1, 0),
        };
        Self {
            live_family,
            query_digest: "query-digest".to_string(),
            plan_digest: "plan-digest".to_string(),
            collection_digest: Some("collection-digest".to_string()),
            view_family,
            basis_posture,
            future_selection,
            policy_digest,
            tenant_digest,
            relationship_proof_digest,
            relationship_proof_posture,
            relevance_digest: "relevance".to_string(),
            delivery_intent_digest: "delivery".to_string(),
            authorized_projection_width,
            ordering_width,
            grouping_width,
            relation_scope_width,
            view_shape_metadata_width,
            construction_source,
        }
    }

    pub(crate) fn for_test_grouped_with_missing_grouping_width() -> Self {
        let mut artifact = Self::for_test(
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::KanbanGrouped),
            QuerySubscriptionConstructionSource::FacadeLive,
        );
        artifact.grouping_width = 0;
        artifact
    }

    pub(crate) fn for_test_with_relationship_proof_posture(
        live_family: LiveQueryFamily,
        view_family: Option<LiveViewShapeFamily>,
        construction_source: QuerySubscriptionConstructionSource,
        relationship_proof_posture: QuerySubscriptionRelationshipProofPosture,
    ) -> Self {
        Self::for_test_with_context(
            live_family,
            view_family,
            construction_source,
            QuerySubscriptionBasisPosture::CurrentHead,
            QuerySubscriptionFutureSelection::ordinary(),
            Some("policy".to_string()),
            Some("tenant".to_string()),
            Some("relationship-proof".to_string()),
            relationship_proof_posture,
        )
    }

    pub(crate) fn for_test_with_future_selection(
        live_family: LiveQueryFamily,
        view_family: Option<LiveViewShapeFamily>,
        construction_source: QuerySubscriptionConstructionSource,
        future_selection: QuerySubscriptionFutureSelection,
    ) -> Self {
        Self::for_test_with_context(
            live_family,
            view_family,
            construction_source,
            QuerySubscriptionBasisPosture::CurrentHead,
            future_selection,
            Some("policy".to_string()),
            Some("tenant".to_string()),
            Some("relationship-proof".to_string()),
            QuerySubscriptionRelationshipProofPosture::Admitted,
        )
    }
}
