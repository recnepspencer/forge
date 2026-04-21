mod tests;

use crate::application::ForgeQueryApplicationFacade;
use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, GuidedAuthoringPath,
    OrderingSelector, RootEntityKey, ScalarPredicateValue,
};
use crate::basis::{
    resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode, ExecutionBasisIntent,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::composition::{
    GuidedCompositionPath, QueryScopeDescriptor, QueryTemplateDescriptor, TemplateBindingSet,
    TemplateParameterSlot,
};
use crate::harness::certification::{
    digest_parts, CanonicalCertificationRow, CertificationMatrix, HostileExpectation, ParityAnchor,
    RejectionCertificationRow,
};
use crate::identity::{BasisDigest, CanonicalQueryDigest};
use crate::identity_evolution::{
    admit_identity_evolution_query_for_scenario, execute_admitted_identity_evolution_query,
    CorrespondenceIdentityComparison, IdentityEvolutionCertificationResultEvidence,
    IdentityEvolutionComparisonBasisFamily, IdentityEvolutionQueryContext,
    IdentityEvolutionSyntheticScenario, InspectorIdentityArtifact, InspectorIdentityClassification,
    LineageTraversalDescriptor,
};
use crate::saved_query::{
    evaluate_saved_query_reuse, freeze_composed_saved_query, freeze_direct_saved_query,
    SavedQueryFreezeContext, SavedQueryPersistenceClaim, SavedQueryReuseDescriptor,
    SavedQueryReuseOutcome,
};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor,
};
use crate::view_shape_live::{
    admit_grouped_live_view, execute_grouped_live_view_shape_change,
    execute_live_view_shape_change, lower_view_shape_plan_to_live,
    materialize_authoritative_grouped_baseline,
    materialize_grouped_execution_surface_from_truth_view,
};
use forge_relational::facade::grouped_truth::{
    materialize_relational_authoritative_row_set, project_relational_grouped_truth,
    GroupedProjectionContract as RelationalGroupedProjectionContract,
};
use forge_runtime_bridge::facade::{
    materialize_bridge_grouped_truth_view_from_projection, materialize_bridge_row_set,
    BridgeDeliveryReceipt, BridgeGroupedTruthViewArtifact, BridgeMappingId,
    BridgeMappingRegistration, BridgeRuntimePolicy, BridgeSignalInvalidationDelivery,
    BridgeSnapshotReadError, BridgeSourceAdapter, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeTruthViewSelector, CoarseRoutingMode, CommittedPatchSource,
    InvalidationSink, MappingSelector, RawCommittedPatchEnvelope, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    SnapshotReadRequest, SnapshotReadSource, SourceDeclaration, SourceDeclarationIdentity,
    TruthBranchHeadSource, TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity,
    TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
};

pub const MILESTONE_EIGHT_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "direct-vs-scope-parity",
    "direct-vs-template-parity",
    "saved-query-freeze-parity",
    "view-shape-non-cosmetic-planning-live",
    "kanban-desired-state-to-delta-parity",
    "kanban-delta-admission-boundary",
    "grouped-refresh-honesty",
    "grouped-bridge-truth-view-authority",
    "grouped-query-execution-surface-authority",
    "grouped-proof-chain-no-payload-rediscovery",
    "inspector-observed-focused-distinction",
    "identity-aware-focused-inspector-parity",
    "identity-break-inspector-explicitness",
    "support-profile-honesty",
];

pub const MILESTONE_EIGHT_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "unsupported-scope-family",
    "unsupported-template-family",
    "saved-query-support-profile-drift",
    "durable-saved-query-deferred-debt",
    "post-admission-view-mutation-forbidden",
    "grouped-hidden-refresh-forbidden",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MilestoneEightPerturbationClass {
    ScopeTemplateDirectParity,
    DirectScopeParity,
    DirectTemplateParity,
    SavedQueryFreezeParity,
    ViewShapePlanningLiveSemantics,
    KanbanDesiredStateDeltaParity,
    KanbanDeltaAdmissionBoundary,
    GroupedRefreshHonesty,
    GroupedBridgeTruthViewAuthority,
    GroupedExecutionSurfaceAuthority,
    GroupedProofChainNoPayloadRediscovery,
    InspectorSemanticDistinction,
    IdentityAwareInspectorParity,
    IdentityBreakInspectorExplicitness,
    SupportProfileHonesty,
    UnsupportedScopeFamily,
    UnsupportedTemplateFamily,
    SavedQuerySupportProfileDrift,
    DurableSavedQueryDeferredDebt,
    PostAdmissionViewMutationForbidden,
    GroupedHiddenRefreshForbidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MilestoneEightFailureClass {
    UnsupportedScopeFamily,
    UnsupportedTemplateFamily,
    SavedQuerySupportProfileDrift,
    DurableSavedQueryDeferredDebt,
    PostAdmissionViewMutationForbidden,
    GroupedHiddenRefreshForbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneEightCertificationBundle {
    pub query_digest: String,
    pub plan_digest: String,
    pub result_shape_digest: String,
    pub delivery_digest: String,
    pub counter_snapshot_digest: String,
    pub artifact_binding_matrix_digest: String,
    pub support_profile_digest: String,
    pub identity_consumption_digest: String,
    pub inspector_identity_digest: String,
    pub inspector_identity_classification: String,
}

impl MilestoneEightCertificationBundle {
    fn has_required_outputs(&self) -> bool {
        !self.query_digest.is_empty()
            && !self.plan_digest.is_empty()
            && !self.result_shape_digest.is_empty()
            && !self.delivery_digest.is_empty()
            && !self.counter_snapshot_digest.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneEightRejectionBundle {
    pub failure_class: MilestoneEightFailureClass,
    pub failure_digest: String,
    pub counter_snapshot_digest: String,
}

pub type MilestoneEightCertificationRow =
    CanonicalCertificationRow<MilestoneEightPerturbationClass, MilestoneEightCertificationBundle>;
pub type MilestoneEightRejectionRow = RejectionCertificationRow<
    MilestoneEightPerturbationClass,
    MilestoneEightCertificationBundle,
    MilestoneEightRejectionBundle,
>;
pub type MilestoneEightCertificationMatrix = CertificationMatrix<
    MilestoneEightPerturbationClass,
    MilestoneEightCertificationBundle,
    MilestoneEightRejectionBundle,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneEightCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub matrix: MilestoneEightCertificationMatrix,
}

impl MilestoneEightCertificationMatrix {
    pub fn into_milestone_eight_artifact(self) -> MilestoneEightCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        MilestoneEightCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            matrix: self,
        }
    }
}

pub struct MilestoneEightCertificationAdapter;

impl MilestoneEightCertificationAdapter {
    pub fn scope_template_view_shape_semantic_parity_certification_artifact(
    ) -> MilestoneEightCertificationArtifact {
        Self::scope_template_view_shape_semantic_parity_test().into_milestone_eight_artifact()
    }

    pub fn scope_template_view_shape_semantic_parity_test() -> MilestoneEightCertificationMatrix {
        MilestoneEightCertificationMatrix {
            suite_name: "Scope / Template / View-Shape Semantic Parity Test",
            rows: canonical_rows(),
            rejection_rows: rejection_rows(),
        }
    }
}

fn detail_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "milestone-eight-detail",
        [
            crate::schema_view::SchemaFieldView::new(
                "identity",
                "id",
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                "profile",
                "display_name",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                "status",
                "lane",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn collection_schema_view() -> crate::schema_view::QuerySchemaView {
    detail_schema_view()
}

fn basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

fn runtime_basis(
    schema_basis: crate::identity::SchemaBasisDigest,
) -> crate::basis::ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        basis_intent(),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            None,
            "snapshot-a",
            schema_basis,
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap()
}

fn detail_query_with_name_filter(name: &str) -> crate::authoring::DetailAuthoredQuery {
    crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .where_equal(
            EqualityPredicate::new(
                "profile",
                "display_name",
                ScalarPredicateValue::String(name.to_string()),
            )
            .unwrap(),
        )
        .build()
        .unwrap()
}

fn detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

fn collection_query() -> crate::authoring::CollectionAuthoredQuery {
    crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("status", "lane").unwrap())
        .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
        .build()
        .unwrap()
}

fn collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
        .build()
        .unwrap()
}

