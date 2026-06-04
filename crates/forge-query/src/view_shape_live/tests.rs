use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, OrderingSelector,
    RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
};
use crate::basis::{
    resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode, ExecutionBasisIntent,
    ResolvedSnapshotIdentity, SnapshotLineageClass,
};
use crate::identity::{BasisDigest, CanonicalQueryDigest, SchemaBasisDigest};
use crate::identity_evolution::{
    admit_identity_evolution_query_for_scenario, execute_admitted_identity_evolution_query,
    CorrespondenceIdentityComparison, IdentityEvolutionCertificationResultEvidence,
    IdentityEvolutionComparisonBasisFamily, IdentityEvolutionQueryContext,
    IdentityEvolutionSyntheticScenario, InspectorIdentityArtifact, InspectorIdentityClassification,
    LineageTraversalDescriptor,
};
use crate::live::{BridgeChangeSummary, BridgeFieldDelta};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor,
};
use forge_foundational::facade::{
    AspectKey, AspectLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
    ScalarAspectType,
};
use forge_relational::facade::grouped_truth::{
    encode_snapshot_aspect_read_value, materialize_relational_authoritative_row_set,
    project_relational_grouped_truth,
    GroupedProjectionContract as RelationalGroupedProjectionContract,
};
use forge_runtime_bridge::facade::{
    materialize_bridge_grouped_truth_view_from_projection, materialize_bridge_row_set,
    AspectKeySelector, BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity,
    BridgeCommittedPatchItem, BridgeCommittedPatchTarget, BridgeDeliveryReceipt,
    BridgeGroupedTruthViewArtifact, BridgeMappingId, BridgeMappingRegistration,
    BridgeRuntimePolicy, BridgeSignalInvalidationDelivery, BridgeSnapshotReadError,
    BridgeSourceAdapter, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeTruthViewSelector, CoarseRoutingMode, CommittedPatchSource, InvalidationSink,
    MappingSelector, RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadContract,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadRequest,
    SnapshotReadSource, SourceDeclaration, SourceDeclarationIdentity, TruthBranchHeadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthPatchTargetSelector, TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::{
    admit_grouped_live_view, execute_grouped_live_view_shape_change,
    execute_live_view_shape_change, lower_view_shape_plan_to_live,
    materialize_authoritative_grouped_baseline,
    materialize_grouped_execution_surface_from_truth_view, ViewShapeLiveFailureClass,
    ViewShapePatchFamily, ViewShapePatchPayload, ViewShapeRefreshDisposition,
};

fn schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-live",
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
            ),
            crate::schema_view::SchemaFieldView::new(
                "status",
                "lane",
                crate::schema_view::SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn detail_canonical() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_detail(
        RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        RawAuthoredResultShape::detail_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn collection_canonical() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(
        RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        RawAuthoredResultShape::collection_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn runtime_basis(schema_basis: SchemaBasisDigest) -> crate::basis::ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
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

fn identity_query_digest(label: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("view-shape-live:{label}")])
}

fn identity_basis_digest(label: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("view-shape-live:{label}")])
}

fn inspector_identity_artifact(
    classification: InspectorIdentityClassification,
) -> InspectorIdentityArtifact {
    let (context, scenario) = match classification {
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
        other => panic!("test helper does not support inspector classification '{other:?}'"),
    };
    let admitted = admit_identity_evolution_query_for_scenario(context, scenario)
        .expect("identity evolution request should admit");
    let artifact = execute_admitted_identity_evolution_query(&admitted)
        .expect("identity evolution request should execute");
    let evidence = IdentityEvolutionCertificationResultEvidence::from_execution_artifact(&artifact);
    InspectorIdentityArtifact::from_result_evidence(&evidence)
}

