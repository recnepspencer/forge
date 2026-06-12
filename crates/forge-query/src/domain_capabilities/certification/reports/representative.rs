use crate::domain_capabilities::canonical_runtime::{
    materialize_canonical_admission_artifact, materialize_canonical_aftermath_artifact,
    materialize_canonical_continuity_artifact, materialize_canonical_explanation_artifact,
    materialize_canonical_invariant_capability_artifact,
    materialize_canonical_support_traceability_artifact, materialize_canonical_workflow_artifact,
    materialize_graph_composition_capability_support_row,
    materialize_graph_composition_domain_invariant_denial,
    materialize_intent_admission_support_traceability_report,
    materialize_intent_declaration_support_traceability_artifact,
    materialize_projection_consumption_contract, materialize_query_causal_inspection_artifact,
    materialize_query_invariant_catalog_registration_artifact,
    materialize_query_workflow_declaration, materialize_runtime_admission_decision,
    materialize_runtime_continuity_evidence,
};
use crate::domain_capabilities::certification::reports::fixtures::{
    admission_requested, admitted_basis_observation_plan, admitted_projection_consumption_plan,
    admitted_ready, aftermath_requested, capability_requested, continuity_requested,
    explanation_requested, intent_declaration, invariant_denial_requested, invariant_requested,
    lower_runtime_envelope, plain_support_requested, plan_support_requested,
    projection_contract_request, store_backed_replay_gap_request, success,
    support_traceability_requested, workflow_requested,
};
use crate::domain_capabilities::certification::{
    forge_query_domain_capability_compile_fail_boundary_digest,
    forge_query_domain_capability_public_surface_inventory,
};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, forge_query_domain,
    ForgeQuerySupportContributionAuthoring,
};
use crate::identity::hash_parts;
use crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData;
use crate::intent_admission::ForgeQueryIntentAdmissionCoveredEntrypoint;
use forge_relational::facade::runtime::InvariantCatalog;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityRepresentativeReport {
    query_digest: String,
    intent_declaration_digest: String,
    domain_capability_contribution_request_digest: String,
    domain_capability_contribution_eligibility_digest: String,
    admitted_domain_capability_contribution_digest: String,
    canonical_runtime_materialization_digest: String,
    admission_artifact_digest: String,
    support_artifact_digest: String,
    workflow_artifact_digest: String,
    continuity_artifact_digest: String,
    aftermath_artifact_digest: String,
    explanation_artifact_digest: String,
    capability_support_row_digest: String,
    domain_invariant_denial_digest: String,
    decision_trace_digest: String,
    support_traceability_digest: String,
    public_boundary_digest: String,
    compile_fail_boundary_digest: String,
    failure_digest: String,
    contribution_width: usize,
    trace_width: usize,
    category_width: usize,
    support_width: usize,
}

impl ForgeQueryDomainCapabilityRepresentativeReport {
    pub fn digest_for(&self, name: &str) -> Option<String> {
        match name {
            "query_digest" => Some(self.query_digest.clone()),
            "intent_declaration_digest" => Some(self.intent_declaration_digest.clone()),
            "domain_capability_contribution_request_digest" => {
                Some(self.domain_capability_contribution_request_digest.clone())
            }
            "domain_capability_contribution_eligibility_digest" => Some(
                self.domain_capability_contribution_eligibility_digest
                    .clone(),
            ),
            "admitted_domain_capability_contribution_digest" => {
                Some(self.admitted_domain_capability_contribution_digest.clone())
            }
            "canonical_runtime_materialization_digest" => {
                Some(self.canonical_runtime_materialization_digest.clone())
            }
            "admission_artifact_digest" => Some(self.admission_artifact_digest.clone()),
            "support_artifact_digest" => Some(self.support_artifact_digest.clone()),
            "workflow_artifact_digest" => Some(self.workflow_artifact_digest.clone()),
            "continuity_artifact_digest" => Some(self.continuity_artifact_digest.clone()),
            "aftermath_artifact_digest" => Some(self.aftermath_artifact_digest.clone()),
            "explanation_artifact_digest" => Some(self.explanation_artifact_digest.clone()),
            "capability_support_row_digest" => Some(self.capability_support_row_digest.clone()),
            "domain_invariant_denial_digest" => Some(self.domain_invariant_denial_digest.clone()),
            "decision_trace_digest" => Some(self.decision_trace_digest.clone()),
            "support_traceability_digest" => Some(self.support_traceability_digest.clone()),
            "public_boundary_digest" => Some(self.public_boundary_digest.clone()),
            "compile_fail_boundary_digest" => Some(self.compile_fail_boundary_digest.clone()),
            "failure_digest" => Some(self.failure_digest.clone()),
            "contribution_width" => Some(self.contribution_width.to_string()),
            "trace_width" => Some(self.trace_width.to_string()),
            "category_width" => Some(self.category_width.to_string()),
            "support_width" => Some(self.support_width.to_string()),
            _ => None,
        }
    }

