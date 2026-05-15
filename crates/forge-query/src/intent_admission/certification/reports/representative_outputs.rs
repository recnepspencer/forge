use crate::identity::hash_parts;
use crate::intent_admission::{
    ForgeQueryIntentDecisionTraceEnvelope, ForgeQueryIntentDecisionTraceEvidence,
    ForgeQueryIntentDecisionTraceRow, ForgeQueryIntentEligibilityTraceEvidence,
};

use super::super::fixtures::{
    certified_admitted_intent_fixture, certified_advisory_intent_fixture,
    certified_failure_intent_fixture, certified_violation_intent_fixture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionRepresentativeOutputReport {
    outputs: Vec<(&'static str, String)>,
    report_digest: String,
}

impl ForgeQueryIntentAdmissionRepresentativeOutputReport {
    pub fn digest_for(&self, name: &str) -> Option<&str> {
        self.outputs
            .iter()
            .find(|(output_name, _)| *output_name == name)
            .map(|(_, digest)| digest.as_str())
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn forge_query_intent_admission_representative_output_report(
) -> ForgeQueryIntentAdmissionRepresentativeOutputReport {
    let admitted = certified_admitted_intent_fixture();
    let advisory = certified_advisory_intent_fixture();
    let violation = certified_violation_intent_fixture();
    let failure = certified_failure_intent_fixture();
    let admitted_eligibility = eligibility_evidence(&admitted.trace);
    let advisory_eligibility = eligibility_evidence(&advisory.trace);
    let violation_eligibility = eligibility_evidence(&violation.trace);
    let failure_eligibility = eligibility_evidence(&failure.trace);
    let outputs = vec![
        output(
            "query_digest",
            hash_parts(&[
                admitted.receipt.receipt_digest().to_string(),
                advisory.trace.trace_digest().to_string(),
                violation.trace.trace_digest().to_string(),
                failure.failure_digest.clone(),
            ]),
        ),
        output(
            "raw_intent_digest",
            hash_parts(&[
                admitted.request.request_digest().to_string(),
                advisory.request.request_digest().to_string(),
                violation.request.request_digest().to_string(),
                failure.request.request_digest().to_string(),
            ]),
        ),
        output(
            "intent_eligibility_digest",
            hash_parts(&[
                admitted_eligibility.eligibility_digest().to_string(),
                advisory_eligibility.eligibility_digest().to_string(),
                violation_eligibility.eligibility_digest().to_string(),
                failure_eligibility.eligibility_digest().to_string(),
            ]),
        ),
        output(
            "admission_decision_digest",
            admitted.plan.decision_digest().to_string(),
        ),
        output(
            "admitted_intent_plan_digest",
            admitted_plan_digest(&admitted.plan),
        ),
        output(
            "admitted_execution_handoff_digest",
            admitted.handoff.handoff_digest().to_string(),
        ),
        output(
            "advisory_decision_digest",
            decision_digest(&advisory.decision),
        ),
        output(
            "violation_decision_digest",
            decision_digest(&violation.decision),
        ),
        output(
            "decision_trace_digest",
            trace_row_digest(&[
                &admitted.trace,
                &advisory.trace,
                &violation.trace,
                &failure.trace,
            ]),
        ),
        output(
            "decision_trace_envelope_digest",
            hash_parts(&[
                admitted.trace.trace_digest().to_string(),
                advisory.trace.trace_digest().to_string(),
                violation.trace.trace_digest().to_string(),
                failure.trace.trace_digest().to_string(),
            ]),
        ),
        output(
            "policy_decision_digest",
            posture_digest(
                admitted_eligibility.policy_posture(),
                advisory_eligibility.policy_posture(),
                violation_eligibility.policy_posture(),
                failure_eligibility.policy_posture(),
            ),
        ),
        output(
            "capability_decision_digest",
            posture_digest(
                admitted_eligibility.capability_posture(),
                advisory_eligibility.capability_posture(),
                violation_eligibility.capability_posture(),
                failure_eligibility.capability_posture(),
            ),
        ),
        output(
            "invariant_decision_digest",
            posture_digest(
                admitted_eligibility.invariant_posture(),
                advisory_eligibility.invariant_posture(),
                violation_eligibility.invariant_posture(),
                failure_eligibility.invariant_posture(),
            ),
        ),
        output(
            "basis_decision_digest",
            posture_digest(
                admitted_eligibility.basis_posture(),
                advisory_eligibility.basis_posture(),
                violation_eligibility.basis_posture(),
                failure_eligibility.basis_posture(),
            ),
        ),
        output(
            "projection_decision_digest",
            posture_digest(
                admitted_eligibility.projection_source_posture(),
                advisory_eligibility.projection_source_posture(),
                violation_eligibility.projection_source_posture(),
                failure_eligibility.projection_source_posture(),
            ),
        ),
        output(
            "routing_posture_digest",
            posture_digest(
                admitted_eligibility.routing_support_posture(),
                advisory_eligibility.routing_support_posture(),
                violation_eligibility.routing_support_posture(),
                failure_eligibility.routing_support_posture(),
            ),
        ),
        output(
            "execution_provenance_chain_digest",
            hash_parts(&[
                admitted
                    .receipt
                    .execution_provenance_chain_digest()
                    .to_string(),
                failure.execution_provenance_chain_digest.clone(),
                admitted.binding.binding_digest().to_string(),
            ]),
        ),
        output("failure_digest", failure.failure_digest.clone()),
    ];
    let report_digest = hash_parts(
        &outputs
            .iter()
            .map(|(name, digest)| format!("{name}:{digest}"))
            .collect::<Vec<_>>(),
    );
    ForgeQueryIntentAdmissionRepresentativeOutputReport {
        outputs,
        report_digest,
    }
}

fn output(name: &'static str, digest: String) -> (&'static str, String) {
    (name, digest)
}

fn decision_digest(decision: &crate::facade::runtime::ForgeQueryIntentAdmissionDecision) -> String {
    match decision {
        crate::facade::runtime::ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            plan.decision_digest().to_string()
        }
        crate::facade::runtime::ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            advisory.decision_digest().to_string()
        }
        crate::facade::runtime::ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            violation.decision_digest().to_string()
        }
    }
}

fn admitted_plan_digest(plan: &crate::intent_admission::ForgeQueryAdmittedIntentPlan) -> String {
    hash_parts(&[
        "forge_query_intent_admission_representative_plan_v1".to_string(),
        format!("family:{}", plan.family().as_str()),
        format!("entrypoint:{}", plan.entrypoint().as_str()),
        format!(
            "execution_seam:{}",
            plan.execution_seam()
                .map(|seam| seam.as_str())
                .unwrap_or("no-execution-handoff")
        ),
        format!("request:{}", plan.request_digest()),
        format!("eligibility:{}", plan.eligibility_digest()),
        format!("decision:{}", plan.decision_digest()),
    ])
}

fn trace_row_digest(traces: &[&ForgeQueryIntentDecisionTraceEnvelope]) -> String {
    hash_parts(
        &traces
            .iter()
            .flat_map(|trace| {
                trace
                    .rows()
                    .iter()
                    .map(ForgeQueryIntentDecisionTraceRow::row_digest)
            })
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
}

fn eligibility_evidence(
    trace: &ForgeQueryIntentDecisionTraceEnvelope,
) -> &ForgeQueryIntentEligibilityTraceEvidence {
    trace
        .rows()
        .iter()
        .find_map(|row| match row.evidence() {
            ForgeQueryIntentDecisionTraceEvidence::Eligibility(evidence) => Some(evidence),
            _ => None,
        })
        .expect("representative trace should preserve eligibility evidence")
}

fn posture_digest<T: std::fmt::Debug>(a: T, b: T, c: T, d: T) -> String {
    hash_parts(&[
        format!("{a:?}"),
        format!("{b:?}"),
        format!("{c:?}"),
        format!("{d:?}"),
    ])
}
