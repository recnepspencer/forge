use worth_kernel::workload_composition::{
    PlanarBooleanDeclaration, PlanarBooleanDeclarationReceipt, PlanarBooleanEntryBasis,
    PlanarBooleanExecutionLane, PlanarBooleanFamily, PlanarBooleanOperandPairIdentity,
    PlanarBooleanOperation, PlanarBooleanOutcomeKind, PlanarBooleanOutcomeReceipt,
};
use worth_spatial::facade::blocker_provenance::{
    PlanarBooleanBlockerProvenanceInput, WorkloadBlockerBoundaryKind, WorkloadBlockerProvenance,
    WorkloadBlockerProvenanceDenialKind, WorkloadBlockerSourceKind,
};
use worth_spatial::facade::user_response::{WorthUserOutcomeCauseKind, WorthUserOutcomeKind};

use super::support::certified_boolean_readiness_workload_receipt;

#[test]
fn planar_boolean_outcome_taxonomy_preserves_all_machine_classes() {
    run_with_large_stack(|| {
        let declaration = bound_declaration(
            "phase3-outcome-taxonomy",
            PlanarBooleanExecutionLane::BRepNow,
            PlanarBooleanOperation::Union,
        );
        let support = declaration
            .clone()
            .classify_outcome()
            .expect("admitted outcome should certify")
            .support()
            .clone();
        let admitted = declaration
            .classify_outcome()
            .expect("admitted outcome should certify");
        let unsupported = PlanarBooleanOutcomeReceipt::unsupported(
            declaration.clone(),
            support.clone(),
            "Planar boolean support matrix rejects this source family.",
            WorkloadBlockerSourceKind::PlanarBooleanDeclaration,
            WorkloadBlockerBoundaryKind::BooleanSupportMatrix,
            declaration.query_declaration_digest(),
            support.query_support_digest(),
        )
        .expect("unsupported outcome should certify");
        let blocked = PlanarBooleanOutcomeReceipt::blocked(
            declaration.clone(),
            support.clone(),
            "Required boolean evidence is missing from the certified basis.",
            WorkloadBlockerSourceKind::PlanarBooleanEntryBasis,
            WorkloadBlockerBoundaryKind::BooleanEvidenceBoundary,
            declaration.readiness_basis_digest(),
            declaration.readiness_workload_digest(),
        )
        .expect("blocked outcome should certify");
        let denied = PlanarBooleanOutcomeReceipt::denied(
            declaration.clone(),
            support.clone(),
            "Certified overlap policy denies this planar boolean cut.",
            WorkloadBlockerSourceKind::PlanarBooleanDeclaration,
            WorkloadBlockerBoundaryKind::BooleanExecutionBoundary,
            declaration.query_declaration_digest(),
            support.query_support_digest(),
        )
        .expect("denied outcome should certify");
        let policy_required = PlanarBooleanDeclaration::new(
            PlanarBooleanFamily::PlanarRegions,
            PlanarBooleanOperation::Union,
            PlanarBooleanOperandPairIdentity::new("phase3-ember-pair")
                .expect("operand identity should certify"),
            PlanarBooleanExecutionLane::EmberFuture,
        )
        .from_basis(
            PlanarBooleanEntryBasis::bind(
                certified_boolean_readiness_workload_receipt("phase3-ember-basis"),
                "phase3 ember basis",
            )
            .expect("ember basis should certify"),
        )
        .declared_by_query("phase3 ember declaration")
        .classify_outcome()
        .expect("policy outcome should certify");
        let integrity_mismatch = PlanarBooleanOutcomeReceipt::integrity_mismatch(
            declaration.clone(),
            support.clone(),
            "Readiness evidence does not match the boolean execution boundary.",
            WorkloadBlockerSourceKind::PlanarBooleanEntryBasis,
            WorkloadBlockerBoundaryKind::BooleanExecutionBoundary,
            declaration.readiness_basis_digest(),
            support.query_support_digest(),
        )
        .expect("integrity mismatch should certify");
        let no_options = PlanarBooleanOutcomeReceipt::no_options(
            declaration,
            support,
            "No certified boolean entry options remain after workload filtering.",
            WorkloadBlockerSourceKind::PlanarBooleanEntryBasis,
            WorkloadBlockerBoundaryKind::BooleanEvidenceBoundary,
            "basis:no-options",
            "boundary:no-options",
        )
        .expect("no-options outcome should certify");

        assert_branch(
            admitted.kind(),
            admitted.user_outcome(),
            PlanarBooleanOutcomeKind::Admitted,
            WorthUserOutcomeKind::Admitted,
            None,
        );
        assert_branch(
            unsupported.kind(),
            unsupported.user_outcome(),
            PlanarBooleanOutcomeKind::Unsupported,
            WorthUserOutcomeKind::Unsupported,
            Some(WorthUserOutcomeCauseKind::UnsupportedInput),
        );
        assert_branch(
            blocked.kind(),
            blocked.user_outcome(),
            PlanarBooleanOutcomeKind::Blocked,
            WorthUserOutcomeKind::NoOptions,
            Some(WorthUserOutcomeCauseKind::MissingEvidence),
        );
        assert_branch(
            denied.kind(),
            denied.user_outcome(),
            PlanarBooleanOutcomeKind::Denied,
            WorthUserOutcomeKind::Denied,
            Some(WorthUserOutcomeCauseKind::OverlapDenied),
        );
        assert_branch(
            policy_required.kind(),
            policy_required.user_outcome(),
            PlanarBooleanOutcomeKind::PolicyRequired,
            WorthUserOutcomeKind::PolicyRequired,
            Some(WorthUserOutcomeCauseKind::PolicyRequired),
        );
        assert_branch(
            integrity_mismatch.kind(),
            integrity_mismatch.user_outcome(),
            PlanarBooleanOutcomeKind::IntegrityMismatch,
            WorthUserOutcomeKind::IntegrityMismatch,
            Some(WorthUserOutcomeCauseKind::IntegrityMismatch),
        );
        assert_branch(
            no_options.kind(),
            no_options.user_outcome(),
            PlanarBooleanOutcomeKind::NoOptions,
            WorthUserOutcomeKind::NoOptions,
            Some(WorthUserOutcomeCauseKind::MissingEvidence),
        );
    });
}

