use super::super::slice_classification::{
    WorthGraphReadAccessUnresolvedSliceKind, WorthGraphReadAccessUnresolvedSliceRow,
};
use super::super::stable_digest;
use super::{
    admitted_posture_projection::admitted_posture_projection,
    denied_posture_projection::denied_posture_projection,
    required_posture_projection::required_posture_projection,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessSpatialDensePostureOutcome {
    ExecutedThroughQueryReceipt,
    RequiredQueryPosture,
    DeniedByQueryPosture,
    CarriedCapabilityGap,
    AdmittedPlanRequiresExecutionReceipt,
}

impl WorthGraphReadAccessSpatialDensePostureOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutedThroughQueryReceipt => "executed_through_query_receipt",
            Self::RequiredQueryPosture => "required_query_posture",
            Self::DeniedByQueryPosture => "denied_by_query_posture",
            Self::CarriedCapabilityGap => "carried_capability_gap",
            Self::AdmittedPlanRequiresExecutionReceipt => {
                "admitted_plan_requires_execution_receipt"
            }
        }
    }

    pub const fn claims_receipt(self) -> bool {
        matches!(self, Self::ExecutedThroughQueryReceipt)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSpatialDensePostureProjection {
    slice_kind: WorthGraphReadAccessUnresolvedSliceKind,
    outcome: WorthGraphReadAccessSpatialDensePostureOutcome,
    source_posture_row_digest: String,
    source_requirement_record_digest: String,
    read_family_identity_digest: Option<String>,
    requirement_row_digest: Option<String>,
    query_family_name: Option<String>,
    query_family_digest_seed: String,
    query_posture: String,
    denial_kind: Option<String>,
    query_plan_digest: Option<String>,
    query_receipt_digest: Option<String>,
    execution_counter_digest: Option<String>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
    projection_digest: String,
}

pub(crate) fn project_spatial_dense_postures(
    rows: &[WorthGraphReadAccessUnresolvedSliceRow],
) -> Vec<WorthGraphReadAccessSpatialDensePostureProjection> {
    rows.iter().map(project_one).collect()
}

fn project_one(
    row: &WorthGraphReadAccessUnresolvedSliceRow,
) -> WorthGraphReadAccessSpatialDensePostureProjection {
    if row.denial_kind().is_some() || row.query_posture() == "denied" {
        return denied_posture_projection(row);
    }
    if matches!(
        row.kind(),
        WorthGraphReadAccessUnresolvedSliceKind::CarriedCapabilityGap
    ) {
        return required_posture_projection(
            row,
            WorthGraphReadAccessSpatialDensePostureOutcome::CarriedCapabilityGap,
        );
    }
    if row.query_posture() == "inline_indexed"
        || row.query_posture() == "bounded_ephemeral_index"
        || row.query_posture() == "admitted_paged_streaming"
    {
        return admitted_posture_projection(row);
    }
    required_posture_projection(
        row,
        WorthGraphReadAccessSpatialDensePostureOutcome::RequiredQueryPosture,
    )
}

impl WorthGraphReadAccessSpatialDensePostureProjection {
    pub(crate) fn new(
        row: &WorthGraphReadAccessUnresolvedSliceRow,
        outcome: WorthGraphReadAccessSpatialDensePostureOutcome,
        query_plan_digest: Option<String>,
        query_receipt_digest: Option<String>,
        execution_counter_digest: Option<String>,
    ) -> Self {
        let projection_digest = stable_digest(&[
            "worth_graph_read_access_spatial_dense_posture_projection_v1".to_string(),
            format!("slice_kind:{}", row.kind().as_str()),
            format!("outcome:{}", outcome.as_str()),
            format!("source_posture:{}", row.source_posture_row_digest()),
            format!("requirement:{}", row.source_requirement_record_digest()),
            format!(
                "read_family:{}",
                row.read_family_identity_digest().unwrap_or("none")
            ),
            format!(
                "requirement_row:{}",
                row.requirement_row_digest().unwrap_or("none")
            ),
            format!("query_posture:{}", row.query_posture()),
            format!("denial:{}", row.denial_kind().unwrap_or("none")),
            format!("plan:{}", query_plan_digest.as_deref().unwrap_or("none")),
            format!(
                "receipt:{}",
                query_receipt_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "counters:{}",
                execution_counter_digest.as_deref().unwrap_or("none")
            ),
        ]);
        Self {
            slice_kind: row.kind(),
            outcome,
            source_posture_row_digest: row.source_posture_row_digest().to_string(),
            source_requirement_record_digest: row.source_requirement_record_digest().to_string(),
            read_family_identity_digest: row.read_family_identity_digest().map(str::to_string),
            requirement_row_digest: row.requirement_row_digest().map(str::to_string),
            query_family_name: row.query_family_name().map(str::to_string),
            query_family_digest_seed: row.query_family_digest_seed().to_string(),
            query_posture: row.query_posture().to_string(),
            denial_kind: row.denial_kind().map(str::to_string),
            query_plan_digest,
            query_receipt_digest,
            execution_counter_digest,
            blocker: row.blocker().map(str::to_string),
            removal_trigger: row.removal_trigger().map(str::to_string),
            projection_digest,
        }
    }

    pub const fn slice_kind(&self) -> WorthGraphReadAccessUnresolvedSliceKind {
        self.slice_kind
    }

    pub const fn outcome(&self) -> WorthGraphReadAccessSpatialDensePostureOutcome {
        self.outcome
    }

    pub fn source_posture_row_digest(&self) -> &str {
        &self.source_posture_row_digest
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn read_family_identity_digest(&self) -> Option<&str> {
        self.read_family_identity_digest.as_deref()
    }

    pub fn requirement_row_digest(&self) -> Option<&str> {
        self.requirement_row_digest.as_deref()
    }

    pub fn query_family_name(&self) -> Option<&str> {
        self.query_family_name.as_deref()
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn query_posture(&self) -> &str {
        &self.query_posture
    }

    pub fn denial_kind(&self) -> Option<&str> {
        self.denial_kind.as_deref()
    }

    pub fn query_plan_digest(&self) -> Option<&str> {
        self.query_plan_digest.as_deref()
    }

    pub fn query_receipt_digest(&self) -> Option<&str> {
        self.query_receipt_digest.as_deref()
    }

    pub fn execution_counter_digest(&self) -> Option<&str> {
        self.execution_counter_digest.as_deref()
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn removal_trigger(&self) -> Option<&str> {
        self.removal_trigger.as_deref()
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }

    pub const fn claims_graph_read_receipt(&self) -> bool {
        self.outcome.claims_receipt()
    }
}
