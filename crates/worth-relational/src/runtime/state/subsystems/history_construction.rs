use crate::history::data::BranchId;

use super::HistorySubsystem;
use crate::runtime::RuntimeSubsystem;

impl RuntimeSubsystem for HistorySubsystem {
    type Config = BranchId;

    fn new(config: &Self::Config) -> Self {
        Self::build_with_main_branch(config.clone())
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