#[test]
fn boolean_blocker_provenance_names_real_boundary_and_source_identities() {
    run_with_large_stack(|| {
        let declaration = bound_declaration(
            "phase3-provenance",
            PlanarBooleanExecutionLane::BRepNow,
            PlanarBooleanOperation::Intersect,
        );
        let support = declaration
            .classify_outcome()
            .expect("admitted outcome should certify")
            .support()
            .clone();
        let blocked = PlanarBooleanOutcomeReceipt::blocked(
            declaration.clone(),
            support.clone(),
            "Required boolean evidence is missing from the certified basis.",
            WorkloadBlockerSourceKind::PlanarBooleanEntryBasis,
            WorkloadBlockerBoundaryKind::BooleanEvidenceBoundary,
            declaration.readiness_basis_digest(),
            declaration.readiness_workload_digest(),
        )
        .expect("blocked outcome should certify");
        let policy_required = PlanarBooleanOutcomeReceipt::policy_required(
            declaration,
            support,
            "EMBER comparison policy is required before boolean entry can continue.",
            WorkloadBlockerSourceKind::PlanarBooleanDeclaration,
            WorkloadBlockerBoundaryKind::BooleanLanePolicy,
            "declaration:phase3-policy",
            "support:phase3-policy",
        )
        .expect("policy outcome should certify");

        let blocked_provenance = blocked
            .blocker_provenance()
            .expect("blocked branch requires provenance");
        assert_eq!(
            blocked_provenance.source_kind(),
            WorkloadBlockerSourceKind::PlanarBooleanEntryBasis
        );
        assert_eq!(
            blocked_provenance.boundary_kind(),
            WorkloadBlockerBoundaryKind::BooleanEvidenceBoundary
        );
        assert_eq!(
            blocked_provenance.source_identity(),
            blocked.declaration().readiness_basis_digest()
        );
        assert_eq!(
            blocked_provenance.boundary_identity(),
            blocked.declaration().readiness_workload_digest()
        );

        let policy_provenance = policy_required
            .blocker_provenance()
            .expect("policy branch requires provenance");
        assert_eq!(
            policy_provenance.source_kind(),
            WorkloadBlockerSourceKind::PlanarBooleanDeclaration
        );
        assert_eq!(
            policy_provenance.boundary_kind(),
            WorkloadBlockerBoundaryKind::BooleanLanePolicy
        );
        assert_eq!(
            policy_provenance.source_identity(),
            "declaration:phase3-policy"
        );
        assert_eq!(
            policy_provenance.boundary_identity(),
            "support:phase3-policy"
        );
    });
}

#[test]
fn automatic_ember_policy_outcome_carries_real_declaration_provenance() {
    run_with_large_stack(|| {
        let declaration = bound_declaration(
            "phase3-ember-policy",
            PlanarBooleanExecutionLane::EmberFuture,
            PlanarBooleanOperation::Union,
        );
        let outcome = declaration
            .classify_outcome()
            .expect("EMBER declaration should classify");

        assert_eq!(outcome.kind(), PlanarBooleanOutcomeKind::PolicyRequired);
        assert_eq!(
            outcome.user_outcome().kind(),
            WorthUserOutcomeKind::PolicyRequired
        );

        let provenance = outcome
            .blocker_provenance()
            .expect("policy-required path must carry provenance");
        assert_eq!(
            provenance.source_kind(),
            WorkloadBlockerSourceKind::PlanarBooleanDeclaration
        );
        assert_eq!(
            provenance.boundary_kind(),
            WorkloadBlockerBoundaryKind::BooleanLanePolicy
        );
        assert_eq!(
            provenance.source_identity(),
            declaration.query_declaration_digest()
        );
        assert_eq!(
            provenance.boundary_identity(),
            declaration.query_handle_digest()
        );
    });
}

