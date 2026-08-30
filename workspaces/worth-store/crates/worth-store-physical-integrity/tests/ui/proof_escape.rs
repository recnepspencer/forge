use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedPageFrame, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedRootManifest, IntegrityValidatedWalFrame,
};

fn escape_current<'media>(
    validation: IntegrityValidatedCurrentRootSelector<'media>,
) -> IntegrityValidatedCurrentRootSelector<'static> {
    validation
}

fn escape_previous<'media>(
    validation: IntegrityValidatedPreviousRootSelector<'media>,
) -> IntegrityValidatedPreviousRootSelector<'static> {
    validation
}

fn escape_manifest<'media>(
    validation: IntegrityValidatedRootManifest<'media>,
) -> IntegrityValidatedRootManifest<'static> {
    validation
}

fn escape_physical_work<'media>(
    validation: IntegrityValidatedPhysicalWorkObligation<'media>,
) -> IntegrityValidatedPhysicalWorkObligation<'static> {
    validation
}

fn escape_page<'media>(
    validation: IntegrityValidatedPageFrame<'media>,
) -> IntegrityValidatedPageFrame<'static> {
    validation
}

fn escape_wal<'media>(
    validation: IntegrityValidatedWalFrame<'media>,
) -> IntegrityValidatedWalFrame<'static> {
    validation
}

fn main() {}