fn direct_detail_canonical(name: &str) -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_detail(detail_query_with_name_filter(name), detail_shape())
        .unwrap()
}

fn direct_collection_canonical() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(collection_query(), collection_shape()).unwrap()
}

fn view_plan(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
    schema_view: crate::schema_view::QuerySchemaView,
    descriptor: ViewShapeDescriptor,
) -> crate::view_shape::ViewShapePlanArtifact {
    let admitted = admit_view_shape(canonical, descriptor).unwrap();
    let validated =
        validate_canonical_bundle_for_admitted_view_shape(canonical, schema_view, admitted)
            .unwrap();
    plan_admitted_view_shape(validated, basis_intent()).unwrap()
}

type GroupedRowFixture = (String, String, String);

#[derive(Clone)]
struct StaticSource {
    rows: std::sync::Arc<Vec<GroupedRowFixture>>,
}

impl CommittedPatchSource for StaticSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("analysis"),
            vec![],
        ))
    }
}

#[derive(Clone)]
struct StaticSnapshotReader {
    rows: std::sync::Arc<Vec<GroupedRowFixture>>,
}

impl TruthSnapshotReader for StaticSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("snapshot-a")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        let rows = self
            .rows
            .iter()
            .map(|(member_key, display_name, lane)| {
                (
                    format!("result:{member_key}"),
                    member_key,
                    display_name,
                    lane,
                )
            })
            .collect::<Vec<_>>();
        Ok(SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            request
                .reads()
                .iter()
                .map(|read| {
                    let payload = rows
                        .iter()
                        .find_map(|(entity_identity, member_key, display_name, lane)| {
                            (read.entity_identity() == entity_identity.as_str()).then(|| match read
                                .aspect_label()
                            {
                                "identity.id" => member_key.as_bytes().to_vec(),
                                "profile.display_name" => display_name.as_bytes().to_vec(),
                                "status.lane" => lane.as_bytes().to_vec(),
                                _ => b"unknown".to_vec(),
                            })
                        })
                        .unwrap_or_else(|| b"unknown".to_vec());
                    SnapshotReadRecord::new(read.request_key(), payload)
                })
                .collect(),
        ))
    }
}

impl SnapshotReadSource for StaticSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(StaticSnapshotReader {
                rows: self.rows.clone(),
            }))
        } else {
            Err(RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

impl TruthBranchHeadSource for StaticSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
            TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
            TruthSnapshotIdentity::new("snapshot-a"),
            branch_identity.clone(),
            vec![],
        ))
    }
}

#[derive(Clone)]
struct StaticSourceAdapter {
    rows: std::sync::Arc<Vec<GroupedRowFixture>>,
}

impl BridgeSourceAdapter for StaticSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ])
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(StaticSnapshotReader {
                rows: self.rows.clone(),
            }))
        } else {
            Err(RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

struct StaticSink;

impl InvalidationSink for StaticSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn grouped_rows_packet(rows: &[GroupedRowFixture]) -> SnapshotReadPacket {
    SnapshotReadPacket::new(
        rows.iter()
            .flat_map(|(member_key, _, _)| {
                let entity = format!("result:{member_key}");
                [
                    SnapshotReadRequest::for_coarse(entity.clone(), "identity.id"),
                    SnapshotReadRequest::for_coarse(entity.clone(), "profile.display_name"),
                    SnapshotReadRequest::for_coarse(entity, "status.lane"),
                ]
            })
            .collect(),
    )
}

fn grouped_rows_result(
    rows: &[GroupedRowFixture],
    packet: &SnapshotReadPacket,
) -> SnapshotReadPacketResult {
    SnapshotReadPacketResult::new(
        TruthSnapshotIdentity::new("snapshot-a"),
        packet
            .reads()
            .iter()
            .map(|read| {
                let payload = rows
                    .iter()
                    .find_map(|(member_key, display_name, lane)| {
                        (read.entity_identity() == format!("result:{member_key}")).then(|| {
                            match read.aspect_label() {
                                "identity.id" => member_key.as_bytes().to_vec(),
                                "profile.display_name" => display_name.as_bytes().to_vec(),
                                "status.lane" => lane.as_bytes().to_vec(),
                                _ => b"unknown".to_vec(),
                            }
                        })
                    })
                    .unwrap_or_else(|| b"unknown".to_vec());
                SnapshotReadRecord::new(read.request_key(), payload)
            })
            .collect(),
    )
}

fn grouped_runtime(rows: &[GroupedRowFixture]) -> RuntimeBridge {
    let rows = std::sync::Arc::new(rows.to_vec());
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(StaticSource { rows: rows.clone() })
        .with_source_adapter(StaticSourceAdapter { rows: rows.clone() })
        .with_truth_branch_head_source(StaticSource { rows })
        .with_signal_sink(StaticSink)
        .register_source(grouped_registered_source())
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("result:task-1"),
                MappingSelector::exact("status"),
                MappingSelector::exact("lane"),
            ),
            SignalInvalidationScope::new("signal:board"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build for grouped certification")
}

fn grouped_registered_source() -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new("source:grouped-board"),
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ]),
    )
}

