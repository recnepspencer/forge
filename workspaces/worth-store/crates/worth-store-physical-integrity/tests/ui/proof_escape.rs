use worth_store_physical_integrity::IntegrityValidatedRootManifest;

fn escape<'media>(
    validation: IntegrityValidatedRootManifest<'media>,
) -> IntegrityValidatedRootManifest<'static> {
    validation
}

fn main() {}
