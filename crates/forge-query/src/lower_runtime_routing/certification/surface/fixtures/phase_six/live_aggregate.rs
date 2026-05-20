use std::num::NonZeroUsize;

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};
use crate::declarative_live::{
    declare_runtime_live_query_session_with_grouped_baseline, DeclarativeLiveQueryRequest,
    DeclarativeLiveViewShape, DeclarativeProjectionField,
};
use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
    LiveViewDeclarationAdmissionBoundaryReceipt, SubscriptionActivationBoundaryReceipt,
};
use crate::runtime::{
    ForgeQueryRuntimeLiveSubscriptionInstallation, LiveViewDeclarationAdmissionReceipt,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use crate::subscription::{
    admit_active_subscription_lane, admit_query_subscription, attach_subscription_consumer,
    declare_query_subscription, lower_query_subscription_to_bridge, open_active_subscription_lane,
    prepare_subscription_activation, select_query_subscription_family, ActiveAllocationScopeWidth,
    ActiveFanoutWidth, ActiveRegistryLookupWidth, ActiveSubscriptionAllocationPosture,
    ActiveSubscriptionRuntime, ActiveSubscriptionWorkBudget, ConsumerDeliveryPacingWidth,
    DeliveryBackpressurePolicy, QuerySubscriptionAdmissionBudget,
    QuerySubscriptionAdmissionDimensions, QuerySubscriptionBasisPosture,
    QuerySubscriptionBridgeLoweringBudget, QuerySubscriptionSliceBudget,
    QuerySubscriptionWorkBudget, SubscriptionActivationInput, SubscriptionConsumerAttachmentBudget,
    SubscriptionConsumerAttachmentRequest,
};

const LIVE_AGGREGATE_VIEW_NAME: &str = "tasks.live-aggregate";
const RUNTIME_SUBSCRIPTION_FAMILY_BUDGET_POLICY: &str =
    "runtime-live-subscription-family:scratch_buffer_only:canonical=64:relationship=64:policy=64:projection=512:tenant=1";
const RUNTIME_SUBSCRIPTION_SLICE_BUDGET_POLICY: &str =
    "runtime-live-subscription-slice:scratch_buffer_only:all-widths=64";
const RUNTIME_SUBSCRIPTION_BRIDGE_BUDGET_POLICY: &str =
    "runtime-live-subscription-bridge:admitted:bridge=1:slice=64:policy=64:basis=64:signal=64";
const RUNTIME_SUBSCRIPTION_ADMISSION_BUDGET_POLICY: &str =
    "runtime-live-subscription-admission:admitted:all-widths=64";
const RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY: &str =
    "runtime-live-active-lifecycle:registry=1:fanout=1:allocation=1:lifecycle_arena";
const RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY: &str =
    "runtime-live-consumer-attachment:fanout=1:pacing=1:allocation=1:retain_within_window";

pub(crate) fn representative_public_live_view_declaration_row() -> RepresentativeArtifacts {
    let fixture = live_aggregate_fixture();
    let subject_digest = hash_parts(&[
        fixture
            .declaration_boundary
            .readmission_receipt()
            .eligibility()
            .request()
            .request_digest()
            .to_string(),
        fixture.installation.installation_digest().to_string(),
    ]);
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "Public live view declaration",
        subject_digest,
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        hash_parts(&[
            fixture
                .declaration_boundary
                .readmission_receipt()
                .eligibility()
                .eligibility_digest()
                .to_string(),
            fixture.installation.installation_digest().to_string(),
        ]),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        hash_parts(&[
            "public-live-view-declaration".to_string(),
            fixture
                .declaration_boundary
                .boundary_execution_receipt()
                .boundary_execution_digest()
                .to_string(),
            fixture.installation.installation_digest().to_string(),
        ]),
    );
    let retained_evidence = hash_parts(&[
        fixture
            .declaration_boundary
            .boundary_envelope()
            .envelope_digest()
            .to_string(),
        fixture.installation.installation_digest().to_string(),
    ]);
    let boundary_receipt =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence,
    );
    let envelope =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
            &route_plan,
            &boundary_receipt,
            &retained_evidence,
        );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_runtime_live_installation_orchestration_row() -> RepresentativeArtifacts
{
    let fixture = live_aggregate_fixture();
    let subject_digest = hash_parts(&[
        fixture
            .activation_boundary
            .route_plan()
            .eligibility()
            .request()
            .request_digest()
            .to_string(),
        fixture.installation.installation_digest().to_string(),
    ]);
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::RuntimeLiveInstallationOrchestration,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "Runtime live installation orchestration",
        subject_digest,
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        hash_parts(&[
            fixture
                .activation_boundary
                .route_plan()
                .route_digest()
                .to_string(),
            fixture.installation.installation_digest().to_string(),
        ]),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        hash_parts(&[
            "runtime-live-installation-orchestration".to_string(),
            fixture.installation.installation_digest().to_string(),
            fixture
                .activation_boundary
                .boundary_envelope()
                .envelope_digest()
                .to_string(),
        ]),
    );
    let retained_evidence = hash_parts(&[
        fixture.installation.installation_digest().to_string(),
        fixture
            .activation_boundary
            .boundary_execution_receipt()
            .boundary_execution_digest()
            .to_string(),
    ]);
    let boundary_receipt =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence,
    );
    let envelope =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
            ForgeQueryLowerRuntimeSeamKey::RuntimeLiveInstallationOrchestration,
            &route_plan,
            &boundary_receipt,
            &retained_evidence,
        );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::RuntimeLiveInstallationOrchestration,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

