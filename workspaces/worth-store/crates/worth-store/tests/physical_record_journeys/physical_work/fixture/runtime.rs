use std::path::Path;

use worth_store::physical_runtime::{PhysicalWorkProfileDeclaration, ServingPhysicalRuntime};

pub(crate) fn serving_from_initialization_with_work_profile(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
) -> ServingPhysicalRuntime {
    let (format, placement, access) = super::super::configuration();
    super::super::success(
        super::super::media(root).initialize_record_store(
            worth_store::physical_runtime::PhysicalRecordInitialization::new(
                format, placement, access,
            )
            .with_physical_work_profile(profile),
        ),
    )
}

pub(in crate::physical_work) fn serving_from_open_with_work_profile(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
) -> ServingPhysicalRuntime {
    let (format, _, access) = super::super::configuration();
    super::super::success(
        super::super::media(root).open_record_store(
            worth_store::physical_runtime::PhysicalRecordOpen::new(format, access)
                .with_physical_work_profile(profile),
        ),
    )
}
