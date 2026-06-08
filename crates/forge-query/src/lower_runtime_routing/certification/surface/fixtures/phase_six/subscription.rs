use std::num::NonZeroUsize;

use crate::declarative_live::{
    declare_runtime_live_query_session_with_grouped_baseline, DeclarativeLiveQueryRequest,
    DeclarativeLiveViewShape, DeclarativeProjectionField,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeSeamKey, SubscriptionActivationBoundaryReceipt,
};
use crate::runtime::SubscriptionActivationReceipt;
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use crate::subscription::{
    admit_query_subscription, declare_query_subscription, lower_query_subscription_to_bridge,
    prepare_subscription_activation, select_query_subscription_family, LiveQueryAdmissionArtifact,
    QuerySubscriptionAdmissionBudget, QuerySubscriptionAdmissionDimensions,
    QuerySubscriptionBasisPosture, QuerySubscriptionBridgeLoweringBudget,
    QuerySubscriptionSliceBudget, QuerySubscriptionWorkBudget,
};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};

pub(crate) fn representative_subscription_activation_row() -> RepresentativeArtifacts {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id"))
        .project(DeclarativeProjectionField::new("status", "value"));
    let schema_view = QuerySchemaView::new(
        "certification-subscription-activation",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [],
    );
    let session = declare_runtime_live_query_session_with_grouped_baseline(
        request.clone(),
        schema_view,
        "subscription-snapshot",
        None::<Vec<(String, String)>>,
    )
    .expect("subscription activation fixture should declare live session");
    let live = LiveQueryAdmissionArtifact::from_live_promotion_with_view(
        session.live_view().core_live_plan().descriptor(),
        QuerySubscriptionBasisPosture::CurrentHead,
        session.live_view().lowering().family(),
        QuerySubscriptionAdmissionDimensions::collection_membership(
            NonZeroUsize::new(2).expect("projection width"),
            NonZeroUsize::new(1).expect("ordering width"),
        ),
    );
    let selection = select_query_subscription_family(live, work_budget())
        .expect("subscription activation fixture should select family");
    let declaration = declare_query_subscription(selection, slice_budget())
        .expect("subscription activation fixture should declare subscription");
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget())
        .expect("subscription activation fixture should lower to bridge");
    let admission = admit_query_subscription(lowering, admission_budget())
        .expect("subscription activation fixture should admit");
    let activation = prepare_subscription_activation(admission);
    let activation_receipt = SubscriptionActivationReceipt::from_activation(
        "tasks.subscription",
        &activation,
        "certified-subscription-activation-support",
        None,
    );
    let boundary = SubscriptionActivationBoundaryReceipt::from_activation(
        "tasks.subscription",
        &activation,
        activation_receipt,
    );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
        request: boundary.route_plan().eligibility().request().clone(),
        eligibility: boundary.route_plan().eligibility().clone(),
        route_plan: Some(boundary.route_plan().clone()),
        boundary_receipt: boundary.boundary_execution_receipt().clone(),
        envelope: boundary.boundary_envelope().clone(),
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

fn work_budget() -> QuerySubscriptionWorkBudget {
    QuerySubscriptionWorkBudget::scratch_buffer_only(64, 64, 64, 512, 1)
}

fn slice_budget() -> QuerySubscriptionSliceBudget {
    QuerySubscriptionSliceBudget::scratch_buffer_only(64, 64, 64, 64, 64, 64, 64, 64)
}

fn lowering_budget() -> QuerySubscriptionBridgeLoweringBudget {
    QuerySubscriptionBridgeLoweringBudget::admitted(1, 64, 64, 64, 64)
}

fn admission_budget() -> QuerySubscriptionAdmissionBudget {
    QuerySubscriptionAdmissionBudget::admitted(64, 64, 64, 64, 64)
}