struct LiveAggregateFixture {
    declaration_boundary: LiveViewDeclarationAdmissionBoundaryReceipt,
    activation_boundary: SubscriptionActivationBoundaryReceipt,
    installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
}

fn live_aggregate_fixture() -> LiveAggregateFixture {
    let request = DeclarativeLiveQueryRequest::new("Task", DeclarativeLiveViewShape::table())
        .project(DeclarativeProjectionField::new("identity", "id"))
        .project(DeclarativeProjectionField::new("status", "value"));
    let schema_view = QuerySchemaView::new(
        "certification-live-aggregate",
        [
            SchemaFieldView::new("identity", "id", SchemaFieldKind::String),
            SchemaFieldView::new("status", "value", SchemaFieldKind::String),
        ],
        [],
    );
    let declaration_receipt =
        LiveViewDeclarationAdmissionReceipt::from_request(LIVE_AGGREGATE_VIEW_NAME, &request);
    let declaration_boundary = LiveViewDeclarationAdmissionBoundaryReceipt::from_request(
        LIVE_AGGREGATE_VIEW_NAME,
        &request,
        declaration_receipt,
    );
    let session = declare_runtime_live_query_session_with_grouped_baseline(
        request.clone(),
        schema_view,
        "live-aggregate-snapshot",
        None::<Vec<(String, String)>>,
    )
    .expect("live aggregate fixture should declare a runtime live session");
    let live_admission =
        crate::subscription::LiveQueryAdmissionArtifact::from_live_promotion_with_view(
            session.live_view().core_live_plan().descriptor(),
            QuerySubscriptionBasisPosture::CurrentHead,
            session.live_view().lowering().family(),
            QuerySubscriptionAdmissionDimensions::collection_membership(
                NonZeroUsize::new(2).expect("projection width"),
                NonZeroUsize::new(1).expect("ordering width"),
            ),
        );
    let selection = select_query_subscription_family(live_admission, work_budget())
        .expect("live aggregate fixture should select a subscription family");
    let subscription_family = selection.family().as_str().to_string();
    let declaration = declare_query_subscription(selection, slice_budget())
        .expect("live aggregate fixture should declare a subscription");
    let subscription_declaration_digest = declaration.declaration_digest().as_str().to_string();
    let lowering = lower_query_subscription_to_bridge(declaration, lowering_budget())
        .expect("live aggregate fixture should lower a subscription");
    let admission = admit_query_subscription(lowering, admission_budget())
        .expect("live aggregate fixture should admit the lowered subscription");
    let bridge_declaration_digest = admission.bridge_declaration_digest().to_string();
    let admission_digest = admission.admission_digest().to_string();
    let basis_binding_digest = admission.basis_binding_digest().to_string();
    let signal_strategy_digest = admission.signal_strategy_digest().to_string();
    let activation = prepare_subscription_activation(admission);
    let activation_receipt = crate::runtime::SubscriptionActivationReceipt::from_activation(
        LIVE_AGGREGATE_VIEW_NAME,
        &activation,
        "certified-live-aggregate-support",
    );
    let support_evidence = activation_receipt.support_evidence().to_string();
    let activation_digest = activation_receipt.activation_digest().to_string();
    let activation_boundary = SubscriptionActivationBoundaryReceipt::from_activation(
        LIVE_AGGREGATE_VIEW_NAME,
        &activation,
        activation_receipt,
    );
    let (attachment, active_lane_counters, consumer_attachment_counters) =
        installation_attachment(&activation);
    let active_lane_digest = attachment.lane_digest().as_str().to_string();
    let installation = ForgeQueryRuntimeLiveSubscriptionInstallation::new(
        LIVE_AGGREGATE_VIEW_NAME,
        session.canonical().query().digest().as_str(),
        session.live_view().lowering().digest(),
        subscription_family,
        subscription_declaration_digest,
        bridge_declaration_digest,
        admission_digest,
        activation_digest,
        basis_binding_digest,
        signal_strategy_digest,
        active_lane_digest,
        &attachment,
        runtime_subscription_budget_policy(),
        RUNTIME_ACTIVE_LIFECYCLE_BUDGET_POLICY,
        RUNTIME_CONSUMER_ATTACHMENT_BUDGET_POLICY,
        active_lane_counters,
        consumer_attachment_counters,
        support_evidence,
        activation.counters().clone(),
    );

    LiveAggregateFixture {
        declaration_boundary,
        activation_boundary,
        installation,
    }
}

