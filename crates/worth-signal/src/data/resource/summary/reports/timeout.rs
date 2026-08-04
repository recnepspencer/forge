use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use crate::data::resource::timeout::{
    DeniedResourceTimeout, DeniedResourceTimeoutHeartbeatExtension,
    ExtendedResourceTimeoutHeartbeat, TimedOutResourceRequest,
};
use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceTimeoutReport {
    timed_out_request: Option<TimedOutResourceRequest>,
    denied_timeout: Option<DeniedResourceTimeout>,
    lifecycle: Option<ResourceLifecycleSummary>,
    transition: Option<ResourceLifecycleTransition>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceTimeoutHeartbeatExtensionReport {
    extended_heartbeat: Option<ExtendedResourceTimeoutHeartbeat>,
    denied_extension: Option<DeniedResourceTimeoutHeartbeatExtension>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceTimeoutHeartbeatExtensionReport {
    pub(crate) fn admitted(
        extended_heartbeat: ExtendedResourceTimeoutHeartbeat,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            extended_heartbeat: Some(extended_heartbeat),
            denied_extension: None,
            performance,
        }
    }

    pub(crate) fn denied(
        denied_extension: DeniedResourceTimeoutHeartbeatExtension,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            extended_heartbeat: None,
            denied_extension: Some(denied_extension),
            performance,
        }
    }

    pub fn extended_heartbeat(&self) -> Option<&ExtendedResourceTimeoutHeartbeat> {
        self.extended_heartbeat.as_ref()
    }

    pub fn denied_extension(&self) -> Option<DeniedResourceTimeoutHeartbeatExtension> {
        self.denied_extension
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}

impl ResourceTimeoutReport {
    pub(crate) fn admitted(
        timed_out_request: TimedOutResourceRequest,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            timed_out_request: Some(timed_out_request),
            denied_timeout: None,
            lifecycle: Some(lifecycle),
            transition: Some(transition),
            performance,
        }
    }

    pub(crate) fn denied(
        denied_timeout: DeniedResourceTimeout,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            timed_out_request: None,
            denied_timeout: Some(denied_timeout),
            lifecycle: None,
            transition: None,
            performance,
        }
    }

    pub fn timed_out_request(&self) -> Option<&TimedOutResourceRequest> {
        self.timed_out_request.as_ref()
    }

    pub fn denied_timeout(&self) -> Option<DeniedResourceTimeout> {
        self.denied_timeout
    }

    pub fn lifecycle(&self) -> Option<ResourceLifecycleSummary> {
        self.lifecycle
    }

    pub fn transition(&self) -> Option<ResourceLifecycleTransition> {
        self.transition
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
