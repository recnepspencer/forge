use worth_signal::facade::{ResourceRuntimeSummary, RuntimeClockBasis};

use super::PhysicalSignalGraph;

pub(in crate::physical_runtime::instance::signal_owner) struct PhysicalSignalGraphObservation {
    resource: ResourceRuntimeSummary,
    request_admission_count: u64,
    active_locality_count: usize,
    active_graph_node_count: usize,
    aspect_invalidation_count: u64,
    clock: RuntimeClockBasis,
}

impl PhysicalSignalGraph {
    pub(super) fn observation(&self) -> PhysicalSignalGraphObservation {
        PhysicalSignalGraphObservation {
            resource: self.runtime.resource_runtime_summary(),
            request_admission_count: self
                .runtime
                .telemetry()
                .resource
                .resource_request_admission_count,
            active_locality_count: self.locality.len(),
            active_graph_node_count: self.runtime.graph().active_node_count(),
            aspect_invalidation_count: self.context.version,
            clock: self.runtime.clock_basis(),
        }
    }
}

impl PhysicalSignalGraphObservation {
    pub(in crate::physical_runtime::instance::signal_owner) const fn resource(
        &self,
    ) -> ResourceRuntimeSummary {
        self.resource
    }

    pub(in crate::physical_runtime::instance::signal_owner) const fn request_admission_count(
        &self,
    ) -> u64 {
        self.request_admission_count
    }

    pub(in crate::physical_runtime::instance::signal_owner) const fn active_locality_count(
        &self,
    ) -> usize {
        self.active_locality_count
    }

    pub(in crate::physical_runtime::instance::signal_owner) const fn active_graph_node_count(
        &self,
    ) -> usize {
        self.active_graph_node_count
    }

    pub(in crate::physical_runtime::instance::signal_owner) const fn aspect_invalidation_count(
        &self,
    ) -> u64 {
        self.aspect_invalidation_count
    }

    pub(in crate::physical_runtime::instance::signal_owner) const fn clock(
        &self,
    ) -> RuntimeClockBasis {
        self.clock
    }
}
