use crate::facade::runtime::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionDecision,
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentSourceLane,
};
use crate::identity::hash_parts;

pub(super) fn expected_admitted_digest() -> String {
    hash_parts(&expected_admitted_parts())
}

pub(super) fn expected_admitted_detail() -> String {
    expected_admitted_parts().join("|")
}

pub(super) fn actual_admitted_digest(
    fixture: &super::super::fixtures::CertifiedAdmittedIntentFixture,
) -> String {
    hash_parts(&actual_admitted_parts(fixture))
}

pub(super) fn actual_admitted_detail(
    fixture: &super::super::fixtures::CertifiedAdmittedIntentFixture,
) -> String {
    actual_admitted_parts(fixture).join("|")
}

fn expected_admitted_parts() -> Vec<String> {
    vec![
        "admitted".to_string(),
        ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentSourceLane::UserAuthored
            .as_str()
            .to_string(),
        "admitted".to_string(),
        "admitted".to_string(),
        "backend-intent-authority-route".to_string(),
        "execution-outcome".to_string(),
    ]
}

fn actual_admitted_parts(
    fixture: &super::super::fixtures::CertifiedAdmittedIntentFixture,
) -> Vec<String> {
    let evidence = fixture.trace.rows()[1]
        .eligibility_evidence()
        .expect("admitted fixture should carry eligibility evidence");
    vec![
        "admitted".to_string(),
        fixture.request.family().as_str().to_string(),
        fixture.request.entrypoint().as_str().to_string(),
        fixture
            .request
            .runtime_declaration()
            .expect("intent-admission oracle rows still use runtime declarations")
            .source_lane()
            .as_str()
            .to_string(),
        match fixture.decision {
            ForgeQueryIntentAdmissionDecision::Admitted(_) => "admitted".to_string(),
            _ => unreachable!("fixture is admitted"),
        },
        evidence.support_posture().as_str().to_string(),
        evidence.routing_support_posture().as_str().to_string(),
        fixture
            .receipt
            .decision_trace_envelope()
            .rows()
            .last()
            .expect("trace should not be empty")
            .stage()
            .as_str()
            .to_string(),
    ]
}

pub(super) fn expected_advisory_digest() -> String {
    hash_parts(&expected_advisory_parts())
}

pub(super) fn expected_advisory_detail() -> String {
    expected_advisory_parts().join("|")
}

pub(super) fn actual_advisory_digest(
    fixture: &super::super::fixtures::CertifiedAdvisoryIntentFixture,
) -> String {
    hash_parts(&actual_advisory_parts(fixture))
}

pub(super) fn actual_advisory_detail(
    fixture: &super::super::fixtures::CertifiedAdvisoryIntentFixture,
) -> String {
    actual_advisory_parts(fixture).join("|")
}

fn expected_advisory_parts() -> Vec<String> {
    vec![
        "advisory".to_string(),
        ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentSourceLane::UserAuthored
            .as_str()
            .to_string(),
        "advisory".to_string(),
        "admitted".to_string(),
        "backend-intent-authority-route".to_string(),
        "advisory-stop".to_string(),
    ]
}

fn actual_advisory_parts(
    fixture: &super::super::fixtures::CertifiedAdvisoryIntentFixture,
) -> Vec<String> {
    let evidence = fixture.trace.rows()[1]
        .eligibility_evidence()
        .expect("advisory fixture should carry eligibility evidence");
    vec![
        "advisory".to_string(),
        fixture.request.family().as_str().to_string(),
        fixture.request.entrypoint().as_str().to_string(),
        fixture
            .request
            .runtime_declaration()
            .expect("intent-admission oracle rows still use runtime declarations")
            .source_lane()
            .as_str()
            .to_string(),
        match fixture.decision {
            ForgeQueryIntentAdmissionDecision::Advisory(_) => "advisory".to_string(),
            _ => unreachable!("fixture is advisory"),
        },
        evidence.support_posture().as_str().to_string(),
        evidence.routing_support_posture().as_str().to_string(),
        fixture
            .trace
            .rows()
            .last()
            .expect("trace should not be empty")
            .stage()
            .as_str()
            .to_string(),
    ]
}

pub(super) fn expected_violation_digest() -> String {
    hash_parts(&expected_violation_parts())
}

pub(super) fn expected_violation_detail() -> String {
    expected_violation_parts().join("|")
}

pub(super) fn actual_violation_digest(
    fixture: &super::super::fixtures::CertifiedViolationIntentFixture,
) -> String {
    hash_parts(&actual_violation_parts(fixture))
}

pub(super) fn actual_violation_detail(
    fixture: &super::super::fixtures::CertifiedViolationIntentFixture,
) -> String {
    actual_violation_parts(fixture).join("|")
}

fn expected_violation_parts() -> Vec<String> {
    vec![
        "violation".to_string(),
        ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentSourceLane::UserAuthored
            .as_str()
            .to_string(),
        "violation".to_string(),
        "violation".to_string(),
        "user-authored".to_string(),
        "violation-stop".to_string(),
    ]
}

