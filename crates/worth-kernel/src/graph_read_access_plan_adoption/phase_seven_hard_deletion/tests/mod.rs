mod deletion_or_cap;
mod manual_plan_hint_residue;
mod phase_chain_fixture;
mod phase_eight_seed;
mod source_firewall;

use std::path::{Path, PathBuf};

use crate::graph_read_access_plan_adoption::{
    current_worth_graph_read_access_hard_deletion_closeout,
    WorthGraphReadAccessHardDeletionCloseout,
};

use phase_chain_fixture::production_phase_seven_seed;

fn production_phase_seven_closeout() -> WorthGraphReadAccessHardDeletionCloseout {
    current_worth_graph_read_access_hard_deletion_closeout(&production_phase_seven_seed())
        .expect("Phase 7 should close from Phase 6 seed")
}

struct TempWorkspace {
    root: PathBuf,
}

impl TempWorkspace {
    fn new(name: &str) -> Self {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("worth_phase_seven_{name}_{unique}"));
        std::fs::create_dir_all(&root).expect("temp workspace root should be created");
        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }

    fn write_source(&self, relative_path: &str, source_text: &str) {
        let path = self.root.join(relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("temp source parent should be created");
        }
        std::fs::write(path, source_text).expect("temp source should be written");
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}