fn planned_view(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
    descriptor: ViewShapeDescriptor,
) -> crate::view_shape::ViewShapePlanArtifact {
    let admitted = admit_view_shape(canonical, descriptor).unwrap();
    let validated =
        validate_canonical_bundle_for_admitted_view_shape(canonical, schema_view(), admitted)
            .unwrap();
    plan_admitted_view_shape(
        validated,
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
    )
    .unwrap()
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
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(native_grouped_patch_envelope(
            request.commit_identity().clone(),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("analysis"),
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
                                .aspect_key()
                                .as_str()
                            {
                                "identity.id" => AspectValue::String(member_key.as_str().into()),
                                "profile.display_name" => {
                                    AspectValue::String(display_name.as_str().into())
                                }
                                "status.lane" => AspectValue::String(lane.as_str().into()),
                                _ => AspectValue::String("unknown".into()),
                            })
                        })
                        .unwrap_or_else(|| AspectValue::String("unknown".into()));
                    SnapshotReadRecord::for_request(read, payload)
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
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(native_grouped_patch_envelope(
            TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
            TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
            TruthSnapshotIdentity::new("snapshot-a"),
            branch_identity.clone(),
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
                    string_snapshot_read(entity.clone(), "identity.id"),
                    string_snapshot_read(entity.clone(), "profile.display_name"),
                    string_snapshot_read(entity, "status.lane"),
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
                let value = rows
                    .iter()
                    .find_map(|(member_key, display_name, lane)| {
                        (read.entity_identity() == format!("result:{member_key}")).then(|| {
                            match read.aspect_key().as_str() {
                                "identity.id" => AspectValue::String(member_key.as_str().into()),
                                "profile.display_name" => {
                                    AspectValue::String(display_name.as_str().into())
                                }
                                "status.lane" => AspectValue::String(lane.as_str().into()),
                                _ => AspectValue::String("unknown".into()),
                            }
                        })
                    })
                    .unwrap_or_else(|| AspectValue::String("unknown".into()));
                SnapshotReadRecord::for_request(read, aspect_value(value))
            })
            .collect(),
    )
}

fn runtime(rows: &[GroupedRowFixture]) -> RuntimeBridge {
    let rows = std::sync::Arc::new(rows.to_vec());
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::default())
        .with_relational_source(StaticSource { rows: rows.clone() })
        .with_source_adapter(StaticSourceAdapter { rows: rows.clone() })
        .with_truth_branch_head_source(StaticSource { rows })
        .with_signal_sink(StaticSink)
        .register_source(registered_source())
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("result:task-1"),
                AspectKeySelector::exact(aspect_key("status")),
                TruthPatchTargetSelector::entity_field(field_key("lane")),
            ),
            SnapshotReadContract::scalar(aspect_key("status"), ScalarAspectType::String),
            SignalInvalidationScope::new("signal:board"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build for grouped truth-view tests")
}

fn registered_source() -> SourceDeclaration {
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

fn grouped_truth_view(
    plan: &crate::view_shape::ViewShapePlanArtifact,
) -> BridgeGroupedTruthViewArtifact {
    grouped_truth_view_with_rows(
        plan,
        &[
            ("task-1".to_string(), "Ada".to_string(), "todo".to_string()),
            ("task-2".to_string(), "Bea".to_string(), "doing".to_string()),
        ],
        "identity.id",
        None,
    )
}

fn grouped_truth_view_with_rows(
    plan: &crate::view_shape::ViewShapePlanArtifact,
    rows: &[GroupedRowFixture],
    identity_field: &str,
    grouping_field_override: Option<&str>,
) -> BridgeGroupedTruthViewArtifact {
    let runtime = runtime(rows);
    let contract = runtime
        .admit_source(registered_source())
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
        relational_grouped_projection_contract(
            plan.grouped_planning_artifact()
                .expect("grouped plan should carry grouped planning")
                .grouping_aspect(),
            identity_field,
            grouping_field_override.unwrap_or(grouping_field),
        ),
    )
    .expect("relational grouped projection");

    materialize_bridge_grouped_truth_view_from_projection(&row_set, &relational_projection)
        .expect("grouped truth view")
}

fn aspect_value(value: AspectValue) -> AspectValue {
    encode_snapshot_aspect_read_value(&value)
}

