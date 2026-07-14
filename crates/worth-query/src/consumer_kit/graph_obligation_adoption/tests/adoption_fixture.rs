use crate::runtime::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationSupportLane,
    WorthQueryGraphObligationSupportMatrix, WorthQueryGraphObligationSupportMatrixRow,
    WorthQueryGraphObligationSupportPosture, WorthQueryGraphObligationSupportStatus,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchReadVerb, WorthQueryGraphTouchSelector,
};
use crate::{
    graph_obligation_consumer_kit, WorthQueryBoundaryAuditSourceSet,
    WorthQueryGraphObligationAdoptionProof, WorthQueryGraphObligationConsumerKitError,
    WorthQueryGraphObligationConsumerRegistrationDeclaration,
    WorthQueryGraphObligationLocalCeremonyAudit, WorthQueryGraphObligationResidueRow,
    WorthQueryGraphObligationSelectorCoverageDeclaration, WorthQueryGraphObligationSupportPin,
};

pub(super) fn reference_registration() -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new("worth.kernel", "active-face-validity", "1.0.0")
            .unwrap(),
        WorthQueryGraphTouchSelector::collection("worth_faces").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
    ))
}

pub(super) fn evaluated_clean_audit(
    crate_name: &str,
) -> WorthQueryGraphObligationLocalCeremonyAudit {
    WorthQueryGraphObligationLocalCeremonyAudit::evaluate(
        &WorthQueryBoundaryAuditSourceSet::new(crate_name).source(
            "consumer-adoption.rs",
            "pub fn consumer_uses_graph_obligation_consumer_kit() {}",
        ),
    )
}

pub(super) fn adoption_attempt_with_pin(
    pin: WorthQueryGraphObligationSupportPin,
    matrix: WorthQueryGraphObligationSupportMatrix,
) -> Result<WorthQueryGraphObligationAdoptionProof, WorthQueryGraphObligationConsumerKitError> {
    graph_obligation_consumer_kit("worth-kernel")
        .register_obligations(
            WorthQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
                "worth-kernel-validity",
                [reference_registration()],
            )
            .unwrap(),
        )
        .declare_selector_coverage(
            WorthQueryGraphObligationSelectorCoverageDeclaration::required([(
                "active face read coverage",
                WorthQueryGraphTouchSelector::collection("worth_faces").unwrap(),
            )]),
        )
        .pin_support(pin)
        .against_support_matrix(matrix)
        .audit_local_ceremony(evaluated_clean_audit("worth-kernel"))
        .prove_in_memory_selection(
            &WorthQueryGraphTouchDescriptor::read_family(
                "worth_faces",
                [WorthQueryGraphTouchReadVerb::ObservesCollection],
            )
            .unwrap(),
            &WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
        )?
        .prove_adoption()
}

pub(super) fn support_matrix_with_status(
    status: WorthQueryGraphObligationSupportStatus,
) -> WorthQueryGraphObligationSupportMatrix {
    WorthQueryGraphObligationSupportMatrix::new(vec![
        WorthQueryGraphObligationSupportMatrixRow::new(
            WorthQueryGraphObligationKind::BlockingInvariant,
            WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
            status,
        ),
    ])
}

pub(super) fn residue_row(current_count: usize) -> WorthQueryGraphObligationResidueRow {
    residue_row_with_cap(current_count, 1)
}

pub(super) fn residue_row_with_cap(
    current_count: usize,
    cap: usize,
) -> WorthQueryGraphObligationResidueRow {
    WorthQueryGraphObligationResidueRow::explicit(
        "manual selector comments",
        "worth-query",
        "phase-14",
        current_count,
        cap,
        "covered selector replacement is incomplete",
        "delete manual selector comments",
        "remove",
    )
    .unwrap()
}
