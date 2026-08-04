use worth_store_compatibility::{ArtifactCompatibilityWindow, ArtifactFormatVersion};
use worth_store_layout_indexes::evolution::migration::{
    layout_backward_read_compatibility, LayoutBackwardReadCompatibilityCaseId,
    LayoutBackwardReadRequest, LayoutCompatibilityWindow, LayoutEvolutionDeclaration,
    LayoutInterruptionPolicy, LayoutReadCompatibilityPosture, LayoutWriteCompatibilityPosture,
};
use worth_store_layout_indexes::{ObserveOwnerCase, OwnerCaseObservation};

use super::world;

pub(super) fn observe() -> Vec<OwnerCaseObservation<LayoutBackwardReadCompatibilityCaseId>> {
    let declaration = world::declaration(LayoutInterruptionPolicy::ResumeDeclaredMigration);
    let authority = world::authority("store.layout_evolution.backward_read");
    let binding = world::source_binding(declaration, &authority);
    let admitted = declaration
        .compatibility_window()
        .artifact_window()
        .admit_backward_read(ArtifactFormatVersion(5))
        .unwrap();
    let wrong_binding = declaration
        .compatibility_window()
        .artifact_window()
        .admit_backward_read(ArtifactFormatVersion(6))
        .unwrap();
    let wrong_window = ArtifactCompatibilityWindow::new(
        ArtifactFormatVersion(4),
        ArtifactFormatVersion(5),
        ArtifactFormatVersion(6),
    )
    .unwrap()
    .admit_backward_read(ArtifactFormatVersion(5))
    .unwrap();
    let foreign_declaration = LayoutEvolutionDeclaration::from_admitted_family(
        world::admitted_family(world::declared_family(), &authority),
        declaration.layout_version(),
        LayoutCompatibilityWindow::new(
            ArtifactFormatVersion(5),
            ArtifactFormatVersion(7),
            ArtifactFormatVersion(7),
            LayoutReadCompatibilityPosture::ReadOldWriteNew,
            LayoutWriteCompatibilityPosture::WriteNewDuringRollingUpgrade,
        )
        .unwrap(),
        world::version(5, 9, 9),
        declaration.migration_target(),
        declaration.rollback_source(),
        declaration.rollback_target(),
        declaration.interruption_policy(),
    );
    let foreign_binding = world::source_binding(foreign_declaration, &authority);

    [
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            admitted,
        )),
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            wrong_window,
        )),
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &binding,
            wrong_binding,
        )),
        layout_backward_read_compatibility().admit(LayoutBackwardReadRequest::new(
            declaration,
            &foreign_binding,
            admitted,
        )),
    ]
    .into_iter()
    .map(|outcome| outcome.owner_case_observation())
    .collect()
}