fn actual_violation_parts(
    fixture: &super::super::fixtures::CertifiedViolationIntentFixture,
) -> Vec<String> {
    let evidence = fixture.trace.rows()[1]
        .eligibility_evidence()
        .expect("violation fixture should carry eligibility evidence");
    vec![
        "violation".to_string(),
        fixture.request.family().as_str().to_string(),
        fixture.request.entrypoint().as_str().to_string(),
        fixture
            .request
            .runtime_declaration()
            .expect("intent-admission oracle rows still use runtime declarations")
            .source_lane()
            .as_str()
            .to_string(),
        match fixture.decision {
            ForgeQueryIntentAdmissionDecision::Violation(_) => "violation".to_string(),
            _ => unreachable!("fixture is violation"),
        },
        evidence.capability_posture().as_str().to_string(),
        evidence.source_lane_posture().as_str().to_string(),
        fixture
            .trace
            .rows()
            .last()
            .expect("trace should not be empty")
            .stage()
            .as_str()
            .to_string(),
    ]
}

pub(super) fn expected_deferred_digest() -> String {
    hash_parts(&expected_deferred_parts())
}

pub(super) fn expected_deferred_detail() -> String {
    expected_deferred_parts().join("|")
}

pub(super) fn actual_deferred_digest(
    fixture: &super::super::fixtures::CertifiedDeferredIntentFixture,
) -> String {
    hash_parts(&actual_deferred_parts(fixture))
}

pub(super) fn actual_deferred_detail(
    fixture: &super::super::fixtures::CertifiedDeferredIntentFixture,
) -> String {
    actual_deferred_parts(fixture).join("|")
}

fn expected_deferred_parts() -> Vec<String> {
    vec![
        "deferred".to_string(),
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred
            .as_str()
            .to_string(),
        ForgeQueryIntentSourceLane::UserAuthored
            .as_str()
            .to_string(),
        "deferred".to_string(),
        "deferred".to_string(),
        "inspection-materialization-neighbor-deferred-until-covered".to_string(),
        "advisory-stop".to_string(),
    ]
}

fn actual_deferred_parts(
    fixture: &super::super::fixtures::CertifiedDeferredIntentFixture,
) -> Vec<String> {
    let evidence = fixture.trace.rows()[1]
        .eligibility_evidence()
        .expect("deferred fixture should carry eligibility evidence");
    vec![
        "deferred".to_string(),
        fixture.request.family().as_str().to_string(),
        fixture.request.entrypoint().as_str().to_string(),
        fixture
            .request
            .runtime_declaration()
            .expect("intent-admission oracle rows still use runtime declarations")
            .source_lane()
            .as_str()
            .to_string(),
        match fixture.decision {
            ForgeQueryIntentAdmissionDecision::Advisory(_) => "deferred".to_string(),
            _ => unreachable!("fixture is deferred"),
        },
        evidence.support_posture().as_str().to_string(),
        evidence.routing_support_posture().as_str().to_string(),
        fixture
            .trace
            .rows()
            .last()
            .expect("trace should not be empty")
            .stage()
            .as_str()
            .to_string(),
    ]
}

pub(super) fn expected_unsupported_digest() -> String {
    hash_parts(&expected_unsupported_parts())
}

pub(super) fn expected_unsupported_detail() -> String {
    expected_unsupported_parts().join("|")
}

pub(super) fn actual_unsupported_digest(
    fixture: &super::super::fixtures::CertifiedUnsupportedIntentFixture,
) -> String {
    hash_parts(&actual_unsupported_parts(fixture))
}

pub(super) fn actual_unsupported_detail(
    fixture: &super::super::fixtures::CertifiedUnsupportedIntentFixture,
) -> String {
    actual_unsupported_parts(fixture).join("|")
}

fn expected_unsupported_parts() -> Vec<String> {
    vec![
        "unsupported".to_string(),
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent
            .as_str()
            .to_string(),
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred
            .as_str()
            .to_string(),
        ForgeQueryIntentSourceLane::UserAuthored
            .as_str()
            .to_string(),
        "unsupported".to_string(),
        "deferred".to_string(),
        "inspection-materialization-neighbor-deferred-until-covered".to_string(),
        "violation-stop".to_string(),
    ]
}

fn actual_unsupported_parts(
    fixture: &super::super::fixtures::CertifiedUnsupportedIntentFixture,
) -> Vec<String> {
    let evidence = fixture.trace.rows()[1]
        .eligibility_evidence()
        .expect("unsupported fixture should carry eligibility evidence");
    vec![
        "unsupported".to_string(),
        fixture.request.family().as_str().to_string(),
        fixture.request.entrypoint().as_str().to_string(),
        fixture
            .request
            .runtime_declaration()
            .expect("intent-admission oracle rows still use runtime declarations")
            .source_lane()
            .as_str()
            .to_string(),
        match fixture.decision {
            ForgeQueryIntentAdmissionDecision::Violation(_) => "unsupported".to_string(),
            _ => unreachable!("fixture is unsupported"),
        },
        evidence.support_posture().as_str().to_string(),
        evidence.routing_support_posture().as_str().to_string(),
        fixture
            .trace
            .rows()
            .last()
            .expect("trace should not be empty")
            .stage()
            .as_str()
            .to_string(),
    ]
}

trait EligibilityEvidenceExt {
    fn eligibility_evidence(
        &self,
    ) -> Option<&crate::intent_admission::ForgeQueryIntentEligibilityTraceEvidence>;
}

impl EligibilityEvidenceExt for crate::intent_admission::ForgeQueryIntentDecisionTraceRow {
    fn eligibility_evidence(
        &self,
    ) -> Option<&crate::intent_admission::ForgeQueryIntentEligibilityTraceEvidence> {
        match self.evidence() {
            crate::intent_admission::ForgeQueryIntentDecisionTraceEvidence::Eligibility(
                evidence,
            ) => Some(evidence),
            _ => None,
        }
    }
}
