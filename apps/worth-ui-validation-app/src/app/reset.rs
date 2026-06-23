use crate::reload::{ValidationReloadEvidenceLog, ValidationReloadLoop};

use super::support::restore_baseline_observed_files;
use super::ValidationWorkbenchApp;

impl ValidationWorkbenchApp {
    pub(crate) fn reset_to_baseline(&mut self) {
        restore_baseline_observed_files(&self.reload_loop_config, &self.baseline_authored_inputs)
            .expect("baseline authored files should be restorable");
        let launch = crate::ValidationWorkbenchLaunch::new()
            .prepare_from_authored_inputs(self.baseline_authored_inputs.clone())
            .expect("baseline validation app launch should remain valid");
        self.workbench = launch.into_runtime_workbench();
        self.reload_loop = ValidationReloadLoop::start(self.reload_loop_config.clone())
            .expect("baseline reload loop should restart");
        self.evidence_log = ValidationReloadEvidenceLog::default();
        self.observed_startup = None;
        self.last_executed_flow = None;
        self.last_primitive_interaction = None;
        self.last_primitive_interaction_denial = None;
        self.staged_manual_reload_edit = None;
    }
}
