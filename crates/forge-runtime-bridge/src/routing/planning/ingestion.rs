use crate::error::BridgeRouteError;
use crate::facade::RuntimeBridge;
use crate::input::envelope::BridgeCommittedPatchEnvelope;
use crate::routing::context::BridgeMappingContext;
use crate::routing::counters::BridgeRoutingCounters;
use crate::routing::eligibility::{
    validate_route_request, EligibleRouteEntry, EligibleRouteRequest,
};
use crate::routing::scope::RouteScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IngestedBridgePatch {
    envelope: BridgeCommittedPatchEnvelope,
    mapping_context: BridgeMappingContext,
    route_scope: RouteScope,
}

impl IngestedBridgePatch {
    pub(crate) fn new(
        envelope: BridgeCommittedPatchEnvelope,
        mapping_context: BridgeMappingContext,
        route_scope: RouteScope,
    ) -> Self {
        Self {
            envelope,
            mapping_context,
            route_scope,
        }
    }

    pub(crate) fn envelope(&self) -> &BridgeCommittedPatchEnvelope {
        &self.envelope
    }

    pub(crate) fn mapping_context(&self) -> &BridgeMappingContext {
        &self.mapping_context
    }

    pub(crate) fn with_mapping_context(mut self, mapping_context: BridgeMappingContext) -> Self {
        self.mapping_context = mapping_context;
        self
    }

    pub(crate) fn route_scope(&self) -> RouteScope {
        self.route_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EligibleBridgeRouting {
    ingested_patch: IngestedBridgePatch,
    entries: Vec<EligibleRouteEntry>,
    counters: BridgeRoutingCounters,
}

impl EligibleBridgeRouting {
    pub(crate) fn new(
        ingested_patch: IngestedBridgePatch,
        entries: Vec<EligibleRouteEntry>,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            ingested_patch,
            entries,
            counters,
        }
    }

    pub(crate) fn envelope(&self) -> &BridgeCommittedPatchEnvelope {
        self.ingested_patch.envelope()
    }

    pub(crate) fn mapping_context(&self) -> &BridgeMappingContext {
        self.ingested_patch.mapping_context()
    }

    pub(crate) fn entries(&self) -> &[EligibleRouteEntry] {
        &self.entries
    }

    pub(crate) fn counters(&self) -> BridgeRoutingCounters {
        self.counters
    }

    pub(crate) fn route_scope(&self) -> RouteScope {
        self.ingested_patch.route_scope()
    }
}

pub(super) fn into_eligible_bridge_routing(
    runtime: &RuntimeBridge,
    ingested: IngestedBridgePatch,
) -> Result<EligibleBridgeRouting, BridgeRouteError> {
    let eligible = validate_route_request(
        ingested.envelope.clone(),
        &runtime.mapping_registry,
        &runtime.aspect_registry,
    )?;
    Ok(EligibleBridgeRouting::from((ingested, eligible)))
}

impl From<(IngestedBridgePatch, EligibleRouteRequest)> for EligibleBridgeRouting {
    fn from(value: (IngestedBridgePatch, EligibleRouteRequest)) -> Self {
        let (ingested_patch, eligible) = value;
        Self::new(
            ingested_patch,
            eligible.entries().to_vec(),
            eligible.counters(),
        )
    }
}