fn native_grouped_patch_envelope(
    commit_identity: TruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    branch_identity: TruthBranchIdentity,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new(
            commit_identity,
            patch_identity,
            snapshot_identity,
            branch_identity,
        ),
        vec![BridgeCommittedPatchItem::with_target(
            "result:task-1",
            BridgeCommittedPatchTarget::entity_field_path(
                AspectLocator::new(LocatorAuthority::Authoritative, aspect_key("status")),
                CanonicalFieldPath::single(field_key("lane")),
            ),
        )],
    )
    .expect("view-shape live native grouped patch envelope should construct")
}

fn string_snapshot_read(
    entity: impl Into<std::sync::Arc<str>>,
    aspect: &str,
) -> SnapshotReadRequest {
    SnapshotReadRequest::for_coarse(
        entity,
        SnapshotReadContract::scalar(aspect_key(aspect), ScalarAspectType::String),
    )
}

fn relational_grouped_projection_contract(
    grouping_aspect: &str,
    identity_binding_aspect: &str,
    grouping_binding_aspect: &str,
) -> RelationalGroupedProjectionContract {
    RelationalGroupedProjectionContract::new(
        aspect_key(grouping_aspect),
        aspect_key(identity_binding_aspect),
        aspect_key(grouping_binding_aspect),
    )
}

fn aspect_key(label: &str) -> AspectKey {
    AspectKey::new(label).expect("test grouped projection aspect key must be foundational")
}

fn field_key(label: &str) -> FieldKey {
    FieldKey::new(label.to_owned()).expect("test grouped projection field key must be foundational")
}

#[test]
fn table_live_lowering_emits_table_row_patch() {
    let planned = planned_view(&collection_canonical(), ViewShapeDescriptor::table());
    let live = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Ada"),
            Some("Ada Lovelace"),
        )),
    )
    .unwrap();

    assert_eq!(
        execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::TableRowPatch)
    );
    assert_eq!(execution.counters().table_ordering_key_count(), 1);
    assert_eq!(
        execution.counters().view_shape_executor_rediscovery_count(),
        0
    );
}

#[test]
fn detail_live_lowering_emits_detail_field_patch() {
    let planned = planned_view(&detail_canonical(), ViewShapeDescriptor::detail());
    let live = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Ada"),
            Some("Ada Lovelace"),
        )),
    )
    .unwrap();

    assert_eq!(
        execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::DetailFieldPatch)
    );
}

#[test]
fn observed_and_focused_inspector_emit_distinct_live_patches() {
    let canonical = detail_canonical();
    let observed_live = lower_view_shape_plan_to_live(
        &planned_view(&canonical, ViewShapeDescriptor::inspector_detail_observed()),
        runtime_basis(
            planned_view(&canonical, ViewShapeDescriptor::inspector_detail_observed())
                .validated()
                .query()
                .schema_basis()
                .clone(),
        ),
        None,
        None,
    )
    .unwrap();
    let focused_plan = planned_view(
        &canonical,
        ViewShapeDescriptor::inspector_detail_focused("profile"),
    );
    let focused_live = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Ada"),
        Some("Ada Lovelace"),
    ));
    let observed_execution = execute_live_view_shape_change(&observed_live, &change).unwrap();
    let focused_execution = execute_live_view_shape_change(&focused_live, &change).unwrap();

    assert_eq!(
        observed_execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::ObservedInspectorPatch)
    );
    assert_eq!(
        focused_execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::FocusedInspectorAspectPatch)
    );
    assert_ne!(
        observed_execution.patch_envelope().replay_digest(),
        focused_execution.patch_envelope().replay_digest()
    );
}

#[test]
fn identity_aware_focused_inspector_emits_explicit_identity_artifact() {
    let canonical = detail_canonical();
    let focused_plan = planned_view(
        &canonical,
        ViewShapeDescriptor::identity_aware_inspector_detail_focused(
            "profile",
            InspectorIdentityClassification::IdentityBreak,
        ),
    );
    let focused_live = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::IdentityBreak,
        )),
    )
    .unwrap();
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Ada"),
        Some("Ada Lovelace"),
    ));
    let execution = execute_live_view_shape_change(&focused_live, &change).unwrap();
    let crate::view_shape_live::ViewShapePatchPayload::FocusedInspectorAspectPatch(patch) =
        execution.patch_envelope().payload()
    else {
        panic!("expected focused inspector aspect patch");
    };
    let inspector_identity = patch
        .inspector_identity()
        .expect("identity-aware focused inspector should attach identity artifact");

    assert_eq!(
        inspector_identity.classification(),
        InspectorIdentityClassification::IdentityBreak
    );
    assert!(inspector_identity.identity_break());
    assert_eq!(
        focused_plan
            .delivery_metadata()
            .identity_consumption()
            .classification(),
        Some(InspectorIdentityClassification::IdentityBreak)
    );
}