fn grouped_truth_view_for_plan(
    plan: &crate::view_shape::ViewShapePlanArtifact,
) -> BridgeGroupedTruthViewArtifact {
    grouped_truth_view_for_plan_with_rows(
        plan,
        &[
            ("task-1".to_string(), "Ada".to_string(), "todo".to_string()),
            ("task-2".to_string(), "Bea".to_string(), "doing".to_string()),
        ],
    )
}

fn grouped_truth_view_for_plan_with_rows(
    plan: &crate::view_shape::ViewShapePlanArtifact,
    rows: &[GroupedRowFixture],
) -> BridgeGroupedTruthViewArtifact {
    let runtime = grouped_runtime(rows);
    let contract = runtime
        .admit_source(grouped_registered_source())
        .expect("registered source should admit");
    let packet = grouped_rows_packet(rows);
    let observation = runtime
        .materialize_source_packet(&contract, packet.clone())
        .expect("grouped source packet should materialize");
    let row_set = materialize_bridge_row_set(&observation).expect("row set");
    let relational_result = grouped_rows_result(rows, &packet);
    let relational_row_set =
        materialize_relational_authoritative_row_set(&packet, &relational_result)
            .expect("relational row set");
    let grouping_field = match plan
        .grouped_planning_artifact()
        .expect("grouped plan should carry grouped planning")
        .grouping_aspect()
    {
        "status" => "status.lane",
        "profile" => "profile.display_name",
        other => other,
    };
    let relational_projection = project_relational_grouped_truth(
        &relational_row_set,
        RelationalGroupedProjectionContract::new(
            plan.grouped_planning_artifact()
                .expect("grouped plan should carry grouped planning")
                .grouping_aspect(),
            "identity.id",
            grouping_field,
        ),
    )
    .expect("relational grouped projection");

    materialize_bridge_grouped_truth_view_from_projection(&row_set, &relational_projection)
        .expect("grouped truth view")
}

fn detail_live_bundle(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> MilestoneEightCertificationBundle {
    let plan = view_plan(
        canonical,
        detail_schema_view(),
        ViewShapeDescriptor::detail(),
    );
    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &crate::live::BridgeChangeSummary::default().with_field_delta(
            crate::live::BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ),
        ),
    )
    .unwrap();
    bundle_from_view_execution(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        execution.patch_envelope().delivery_digest().to_string(),
        vec![
            format!(
                "view_patch_width:{}",
                execution.counters().view_patch_width()
            ),
            format!(
                "view_delivery_width:{}",
                execution.counters().view_delivery_width()
            ),
            format!(
                "focused_widening_denial:{}",
                execution
                    .counters()
                    .focused_inspector_widening_denial_count()
            ),
        ],
        "artifact:none".to_string(),
        "support:none".to_string(),
    )
}

fn direct_detail_bundle() -> MilestoneEightCertificationBundle {
    detail_live_bundle(&direct_detail_canonical("Alice"))
}

fn template_detail_bundle() -> MilestoneEightCertificationBundle {
    let predicate_slot = TemplateParameterSlot::predicate("name_filter");
    let template = QueryTemplateDescriptor::detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
    )
    .with_slot(predicate_slot.clone());
    let bindings = TemplateBindingSet::new().bind_predicate(
        &predicate_slot,
        crate::authoring::PredicateSelector::Equality(
            EqualityPredicate::new(
                "profile",
                "display_name",
                ScalarPredicateValue::String("Alice".to_string()),
            )
            .unwrap(),
        ),
    );
    let (_template_artifact, expanded_template) =
        GuidedCompositionPath::instantiate_detail_template(template, bindings).unwrap();
    let template_canonical =
        GuidedCompositionPath::canonicalize_expanded(expanded_template).unwrap();
    detail_live_bundle(template_canonical.canonical())
}

fn scope_detail_bundle() -> MilestoneEightCertificationBundle {
    let scope = QueryScopeDescriptor::predicate(
        "named_filter",
        [crate::authoring::PredicateSelector::Equality(
            EqualityPredicate::new(
                "profile",
                "display_name",
                ScalarPredicateValue::String("Alice".to_string()),
            )
            .unwrap(),
        )],
    );
    let (_scope_artifact, expanded_scope) = GuidedCompositionPath::expand_detail_scopes(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
        [scope],
    )
    .unwrap();
    let scope_canonical = GuidedCompositionPath::canonicalize_expanded(expanded_scope).unwrap();
    detail_live_bundle(scope_canonical.canonical())
}

fn table_live_bundle(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> MilestoneEightCertificationBundle {
    let plan = view_plan(
        canonical,
        collection_schema_view(),
        ViewShapeDescriptor::table(),
    );
    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &crate::live::BridgeChangeSummary::default().with_field_delta(
            crate::live::BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ),
        ),
    )
    .unwrap();
    bundle_from_view_execution(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        execution.patch_envelope().delivery_digest().to_string(),
        vec![
            format!(
                "view_patch_width:{}",
                execution.counters().view_patch_width()
            ),
            format!(
                "table_ordering_keys:{}",
                execution.counters().table_ordering_key_count()
            ),
        ],
        "artifact:none".to_string(),
        "support:none".to_string(),
    )
}

