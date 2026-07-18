use std::process::Command;

use worth_store_physical_certification::{
    FreshProcessDestroyedPrimaryEvidence, FreshProcessOfflineTruthBaseline,
    FreshProcessOfflineTruthRunner,
};

use super::ExecutedOwnerWorld;

impl ExecutedOwnerWorld {
    pub fn fresh_process_destroyed_primary_verification(
        &self,
    ) -> FreshProcessDestroyedPrimaryEvidence {
        let page = self.media.source_root().join("page.media");
        let baseline = FreshProcessOfflineTruthBaseline::capture(&page)
            .expect("capture live primary declaration before damage");
        std::fs::write(&page, b"destroyed-primary-page")
            .expect("destroy primary page after live declaration");

        let mut observer = Command::new(std::env::current_exe().expect("S10 test executable"));
        observer
            .arg("--exact")
            .arg("s10_operational_world::fresh_process_destroyed_primary_observer_child")
            .arg("--nocapture");
        FreshProcessOfflineTruthRunner::new(
            self.media
                .workspace_root()
                .join("fresh-process-offline-truth"),
        )
        .certify_destroyed_primary(&baseline, &mut observer)
        .expect("fresh process independently classifies destroyed primary")
    }
}