#[test]
fn identity_aware_focused_inspector_requires_matching_identity_binding() {
    let canonical = detail_canonical();
    let focused_plan = planned_view(
        &canonical,
        ViewShapeDescriptor::identity_aware_inspector_detail_focused(
            "profile",
            InspectorIdentityClassification::IdentityBreak,
        ),
    );
    let missing_error = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap_err();
    assert_eq!(
        missing_error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );

    let mismatch_error = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::AuthoritativeContinuity,
        )),
    )
    .unwrap_err();
    assert_eq!(
        mismatch_error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );
}

#[test]
fn ordinary_detail_live_lowering_rejects_smuggled_identity_binding() {
    let planned = planned_view(&detail_canonical(), ViewShapeDescriptor::detail());
    let error = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::AuthoritativeContinuity,
        )),
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );
}

#[test]
fn focused_inspector_widening_is_denied_and_counted() {
    let planned = planned_view(
        &detail_canonical(),
        ViewShapeDescriptor::inspector_detail_focused("profile"),
    );
    let live = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let error = execute_live_view_shape_change(
        &live,
        &BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            )),
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        &ViewShapeLiveFailureClass::FocusedInspectorWideningDenied
    );
    assert_eq!(
        error.counters().focused_inspector_widening_denial_count(),
        1
    );
}

#[test]
fn grouped_delta_is_explicit_and_deterministic() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let basis = runtime_basis(planned.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view(&planned);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&planned, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&planned, basis.clone(), &grouped_execution)
            .unwrap();
    let member_key = baseline.desired_state().result().member_states()[0]
        .member_key()
        .to_string();
    let live = lower_view_shape_plan_to_live(&planned, basis, Some(baseline), None).unwrap();
    let change = BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some(member_key.as_str()),
            Some(member_key.as_str()),
        ))
        .with_field_delta(BridgeFieldDelta::new(
            "status",
            "lane",
            Some("todo"),
            Some("doing"),
        ))
        .with_membership_transition(true, true);
    let next_truth_view = grouped_truth_view_with_rows(
        &planned,
        &[
            ("task-1".to_string(), "Ada".to_string(), "doing".to_string()),
            ("task-2".to_string(), "Bea".to_string(), "doing".to_string()),
        ],
        "identity.id",
        None,
    );
    let next_grouped_execution = materialize_grouped_execution_surface_from_truth_view(
        &planned,
        live.basis().clone(),
        &next_truth_view,
    )
    .unwrap();

    let grouped_live = admit_grouped_live_view(&live).unwrap();
    let first =
        execute_grouped_live_view_shape_change(grouped_live, &change, &next_grouped_execution)
            .unwrap();
    let second =
        execute_grouped_live_view_shape_change(grouped_live, &change, &next_grouped_execution)
            .unwrap();

    match first.patch_envelope().payload() {
        ViewShapePatchPayload::KanbanGroupMembershipPatch(delta) => {
            assert_eq!(
                delta.digest(),
                match second.patch_envelope().payload() {
                    ViewShapePatchPayload::KanbanGroupMembershipPatch(second_delta) =>
                        second_delta.digest(),
                    other => panic!("expected grouped delta payload, got {other:?}"),
                }
            );
            assert_eq!(delta.transitions().len(), 1);
            assert_eq!(delta.prior().result().lane_count(), 2);
            assert_eq!(delta.next().result().lane_count(), 1);
            assert_eq!(delta.next().result().row_count(), 2);
        }
        other => panic!("expected grouped delta payload, got {other:?}"),
    }
}

