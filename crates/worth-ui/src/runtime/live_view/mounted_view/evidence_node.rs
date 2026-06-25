use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiLiveViewEditReceipt, WorthUiLiveViewInteractionActivationDenial,
    WorthUiLiveViewInteractionSubmissionReceipt, WorthUiLiveViewProjectionAdmissionReceipt,
    WorthUiLiveViewProjectionRenderPlan, WorthUiLiveViewStateEditDenial,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedEvidenceNodeReceipt {
    node_id: String,
    semantic_slice: &'static str,
    rows: Vec<WorthUiMountedEvidenceRowReceipt>,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedEvidenceRowReceipt {
    label: String,
    value: String,
    receipt_digest: u64,
}

impl WorthUiMountedEvidenceNodeReceipt {
    pub(in crate::runtime::live_view) fn from_live_view_projection(
        projection: &WorthUiLiveViewProjectionAdmissionReceipt,
        render_plan: &WorthUiLiveViewProjectionRenderPlan,
    ) -> Self {
        let rows = live_view_evidence_rows(projection, render_plan);
        let receipt_digest = digest_parts(
            std::iter::once(projection.live_view_id().to_owned())
                .chain(std::iter::once("evidence".to_owned()))
                .chain(rows.iter().map(|row| row.receipt_digest().to_string())),
        );
        Self {
            node_id: "live_view.evidence".to_owned(),
            semantic_slice: "LiveViewEvidence",
            rows,
            receipt_digest,
        }
    }

    pub(in crate::runtime::live_view) fn from_live_view_observations(
        last_edit: Option<&WorthUiLiveViewEditReceipt>,
        last_edit_denial: Option<&WorthUiLiveViewStateEditDenial>,
        last_submission: Option<&WorthUiLiveViewInteractionSubmissionReceipt>,
        last_submission_denial: Option<&WorthUiLiveViewInteractionActivationDenial>,
        last_source_denial: Option<&str>,
    ) -> Self {
        let rows = live_view_observation_rows(
            last_edit,
            last_edit_denial,
            last_submission,
            last_submission_denial,
            last_source_denial,
        );
        let receipt_digest = digest_parts(
            std::iter::once("live_view.observations".to_owned())
                .chain(rows.iter().map(|row| row.receipt_digest().to_string())),
        );
        Self {
            node_id: "live_view.observations".to_owned(),
            semantic_slice: "LiveViewObservations",
            rows,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn semantic_slice(&self) -> &'static str {
        self.semantic_slice
    }

    pub fn rows(&self) -> &[WorthUiMountedEvidenceRowReceipt] {
        &self.rows
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedEvidenceRowReceipt {
    fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        let label = label.into();
        let value = value.into();
        let receipt_digest = digest_parts([label.as_str(), value.as_str()]);
        Self {
            label,
            value,
            receipt_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn live_view_evidence_rows(
    projection: &WorthUiLiveViewProjectionAdmissionReceipt,
    render_plan: &WorthUiLiveViewProjectionRenderPlan,
) -> Vec<WorthUiMountedEvidenceRowReceipt> {
    let mut rows = vec![WorthUiMountedEvidenceRowReceipt::new(
        "projection",
        format!(
            "live_view={} admission={} render_plan={} controls={} actions={} consumers={}",
            projection.live_view_id(),
            projection.admission_digest(),
            render_plan.render_plan_digest(),
            render_plan.controls().len(),
            render_plan.interactions().len(),
            render_plan.consumers().len()
        ),
    )];
    rows.extend(projection.controls().iter().map(|control| {
        WorthUiMountedEvidenceRowReceipt::new(
            "control",
            format!(
                "{} binding={} kind={} digest={} graph={}",
                control.control_id(),
                control.binding().binding_id(),
                control.kind().token(),
                control.control_projection_digest(),
                control.query_graph_execution().execution_digest()
            ),
        )
    }));
    rows.extend(projection.conditionals().iter().map(|conditional| {
        WorthUiMountedEvidenceRowReceipt::new(
            "conditional",
            format!(
                "{} posture={} layout={} events={} a11y={}",
                conditional.control().control_id(),
                conditional.participation().posture().token(),
                conditional.participation().participates_in_layout(),
                conditional.participation().participates_in_events(),
                conditional.participation().participates_in_accessibility()
            ),
        )
    }));
    rows.extend(projection.readinesses().iter().map(|readiness| {
        WorthUiMountedEvidenceRowReceipt::new(
            "readiness",
            format!(
                "{} posture={} digest={} graph={}",
                readiness.readiness_id(),
                readiness.posture().token(),
                readiness.readiness_digest(),
                readiness.query_graph_execution().execution_digest()
            ),
        )
    }));
    rows.extend(projection.payloads().iter().map(|payload| {
        WorthUiMountedEvidenceRowReceipt::new(
            "payload",
            format!(
                "{} shape={} digest={} graph={}",
                payload.payload_id(),
                payload.shape().token(),
                payload.payload_projection_digest(),
                payload.query_graph_execution().execution_digest()
            ),
        )
    }));
    rows.extend(projection.interactions().iter().map(|interaction| {
        WorthUiMountedEvidenceRowReceipt::new(
            "interaction",
            format!(
                "{} readiness={} payload={} digest={} graph={}",
                interaction.interaction_id(),
                interaction.readiness().readiness_digest(),
                interaction.payload_projection().payload_projection_digest(),
                interaction.interaction_intent_digest(),
                interaction.query_graph_execution().execution_digest()
            ),
        )
    }));
    rows
}

fn live_view_observation_rows(
    last_edit: Option<&WorthUiLiveViewEditReceipt>,
    last_edit_denial: Option<&WorthUiLiveViewStateEditDenial>,
    last_submission: Option<&WorthUiLiveViewInteractionSubmissionReceipt>,
    last_submission_denial: Option<&WorthUiLiveViewInteractionActivationDenial>,
    last_source_denial: Option<&str>,
) -> Vec<WorthUiMountedEvidenceRowReceipt> {
    let mut rows = Vec::new();
    if let Some(edit) = last_edit {
        rows.push(WorthUiMountedEvidenceRowReceipt::new(
            "live_view_edit",
            format!(
                "binding={} changed={} edit={} graph={}",
                edit.binding().binding_id(),
                edit.changed_fact().identity(),
                edit.receipt_digest(),
                edit.query_graph_execution().execution_digest()
            ),
        ));
    }
    if let Some(denial) = last_edit_denial {
        rows.push(WorthUiMountedEvidenceRowReceipt::new(
            "live_view_edit_denial",
            live_view_edit_denial_row(denial),
        ));
    }
    if let Some(receipt) = last_submission {
        rows.push(WorthUiMountedEvidenceRowReceipt::new(
            "live_view_submission",
            format!(
                "interaction={} payload={} digest={}",
                receipt.interaction().interaction_id(),
                receipt.emitted_payload().display_shape(),
                receipt.submission_digest()
            ),
        ));
    }
    if let Some(denial) = last_submission_denial {
        rows.push(WorthUiMountedEvidenceRowReceipt::new(
            "live_view_submission_denial",
            live_view_submission_denial_row(denial),
        ));
    }
    if let Some(denial) = last_source_denial {
        rows.push(WorthUiMountedEvidenceRowReceipt::new(
            "live_view_source_denial",
            format!("code=live_view.source.rejected detail={denial}"),
        ));
    }
    rows
}

fn live_view_edit_denial_row(denial: &WorthUiLiveViewStateEditDenial) -> String {
    match denial {
        WorthUiLiveViewStateEditDenial::StaleTargetBinding {
            binding_id,
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id,
        } => format!(
            "code=live_view.edit.stale_target binding={} slot={} surface={} expected={} actual={}",
            binding_id,
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id.as_deref().unwrap_or("<none>")
        ),
        WorthUiLiveViewStateEditDenial::ValueKindMismatch {
            binding_id,
            expected,
            actual,
        } => format!(
            "code=live_view.edit.value_kind_mismatch binding={} expected={} actual={}",
            binding_id,
            expected.token(),
            actual.token()
        ),
        WorthUiLiveViewStateEditDenial::ReadOnlyBinding { binding_id } => {
            format!("code=live_view.edit.read_only binding={binding_id}")
        }
    }
}

fn live_view_submission_denial_row(denial: &WorthUiLiveViewInteractionActivationDenial) -> String {
    match denial {
        WorthUiLiveViewInteractionActivationDenial::ReadinessDenied {
            interaction_id,
            readiness_digest,
            posture,
        } => format!(
            "code=live_view.submit.readiness_denied interaction={} readiness={} posture={}",
            interaction_id,
            readiness_digest,
            posture.token()
        ),
        WorthUiLiveViewInteractionActivationDenial::StaleTargetBinding {
            interaction_id,
            readiness_digest,
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id,
        } => format!(
            "code=live_view.submit.stale_target interaction={} readiness={} slot={} surface={} expected={} actual={}",
            interaction_id,
            readiness_digest,
            slot_name,
            surface_id,
            expected_component_id,
            actual_component_id.as_deref().unwrap_or("<none>")
        ),
        WorthUiLiveViewInteractionActivationDenial::ContextSuppressed {
            interaction_id,
            interaction_digest,
            context_digest,
            disabled,
            inert,
        } => format!(
            "code=live_view.submit.context_suppressed interaction={} interaction_digest={} context={} disabled={} inert={}",
            interaction_id, interaction_digest, context_digest, disabled, inert
        ),
    }
}
