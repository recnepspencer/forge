use forge_store_operations::{ImportPlacementPlan, ImportPlacementSource};

fn main() {
    let _ = ImportPlacementPlan::already_present_locally(ImportPlacementSource::InlineInBundle, 1)
        .admit_import_readmission_layout();
}