#[test]
fn grouped_baseline_is_derived_from_authoritative_execution_bindings() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let basis = runtime_basis(planned.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view(&planned);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&planned, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&planned, basis, &grouped_execution).unwrap();

    assert_eq!(
        grouped_execution.grouped_planning().grouping_aspect(),
        "status"
    );
    assert_eq!(grouped_execution.member_rows().len(), 2);
    assert_eq!(
        grouped_execution.member_rows()[0].lane().grouping_aspect(),
        "status"
    );
    assert_eq!(
        grouped_execution.truth_view_digest(),
        truth_view.digest().as_str()
    );
    assert_eq!(baseline.desired_state().result().row_count(), 2);
    assert_eq!(baseline.desired_state().result().lane_count(), 2);
    assert_eq!(
        baseline.desired_state().result().member_states()[0].member_key(),
        "task-1"
    );
    assert_eq!(
        baseline.desired_state().result().member_states()[0]
            .lane()
            .lane_key(),
        "todo"
    );
}

#[test]
fn grouped_baseline_rejects_mismatched_grouped_execution_surface() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let basis = runtime_basis(planned.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view(&planned);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&planned, basis.clone(), &truth_view)
            .unwrap();
    let other_plan = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("profile")),
    );
    let error = materialize_authoritative_grouped_baseline(&other_plan, basis, &grouped_execution)
        .unwrap_err();

    assert_eq!(
        error.failure_class(),
        &ViewShapeLiveFailureClass::GroupedBaselineMismatch
    );
}

#[test]
fn grouped_execution_rejects_truth_view_with_mismatched_identity_binding() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let basis = runtime_basis(planned.validated().query().schema_basis().clone());
    let wrong_truth_view = grouped_truth_view_with_rows(
        &planned,
        &[("task-1".to_string(), "Ada".to_string(), "todo".to_string())],
        "profile.display_name",
        None,
    );

    let error =
        materialize_grouped_execution_surface_from_truth_view(&planned, basis, &wrong_truth_view)
            .unwrap_err();

    assert_eq!(
        error.failure_class(),
        &ViewShapeLiveFailureClass::GroupedBaselineMismatch
    );
}

#[test]
fn grouped_execution_rejects_truth_view_with_mismatched_snapshot_identity() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let wrong_basis = resolve_snapshot_basis(
        ExecutionBasisIntent::new(
            BasisAuthorityFamily::Runtime,
            SnapshotLineageClass::CurrentHead,
            false,
        ),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            None,
            "snapshot-b",
            planned.validated().query().schema_basis().clone(),
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap();
    let truth_view = grouped_truth_view(&planned);

    let error =
        materialize_grouped_execution_surface_from_truth_view(&planned, wrong_basis, &truth_view)
            .unwrap_err();

    assert_eq!(
        error.failure_class(),
        &ViewShapeLiveFailureClass::GroupedBaselineMismatch
    );
    assert!(error.message().contains("snapshot"));
}

#[test]
fn grouped_churn_overrun_is_explicit_refresh_debt() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let basis = runtime_basis(planned.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view(&planned);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&planned, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&planned, basis.clone(), &grouped_execution)
            .unwrap();
    let member_key = baseline.desired_state().result().member_states()[0]
        .member_key()
        .to_string();
    let live = lower_view_shape_plan_to_live(&planned, basis, Some(baseline), None).unwrap();
    let next_truth_view = grouped_truth_view_with_rows(
        &planned,
        &[
            ("task-1".to_string(), "Ada".to_string(), "doing".to_string()),
            ("task-2".to_string(), "Bea".to_string(), "done".to_string()),
        ],
        "identity.id",
        None,
    );
    let next_grouped_execution = materialize_grouped_execution_surface_from_truth_view(
        &planned,
        live.basis().clone(),
        &next_truth_view,
    )
    .unwrap();
    let grouped_live = admit_grouped_live_view(&live).unwrap();
    let execution = execute_grouped_live_view_shape_change(
        grouped_live,
        &BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some(member_key.as_str()),
                Some(member_key.as_str()),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "status",
                "lane",
                Some("todo"),
                Some("doing"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "status",
                "lane",
                Some("doing"),
                Some("done"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some(member_key.as_str()),
                Some(member_key.as_str()),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "meta",
                "priority",
                Some("p1"),
                Some("p2"),
            )),
        &next_grouped_execution,
    )
    .unwrap();

    assert_eq!(execution.patch_envelope().patch_family(), None);
    assert!(matches!(
        execution.patch_envelope().payload(),
        ViewShapePatchPayload::Refresh(ViewShapeRefreshDisposition::GroupedDeferredDebt { .. })
    ));
    assert_eq!(execution.counters().grouped_full_regroup_denial_count(), 1);
    assert_eq!(
        execution.counters().view_family_refresh_admission_count(),
        1
    );
}