    pub fn contribution_width(&self) -> usize {
        self.contribution_width
    }

    pub fn trace_width(&self) -> usize {
        self.trace_width
    }

    pub fn category_width(&self) -> usize {
        self.category_width
    }

    pub fn support_width(&self) -> usize {
        self.support_width
    }
}

pub fn forge_query_domain_capability_representative_report(
) -> ForgeQueryDomainCapabilityRepresentativeReport {
    let declaration = intent_declaration("domain-capability-certification");
    let admitted_plan = admitted_basis_observation_plan();
    let projection_plan = admitted_projection_consumption_plan();
    let lower_runtime = lower_runtime_envelope("domain-capability-certification");
    let public_boundary_digest =
        forge_query_domain_capability_public_surface_inventory().public_surface_digest();

    let support_requested = support_traceability_requested(&declaration);
    let request_digest = support_requested.payload().request_digest().to_string();
    let support_eligible = success(evaluate_requested_domain_capability_contribution(
        support_requested,
    ));
    let eligibility_digest = support_eligible.eligibility_digest();
    let support_admitted = success(admit_eligible_domain_capability_contribution(
        support_eligible,
    ));
    let admitted_digest = support_admitted.admitted_digest();

    let support_artifact = success(
        materialize_intent_declaration_support_traceability_artifact(admitted_ready(
            support_traceability_requested(&declaration),
        )),
    );
    let support_common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_traceability("traceability.edge_split")
        .because("declaration-scoped support remains declaration scoped")
        .materialize()
        .expect("common support lane should materialize");
    assert_eq!(support_common, support_artifact);

    let different_support = success(
        materialize_intent_declaration_support_traceability_artifact(admitted_ready(
            plain_support_requested(&declaration),
        )),
    );
    assert_ne!(
        support_artifact.materialization_digest(),
        different_support.materialization_digest()
    );
    assert_eq!(support_artifact.intent_name(), declaration.name());

    let admission_decision = success(materialize_runtime_admission_decision(admitted_ready(
        admission_requested(&admitted_plan),
    )));
    let admission_common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan)
        .advises("admission.routing_gap")
        .because("runtime routing still needs clarification")
        .materialize()
        .expect("common admission lane should materialize");
    assert_eq!(admission_common, admission_decision);

    let workflow_declaration = success(materialize_query_workflow_declaration(admitted_ready(
        workflow_requested(&declaration),
    )));
    let workflow_common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .plans_preview_mutation(
            "workflow.preview_mutation",
            crate::facade::runtime::BridgePreviewSessionIdentity::from_stable_name(
                "preview-session:certification",
            ),
        )
        .because("preview mutation planning should preserve canonical workflow semantics")
        .materialize()
        .expect("common workflow lane should materialize");
    assert_eq!(
        workflow_common.workflow_declaration(),
        &workflow_declaration
    );

    let continuity_evidence = success(materialize_runtime_continuity_evidence(admitted_ready(
        continuity_requested(&admitted_plan),
    )));
    let continuity_common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&admitted_plan)
        .preserves_continuity("continuity.edge_split", "edge:before", "edge:after")
        .because("edge split preserves one authoritative successor")
        .materialize()
        .expect("common continuity lane should materialize");
    assert_eq!(continuity_common, continuity_evidence);

    let projection_contract = success(materialize_projection_consumption_contract(admitted_ready(
        aftermath_requested(&projection_plan),
    )));
    let aftermath_common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&projection_plan)
        .consumes_projection_contract(
            "aftermath.projection_contract",
            projection_contract_request(),
        )
        .because("projection aftermath should bind a stable contract")
        .materialize()
        .expect("common aftermath lane should materialize");
    assert_eq!(aftermath_common, projection_contract);

    let explanation_artifact = success(materialize_query_causal_inspection_artifact(
        admitted_ready(explanation_requested(&lower_runtime)),
    ));
    let explanation_common = forge_query_domain("worth.spatial")
        .for_lower_runtime_boundary_envelope(&lower_runtime)
        .explains_store_backed_replay_gap(
            "explanation.store_backed_replay",
            store_backed_replay_gap_request(),
        )
        .because("store-backed replay should preserve denied explanation identity")
        .materialize_artifact()
        .expect("common explanation lane should materialize");
    assert_eq!(explanation_common, explanation_artifact);

    let invariant_artifact = success(materialize_query_invariant_catalog_registration_artifact(
        admitted_ready(invariant_requested(&declaration)),
    ));
    let invariant_common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .register_invariant_catalog("invariant.edge_split", InvariantCatalog::default())
        .because("geometry kernel must reject invalid edge splits")
        .materialize()
        .expect("common invariant lane should materialize");
    assert_eq!(invariant_common, invariant_artifact);

    let support_traceability_report =
        success(materialize_intent_admission_support_traceability_report(
            admitted_ready(plan_support_requested(&admitted_plan)),
        ));

    let capability_row = success(materialize_graph_composition_capability_support_row(
        admitted_ready(capability_requested(&lower_runtime)),
    ));
    let invariant_denial = success(materialize_graph_composition_domain_invariant_denial(
        admitted_ready(invariant_denial_requested(&lower_runtime)),
    ));

    let advisory_review = ForgeQueryRuntimeIntentAdmissionReviewData::from_request(
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::deferred_neighbor(
            ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
            intent_declaration("certification.trace"),
        )
        .expect("deferred-neighbor request should build"),
    );
    let decision_trace = advisory_review
        .decision_trace_envelope()
        .expect("advisory review should preserve decision trace")
        .clone();

    let denial = match evaluate_requested_domain_capability_contribution(
        ForgeQuerySupportContributionAuthoring::declaration_support(
            "worth.spatial.support.failure",
            "",
        )
        .for_intent_declaration(&declaration),
    ) {
        forge_proof::TransitionOutcome::Denied(denial) => denial,
        _ => panic!("expected typed denial"),
    };

    ForgeQueryDomainCapabilityRepresentativeReport {
        query_digest: projection_contract
            .query_digest()
            .expect("projection contract should preserve query digest")
            .to_string(),
        intent_declaration_digest: declaration.input_digest(),
        domain_capability_contribution_request_digest: request_digest,
        domain_capability_contribution_eligibility_digest: eligibility_digest,
        admitted_domain_capability_contribution_digest: admitted_digest,
        canonical_runtime_materialization_digest: hash_parts(&[
            materialize_canonical_admission_artifact(admitted_ready(admission_requested(
                &admitted_plan,
            )))
            .materialization_digest()
            .to_string(),
            materialize_canonical_support_traceability_artifact(admitted_ready(
                support_traceability_requested(&declaration),
            ))
            .materialization_digest()
            .to_string(),
            materialize_canonical_invariant_capability_artifact(admitted_ready(
                invariant_requested(&declaration),
            ))
            .materialization_digest()
            .to_string(),
            materialize_canonical_workflow_artifact(admitted_ready(workflow_requested(
                &declaration,
            )))
            .materialization_digest()
            .to_string(),
            materialize_canonical_continuity_artifact(admitted_ready(continuity_requested(
                &admitted_plan,
            )))
            .materialization_digest()
            .to_string(),
            materialize_canonical_aftermath_artifact(admitted_ready(aftermath_requested(
                &projection_plan,
            )))
            .materialization_digest()
            .to_string(),
            materialize_canonical_explanation_artifact(admitted_ready(explanation_requested(
                &lower_runtime,
            )))
            .materialization_digest()
            .to_string(),
        ]),
        admission_artifact_digest: decision_digest(&admission_decision),
        support_artifact_digest: support_artifact.materialization_digest().to_string(),
        workflow_artifact_digest: workflow_declaration
            .report()
            .declaration_digest()
            .to_string(),
        continuity_artifact_digest: continuity_evidence
            .continuity_resolution_digest()
            .as_str()
            .to_string(),
        aftermath_artifact_digest: projection_contract.contract_digest().to_string(),
        explanation_artifact_digest: explanation_artifact.artifact_digest().to_string(),
        capability_support_row_digest: capability_row.row_digest().to_string(),
        domain_invariant_denial_digest: invariant_denial.denial_digest().to_string(),
        decision_trace_digest: decision_trace.trace_digest().to_string(),
        support_traceability_digest: support_traceability_report
            .decision_support_traceability_digest()
            .to_string(),
        public_boundary_digest,
        compile_fail_boundary_digest: forge_query_domain_capability_compile_fail_boundary_digest(),
        failure_digest: denial.failure_digest(),
        contribution_width: forge_query_domain_capability_public_surface_inventory()
            .rows()
            .len(),
        trace_width: decision_trace.rows().len(),
        category_width: forge_query_domain_capability_public_surface_inventory()
            .rows()
            .len(),
        support_width: support_traceability_report.rows().len() + 2,
    }
}

fn decision_digest(decision: &crate::runtime::ForgeQueryIntentAdmissionDecision) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn representative_report_emits_all_required_foundation_digests() {
        let report = forge_query_domain_capability_representative_report();

        for key in [
            "query_digest",
            "intent_declaration_digest",
            "domain_capability_contribution_request_digest",
            "admission_artifact_digest",
            "support_artifact_digest",
            "workflow_artifact_digest",
            "continuity_artifact_digest",
            "aftermath_artifact_digest",
            "explanation_artifact_digest",
            "capability_support_row_digest",
            "domain_invariant_denial_digest",
            "decision_trace_digest",
            "support_traceability_digest",
            "failure_digest",
        ] {
            assert!(report
                .digest_for(key)
                .is_some_and(|digest| !digest.is_empty()));
        }
        assert!(report.trace_width() > 0);
        assert!(report.support_width() >= 2);
    }
}
