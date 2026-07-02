use worth_ui::facade::obligations::{
    UiObligationCheckKind, UiObligationFamily, UiObligationFamilyCatalog,
    UiObligationStarterMatrixTopology, UiObligationSupportBasis,
};

#[test]
fn starter_matrix_topology_is_closed_and_kept_in_sync_with_the_family_catalog() {
    let catalog_families = UiObligationFamilyCatalog::closed().families();
    let starter_rows = UiObligationStarterMatrixTopology::starter();
    let starter_tuples = starter_rows
        .rows()
        .iter()
        .map(|row| (row.family(), row.check_kind(), row.support_basis()))
        .collect::<Vec<_>>();

    assert_eq!(
        starter_tuples,
        vec![
            (
                UiObligationFamily::StructuralLegality,
                UiObligationCheckKind::BlockingInvariant,
                UiObligationSupportBasis::TouchMeaning,
            ),
            (
                UiObligationFamily::SlotContract,
                UiObligationCheckKind::BlockingInvariant,
                UiObligationSupportBasis::TouchMeaning,
            ),
            (
                UiObligationFamily::ParticipationLegality,
                UiObligationCheckKind::BlockingInvariant,
                UiObligationSupportBasis::TouchMeaning,
            ),
            (
                UiObligationFamily::MeasurementRequirement,
                UiObligationCheckKind::PrerequisiteRequirement,
                UiObligationSupportBasis::MeasurementPolicy,
            ),
            (
                UiObligationFamily::HostCapabilityRequirement,
                UiObligationCheckKind::CapabilityGapScreen,
                UiObligationSupportBasis::HostCapability,
            ),
            (
                UiObligationFamily::QueryBindingRequirement,
                UiObligationCheckKind::PrerequisiteRequirement,
                UiObligationSupportBasis::QueryBinding,
            ),
            (
                UiObligationFamily::PortalHostRequirement,
                UiObligationCheckKind::PrerequisiteRequirement,
                UiObligationSupportBasis::ServiceUsage,
            ),
            (
                UiObligationFamily::DiagnosticSurfaceRequirement,
                UiObligationCheckKind::DiagnosticOnlyCheck,
                UiObligationSupportBasis::ServiceUsage,
            ),
        ]
    );

    assert!(!starter_tuples.iter().enumerate().any(|(index, tuple)| {
        starter_tuples
            .iter()
            .skip(index + 1)
            .any(|candidate| candidate == tuple)
    }));
    assert!(starter_tuples
        .iter()
        .all(|(family, _, _)| catalog_families.contains(family)));
    assert!(starter_tuples.contains(&(
        UiObligationFamily::StructuralLegality,
        UiObligationCheckKind::BlockingInvariant,
        UiObligationSupportBasis::TouchMeaning,
    )));
    assert!(starter_tuples.contains(&(
        UiObligationFamily::MeasurementRequirement,
        UiObligationCheckKind::PrerequisiteRequirement,
        UiObligationSupportBasis::MeasurementPolicy,
    )));
    assert!(starter_tuples.contains(&(
        UiObligationFamily::QueryBindingRequirement,
        UiObligationCheckKind::PrerequisiteRequirement,
        UiObligationSupportBasis::QueryBinding,
    )));
    assert!(starter_tuples.contains(&(
        UiObligationFamily::DiagnosticSurfaceRequirement,
        UiObligationCheckKind::DiagnosticOnlyCheck,
        UiObligationSupportBasis::ServiceUsage,
    )));
}
