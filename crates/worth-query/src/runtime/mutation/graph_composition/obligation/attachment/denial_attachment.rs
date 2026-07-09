use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryAuthoritativeMutationObligationDispatchProjection,
    WorthQueryGraphObligationDenialProjection, WorthQueryGraphObligationDenialProjectionRow,
    WorthQueryGraphObligationDispatchContextKind, WorthQueryGraphObligationExecutionStatus,
    WorthQueryGraphObligationKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDenialAttachmentProjection {
    envelope_digest: String,
    execution_point: WorthQueryGraphObligationDispatchContextKind,
    touch_descriptor_digest: String,
    operating_world_digest: String,
    dispatch_digest: String,
    denial_projection_digest: String,
    rows: Vec<WorthQueryGraphObligationDenialAttachmentProjectionRow>,
    projection_digest: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDenialAttachmentProjectionRow {
    rule_identity_digest: String,
    rule_namespace: String,
    rule_name: String,
    rule_semantic_version: String,
    obligation_kind: WorthQueryGraphObligationKind,
    execution_point: WorthQueryGraphObligationDispatchContextKind,
    execution_status: WorthQueryGraphObligationExecutionStatus,
    verdict: String,
    verdict_context: Option<String>,
    support_lane: String,
    touch_descriptor_digest: String,
    operating_world_digest: String,
    execution_input_digest: String,
    dispatch_plan_digest: String,
    envelope_digest: String,
    row_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationDenialAttachmentProjection {
    pub(crate) fn from_dispatch_projection_and_denial(
        dispatch: &WorthQueryAuthoritativeMutationObligationDispatchProjection,
        denial: &WorthQueryGraphObligationDenialProjection,
    ) -> Option<Self> {
        let envelope_digest = dispatch.envelope_digest()?.to_string();
        let execution_point = dispatch.context_kind()?;
        let touch_descriptor_digest = dispatch.touch_descriptor_digest()?.to_string();
        let operating_world_digest = dispatch.operating_world_digest()?.to_string();
        let rows = denial
            .rows()
            .iter()
            .filter_map(|row| {
                WorthQueryGraphObligationDenialAttachmentProjectionRow::from_denial_row(
                    row,
                    dispatch,
                    &envelope_digest,
                    execution_point,
                    &touch_descriptor_digest,
                    &operating_world_digest,
                )
            })
            .collect::<Vec<_>>();
        if rows.is_empty() {
            return None;
        }
        let row_identities = rows
            .iter()
            .map(WorthQueryGraphObligationDenialAttachmentProjectionRow::row_evidence_identity)
            .collect::<Vec<_>>();
        let denial_projection_digest = denial.projection_digest().to_string();
        let projection_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationDenialAttachmentProjection,
        )
        .field_value(WorthQueryEvidenceTag::new("envelope"), &envelope_digest)
        .field_shape(
            WorthQueryEvidenceTag::new("execution_point"),
            execution_point.as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("touch_descriptor"),
            &touch_descriptor_digest,
        )
        .field_value(
            WorthQueryEvidenceTag::new("operating_world"),
            &operating_world_digest,
        )
        .field_value(
            WorthQueryEvidenceTag::new("dispatch"),
            dispatch.dispatch_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("denial_projection"),
            &denial_projection_digest,
        )
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("row"), row_identities)
        .seal();
        Some(Self {
            envelope_digest,
            execution_point,
            touch_descriptor_digest,
            operating_world_digest,
            dispatch_digest: dispatch.dispatch_digest().to_string(),
            denial_projection_digest,
            rows,
            projection_digest,
        })
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn execution_point(&self) -> WorthQueryGraphObligationDispatchContextKind {
        self.execution_point
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        &self.touch_descriptor_digest
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub fn dispatch_digest(&self) -> &str {
        &self.dispatch_digest
    }

    pub fn denial_projection_digest(&self) -> &str {
        &self.denial_projection_digest
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationDenialAttachmentProjectionRow] {
        &self.rows
    }

    pub fn projection_digest(&self) -> &str {
        self.projection_digest.as_str()
    }

    pub(crate) fn projection_evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.projection_digest
    }
}

impl WorthQueryGraphObligationDenialAttachmentProjectionRow {
    fn from_denial_row(
        row: &WorthQueryGraphObligationDenialProjectionRow,
        dispatch: &WorthQueryAuthoritativeMutationObligationDispatchProjection,
        envelope_digest: &str,
        execution_point: WorthQueryGraphObligationDispatchContextKind,
        touch_descriptor_digest: &str,
        operating_world_digest: &str,
    ) -> Option<Self> {
        let dispatch_row = dispatch.rows().iter().find(|dispatch_row| {
            dispatch_row.rule_identity_digest() == row.rule_identity_digest()
                && dispatch_row.execution_input_digest() == row.execution_input_digest()
        })?;
        let rule_identity_digest = row.rule_identity_digest().to_string();
        let execution_input_digest = row.execution_input_digest().to_string();
        let dispatch_plan_digest = dispatch_row.dispatch_plan_digest().to_string();
        let row_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationDenialAttachmentProjectionRow,
        )
        .field_value(WorthQueryEvidenceTag::new("rule"), &rule_identity_digest)
        .field_shape(
            WorthQueryEvidenceTag::new("execution_point"),
            execution_point.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("status"),
            row.execution_status().as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("touch_descriptor"),
            touch_descriptor_digest,
        )
        .field_value(
            WorthQueryEvidenceTag::new("operating_world"),
            operating_world_digest,
        )
        .field_value(
            WorthQueryEvidenceTag::new("execution_input"),
            &execution_input_digest,
        )
        .field_value(
            WorthQueryEvidenceTag::new("dispatch_plan"),
            &dispatch_plan_digest,
        )
        .field_value(WorthQueryEvidenceTag::new("envelope"), envelope_digest)
        .seal();
        Some(Self {
            rule_identity_digest,
            rule_namespace: row.rule_namespace().to_string(),
            rule_name: row.rule_name().to_string(),
            rule_semantic_version: row.rule_semantic_version().to_string(),
            obligation_kind: row.obligation_kind(),
            execution_point,
            execution_status: row.execution_status(),
            verdict: row.verdict().to_string(),
            verdict_context: row.verdict_context().map(str::to_string),
            support_lane: row.support_lane().as_str().to_string(),
            touch_descriptor_digest: touch_descriptor_digest.to_string(),
            operating_world_digest: operating_world_digest.to_string(),
            execution_input_digest,
            dispatch_plan_digest,
            envelope_digest: envelope_digest.to_string(),
            row_digest,
        })
    }

    pub fn rule_identity_digest(&self) -> &str {
        &self.rule_identity_digest
    }

    pub fn rule_namespace(&self) -> &str {
        &self.rule_namespace
    }

    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }

    pub fn rule_semantic_version(&self) -> &str {
        &self.rule_semantic_version
    }

    pub fn obligation_kind(&self) -> WorthQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn execution_point(&self) -> WorthQueryGraphObligationDispatchContextKind {
        self.execution_point
    }

    pub fn execution_status(&self) -> WorthQueryGraphObligationExecutionStatus {
        self.execution_status
    }

    pub fn verdict(&self) -> &str {
        &self.verdict
    }

    pub fn verdict_context(&self) -> Option<&str> {
        self.verdict_context.as_deref()
    }

    pub fn support_lane(&self) -> &str {
        &self.support_lane
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        &self.touch_descriptor_digest
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub fn execution_input_digest(&self) -> &str {
        &self.execution_input_digest
    }

    pub fn dispatch_plan_digest(&self) -> &str {
        &self.dispatch_plan_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    fn row_evidence_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.row_digest
    }
}
