use forge_proof::TransitionOutcome;

use super::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
};
use super::test_support::{admitted_plan_target, ready, success};
use super::{
    materialize_intent_admission_support_traceability_row, materialize_runtime_admission_decision,
    materialize_runtime_admission_support_traceability_report,
    materialize_runtime_admission_support_traceability_row,
    materialize_runtime_continuity_evidence, ForgeQueryAdmissionContributionAuthoring,
    ForgeQueryContinuityContributionAuthoring, ForgeQuerySupportContributionAuthoring,
};

#[test]
fn admission_runtime_materializer_builds_query_decisions() {
    let advisory = success(materialize_runtime_admission_decision(ready_admission(
        ForgeQueryAdmissionContributionAuthoring::advisory_at_stage(
            "spatial_arbitration",
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    )));
    let violation = success(materialize_runtime_admission_decision(ready_admission(
        ForgeQueryAdmissionContributionAuthoring::violation_at_stage(
            "spatial_arbitration",
            "spatial.arbitration.invalid_target",
            "requested target violates spatial law",
        ),
    )));

    match advisory {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            assert_eq!(advisory.family().as_str(), "authoritative-user-intent");
            assert_eq!(
                advisory.entrypoint().as_str(),
                "ForgeQueryRuntime::execute_intent"
            );
            assert_eq!(advisory.stage(), "spatial_arbitration");
        }
        other => panic!("expected advisory decision, got {other:?}"),
    }

    match violation {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            assert_eq!(violation.family().as_str(), "authoritative-user-intent");
            assert_eq!(
                violation.entrypoint().as_str(),
                "ForgeQueryRuntime::execute_intent"
            );
            assert_eq!(violation.stage(), "spatial_arbitration");
        }
        other => panic!("expected violation decision, got {other:?}"),
    }
}

#[test]
fn admission_support_only_runtime_materializer_builds_support_traceability() {
    let row = success(materialize_runtime_admission_support_traceability_row(
        ready_admission(ForgeQueryAdmissionContributionAuthoring::support_only(
            "spatial.arbitration.support_only",
            "declaration remains support-scoped only",
        )),
    ));
    let report = success(materialize_runtime_admission_support_traceability_report(
        ready_admission(ForgeQueryAdmissionContributionAuthoring::support_only(
            "spatial.arbitration.support_only",
            "declaration remains support-scoped only",
        )),
    ));

    assert_eq!(row.family(), "authoritative-user-intent");
    assert_eq!(row.entrypoint(), "ForgeQueryRuntime::execute_intent");
    assert_eq!(row.lane(), "admission_local_support");
    assert_eq!(
        row.support_detail(),
        "spatial.arbitration.support_only:declaration remains support-scoped only"
    );
    assert_eq!(report.rows().len(), 1);
    assert_eq!(report.rows()[0], row);
}

#[test]
fn admission_support_traceability_materializer_denies_non_support_postures() {
    let advisory = materialize_runtime_admission_support_traceability_row(ready_admission(
        ForgeQueryAdmissionContributionAuthoring::advisory(
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    ));
    let violation = materialize_runtime_admission_support_traceability_report(ready_admission(
        ForgeQueryAdmissionContributionAuthoring::violation(
            "spatial.arbitration.invalid_target",
            "requested target violates spatial law",
        ),
    ));

    assert!(matches!(
        advisory,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
    ));
    assert!(matches!(
        violation,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
    ));
}

#[test]
fn admission_local_support_and_support_traceability_rows_stay_distinct() {
    let admission_row = success(materialize_runtime_admission_support_traceability_row(
        ready_admission(ForgeQueryAdmissionContributionAuthoring::support_only(
            "spatial.shared.support",
            "same support detail through shared runtime family",
        )),
    ));
    let support_row = success(materialize_intent_admission_support_traceability_row(
        ready_support(ForgeQuerySupportContributionAuthoring::declaration_support(
            "spatial.shared.support",
            "same support detail through shared runtime family",
        )),
    ));

    assert_eq!(admission_row.support_detail(), support_row.support_detail());
    assert_eq!(admission_row.family(), support_row.family());
    assert_eq!(admission_row.entrypoint(), support_row.entrypoint());
    assert_ne!(admission_row.lane(), support_row.lane());
    assert_ne!(admission_row.row_digest(), support_row.row_digest());
}

#[test]
fn continuity_runtime_materializer_builds_continuity_evidence() {
    let preserved = success(materialize_runtime_continuity_evidence(ready_continuity(
        ForgeQueryContinuityContributionAuthoring::preserved_rebind(
            "edge:12",
            "edge:14",
            "continuity.identity.preserved",
            "edge retarget keeps a single successor identity",
        ),
    )));
    let split = success(materialize_runtime_continuity_evidence(ready_continuity(
        ForgeQueryContinuityContributionAuthoring::split_successors(
            "edge:12",
            ["edge:14", "edge:15"],
            "continuity.identity.split",
            "edge split produces two descendant identities",
        ),
    )));

    assert_eq!(
        preserved.outcome_class(),
        crate::runtime::ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
    );
    assert_eq!(preserved.prior_authoritative_identity(), "edge:12");
    assert_eq!(
        preserved.basis_binding_digest(),
        Some(
            ForgeQueryAdmittedPlanBoundContributionTarget::from_digest("plan-continuity")
                .binding_digest()
        )
    );
    assert_eq!(
        preserved.successor_authoritative_identity(),
        Some("edge:14")
    );

    assert_eq!(
        split.outcome_class(),
        crate::runtime::ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
    );
    assert_eq!(
        split.successor_authoritative_identities(),
        &["edge:14".to_string(), "edge:15".to_string()]
    );
}

#[test]
fn continuity_runtime_materializer_denies_correspondence_only() {
    let outcome = materialize_runtime_continuity_evidence(ready_continuity(
        ForgeQueryContinuityContributionAuthoring::correspondence_only(
            "continuity.correspondence_only",
            "semantic correspondence exists without authoritative continuity",
        ),
    ));

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
}

#[test]
fn continuity_runtime_materializer_denies_inconsistent_runtime_semantics() {
    let outcome = materialize_runtime_continuity_evidence(ready_continuity_payload(
        super::ForgeQueryContinuityContributionPayload::with_runtime_semantics(
            super::ForgeQueryContinuityContributionPosture::Preserved,
            "continuity.identity.preserved",
            "posture claims preserved while runtime semantics claim split",
            Some(super::ForgeQueryContinuityRuntimeSemantics::new(
                crate::runtime::ForgeQueryContinuityMutationFamily::SplitExistingTarget,
                crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors,
                "edge:12",
                ["edge:14", "edge:15"],
            )),
        ),
    ));

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics
    ));
}

fn ready_admission(
    authoring: ForgeQueryAdmissionContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyAdmissionContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready(authoring.bind_to_admitted_plan_target(admitted_plan_target("plan-admission")))
}

fn ready_support(
    authoring: ForgeQuerySupportContributionAuthoring,
) -> super::ForgeQueryMaterializationReadySupportContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready(authoring.bind_to_admitted_plan_target(admitted_plan_target("plan-admission")))
}

fn ready_continuity(
    authoring: ForgeQueryContinuityContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyContinuityContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready_continuity_payload(
        authoring
            .bind_to_admitted_plan_target(admitted_plan_target("plan-continuity"))
            .payload()
            .payload()
            .clone(),
    )
}

fn ready_continuity_payload(
    payload: super::ForgeQueryContinuityContributionPayload,
) -> super::ForgeQueryMaterializationReadyContinuityContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            admitted_plan_target("plan-continuity"),
            payload,
        ),
    )
}
