use worth_kernel::workload_composition::{
    PlanarBooleanDeclaration, PlanarBooleanDeclarationReceipt, PlanarBooleanEntryBasis,
    PlanarBooleanExecutionLane, PlanarBooleanFamily, PlanarBooleanOperandPairIdentity,
    PlanarBooleanOperation, PlanarBooleanOutcomeKind, PlanarBooleanOutcomeReceipt,
    PlanarBooleanSupportReceipt,
};
use worth_spatial::facade::blocker_provenance::{
    WorkloadBlockerBoundaryKind, WorkloadBlockerSourceKind,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventExtractionDenialKind, PlanarBooleanEventExtractionPhaseStop,
    PlanarBooleanEventExtractionPhaseStopError, PlanarBooleanEventPredicateBinding,
    PlanarBooleanPointEventExtraction,
};
use worth_spatial::facade::user_response::{WorthUserOutcomeCauseKind, WorthUserOutcomeKind};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

#[path = "public_api_planar_boolean_entry/tests/support.rs"]
#[allow(dead_code)]
mod entry_support;
#[path = "public_api_planar_boolean_point_events_support/mod.rs"]
#[allow(dead_code)]
mod point_event_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use point_event_support::SyntheticPointRelation;

#[test]
fn event_extraction_denies_predicate_ambiguous_near_contact_without_event() {
    reduced_pair_support::run_with_large_stack(|| {
        let (subject, denial) =
            ambiguous_point_denial("phase7.2 event extraction ambiguous near contact");
        let binding = predicate_binding_from_subject(&subject);
        let bound_pair = binding
            .bound_pair(denial.segment_pair_identity())
            .expect("ambiguous denial must reference a bound pair");
        let stop =
            PlanarBooleanEventExtractionPhaseStop::from_point_event_denial(&binding, &denial)
                .expect("point denial should assemble from its predicate binding");

        let PlanarBooleanEventExtractionPhaseStop::Denied(phase_denial) = stop else {
            panic!("ambiguous near contact must be a typed denial");
        };
        assert_eq!(
            phase_denial.kind(),
            PlanarBooleanEventExtractionDenialKind::PredicateAmbiguousNearContact
        );
        assert_eq!(
            phase_denial.reduced_pair_identity(),
            subject.reduced_pair_identity
        );
        assert_eq!(
            phase_denial.segment_pair_identity(),
            Some(denial.segment_pair_identity())
        );
        assert_eq!(
            phase_denial.predicate_binding_identity(),
            Some(denial.predicate_binding_identity())
        );
        assert_eq!(
            phase_denial.precision_basis_identity(),
            Some(bound_pair.precision_basis_identity())
        );
        assert_eq!(
            phase_denial.workload_evidence_stage(),
            WorkloadEvidenceStage::BooleanEventExtractionRequest
        );
        assert_eq!(phase_denial.counters().denied_micro_events(), 1);
        assert_eq!(phase_denial.counters().policy_exits(), 0);
    });
}

#[test]
fn event_extraction_stop_projects_to_denied_outcome_with_real_provenance() {
    reduced_pair_support::run_with_large_stack(|| {
        let (subject, denial) = ambiguous_point_denial("phase7.2 event extraction outcome denial");
        let binding = predicate_binding_from_subject(&subject);
        let stop =
            PlanarBooleanEventExtractionPhaseStop::from_point_event_denial(&binding, &denial)
                .expect("point denial should assemble from its predicate binding");
        let (declaration, support) =
            declaration_and_support("phase7.2 event extraction outcome denial", &subject.pair);
        let outcome =
            PlanarBooleanOutcomeReceipt::from_event_extraction_stop(declaration, support, &stop)
                .expect("event-extraction denial should compose into a user outcome");

        assert_eq!(outcome.kind(), PlanarBooleanOutcomeKind::Denied);
        assert_eq!(outcome.user_outcome().kind(), WorthUserOutcomeKind::Denied);
        assert_eq!(
            outcome.user_outcome().cause().map(|cause| cause.kind()),
            Some(WorthUserOutcomeCauseKind::OverlapDenied)
        );
        let provenance = outcome
            .blocker_provenance()
            .expect("denied event-extraction stops require blocker provenance");
        assert_eq!(
            provenance.source_kind(),
            WorkloadBlockerSourceKind::PlanarBooleanEventExtraction
        );
        assert_eq!(
            provenance.boundary_kind(),
            WorkloadBlockerBoundaryKind::BooleanEventExtractionBoundary
        );
        assert_eq!(provenance.source_identity(), stop.evidence_identity());
        assert_eq!(provenance.boundary_identity(), stop.reduced_pair_identity());
    });
}