#[test]
fn blocker_provenance_rejects_admitted_boolean_outcomes() {
    run_with_large_stack(|| {
        let declaration = bound_declaration(
            "phase3-admitted-provenance-guard",
            PlanarBooleanExecutionLane::BRepNow,
            PlanarBooleanOperation::Union,
        );
        let outcome = declaration
            .classify_outcome()
            .expect("B-rep declaration should admit");
        let provenance = WorkloadBlockerProvenance::from_planar_boolean_outcome(
            &PlanarBooleanBlockerProvenanceInput::new(
                WorkloadBlockerSourceKind::PlanarBooleanDeclaration,
                WorkloadBlockerBoundaryKind::BooleanExecutionBoundary,
                declaration.query_declaration_digest(),
                declaration.query_handle_digest(),
                "Admitted planar booleans must not certify blocker provenance.",
            ),
        )
        .certify_non_admitted(outcome.user_outcome())
        .expect_err("admitted outcomes must not certify non-admitted provenance");

        assert_eq!(
            provenance.kind(),
            WorkloadBlockerProvenanceDenialKind::OutcomeReportedAdmitted
        );
    });
}

#[test]
fn boolean_no_options_outcomes_cannot_drop_required_provenance() {
    run_with_large_stack(|| {
        let declaration = bound_declaration(
            "phase3-no-options",
            PlanarBooleanExecutionLane::BRepNow,
            PlanarBooleanOperation::Subtract,
        );
        let support = declaration
            .classify_outcome()
            .expect("admitted outcome should certify")
            .support()
            .clone();
        let no_options = PlanarBooleanOutcomeReceipt::no_options(
            declaration.clone(),
            support,
            "No certified boolean entry options remain after workload filtering.",
            WorkloadBlockerSourceKind::PlanarBooleanEntryBasis,
            WorkloadBlockerBoundaryKind::BooleanEvidenceBoundary,
            declaration.readiness_basis_digest(),
            declaration.readiness_workload_digest(),
        )
        .expect("no-options branch should certify");

        let provenance = no_options
            .blocker_provenance()
            .expect("no-options branch must preserve provenance");
        assert_eq!(no_options.kind(), PlanarBooleanOutcomeKind::NoOptions);
        assert_eq!(
            no_options.user_outcome().kind(),
            WorthUserOutcomeKind::NoOptions
        );
        assert_eq!(
            provenance.source_identity(),
            no_options.declaration().readiness_basis_digest()
        );
        assert_eq!(
            provenance.boundary_identity(),
            no_options.declaration().readiness_workload_digest()
        );
        assert!(provenance
            .human_reason()
            .contains("No certified boolean entry options"));
    });
}

fn bound_declaration(
    stem: &str,
    lane: PlanarBooleanExecutionLane,
    operation: PlanarBooleanOperation,
) -> PlanarBooleanDeclarationReceipt {
    let readiness_stem = Box::leak(stem.to_string().into_boxed_str());
    let basis = PlanarBooleanEntryBasis::bind(
        certified_boolean_readiness_workload_receipt(readiness_stem),
        format!("{stem} basis"),
    )
    .expect("basis should certify");
    PlanarBooleanDeclaration::new(
        PlanarBooleanFamily::PlanarRegions,
        operation,
        PlanarBooleanOperandPairIdentity::new(format!("{stem}-pair"))
            .expect("operand identity should certify"),
        lane,
    )
    .from_basis(basis)
    .declared_by_query(format!("{stem} declaration"))
    .bind()
    .expect("declaration should certify")
}

fn assert_branch(
    actual_kind: PlanarBooleanOutcomeKind,
    user_outcome: &worth_spatial::facade::user_response::WorthUserOutcome,
    expected_kind: PlanarBooleanOutcomeKind,
    expected_user_kind: WorthUserOutcomeKind,
    expected_cause: Option<WorthUserOutcomeCauseKind>,
) {
    assert_eq!(actual_kind, expected_kind);
    assert_eq!(user_outcome.kind(), expected_user_kind);
    assert_eq!(
        user_outcome.cause().map(|cause| cause.kind()),
        expected_cause
    );
}

fn run_with_large_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("spawn large-stack planar boolean test")
        .join()
        .expect("join large-stack planar boolean test");
}