#[test]
fn grouped_core_refresh_is_trapped_as_grouped_debt() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let basis = runtime_basis(planned.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view(&planned);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&planned, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&planned, basis.clone(), &grouped_execution)
            .unwrap();
    let live = lower_view_shape_plan_to_live(&planned, basis, Some(baseline), None).unwrap();
    let next_truth_view = grouped_truth_view(&planned);
    let next_grouped_execution = materialize_grouped_execution_surface_from_truth_view(
        &planned,
        live.basis().clone(),
        &next_truth_view,
    )
    .unwrap();
    let mut change = BridgeChangeSummary::default();
    for _ in 0..128 {
        change = change.with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Ada"),
            Some("Ada Lovelace"),
        ));
    }

    let execution = execute_grouped_live_view_shape_change(
        admit_grouped_live_view(&live).unwrap(),
        &change,
        &next_grouped_execution,
    )
    .unwrap();

    assert!(matches!(
        execution.patch_envelope().payload(),
        ViewShapePatchPayload::Refresh(ViewShapeRefreshDisposition::GroupedDeferredDebt {
            reason: super::GroupedRefreshReason::CoreRefreshRequested,
            ..
        })
    ));
    assert_eq!(execution.counters().grouped_full_regroup_denial_count(), 1);
    assert_eq!(
        execution.counters().view_family_refresh_admission_count(),
        1
    );
}

#[test]
fn grouped_delta_mixed_member_churn_is_explicit_refresh_debt() {
    let planned = planned_view(
        &collection_canonical(),
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    );
    let basis = runtime_basis(planned.validated().query().schema_basis().clone());
    let truth_view = grouped_truth_view(&planned);
    let grouped_execution =
        materialize_grouped_execution_surface_from_truth_view(&planned, basis.clone(), &truth_view)
            .unwrap();
    let baseline =
        materialize_authoritative_grouped_baseline(&planned, basis.clone(), &grouped_execution)
            .unwrap();
    let live = lower_view_shape_plan_to_live(&planned, basis, Some(baseline), None).unwrap();
    let next_truth_view = grouped_truth_view_with_rows(
        &planned,
        &[
            ("task-1".to_string(), "Ada".to_string(), "doing".to_string()),
            ("task-3".to_string(), "Cy".to_string(), "todo".to_string()),
        ],
        "identity.id",
        None,
    );
    let next_grouped_execution = materialize_grouped_execution_surface_from_truth_view(
        &planned,
        live.basis().clone(),
        &next_truth_view,
    )
    .unwrap();

    let execution = execute_grouped_live_view_shape_change(
        admit_grouped_live_view(&live).unwrap(),
        &BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("task-1"),
                Some("task-1"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "status",
                "lane",
                Some("todo"),
                Some("doing"),
            ))
            .with_membership_transition(true, true),
        &next_grouped_execution,
    )
    .unwrap();

    assert!(matches!(
        execution.patch_envelope().payload(),
        ViewShapePatchPayload::Refresh(ViewShapeRefreshDisposition::GroupedDeferredDebt {
            reason: super::GroupedRefreshReason::PolicyExceeded,
            ..
        })
    ));
    assert_eq!(execution.counters().grouped_full_regroup_denial_count(), 1);
    assert_eq!(
        execution.counters().view_family_refresh_admission_count(),
        1
    );
}
