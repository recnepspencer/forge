use worth_store_physical_isolation::{CurrentPhysicalRoot, GenerationCountedPhysicalReference};

fn attempt_publication_admission(
    root: CurrentPhysicalRoot,
    counted: GenerationCountedPhysicalReference,
) {
    let _ = root.admit_page_publication_epoch(counted);
}

fn main() {}
