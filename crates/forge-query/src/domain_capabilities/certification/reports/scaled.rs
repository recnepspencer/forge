use crate::domain_capabilities::certification::reports::fixtures::{
    admitted_basis_observation_plan, admitted_projection_consumption_plan, admitted_ready,
    intent_declaration, lower_runtime_envelope, plan_support_requested,
    projection_contract_request, store_backed_replay_gap_request, success,
};
use crate::domain_capabilities::materialize_intent_admission_support_traceability_report;
use crate::domain_capabilities::{forge_query_domain, ForgeQueryDomainCapabilityCategory};
use crate::identity::hash_parts;
use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::ForgeQueryIntentAdmissionCoveredEntrypoint;

pub(crate) struct ForgeQueryDomainCapabilityScaledEvidence {
    contribution_width: usize,
    trace_width: usize,
    category_width: usize,
    support_width: usize,
    contribution_digest: String,
    trace_digest: String,
    category_digest: String,
    support_digest: String,
}

impl ForgeQueryDomainCapabilityScaledEvidence {
    pub(crate) fn contribution_width(&self) -> usize {
        self.contribution_width
    }

    pub(crate) fn trace_width(&self) -> usize {
        self.trace_width
    }

    pub(crate) fn category_width(&self) -> usize {
        self.category_width
    }

    pub(crate) fn support_width(&self) -> usize {
        self.support_width
    }

    pub(crate) fn contribution_digest(&self) -> &str {
        &self.contribution_digest
    }

    pub(crate) fn trace_digest(&self) -> &str {
        &self.trace_digest
    }

    pub(crate) fn category_digest(&self) -> &str {
        &self.category_digest
    }

    pub(crate) fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

pub(crate) fn forge_query_domain_capability_scaled_evidence(
) -> [ForgeQueryDomainCapabilityScaledEvidence; 3] {
    [scaled_evidence(1), scaled_evidence(2), scaled_evidence(3)]
}

fn scaled_evidence(scale: usize) -> ForgeQueryDomainCapabilityScaledEvidence {
    let declaration = intent_declaration(&format!("domain-capability-scale-{scale}"));
    let admitted_plan = admitted_basis_observation_plan();
    let projection_plan = admitted_projection_consumption_plan();
    let lower_runtime = lower_runtime_envelope(&format!("domain-capability-scale-{scale}"));
    let categories = category_set_for_scale(scale);

    let mut contribution_digests = Vec::new();
    let mut trace_digests = Vec::new();
    let mut support_digests = Vec::new();
    let mut support_width = 0usize;
    let mut trace_width = 0usize;

    if categories.contains(&ForgeQueryDomainCapabilityCategory::Admission) {
        let admission = forge_query_domain("worth.spatial")
            .for_admitted_intent_plan(&admitted_plan)
            .advises(format!("admission.scale_{scale}"))
            .because("scaled admission evidence should remain canonical")
            .materialize()
            .expect("scaled admission should materialize");
        contribution_digests.push(admission_digest(&admission));
    }

    if categories.contains(&ForgeQueryDomainCapabilityCategory::SupportTraceability) {
        let support = forge_query_domain("worth.spatial")
            .for_intent(&declaration)
            .supports_traceability(format!("traceability.scale_{scale}"))
            .because("scaled support evidence should remain declaration scoped")
            .materialize()
            .expect("scaled support should materialize");
        contribution_digests.push(support.materialization_digest().to_string());

        for _ in 0..scale {
            let report = success(materialize_intent_admission_support_traceability_report(
                admitted_ready(plan_support_requested(&admitted_plan)),
            ));
            support_width += report.rows().len();
            support_digests.push(report.decision_support_traceability_digest().to_string());
        }
    }

    if categories.contains(&ForgeQueryDomainCapabilityCategory::WorkflowPreview) {
        let workflow = forge_query_domain("worth.spatial")
            .for_intent(&declaration)
            .plans_preview_mutation(
                format!("workflow.scale_{scale}"),
                crate::facade::runtime::BridgePreviewSessionIdentity::new(format!(
                    "preview-session:scale-{scale}"
                )),
            )
            .because("scaled workflow evidence should remain canonical")
            .materialize()
            .expect("scaled workflow should materialize");
        contribution_digests.push(
            workflow
                .workflow_declaration()
                .report()
                .declaration_digest()
                .to_string(),
        );
    }

    if categories.contains(&ForgeQueryDomainCapabilityCategory::ContinuityLineage) {
        let continuity = forge_query_domain("worth.spatial")
            .for_admitted_intent_plan(&admitted_plan)
            .preserves_continuity(
                format!("continuity.scale_{scale}"),
                format!("edge:{scale}:before"),
                format!("edge:{scale}:after"),
            )
            .because("scaled continuity evidence should remain canonical")
            .materialize()
            .expect("scaled continuity should materialize");
        contribution_digests.push(continuity.continuity_resolution_digest().to_string());
    }

    if categories.contains(&ForgeQueryDomainCapabilityCategory::ConsequenceAftermath) {
        let aftermath = forge_query_domain("worth.spatial")
            .for_admitted_intent_plan(&projection_plan)
            .consumes_projection_contract(
                format!("aftermath.scale_{scale}"),
                projection_contract_request(),
            )
            .because("scaled aftermath evidence should remain canonical")
            .materialize()
            .expect("scaled aftermath should materialize");
        contribution_digests.push(aftermath.contract_digest().to_string());
    }

    if categories.contains(&ForgeQueryDomainCapabilityCategory::ExplanationInspection) {
        let explanation = forge_query_domain("worth.spatial")
            .for_lower_runtime_boundary_envelope(&lower_runtime)
            .explains_store_backed_replay_gap(
                format!("explanation.scale_{scale}"),
                store_backed_replay_gap_request(),
            )
            .because("scaled explanation evidence should remain canonical")
            .materialize_artifact()
            .expect("scaled explanation should materialize");
        contribution_digests.push(explanation.artifact_digest().to_string());
    }

    if categories.contains(&ForgeQueryDomainCapabilityCategory::InvariantCapability) {
        let invariant = forge_query_domain("worth.spatial")
            .for_intent(&declaration)
            .register_invariant_catalog(
                format!("invariant.scale_{scale}"),
                forge_relational::facade::runtime::InvariantCatalog::default(),
            )
            .because("scaled invariant evidence should remain canonical")
            .materialize()
            .expect("scaled invariant should materialize");
        contribution_digests.push(invariant.materialization_digest().to_string());
    }

    for index in 0..scale {
        let review = ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
            crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
                intent_declaration(&format!("trace.scale.{scale}.{index}")),
            )
            .expect("scaled deferred-neighbor request should build"),
        );
        let trace = review
            .decision_trace_envelope()
            .expect("scaled review should preserve decision trace")
            .clone();
        trace_width += trace.rows().len();
        trace_digests.push(trace.trace_digest().to_string());
    }

    ForgeQueryDomainCapabilityScaledEvidence {
        contribution_width: contribution_digests.len(),
        trace_width,
        category_width: categories.len(),
        support_width,
        contribution_digest: hash_parts(&contribution_digests),
        trace_digest: hash_parts(&trace_digests),
        category_digest: hash_parts(
            &categories
                .iter()
                .map(|category| category.as_str().to_string())
                .collect::<Vec<_>>(),
        ),
        support_digest: hash_parts(&support_digests),
    }
}

