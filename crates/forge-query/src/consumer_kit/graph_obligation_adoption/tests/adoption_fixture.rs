use crate::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportMatrixRow,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphObligationSupportStatus,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector,
};
use crate::{
    graph_obligation_consumer_kit, ForgeQueryBoundaryAuditSourceSet,
    ForgeQueryGraphObligationAdoptionProof, ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationLocalCeremonyAudit, ForgeQueryGraphObligationResidueRow,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};

pub(super) fn reference_registration() -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new("worth.kernel", "active-face-validity", "1.0.0")
            .unwrap(),
        ForgeQueryGraphTouchSelector::collection("worth_faces").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
    ))
}

pub(super) fn evaluated_clean_audit(
    crate_name: &str,
) -> ForgeQueryGraphObligationLocalCeremonyAudit {
    ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
        &ForgeQueryBoundaryAuditSourceSet::new(crate_name).source(
            "consumer-adoption.rs",
            "pub fn consumer_uses_graph_obligation_consumer_kit() {}",
        ),
    )
}

pub(super) fn adoption_attempt_with_pin(
    pin: ForgeQueryGraphObligationSupportPin,
    matrix: ForgeQueryGraphObligationSupportMatrix,
) -> Result<ForgeQueryGraphObligationAdoptionProof, ForgeQueryGraphObligationConsumerKitError> {
    graph_obligation_consumer_kit("worth-kernel")
        .register_obligations(
            ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-kernel-validity",
                [reference_registration()],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            ForgeQueryGraphObligationSelectorCoverageDeclaration::required([(
                "active face read coverage",
                ForgeQueryGraphTouchSelector::collection("worth_faces").unwrap(),
            )]),
        )
        .pin_support(pin)
        .against_support_matrix(matrix)
        .audit_local_ceremony(evaluated_clean_audit("worth-kernel"))
        .prove_in_memory_selection(
            &ForgeQueryGraphTouchDescriptor::read_family(
                "worth_faces",
                [ForgeQueryGraphTouchReadVerb::ObservesCollection],
            )
            .unwrap(),
            &ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )?
        .prove_adoption()
}

pub(super) fn support_matrix_with_status(
    status: ForgeQueryGraphObligationSupportStatus,
) -> ForgeQueryGraphObligationSupportMatrix {
    ForgeQueryGraphObligationSupportMatrix::new(vec![
        ForgeQueryGraphObligationSupportMatrixRow::new(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
            status,
        ),
    ])
}

pub(super) fn residue_row(current_count: usize) -> ForgeQueryGraphObligationResidueRow {
    residue_row_with_cap(current_count, 1)
}

pub(super) fn residue_row_with_cap(
    current_count: usize,
    cap: usize,
) -> ForgeQueryGraphObligationResidueRow {
    ForgeQueryGraphObligationResidueRow::explicit(
        "manual selector comments",
        "forge-query",
        "phase-14",
        current_count,
        cap,
        "covered selector replacement is incomplete",
        "delete manual selector comments",
        "remove",
    )
    .unwrap()
}