fn installation_attachment(
    activation: &SubscriptionActivationInput,
) -> (
    crate::subscription::SubscriptionConsumerAttachment,
    crate::subscription::ActiveSubscriptionCounters,
    crate::subscription::ActiveSubscriptionCounters,
) {
    let mut active_runtime = ActiveSubscriptionRuntime::new();
    let active_lane_admission =
        admit_active_subscription_lane(activation.clone(), active_lifecycle_budget())
            .expect("live aggregate fixture should admit an active lane");
    let handle = open_active_subscription_lane(&mut active_runtime, active_lane_admission)
        .expect("live aggregate fixture should open an active lane");
    let active_lane_counters = active_runtime.counters().clone();
    let attachment = attach_subscription_consumer(
        &mut active_runtime,
        &handle,
        SubscriptionConsumerAttachmentRequest::admitted(
            "runtime-live-aggregate",
            activation.activation_digest(),
        ),
        consumer_attachment_budget(),
    )
    .expect("live aggregate fixture should attach a subscription consumer");
    let consumer_attachment_counters = active_runtime.counters().clone();
    (
        attachment,
        active_lane_counters,
        consumer_attachment_counters,
    )
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

fn active_lifecycle_budget() -> ActiveSubscriptionWorkBudget {
    ActiveSubscriptionWorkBudget::admitted(
        ActiveRegistryLookupWidth::measured(1),
        ActiveFanoutWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        ActiveSubscriptionAllocationPosture::LifecycleArena,
    )
}

fn consumer_attachment_budget() -> SubscriptionConsumerAttachmentBudget {
    SubscriptionConsumerAttachmentBudget::admitted(
        ActiveFanoutWidth::measured(1),
        ConsumerDeliveryPacingWidth::measured(1),
        ActiveAllocationScopeWidth::measured(1),
        DeliveryBackpressurePolicy::RetainWithinWindow,
    )
}

fn runtime_subscription_budget_policy() -> String {
    [
        RUNTIME_SUBSCRIPTION_FAMILY_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_SLICE_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_BRIDGE_BUDGET_POLICY,
        RUNTIME_SUBSCRIPTION_ADMISSION_BUDGET_POLICY,
    ]
    .join("|")
}