fn grouped_live_bundle(delta_bound: bool) -> MilestoneEightCertificationBundle {
    let canonical = direct_collection_canonical();
    let plan = view_plan(
        &canonical,
        collection_schema_view(),
        ViewShapeDescriptor::kanban_grouped("status"),
    );
    let basis = runtime_basis(plan.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view_for_plan(&plan);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&plan, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&plan, basis.clone(), &grouped_execution)
            .unwrap();
    let member_key = baseline.desired_state().result().member_states()[0]
        .member_key()
        .to_string();
    let live = lower_view_shape_plan_to_live(&plan, basis, Some(baseline), None).unwrap();
    let change = if delta_bound {
        crate::live::BridgeChangeSummary::default()
            .with_field_delta(crate::live::BridgeFieldDelta::new(
                "identity",
                "id",
                Some(member_key.as_str()),
                Some(member_key.as_str()),
            ))
            .with_field_delta(crate::live::BridgeFieldDelta::new(
                "status",
                "lane",
                Some("todo"),
                Some("doing"),
            ))
            .with_membership_transition(true, true)
    } else {
        let mut change = crate::live::BridgeChangeSummary::default();
        for _ in 0..128 {
            change = change.with_field_delta(crate::live::BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ));
        }
        change
    };
    let next_grouped_execution = if delta_bound {
        let next_truth_view = grouped_truth_view_for_plan_with_rows(
            &plan,
            &[
                ("task-1".to_string(), "Ada".to_string(), "doing".to_string()),
                ("task-2".to_string(), "Bea".to_string(), "doing".to_string()),
            ],
        );
        materialize_grouped_execution_surface_from_truth_view(
            &plan,
            live.basis().clone(),
            &next_truth_view,
        )
        .unwrap()
    } else {
        materialize_grouped_execution_surface_from_truth_view(
            &plan,
            live.basis().clone(),
            &truth_view,
        )
        .unwrap()
    };
    let execution = execute_grouped_live_view_shape_change(
        admit_grouped_live_view(&live).unwrap(),
        &change,
        &next_grouped_execution,
    )
    .unwrap();

    bundle_from_view_execution(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        execution.patch_envelope().delivery_digest().to_string(),
        vec![
            format!(
                "grouped_regroup_denial:{}",
                execution.counters().grouped_full_regroup_denial_count()
            ),
            format!(
                "grouped_refresh_admission:{}",
                execution.counters().view_family_refresh_admission_count()
            ),
            format!(
                "grouped_lane_count:{}",
                execution.counters().grouped_lane_count()
            ),
        ],
        grouped_execution.digest().to_string(),
        "support:none".to_string(),
    )
}

fn grouped_truth_view_bundle(rows: &[GroupedRowFixture]) -> MilestoneEightCertificationBundle {
    let canonical = direct_collection_canonical();
    let plan = view_plan(
        &canonical,
        collection_schema_view(),
        ViewShapeDescriptor::kanban_grouped("status"),
    );
    let truth_view = grouped_truth_view_for_plan_with_rows(&plan, rows);

    bundle_from_view_execution(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        truth_view.digest().as_str().to_string(),
        vec![
            format!("members:{}", truth_view.members().len()),
            format!("grouping:{}", truth_view.contract().grouping_aspect()),
            format!("truth_view_digest:{}", truth_view.truth_view_digest()),
        ],
        truth_view.digest().as_str().to_string(),
        "support:none".to_string(),
    )
}

fn grouped_execution_surface_bundle(
    rows: &[GroupedRowFixture],
) -> MilestoneEightCertificationBundle {
    let canonical = direct_collection_canonical();
    let plan = view_plan(
        &canonical,
        collection_schema_view(),
        ViewShapeDescriptor::kanban_grouped("status"),
    );
    let basis = runtime_basis(plan.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view_for_plan_with_rows(&plan, rows);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&plan, basis, &truth_view).unwrap();

    bundle_from_view_execution(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        grouped_execution.digest().to_string(),
        vec![
            format!("members:{}", grouped_execution.member_rows().len()),
            format!("truth_view:{}", grouped_execution.truth_view_digest()),
        ],
        grouped_execution.digest().to_string(),
        "support:none".to_string(),
    )
}

fn grouped_payload_rediscovery_free_bundle(
    rows: &[GroupedRowFixture],
) -> MilestoneEightCertificationBundle {
    let canonical = direct_collection_canonical();
    let plan = view_plan(
        &canonical,
        collection_schema_view(),
        ViewShapeDescriptor::kanban_grouped("status"),
    );
    let basis = runtime_basis(plan.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view_for_plan_with_rows(&plan, rows);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&plan, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&plan, basis, &grouped_execution).unwrap();

    bundle_from_view_execution(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        baseline.desired_state().digest().to_string(),
        vec![
            format!("members:{}", baseline.desired_state().result().row_count()),
            format!("truth_view:{}", truth_view.digest().as_str()),
            format!("grouped_execution:{}", grouped_execution.digest()),
            format!("baseline:{}", baseline.desired_state().digest()),
        ],
        grouped_execution.digest().to_string(),
        "support:none".to_string(),
    )
}

fn inspector_bundle(descriptor: ViewShapeDescriptor) -> MilestoneEightCertificationBundle {
    let canonical = GuidedAuthoringPath::canonicalize_detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
    )
    .unwrap();
    let plan = view_plan(&canonical, detail_schema_view(), descriptor);
    let bound_identity = match plan.delivery_metadata().identity_consumption() {
        crate::view_shape::ViewShapeIdentityConsumption::None => None,
        crate::view_shape::ViewShapeIdentityConsumption::InspectorIdentitySummary => {
            Some(inspector_identity_artifact_for_classification(
                InspectorIdentityClassification::IdentitySummary,
            ))
        }
        crate::view_shape::ViewShapeIdentityConsumption::FocusedInspectorIdentityClassification(
            classification,
        ) => Some(inspector_identity_artifact_for_classification(
            *classification,
        )),
    };
    let live = lower_view_shape_plan_to_live(
        &plan,
        runtime_basis(plan.validated().query().schema_basis().clone()),
        None,
        bound_identity,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &crate::live::BridgeChangeSummary::default().with_field_delta(
            crate::live::BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ),
        ),
    )
    .unwrap();
    let (inspector_identity_digest, inspector_identity_classification) = match execution
        .patch_envelope()
        .payload()
    {
        crate::view_shape_live::ViewShapePatchPayload::ObservedInspectorPatch(patch) => patch
            .inspector_identity()
            .map(|identity| {
                (
                    identity.digest().as_str().to_string(),
                    identity.classification().as_str().to_string(),
                )
            })
            .unwrap_or_else(|| ("none".to_string(), "none".to_string())),
        crate::view_shape_live::ViewShapePatchPayload::FocusedInspectorAspectPatch(patch) => patch
            .inspector_identity()
            .map(|identity| {
                (
                    identity.digest().as_str().to_string(),
                    identity.classification().as_str().to_string(),
                )
            })
            .unwrap_or_else(|| ("none".to_string(), "none".to_string())),
        _ => ("none".to_string(), "none".to_string()),
    };

    bundle_from_view_execution_with_identity(
        canonical.query().digest().as_str().to_string(),
        plan.view_plan_digest().as_str().to_string(),
        canonical.result_shape().digest().as_str().to_string(),
        execution.patch_envelope().delivery_digest().to_string(),
        vec![
            format!(
                "patch_family:{:?}",
                execution.patch_envelope().patch_family()
            ),
            format!(
                "observed_delivery_width:{}",
                execution.counters().observed_inspector_delivery_width()
            ),
            format!(
                "focused_projection_width:{}",
                execution.counters().focused_inspector_projection_width()
            ),
            format!(
                "identity_consumption:{}",
                plan.delivery_metadata().identity_consumption().as_str()
            ),
        ],
        "artifact:none".to_string(),
        "support:none".to_string(),
        plan.delivery_metadata()
            .identity_consumption()
            .digest()
            .as_str()
            .to_string(),
        inspector_identity_digest,
        inspector_identity_classification,
    )
}