fn category_set_for_scale(scale: usize) -> Vec<ForgeQueryDomainCapabilityCategory> {
    match scale {
        1 => vec![
            ForgeQueryDomainCapabilityCategory::Admission,
            ForgeQueryDomainCapabilityCategory::SupportTraceability,
            ForgeQueryDomainCapabilityCategory::WorkflowPreview,
        ],
        2 => vec![
            ForgeQueryDomainCapabilityCategory::Admission,
            ForgeQueryDomainCapabilityCategory::SupportTraceability,
            ForgeQueryDomainCapabilityCategory::WorkflowPreview,
            ForgeQueryDomainCapabilityCategory::ContinuityLineage,
            ForgeQueryDomainCapabilityCategory::ConsequenceAftermath,
        ],
        _ => vec![
            ForgeQueryDomainCapabilityCategory::Admission,
            ForgeQueryDomainCapabilityCategory::SupportTraceability,
            ForgeQueryDomainCapabilityCategory::WorkflowPreview,
            ForgeQueryDomainCapabilityCategory::ContinuityLineage,
            ForgeQueryDomainCapabilityCategory::ConsequenceAftermath,
            ForgeQueryDomainCapabilityCategory::ExplanationInspection,
            ForgeQueryDomainCapabilityCategory::InvariantCapability,
        ],
    }
}

fn admission_digest(decision: &crate::runtime::ForgeQueryIntentAdmissionDecision) -> String {
    match decision {
        crate::runtime::ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            plan.decision_digest().to_string()
        }
        crate::runtime::ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            advisory.decision_digest().to_string()
        }
        crate::runtime::ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            violation.decision_digest().to_string()
        }
    }
}
