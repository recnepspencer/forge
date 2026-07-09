use crate::basis::ResolvedSnapshotBasis;
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::identity::BasisDigest;
use crate::view_shape::{GroupedViewPlanningArtifact, ViewShapePlanArtifact, ViewShapePlanDigest};
use worth_foundational::facade::{AspectKey, AspectValue, InternedString};
use worth_runtime_bridge::facade::BridgeGroupedTruthViewArtifact;

use super::counters::ViewShapeLiveCounters;
use super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
use super::grouped_state::GroupedLaneIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedExecutionLaneValue {
    identity: GroupedLaneIdentity,
}

impl GroupedExecutionLaneValue {
    pub(crate) fn new(grouping_aspect: AspectKey, lane_key: impl Into<String>) -> Self {
        Self {
            identity: GroupedLaneIdentity::new(grouping_aspect, lane_key),
        }
    }

    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        self.identity.native_grouping_aspect_key()
    }

    pub fn lane_key(&self) -> &str {
        self.identity.lane_key()
    }

    pub fn native_lane_identity(&self) -> &GroupedLaneIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedExecutionMemberRow {
    member_key: String,
    lane: GroupedExecutionLaneValue,
}

impl GroupedExecutionMemberRow {
    pub fn member_key(&self) -> &str {
        &self.member_key
    }

    pub fn lane(&self) -> &GroupedExecutionLaneValue {
        &self.lane
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedExecutionSurfaceArtifact {
    identity: WorthQueryEvidenceIdentity,
    plan_digest: ViewShapePlanDigest,
    basis_digest: BasisDigest,
    truth_view_evidence_identity: WorthQueryEvidenceIdentity,
    grouped_planning: GroupedViewPlanningArtifact,
    member_rows: Vec<GroupedExecutionMemberRow>,
}

impl GroupedExecutionSurfaceArtifact {
    pub fn digest(&self) -> &str {
        self.identity.as_str()
    }

    pub fn evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }

    pub fn plan_digest(&self) -> &ViewShapePlanDigest {
        &self.plan_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn truth_view_evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.truth_view_evidence_identity
    }

    pub fn grouped_planning(&self) -> &GroupedViewPlanningArtifact {
        &self.grouped_planning
    }

    pub fn member_rows(&self) -> &[GroupedExecutionMemberRow] {
        &self.member_rows
    }
}

pub fn materialize_grouped_execution_surface_from_truth_view(
    plan: &ViewShapePlanArtifact,
    basis: ResolvedSnapshotBasis,
    truth_view: &BridgeGroupedTruthViewArtifact,
) -> Result<GroupedExecutionSurfaceArtifact, ViewShapeLiveError> {
    if plan.family() != crate::view_shape::ViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped execution surface may only be materialized for kanban grouped plans",
            ViewShapeLiveCounters::default(),
        ));
    }
    if basis.identity().schema_basis() != plan.validated().query().schema_basis()
        || basis.identity().schema_basis() != plan.validated().result_shape().schema_basis()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::BasisInvariantRejected,
            format!(
                "grouped execution basis schema '{}' does not match validated query/result-shape schema '{}'",
                basis.identity().schema_basis().as_str(),
                plan.validated().query().schema_basis().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }

    let grouped_planning = plan.grouped_planning_artifact().cloned().ok_or_else(|| {
        ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped execution surface requires planner-issued grouped planning artifact",
            ViewShapeLiveCounters::default(),
        )
    })?;
    if truth_view.contract().native_grouping_aspect_key()
        != grouped_planning.native_grouping_aspect_key()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view aspect '{}' does not match planned grouping aspect '{}'",
                truth_view.contract().grouping_aspect(),
                grouped_planning.native_grouping_aspect_key().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    let truth_view_snapshot_identity =
        crate::basis::bridge_snapshot_evidence_identity(truth_view.basis_snapshot_identity())
            .map_err(|error| {
                ViewShapeLiveError::new(
                    ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                    format!("grouped truth-view snapshot identity cannot be lowered: {error:?}"),
                    ViewShapeLiveCounters::default(),
                )
            })?;
    if &truth_view_snapshot_identity != basis.identity().snapshot_identity() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view snapshot '{}' does not match grouped execution basis snapshot '{}'",
                truth_view_snapshot_identity.as_str(),
                basis.identity().snapshot_identity().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    if truth_view.contract().identity_binding().native_aspect_key()
        != grouped_planning
            .identity_binding()
            .native_binding_aspect_key()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view identity binding '{}' does not match planned identity binding '{}'",
                truth_view.contract().identity_binding().aspect_key(),
                grouped_planning
                    .identity_binding()
                    .native_binding_aspect_key()
                    .as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    if truth_view.contract().grouping_binding().native_aspect_key()
        != grouped_planning
            .grouping_binding()
            .native_binding_aspect_key()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view grouping binding '{}' does not match planned grouping binding '{}'",
                truth_view.contract().grouping_binding().aspect_key(),
                grouped_planning
                    .grouping_binding()
                    .native_binding_aspect_key()
                    .as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }

    let member_rows = truth_view
        .members()
        .iter()
        .map(|member| GroupedExecutionMemberRow {
            member_key: canonical_value_text(member.identity_value()),
            lane: GroupedExecutionLaneValue::new(
                grouped_planning.native_grouping_aspect_key().clone(),
                canonical_value_text(member.lane().value()),
            ),
        })
        .collect::<Vec<_>>();
    if member_rows.is_empty() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped truth-view artifact produced no member rows",
            ViewShapeLiveCounters::default(),
        ));
    }

    let plan_evidence_identity = plan.view_plan_digest().evidence_identity();
    let basis_evidence_identity = basis.proof().digest().evidence_identity();
    let grouped_truth_view_evidence_identity =
        bridge_grouped_truth_view_digest_evidence_identity(truth_view.digest());
    let identity = WorthQueryEvidenceIdentity::compose(
        WorthQueryEvidenceScope::GroupedExecutionSurfaceArtifact,
    )
    .field_evidence_identity(WorthQueryEvidenceTag::new("plan"), &plan_evidence_identity)
    .field_evidence_identity(
        WorthQueryEvidenceTag::new("basis"),
        &basis_evidence_identity,
    )
    .field_evidence_identity(
        WorthQueryEvidenceTag::new("grouped_truth"),
        &grouped_truth_view_evidence_identity,
    )
    .field_usize(WorthQueryEvidenceTag::new("members"), member_rows.len())
    .seal();

    Ok(GroupedExecutionSurfaceArtifact {
        identity,
        plan_digest: plan.view_plan_digest().clone(),
        basis_digest: basis.proof().digest().clone(),
        truth_view_evidence_identity: grouped_truth_view_evidence_identity,
        grouped_planning,
        member_rows,
    })
}