fn identity_query_digest(label: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("milestone-eight:{label}")])
}

fn identity_basis_digest(label: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("milestone-eight:{label}")])
}

fn inspector_identity_artifact_for_classification(
    classification: InspectorIdentityClassification,
) -> InspectorIdentityArtifact {
    let (context, scenario) = match classification {
        InspectorIdentityClassification::IdentitySummary => (
            IdentityEvolutionQueryContext::lineage_traversal(
                identity_query_digest("identity-summary"),
                identity_basis_digest("identity-summary-basis"),
                LineageTraversalDescriptor::direct_split_successors("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::AuthoritativeContinuity => (
            IdentityEvolutionQueryContext::lineage_traversal(
                identity_query_digest("authoritative"),
                identity_basis_digest("authoritative-basis"),
                LineageTraversalDescriptor::direct_replacement("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::AdvisoryCandidates => (
            IdentityEvolutionQueryContext::correspondence_identity_comparison(
                identity_query_digest("advisory"),
                IdentityEvolutionComparisonBasisFamily::BranchToBranch,
                identity_basis_digest("left"),
                identity_basis_digest("right"),
                CorrespondenceIdentityComparison::advisory_between("left-id", "right-id"),
            ),
            IdentityEvolutionSyntheticScenario::Standard,
        ),
        InspectorIdentityClassification::IdentityBreak => (
            IdentityEvolutionQueryContext::lineage_traversal(
                identity_query_digest("identity-break"),
                identity_basis_digest("identity-break-basis"),
                LineageTraversalDescriptor::branch_local_direct_evolution("anchor"),
            ),
            IdentityEvolutionSyntheticScenario::IdentityBreak,
        ),
        other => panic!("milestone eight helper does not support '{other:?}'"),
    };
    let admitted = admit_identity_evolution_query_for_scenario(context, scenario)
        .expect("milestone eight identity artifact should admit");
    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("milestone eight identity artifact should execute");
    let evidence = IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&artifact);
    InspectorIdentityArtifact::from_result_evidence(&evidence)
}

fn saved_query_bundle(composed: bool) -> MilestoneEightCertificationBundle {
    let support_profile_digest =
        crate::composition::runtime_backed_query_composition_support_profile()
            .profile_digest()
            .to_string();
    if composed {
        let scope = QueryScopeDescriptor::predicate("noop", Vec::new());
        let (_artifact, expanded) = GuidedCompositionPath::expand_detail_scopes(
            detail_query_with_name_filter("Alice"),
            detail_shape(),
            [scope],
        )
        .unwrap();
        let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();
        let plan = view_plan(
            composed.canonical(),
            detail_schema_view(),
            ViewShapeDescriptor::detail(),
        );
        let saved = freeze_composed_saved_query(
            &composed,
            &plan,
            SavedQueryFreezeContext::new(&support_profile_digest, "query_composition"),
        )
        .unwrap();
        bundle_from_view_execution(
            saved
                .metadata()
                .canonical_query_digest()
                .as_str()
                .to_string(),
            plan.view_plan_digest().as_str().to_string(),
            saved
                .metadata()
                .canonical_result_shape_digest()
                .as_str()
                .to_string(),
            saved.digest().as_str().to_string(),
            vec![
                format!("template_slots:{}", saved.metadata().template_slot_count()),
                format!(
                    "composition:{}",
                    saved.metadata().composition_digest().as_str()
                ),
            ],
            saved.digest().as_str().to_string(),
            support_profile_digest,
        )
    } else {
        let canonical = direct_detail_canonical("Alice");
        let plan = view_plan(
            &canonical,
            detail_schema_view(),
            ViewShapeDescriptor::detail(),
        );
        let saved = freeze_direct_saved_query(
            &canonical,
            &plan,
            SavedQueryFreezeContext::new(&support_profile_digest, "query_direct"),
        )
        .unwrap();
        bundle_from_view_execution(
            saved
                .metadata()
                .canonical_query_digest()
                .as_str()
                .to_string(),
            plan.view_plan_digest().as_str().to_string(),
            saved
                .metadata()
                .canonical_result_shape_digest()
                .as_str()
                .to_string(),
            saved.digest().as_str().to_string(),
            vec![
                format!("template_slots:{}", saved.metadata().template_slot_count()),
                format!(
                    "composition:{}",
                    saved.metadata().composition_digest().as_str()
                ),
            ],
            saved.digest().as_str().to_string(),
            support_profile_digest,
        )
    }
}

fn support_profile_bundle(enabled: bool) -> MilestoneEightCertificationBundle {
    let facade = if enabled {
        ForgeQueryApplicationFacade::runtime_backed_default()
    } else {
        ForgeQueryApplicationFacade::new(
            crate::application::ForgeQueryConfig::runtime_backed_default()
                .with_query(crate::application::ForgeQueryQueryConfig::disabled())
                .with_signal(crate::application::ForgeQuerySignalConfig::disabled())
                .with_runtime_bridge(crate::application::ForgeQueryRuntimeBridgeConfig::disabled())
                .with_relational(crate::application::ForgeQueryRelationalConfig::disabled()),
        )
        .expect("disabled query config should still admit a support-report facade")
    };
    let report = facade.support_report();
    let composition_profile = report
        .query_composition_support_profile()
        .map(|profile| profile.profile_digest().to_string())
        .unwrap_or_else(|| "none".to_string());
    let identity_evolution_profile = report
        .identity_evolution_support_profile()
        .map(|profile| profile.profile_digest().to_string())
        .unwrap_or_else(|| "none".to_string());
    let query_context_profile = report
        .query_context_support_profile()
        .map(|profile| profile.profile_digest().to_string())
        .unwrap_or_else(|| "none".to_string());
    bundle_from_view_execution(
        report.report_digest().to_string(),
        report.support_matrix().support_matrix_digest().to_string(),
        report.validated_config_digest().to_string(),
        report.report_digest().to_string(),
        vec![
            format!("admitted:{}", report.admitted_capability_count()),
            format!("deferred:{}", report.deferred_capability_count()),
            format!("unsupported:{}", report.unsupported_capability_count()),
            format!("query_composition_profile:{composition_profile}"),
            format!("query_context_profile:{query_context_profile}"),
            format!("identity_evolution_profile:{identity_evolution_profile}"),
        ],
        digest_parts(&[
            report.support_matrix().support_matrix_digest().to_string(),
            composition_profile,
            query_context_profile,
            identity_evolution_profile,
        ]),
        report.report_digest().to_string(),
    )
}

fn durable_saved_query_deferred_rejection_bundle() -> MilestoneEightRejectionBundle {
    let canonical = direct_detail_canonical("Alice");
    let plan = view_plan(
        &canonical,
        detail_schema_view(),
        ViewShapeDescriptor::detail(),
    );
    let saved = freeze_direct_saved_query(
        &canonical,
        &plan,
        SavedQueryFreezeContext::new(
            crate::composition::runtime_backed_query_composition_support_profile().profile_digest(),
            "query_direct",
        ),
    )
    .unwrap();
    let error = saved
        .admit_persistence_claim(SavedQueryPersistenceClaim::DurableReload)
        .expect_err("durable reload should remain deferred debt in milestone eight");

    MilestoneEightRejectionBundle {
        failure_class: MilestoneEightFailureClass::DurableSavedQueryDeferredDebt,
        failure_digest: digest_parts(&[
            format!("{:?}", error.failure_class()),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&[format!(
            "durable_claim:{:?}",
            error.failure_class()
        )]),
    }
}

fn grouped_hidden_refresh_forbidden_rejection_bundle() -> MilestoneEightRejectionBundle {
    let canonical = direct_collection_canonical();
    let plan = view_plan(
        &canonical,
        collection_schema_view(),
        ViewShapeDescriptor::kanban_grouped("status"),
    );
    let basis = runtime_basis(plan.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view_for_plan(&plan);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&plan, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&plan, basis.clone(), &grouped_execution)
            .unwrap();
    let live = lower_view_shape_plan_to_live(&plan, basis, Some(baseline), None).unwrap();
    let error = execute_live_view_shape_change(
        &live,
        &crate::live::BridgeChangeSummary::default().with_field_delta(
            crate::live::BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ),
        ),
    )
    .expect_err("grouped hidden refresh should be forbidden on the ungrouped entrypoint");

    MilestoneEightRejectionBundle {
        failure_class: MilestoneEightFailureClass::GroupedHiddenRefreshForbidden,
        failure_digest: digest_parts(&[
            format!("{:?}", error.failure_class()),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&[format!(
            "grouped_hidden_refresh:{:?}",
            error.failure_class()
        )]),
    }
}

fn post_admission_view_mutation_forbidden_rejection_bundle() -> MilestoneEightRejectionBundle {
    let stderr = include_str!("../../../tests/ui/post_admission_view_mutation_forbidden.stderr");
    MilestoneEightRejectionBundle {
        failure_class: MilestoneEightFailureClass::PostAdmissionViewMutationForbidden,
        failure_digest: digest_parts(&[
            "compile_fail:tests/ui/post_admission_view_mutation_forbidden.rs".to_string(),
            stderr.to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&[
            "compile_fail:post_admission_view_mutation_forbidden".to_string(),
        ]),
    }
}

fn bundle_from_view_execution(
    query_digest: String,
    plan_digest: String,
    result_shape_digest: String,
    delivery_digest: String,
    counters: Vec<String>,
    artifact_binding_matrix_digest: String,
    support_profile_digest: String,
) -> MilestoneEightCertificationBundle {
    bundle_from_view_execution_with_identity(
        query_digest,
        plan_digest,
        result_shape_digest,
        delivery_digest,
        counters,
        artifact_binding_matrix_digest,
        support_profile_digest,
        String::new(),
        String::new(),
        String::new(),
    )
}

fn bundle_from_view_execution_with_identity(
    query_digest: String,
    plan_digest: String,
    result_shape_digest: String,
    delivery_digest: String,
    counters: Vec<String>,
    artifact_binding_matrix_digest: String,
    support_profile_digest: String,
    identity_consumption_digest: String,
    inspector_identity_digest: String,
    inspector_identity_classification: String,
) -> MilestoneEightCertificationBundle {
    let counter_snapshot_digest = digest_parts(&counters);
    MilestoneEightCertificationBundle {
        query_digest,
        plan_digest,
        result_shape_digest,
        delivery_digest,
        counter_snapshot_digest,
        artifact_binding_matrix_digest,
        support_profile_digest,
        identity_consumption_digest,
        inspector_identity_digest,
        inspector_identity_classification,
    }
}

fn canonical_rows() -> Vec<MilestoneEightCertificationRow> {
    let direct = direct_detail_bundle();
    let template_lane = template_detail_bundle();
    let scope_lane = scope_detail_bundle();
    let grouped_control_rows = &[
        ("task-1".to_string(), "Ada".to_string(), "todo".to_string()),
        ("task-2".to_string(), "Bea".to_string(), "doing".to_string()),
    ];
    let grouped_hostile_rows = &[
        ("task-1".to_string(), "Ada".to_string(), "doing".to_string()),
        ("task-3".to_string(), "Cy".to_string(), "todo".to_string()),
    ];

    vec![
        MilestoneEightCertificationRow {
            row_name: "direct-vs-scope-parity",
            perturbation_class: MilestoneEightPerturbationClass::DirectScopeParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: direct,
            hostile_lane: scope_lane.clone(),
            parity_lane: scope_lane,
        },
        MilestoneEightCertificationRow {
            row_name: "direct-vs-template-parity",
            perturbation_class: MilestoneEightPerturbationClass::DirectTemplateParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: direct_detail_bundle(),
            hostile_lane: template_lane.clone(),
            parity_lane: template_detail_bundle(),
        },
        MilestoneEightCertificationRow {
            row_name: "scope-template-direct-parity",
            perturbation_class: MilestoneEightPerturbationClass::ScopeTemplateDirectParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: direct_detail_bundle(),
            hostile_lane: template_lane,
            parity_lane: scope_detail_bundle(),
        },
        MilestoneEightCertificationRow {
            row_name: "saved-query-freeze-parity",
            perturbation_class: MilestoneEightPerturbationClass::SavedQueryFreezeParity,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: saved_query_bundle(false),
            hostile_lane: saved_query_bundle(true),
            parity_lane: saved_query_bundle(false),
        },
        MilestoneEightCertificationRow {
            row_name: "view-shape-non-cosmetic-planning-live",
            perturbation_class: MilestoneEightPerturbationClass::ViewShapePlanningLiveSemantics,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: table_live_bundle(&direct_collection_canonical()),
            hostile_lane: grouped_live_bundle(true),
            parity_lane: grouped_live_bundle(false),
        },
        MilestoneEightCertificationRow {
            row_name: "kanban-desired-state-to-delta-parity",
            perturbation_class: MilestoneEightPerturbationClass::KanbanDesiredStateDeltaParity,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: grouped_live_bundle(true),
            hostile_lane: grouped_live_bundle(false),
            parity_lane: grouped_live_bundle(true),
        },
        MilestoneEightCertificationRow {
            row_name: "kanban-delta-admission-boundary",
            perturbation_class: MilestoneEightPerturbationClass::KanbanDeltaAdmissionBoundary,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: grouped_live_bundle(true),
            hostile_lane: grouped_live_bundle(false),
            parity_lane: grouped_live_bundle(false),
        },
        MilestoneEightCertificationRow {
            row_name: "grouped-refresh-honesty",
            perturbation_class: MilestoneEightPerturbationClass::GroupedRefreshHonesty,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: grouped_live_bundle(true),
            hostile_lane: grouped_live_bundle(false),
            parity_lane: grouped_live_bundle(false),
        },
        MilestoneEightCertificationRow {
            row_name: "grouped-bridge-truth-view-authority",
            perturbation_class: MilestoneEightPerturbationClass::GroupedBridgeTruthViewAuthority,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: grouped_truth_view_bundle(grouped_control_rows),
            hostile_lane: grouped_truth_view_bundle(grouped_hostile_rows),
            parity_lane: grouped_truth_view_bundle(grouped_control_rows),
        },
        MilestoneEightCertificationRow {
            row_name: "grouped-query-execution-surface-authority",
            perturbation_class: MilestoneEightPerturbationClass::GroupedExecutionSurfaceAuthority,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: grouped_execution_surface_bundle(grouped_control_rows),
            hostile_lane: grouped_execution_surface_bundle(grouped_hostile_rows),
            parity_lane: grouped_execution_surface_bundle(grouped_control_rows),
        },
        MilestoneEightCertificationRow {
            row_name: "grouped-proof-chain-no-payload-rediscovery",
            perturbation_class:
                MilestoneEightPerturbationClass::GroupedProofChainNoPayloadRediscovery,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: grouped_payload_rediscovery_free_bundle(grouped_control_rows),
            hostile_lane: grouped_payload_rediscovery_free_bundle(grouped_hostile_rows),
            parity_lane: grouped_payload_rediscovery_free_bundle(grouped_control_rows),
        },
        MilestoneEightCertificationRow {
            row_name: "inspector-observed-focused-distinction",
            perturbation_class: MilestoneEightPerturbationClass::InspectorSemanticDistinction,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: inspector_bundle(ViewShapeDescriptor::inspector_detail_observed()),
            hostile_lane: inspector_bundle(ViewShapeDescriptor::inspector_detail_focused(
                "profile",
            )),
            parity_lane: inspector_bundle(ViewShapeDescriptor::inspector_detail_focused("profile")),
        },
        MilestoneEightCertificationRow {
            row_name: "identity-aware-focused-inspector-parity",
            perturbation_class: MilestoneEightPerturbationClass::IdentityAwareInspectorParity,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: inspector_bundle(
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::AuthoritativeContinuity,
                ),
            ),
            hostile_lane: inspector_bundle(
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::AdvisoryCandidates,
                ),
            ),
            parity_lane: inspector_bundle(
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::AuthoritativeContinuity,
                ),
            ),
        },
        MilestoneEightCertificationRow {
            row_name: "identity-break-inspector-explicitness",
            perturbation_class: MilestoneEightPerturbationClass::IdentityBreakInspectorExplicitness,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: inspector_bundle(
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::AuthoritativeContinuity,
                ),
            ),
            hostile_lane: inspector_bundle(
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::IdentityBreak,
                ),
            ),
            parity_lane: inspector_bundle(
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::IdentityBreak,
                ),
            ),
        },
        MilestoneEightCertificationRow {
            row_name: "support-profile-honesty",
            perturbation_class: MilestoneEightPerturbationClass::SupportProfileHonesty,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: support_profile_bundle(true),
            hostile_lane: support_profile_bundle(false),
            parity_lane: support_profile_bundle(true),
        },
    ]
}

