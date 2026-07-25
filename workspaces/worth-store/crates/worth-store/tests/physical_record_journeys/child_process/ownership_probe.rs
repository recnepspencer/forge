use std::{io::Write, path::Path};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
};
use worth_store_physical_backend::FilesystemAccessPosture;

pub(super) fn run(root: &Path) {
    let admitted = PhysicalRuntimeAdmission::new(root)
        .ok()
        .and_then(|request| PhysicalStore::admit(request).ok());
    let admitted = admitted.is_some_and(|runtime| {
        matches!(
            runtime
                .try_admit_filesystem_media(FilesystemMediaAdmission::production(
                    FilesystemAccessPosture::CoordinatedServiceAccount,
                ))
                .into_raw(),
            TransitionOutcome::Success(_)
        )
    });
    println!(
        "C5_SECOND_OWNER {}",
        if admitted { "admitted" } else { "denied" }
    );
    std::io::stdout().flush().unwrap();
}