pub(crate) fn bridge_grouped_truth_view_digest_evidence_identity(
    digest: &worth_runtime_bridge::facade::BridgeGroupedTruthViewDigest,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::BridgeGroupedTruthViewDigest)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "bridge_grouped_truth_view_digest_v1",
        )
        .field_value(
            WorthQueryEvidenceTag::new("grouped_truth_view_digest"),
            digest.as_str(),
        )
        .seal()
}

fn canonical_value_text(value: &AspectValue) -> String {
    match value {
        AspectValue::String(text) => interned_string_text(text),
        AspectValue::Null => "null".to_string(),
        AspectValue::Bool(value) => format!("bool:{value}"),
        AspectValue::Int8(value) => format!("i8:{value}"),
        AspectValue::Int16(value) => format!("i16:{value}"),
        AspectValue::Int32(value) => format!("i32:{value}"),
        AspectValue::Int64(value) => format!("i64:{value}"),
        AspectValue::UInt8(value) => format!("u8:{value}"),
        AspectValue::UInt16(value) => format!("u16:{value}"),
        AspectValue::UInt32(value) => format!("u32:{value}"),
        AspectValue::UInt64(value) => format!("u64:{value}"),
        AspectValue::Float32(value) => format!("f32-bits:{}", value.bits()),
        AspectValue::Float64(value) => format!("f64-bits:{}", value.bits()),
        AspectValue::Decimal(value) => format!("decimal:{}", value.as_str()),
        AspectValue::BigInt(value) => format!("bigint:{}", value.as_str()),
        AspectValue::Rational(value) => format!(
            "rational:{}/{}",
            value.numerator.as_str(),
            value.denominator.as_str()
        ),
        AspectValue::Bytes(value) => format!("bytes-ref:{}", value.0),
        AspectValue::Uuid(value) => value.iter().map(|byte| format!("{byte:02x}")).collect(),
        AspectValue::Date(value) => format!("date-days:{}", value.days_from_unix_epoch),
        AspectValue::Time(value) => format!("time-nanos:{}", value.nanos_since_midnight),
        AspectValue::Timestamp(value) => {
            format!("timestamp-micros:{}", value.micros_since_unix_epoch)
        }
        AspectValue::TimestampTz(value) => format!(
            "timestamp-tz:{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        ),
        AspectValue::EntityRef(value) => format!(
            "entity-ref:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        ),
        AspectValue::ContentRef(value) => format!("content-ref:{}", value.0),
    }
}

fn interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(text) => text.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}