fn rejection_rows() -> Vec<MilestoneEightRejectionRow> {
    let control_lane = detail_live_bundle(&direct_detail_canonical("Alice"));
    let saved_control = saved_query_bundle(false);

    let unsupported_scope = GuidedCompositionPath::expand_detail_scopes(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        detail_shape(),
        [QueryScopeDescriptor::unsupported_for_test("nope")],
    )
    .expect_err("unsupported scope should deny");
    let unsupported_template = GuidedCompositionPath::instantiate_detail_template(
        QueryTemplateDescriptor::observed_inspector_deferred_for_test(
            crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
                .project(AspectFieldSelector::new("identity", "id").unwrap())
                .project(AspectFieldSelector::new("profile", "display_name").unwrap())
                .build()
                .unwrap(),
            detail_shape(),
        ),
        TemplateBindingSet::new(),
    )
    .expect_err("unsupported template family should deny");

    let canonical = direct_detail_canonical("Alice");
    let plan = view_plan(
        &canonical,
        detail_schema_view(),
        ViewShapeDescriptor::detail(),
    );
    let saved = freeze_direct_saved_query(
        &canonical,
        &plan,
        SavedQueryFreezeContext::new(
            crate::composition::runtime_backed_query_composition_support_profile().profile_digest(),
            "query_direct",
        ),
    )
    .unwrap();
    let descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        saved.metadata().basis_family().cloned(),
        saved.metadata().template_binding_digest().cloned(),
        saved.metadata().template_slot_count(),
        saved.metadata().view_shape_digest().clone(),
        saved.metadata().view_shape_family(),
        saved.metadata().result_shape_family().clone(),
        saved.metadata().composition_digest().clone(),
        saved.metadata().scope_lineage_digest().cloned(),
        "different-support",
        saved.metadata().capability_family_identity().to_string(),
    );
    let saved_drift = evaluate_saved_query_reuse(&saved, &descriptor);
    let SavedQueryReuseOutcome::Denied(saved_denial) = saved_drift else {
        panic!("saved query support drift should deny");
    };

    vec![
        MilestoneEightRejectionRow {
            row_name: "unsupported-scope-family",
            perturbation_class: MilestoneEightPerturbationClass::UnsupportedScopeFamily,
            control_lane: control_lane.clone(),
            hostile_lane: MilestoneEightRejectionBundle {
                failure_class: MilestoneEightFailureClass::UnsupportedScopeFamily,
                failure_digest: digest_parts(&[
                    format!("{:?}", unsupported_scope.failure_class()),
                    unsupported_scope.message().to_string(),
                ]),
                counter_snapshot_digest: digest_parts(&[format!(
                    "scope_denial:{:?}",
                    unsupported_scope.failure_class()
                )]),
            },
            parity_lane: control_lane.clone(),
        },
        MilestoneEightRejectionRow {
            row_name: "unsupported-template-family",
            perturbation_class: MilestoneEightPerturbationClass::UnsupportedTemplateFamily,
            control_lane: control_lane.clone(),
            hostile_lane: MilestoneEightRejectionBundle {
                failure_class: MilestoneEightFailureClass::UnsupportedTemplateFamily,
                failure_digest: digest_parts(&[
                    format!("{:?}", unsupported_template.failure_class()),
                    unsupported_template.message().to_string(),
                ]),
                counter_snapshot_digest: digest_parts(&[format!(
                    "template_denial:{:?}",
                    unsupported_template.failure_class()
                )]),
            },
            parity_lane: control_lane,
        },
        MilestoneEightRejectionRow {
            row_name: "saved-query-support-profile-drift",
            perturbation_class: MilestoneEightPerturbationClass::SavedQuerySupportProfileDrift,
            control_lane: saved_control.clone(),
            hostile_lane: MilestoneEightRejectionBundle {
                failure_class: MilestoneEightFailureClass::SavedQuerySupportProfileDrift,
                failure_digest: digest_parts(&[
                    format!("{:?}", saved_denial.failure_class()),
                    format!("{:?}", saved_denial.overall()),
                ]),
                counter_snapshot_digest: digest_parts(
                    &saved_denial
                        .matrix()
                        .rows()
                        .iter()
                        .map(|row| format!("{:?}:{:?}", row.dimension(), row.legality()))
                        .collect::<Vec<_>>(),
                ),
            },
            parity_lane: saved_control,
        },
        MilestoneEightRejectionRow {
            row_name: "durable-saved-query-deferred-debt",
            perturbation_class: MilestoneEightPerturbationClass::DurableSavedQueryDeferredDebt,
            control_lane: saved_query_bundle(false),
            hostile_lane: durable_saved_query_deferred_rejection_bundle(),
            parity_lane: saved_query_bundle(false),
        },
        MilestoneEightRejectionRow {
            row_name: "post-admission-view-mutation-forbidden",
            perturbation_class: MilestoneEightPerturbationClass::PostAdmissionViewMutationForbidden,
            control_lane: detail_live_bundle(&direct_detail_canonical("Alice")),
            hostile_lane: post_admission_view_mutation_forbidden_rejection_bundle(),
            parity_lane: detail_live_bundle(&direct_detail_canonical("Alice")),
        },
        MilestoneEightRejectionRow {
            row_name: "grouped-hidden-refresh-forbidden",
            perturbation_class: MilestoneEightPerturbationClass::GroupedHiddenRefreshForbidden,
            control_lane: grouped_live_bundle(true),
            hostile_lane: grouped_hidden_refresh_forbidden_rejection_bundle(),
            parity_lane: grouped_live_bundle(true),
        },
    ]
}