#[test]
fn event_extraction_policy_exit_preserves_phase_and_pair_identity() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = point_event_support::binding_subject_with_relation(
            "phase7.2 event extraction policy exit",
            SyntheticPointRelation::PolicyRequiredCollinearOverlap,
        );
        let binding = predicate_binding_from_subject(&subject);
        let bound_pair = binding
            .bound_pairs()
            .first()
            .expect("policy-exit subject must expose a bound pair");
        let stop = PlanarBooleanEventExtractionPhaseStop::policy_exit_for_collinear_overlap(
            &binding,
            bound_pair,
            "collinear overlap requires explicit imprint policy before event extraction",
        )
        .expect("policy exit should assemble from predicate binding evidence");
        let (declaration, support) =
            declaration_and_support("phase7.2 event extraction policy exit", &subject.pair);
        let outcome =
            PlanarBooleanOutcomeReceipt::from_event_extraction_stop(declaration, support, &stop)
                .expect("event-extraction policy exit should compose");

        assert_eq!(outcome.kind(), PlanarBooleanOutcomeKind::PolicyRequired);
        assert_eq!(
            outcome.user_outcome().kind(),
            WorthUserOutcomeKind::PolicyRequired
        );
        assert_eq!(stop.reduced_pair_identity(), subject.reduced_pair_identity);
        assert_eq!(
            stop.segment_pair_identity(),
            Some(bound_pair.segment_pair_identity())
        );
        assert_eq!(
            stop.predicate_binding_identity(),
            Some(binding.predicate_binding_identity())
        );
        assert_eq!(
            stop.precision_basis_identity(),
            Some(bound_pair.precision_basis_identity())
        );
        assert_eq!(
            outcome
                .blocker_provenance()
                .expect("policy exit must preserve provenance")
                .boundary_identity(),
            subject.reduced_pair_identity
        );
    });
}

#[test]
fn event_extraction_stop_rejects_mismatched_predicate_binding_evidence() {
    reduced_pair_support::run_with_large_stack(|| {
        let (subject, denial) = ambiguous_point_denial("phase7.2 event extraction denial source");
        let (_, foreign_denial) =
            ambiguous_point_denial("phase7.2 event extraction denial foreign");
        let binding = predicate_binding_from_subject(&subject);

        let wrong_denial_error = PlanarBooleanEventExtractionPhaseStop::from_point_event_denial(
            &binding,
            &foreign_denial,
        )
        .expect_err("denial from a foreign predicate binding must not assemble");
        assert_eq!(
            wrong_denial_error,
            PlanarBooleanEventExtractionPhaseStopError::PredicateBindingIdentityMismatch
        );

        let honest_stop =
            PlanarBooleanEventExtractionPhaseStop::from_point_event_denial(&binding, &denial)
                .expect("honest denial should still assemble after hostile attempt");
        assert_eq!(
            honest_stop.reduced_pair_identity(),
            binding.reduced_pair_identity()
        );
    });
}

#[test]
fn event_extraction_policy_exit_rejects_foreign_bound_pair_identity() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = point_event_support::binding_subject_with_relation(
            "phase7.2 event extraction policy source",
            SyntheticPointRelation::PolicyRequiredCollinearOverlap,
        );
        let foreign_subject = point_event_support::binding_subject_with_relation(
            "phase7.2 event extraction policy foreign",
            SyntheticPointRelation::PolicyRequiredCollinearOverlap,
        );
        let binding = predicate_binding_from_subject(&subject);
        let foreign_binding = predicate_binding_from_subject(&foreign_subject);
        let foreign_bound_pair = foreign_binding
            .bound_pairs()
            .first()
            .expect("foreign binding must expose a bound pair");

        let error = PlanarBooleanEventExtractionPhaseStop::policy_exit_for_collinear_overlap(
            &binding,
            foreign_bound_pair,
            "foreign bound pair must not be promoted into a policy exit",
        )
        .expect_err("foreign bound pair must not assemble into a policy exit");
        assert_eq!(
            error,
            PlanarBooleanEventExtractionPhaseStopError::ReducedPairIdentityMismatch
        );
    });
}

fn ambiguous_point_denial(
    readiness_scope: &'static str,
) -> (
    predicate_binding_support::BindingSubject,
    worth_spatial::facade::planar_boolean_events::PlanarBooleanPointEventExtractionDenial,
) {
    let subject = point_event_support::binding_subject_with_relation(
        readiness_scope,
        SyntheticPointRelation::PolicyRequiredCollinearOverlap,
    );
    let binding = predicate_binding_from_subject(&subject);
    let denial = PlanarBooleanPointEventExtraction::from_predicate_binding(&binding)
        .compile()
        .expect("point-event extraction plan should compile before denial")
        .certify()
        .expect_err("ambiguous point relation must deny instead of emitting an event");
    (subject, denial)
}

fn predicate_binding_from_subject(
    subject: &predicate_binding_support::BindingSubject,
) -> worth_spatial::facade::planar_boolean_events::PlanarBooleanEventPredicateBinding {
    PlanarBooleanEventPredicateBinding::plan(&subject.pair_worklist)
        .for_reduced_pair(subject.reduced_pair_identity.clone())
        .with_segment_segment_receipts(subject.segment_receipts.clone())
        .with_predicate_consumption_receipt(subject.predicate_consumption.clone())
        .compile()
        .expect("predicate binding plan should compile")
        .certify()
        .expect("predicate binding should certify")
}

fn declaration_and_support(
    readiness_scope: &'static str,
    pair: &worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> (PlanarBooleanDeclarationReceipt, PlanarBooleanSupportReceipt) {
    let declaration = PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        PlanarBooleanOperation::Union,
        PlanarBooleanOperandPairIdentity::new(pair.operand_pair_identity())
            .expect("operand-pair identity should certify"),
        PlanarBooleanExecutionLane::BRepNow,
    )
    .from_basis(
        PlanarBooleanEntryBasis::bind(
            entry_support::certified_boolean_readiness_workload_receipt(readiness_scope),
            format!("{readiness_scope} basis"),
        )
        .expect("entry basis should certify"),
    )
    .declared_by_query(format!("{readiness_scope} declaration"))
    .bind()
    .expect("declaration should certify");
    let support = PlanarBooleanSupportReceipt::for_declaration(&declaration)
        .expect("support should certify for B-rep declaration");
    (declaration, support)
}

#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
#[allow(dead_code)]
mod predicate_binding_support;
