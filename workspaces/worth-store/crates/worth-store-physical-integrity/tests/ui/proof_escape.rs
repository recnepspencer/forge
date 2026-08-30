use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedRootManifest,
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

fn main() {}
