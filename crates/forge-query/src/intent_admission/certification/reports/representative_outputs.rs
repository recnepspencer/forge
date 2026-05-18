use crate::facade::runtime::ForgeQueryIntentAdmissionDecision;
use crate::identity::hash_parts;
use crate::intent_admission::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDecisionTraceEnvelope,
    ForgeQueryIntentDecisionTraceEvidence, ForgeQueryIntentDecisionTraceRow,
    ForgeQueryIntentEligibilityTraceEvidence,
};

use super::super::fixtures::{
    certified_admitted_intent_fixture, certified_advisory_intent_fixture,
    certified_failure_intent_fixture, certified_read_intent_fixture,
    certified_violation_intent_fixture, CertifiedAdmittedIntentFixture,
    CertifiedAdvisoryIntentFixture, CertifiedFailureIntentFixture, CertifiedReadIntentFixture,
    CertifiedViolationIntentFixture,
};

type OutputRow = (&'static str, String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionRepresentativeOutputReport {
    outputs: Vec<OutputRow>,
    report_digest: String,
}

#[derive(Clone)]
struct RepresentativeOutputFixtures {
    admitted: CertifiedAdmittedIntentFixture,
    advisory: CertifiedAdvisoryIntentFixture,
    violation: CertifiedViolationIntentFixture,
    failure: CertifiedFailureIntentFixture,
    read: CertifiedReadIntentFixture,
}

#[derive(Clone)]
struct RepresentativeEligibilityDigests {
    admitted: ForgeQueryIntentEligibilityTraceEvidence,
    advisory: ForgeQueryIntentEligibilityTraceEvidence,
    violation: ForgeQueryIntentEligibilityTraceEvidence,
    failure: ForgeQueryIntentEligibilityTraceEvidence,
    read: ForgeQueryIntentEligibilityTraceEvidence,
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
    let fixtures = RepresentativeOutputFixtures::load();
    let eligibility = fixtures.eligibility();
    let outputs = representative_outputs(&fixtures, &eligibility);
    let report_digest = digest_output_rows(&outputs);
    ForgeQueryIntentAdmissionRepresentativeOutputReport {
        outputs,
        report_digest,
    }
}

impl RepresentativeOutputFixtures {
    fn load() -> Self {
        Self {
            admitted: certified_admitted_intent_fixture(),
            advisory: certified_advisory_intent_fixture(),
            violation: certified_violation_intent_fixture(),
            failure: certified_failure_intent_fixture(),
            read: certified_read_intent_fixture(),
        }
    }

    fn eligibility(&self) -> RepresentativeEligibilityDigests {
        RepresentativeEligibilityDigests {
            admitted: eligibility_evidence(&self.admitted.trace).clone(),
            advisory: eligibility_evidence(&self.advisory.trace).clone(),
            violation: eligibility_evidence(&self.violation.trace).clone(),
            failure: eligibility_evidence(&self.failure.trace).clone(),
            read: eligibility_evidence(&self.read.trace).clone(),
        }
    }
}

fn representative_outputs(
    fixtures: &RepresentativeOutputFixtures,
    eligibility: &RepresentativeEligibilityDigests,
) -> Vec<OutputRow> {
    vec![
        query_digest_output(fixtures),
        raw_intent_digest_output(fixtures),
        intent_eligibility_digest_output(eligibility),
        admission_decision_digest_output(fixtures),
        admitted_intent_plan_digest_output(fixtures),
        admitted_execution_handoff_digest_output(fixtures),
        advisory_decision_digest_output(fixtures),
        violation_decision_digest_output(fixtures),
        decision_trace_digest_output(fixtures),
        decision_trace_envelope_digest_output(fixtures),
        posture_output(
            "policy_decision_digest",
            [
                eligibility.admitted.policy_posture(),
                eligibility.advisory.policy_posture(),
                eligibility.violation.policy_posture(),
                eligibility.failure.policy_posture(),
                eligibility.read.policy_posture(),
            ],
        ),
        posture_output(
            "capability_decision_digest",
            [
                eligibility.admitted.capability_posture(),
                eligibility.advisory.capability_posture(),
                eligibility.violation.capability_posture(),
                eligibility.failure.capability_posture(),
                eligibility.read.capability_posture(),
            ],
        ),
        posture_output(
            "invariant_decision_digest",
            [
                eligibility.admitted.invariant_posture(),
                eligibility.advisory.invariant_posture(),
                eligibility.violation.invariant_posture(),
                eligibility.failure.invariant_posture(),
                eligibility.read.invariant_posture(),
            ],
        ),
        posture_output(
            "basis_decision_digest",
            [
                eligibility.admitted.basis_posture(),
                eligibility.advisory.basis_posture(),
                eligibility.violation.basis_posture(),
                eligibility.failure.basis_posture(),
                eligibility.read.basis_posture(),
            ],
        ),
        posture_output(
            "projection_decision_digest",
            [
                eligibility.admitted.projection_source_posture(),
                eligibility.advisory.projection_source_posture(),
                eligibility.violation.projection_source_posture(),
                eligibility.failure.projection_source_posture(),
                eligibility.read.projection_source_posture(),
            ],
        ),
        posture_output(
            "routing_posture_digest",
            [
                eligibility.admitted.routing_support_posture(),
                eligibility.advisory.routing_support_posture(),
                eligibility.violation.routing_support_posture(),
                eligibility.failure.routing_support_posture(),
                eligibility.read.routing_support_posture(),
            ],
        ),
        execution_provenance_chain_digest_output(fixtures),
        output("failure_digest", fixtures.failure.failure_digest.clone()),
    ]
}

