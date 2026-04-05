use crate::durability::access::DurabilityAccess;
use crate::history::logic::HistoryAccess;
use crate::inspection::logic::InspectionAccess;
use crate::logic::runtime::RelationalRuntime;
use crate::merge::logic::MergeAccess;
use crate::publication::logic::PublicationAccess;
use crate::replay::logic::ReplayAccess;
use crate::simulation::logic::{SimulationAccess, SimulationAuthority};
use crate::validation::logic::InvariantAccess;
use crate::visibility::authority::VisibilityAuthority;
use crate::visibility::materialization::read_records::VisibilityReadContext;
use crate::visibility::retention::VisibilityRetentionAuthority;

impl RelationalRuntime {
    pub fn snapshots(&mut self) -> VisibilityAuthority<'_> {
        self.visibility_authority()
    }

    pub fn read_truth(&self) -> VisibilityReadContext<'_> {
        self.visibility_reads()
    }

    pub fn validation(&self) -> InvariantAccess<'_> {
        self.invariant_access()
    }

    pub fn compiled_artifacts(&self) -> SimulationAccess<'_> {
        self.simulation_access()
    }

    pub fn compiled_artifacts_authority(&mut self) -> SimulationAuthority<'_> {
        self.simulation_authority()
    }

    pub fn retention(&mut self) -> VisibilityRetentionAuthority<'_> {
        self.retention_authority()
    }

    pub fn inspect_what_happened(&self) -> InspectionAccess<'_> {
        self.inspection_access()
    }

    pub fn history(&self) -> HistoryAccess<'_> {
        self.history_access()
    }

    pub fn replay(&self) -> ReplayAccess<'_> {
        self.replay_access()
    }

    pub fn publication(&self) -> PublicationAccess<'_> {
        self.publication_access()
    }

    pub fn durability(&self) -> DurabilityAccess<'_> {
        self.durability_access()
    }

    pub fn merge(&self) -> MergeAccess<'_> {
        self.merge_access()
    }
}