fn bundle_digest_parts(matrix: &MilestoneEightCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("row:{}", row.row_name));
        parts.push(format!(
            "control:{}:{}:{}:{}:{}:{}:{}:{}",
            row.control_lane.query_digest,
            row.control_lane.plan_digest,
            row.control_lane.result_shape_digest,
            row.control_lane.delivery_digest,
            row.control_lane.counter_snapshot_digest,
            row.control_lane.identity_consumption_digest,
            row.control_lane.inspector_identity_digest,
            row.control_lane.inspector_identity_classification,
        ));
        parts.push(format!(
            "hostile:{}:{}:{}:{}:{}:{}:{}:{}",
            row.hostile_lane.query_digest,
            row.hostile_lane.plan_digest,
            row.hostile_lane.result_shape_digest,
            row.hostile_lane.delivery_digest,
            row.hostile_lane.counter_snapshot_digest,
            row.hostile_lane.identity_consumption_digest,
            row.hostile_lane.inspector_identity_digest,
            row.hostile_lane.inspector_identity_classification,
        ));
        parts.push(format!(
            "parity:{}:{}:{}:{}:{}:{}:{}:{}",
            row.parity_lane.query_digest,
            row.parity_lane.plan_digest,
            row.parity_lane.result_shape_digest,
            row.parity_lane.delivery_digest,
            row.parity_lane.counter_snapshot_digest,
            row.parity_lane.identity_consumption_digest,
            row.parity_lane.inspector_identity_digest,
            row.parity_lane.inspector_identity_classification,
        ));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection:{}", row.row_name));
        parts.push(format!(
            "hostile:{}:{}",
            match row.hostile_lane.failure_class {
                MilestoneEightFailureClass::UnsupportedScopeFamily => "unsupported_scope_family",
                MilestoneEightFailureClass::UnsupportedTemplateFamily =>
                    "unsupported_template_family",
                MilestoneEightFailureClass::SavedQuerySupportProfileDrift => {
                    "saved_query_support_profile_drift"
                }
                MilestoneEightFailureClass::DurableSavedQueryDeferredDebt => {
                    "durable_saved_query_deferred_debt"
                }
                MilestoneEightFailureClass::PostAdmissionViewMutationForbidden => {
                    "post_admission_view_mutation_forbidden"
                }
                MilestoneEightFailureClass::GroupedHiddenRefreshForbidden => {
                    "grouped_hidden_refresh_forbidden"
                }
            },
            row.hostile_lane.failure_digest
        ));
    }
    parts
}

fn coverage_digest_parts(matrix: &MilestoneEightCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("row:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}