fn query_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "query_digest",
        hash_parts(&[
            fixtures.admitted.receipt.receipt_digest().to_string(),
            fixtures.advisory.trace.trace_digest().to_string(),
            fixtures.violation.trace.trace_digest().to_string(),
            fixtures.failure.failure_digest.clone(),
            fixtures.read.result.receipt().result_digest().to_string(),
        ]),
    )
}

fn raw_intent_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "raw_intent_digest",
        hash_parts(&[
            fixtures.admitted.request.request_digest().to_string(),
            fixtures.advisory.request.request_digest().to_string(),
            fixtures.violation.request.request_digest().to_string(),
            fixtures.failure.request.request_digest().to_string(),
            fixtures.read.request.request_digest().to_string(),
        ]),
    )
}

fn intent_eligibility_digest_output(eligibility: &RepresentativeEligibilityDigests) -> OutputRow {
    output(
        "intent_eligibility_digest",
        hash_parts(&[
            eligibility.admitted.eligibility_digest().to_string(),
            eligibility.advisory.eligibility_digest().to_string(),
            eligibility.violation.eligibility_digest().to_string(),
            eligibility.failure.eligibility_digest().to_string(),
            eligibility.read.eligibility_digest().to_string(),
        ]),
    )
}

fn admission_decision_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "admission_decision_digest",
        hash_parts(&[
            fixtures.admitted.plan.decision_digest().to_string(),
            fixtures.read.plan.decision_digest().to_string(),
        ]),
    )
}

fn admitted_intent_plan_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "admitted_intent_plan_digest",
        hash_parts(&[
            admitted_plan_digest(&fixtures.admitted.plan),
            admitted_plan_digest(&ForgeQueryAdmittedIntentPlan::ReadExecution(
                fixtures.read.plan.clone(),
            )),
        ]),
    )
}

fn admitted_execution_handoff_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "admitted_execution_handoff_digest",
        hash_parts(&[
            fixtures.admitted.handoff.handoff_digest().to_string(),
            fixtures.read.handoff.handoff_digest().to_string(),
        ]),
    )
}

fn advisory_decision_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "advisory_decision_digest",
        decision_digest(&fixtures.advisory.decision),
    )
}

fn violation_decision_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "violation_decision_digest",
        decision_digest(&fixtures.violation.decision),
    )
}

fn decision_trace_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "decision_trace_digest",
        trace_row_digest(&[
            &fixtures.admitted.trace,
            &fixtures.advisory.trace,
            &fixtures.violation.trace,
            &fixtures.failure.trace,
        ]),
    )
}

fn decision_trace_envelope_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "decision_trace_envelope_digest",
        hash_parts(&[
            fixtures.admitted.trace.trace_digest().to_string(),
            fixtures.advisory.trace.trace_digest().to_string(),
            fixtures.violation.trace.trace_digest().to_string(),
            fixtures.failure.trace.trace_digest().to_string(),
            fixtures.read.trace.trace_digest().to_string(),
        ]),
    )
}

fn execution_provenance_chain_digest_output(fixtures: &RepresentativeOutputFixtures) -> OutputRow {
    output(
        "execution_provenance_chain_digest",
        hash_parts(&[
            fixtures
                .admitted
                .receipt
                .execution_provenance_chain_digest()
                .to_string(),
            fixtures.failure.execution_provenance_chain_digest.clone(),
            fixtures.admitted.binding.binding_digest().to_string(),
            fixtures
                .read
                .result
                .receipt()
                .execution_provenance_chain_digest()
                .unwrap_or("no-read-provenance")
                .to_string(),
            fixtures.read.binding.binding_digest().to_string(),
        ]),
    )
}

fn posture_output<T: std::fmt::Debug, const N: usize>(
    name: &'static str,
    values: [T; N],
) -> OutputRow {
    output(name, posture_digest(values))
}

fn output(name: &'static str, digest: String) -> OutputRow {
    (name, digest)
}

fn digest_output_rows(outputs: &[OutputRow]) -> String {
    hash_parts(
        &outputs
            .iter()
            .map(|(name, digest)| format!("{name}:{digest}"))
            .collect::<Vec<_>>(),
    )
}

fn decision_digest(decision: &ForgeQueryIntentAdmissionDecision) -> String {
    match decision {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan.decision_digest().to_string(),
        ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            advisory.decision_digest().to_string()
        }
        ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            violation.decision_digest().to_string()
        }
    }
}

fn admitted_plan_digest(plan: &ForgeQueryAdmittedIntentPlan) -> String {
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

fn posture_digest<T: std::fmt::Debug, const N: usize>(values: [T; N]) -> String {
    hash_parts(&values.map(|value| format!("{value:?}")))
}
