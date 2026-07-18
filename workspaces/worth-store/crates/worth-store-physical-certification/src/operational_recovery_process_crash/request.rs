use std::path::Path;
use std::process::Command;

use crate::{OperationalRecoveryDriverTrace, OperationalRecoveryYieldpoint};

pub struct OperationalRecoveryControlCutRequest<'a> {
    pub(super) media_root: &'a Path,
    pub(super) scenario_identity: &'a str,
    pub(super) cut_command: &'a mut Command,
    pub(super) reopen_command: &'a mut Command,
    pub(super) scenario_environment_keys: &'a [&'a str],
    pub(super) yieldpoint: OperationalRecoveryYieldpoint,
    pub(super) uninterrupted_trace: &'a OperationalRecoveryDriverTrace,
}

impl<'a> OperationalRecoveryControlCutRequest<'a> {
    pub fn new(
        media_root: &'a Path,
        scenario_identity: &'a str,
        cut_command: &'a mut Command,
        reopen_command: &'a mut Command,
        scenario_environment_keys: &'a [&'a str],
        yieldpoint: OperationalRecoveryYieldpoint,
        uninterrupted_trace: &'a OperationalRecoveryDriverTrace,
    ) -> Self {
        Self {
            media_root,
            scenario_identity,
            cut_command,
            reopen_command,
            scenario_environment_keys,
            yieldpoint,
            uninterrupted_trace,
        }
    }
}
