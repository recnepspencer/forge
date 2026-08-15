use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;
use crate::data::output::PartitionInterner;
use crate::data::telemetry::RuntimeTelemetry;
use crate::diagnostics::state::DiagnosticsState;

use super::{BranchMutationRecord, ReconstructionCounters};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct RuntimeObservation {
    #[serde(skip, default)]
    pub(in crate::data::graph) telemetry: RuntimeTelemetry,
    #[serde(skip, default)]
    pub(in crate::data::graph) reconstruction_counters: ReconstructionCounters,
    #[serde(default)]
    pub(in crate::data::graph) partition_interner: PartitionInterner,
    #[serde(default)]
    pub(in crate::data::graph) branch_mutation_view: BTreeMap<NodeId, BranchMutationRecord>,
    #[serde(default)]
    pub(in crate::data::graph) branch_mutation_records: BTreeMap<NodeId, BranchMutationRecord>,
    #[serde(skip, default)]
    pub(in crate::data::graph) diagnostics: DiagnosticsState,
}

impl RuntimeObservation {
    pub(crate) fn telemetry_mut(&mut self) -> &mut RuntimeTelemetry {
        &mut self.telemetry
    }

    pub(crate) fn partition_interner_mut(&mut self) -> &mut PartitionInterner {
        &mut self.partition_interner
    }

    pub(crate) fn partition_interner(&self) -> &PartitionInterner {
        &self.partition_interner
    }
}
