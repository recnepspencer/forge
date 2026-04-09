use super::*;

impl RuntimeBridge {
    pub fn builder() -> RuntimeBridgeBuilder {
        RuntimeBridgeBuilder::new()
    }

    pub fn policy(&self) -> &BridgeRuntimePolicy {
        &self.policy
    }

    pub fn ingest_committed_patch(
        &self,
        request: BridgeRouteRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, BridgeRouteError> {
        Ok(
            crate::input::ingress::ingest_committed_patch(self, request)?
                .envelope()
                .clone(),
        )
    }

    pub fn plan_envelope(
        &self,
        envelope: BridgeCommittedPatchEnvelope,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.plan_envelope_with_mapping_context(envelope, BridgeMappingContext::default())
    }

    pub fn plan_envelope_with_mapping_context(
        &self,
        envelope: BridgeCommittedPatchEnvelope,
        mapping_context: BridgeMappingContext,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        crate::routing::planning::plan_ingested_patch(
            self,
            crate::routing::IngestedBridgePatch::new(
                envelope,
                mapping_context,
                crate::routing::scope::RouteScope::begin(),
            ),
        )
    }

    pub fn plan_committed_patch(
        &self,
        request: BridgeRouteRequest,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.plan_committed_patch_with_mapping_context(request, BridgeMappingContext::default())
    }

    pub fn plan_committed_patch_with_mapping_context(
        &self,
        request: BridgeRouteRequest,
        mapping_context: BridgeMappingContext,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        let ingested = crate::input::ingress::ingest_committed_patch(self, request)?;
        crate::routing::planning::plan_ingested_patch(
            self,
            ingested.with_mapping_context(mapping_context),
        )
    }

    pub fn plan_bulk_workload(
        &self,
        request: BridgeBulkWorkloadRequest,
    ) -> Result<BridgeBulkWorkloadPlan, BridgeRouteError> {
        crate::routing::planning::plan_bulk_workload(self, request)
    }

    pub fn canonicalize_bulk_workload_plan(
        &self,
        plan: &BridgeBulkWorkloadPlan,
    ) -> BridgeCanonicalBulkPlanRecord {
        let record = crate::routing::BridgeCanonicalBulkPlanRecord::from_bulk_workload_plan(plan);
        self.diagnostics.record_bulk(record.clone());
        record
    }

    pub fn deliver_invalidation(
        &self,
        route: BridgePlannedRoute,
    ) -> Result<BridgeRouteResult, BridgeDeliveryError> {
        crate::delivery::deliver_planned_route(self, route)
    }

    pub fn prepare_delivery(&self, route: BridgePlannedRoute) -> BridgePreparedDeliveryRequest {
        crate::delivery::prepare_planned_route_for_delivery(route)
    }

    pub fn deliver_prepared(
        &self,
        prepared: BridgePreparedDeliveryRequest,
    ) -> Result<BridgeRouteResult, BridgeDeliveryError> {
        crate::delivery::deliver_prepared_route(self, prepared)
    }

    pub fn deliver_bulk_workload_plan(
        &self,
        plan: BridgeBulkWorkloadPlan,
    ) -> Result<BridgeBulkWorkloadResult, BridgeDeliveryError> {
        crate::delivery::deliver_bulk_workload_plan(self, plan)
    }

    pub fn prepare_signal_evaluation(
        &self,
        route: BridgePlannedRoute,
    ) -> Result<BridgeSignalEvaluationRequest, BridgeDeliveryError> {
        crate::delivery::prepare_signal_evaluation(self, route)
    }

    pub fn replay_canonical_record(
        &self,
        record: &BridgeCanonicalRouteRecord,
    ) -> Result<BridgeReplaySummary, BridgeReplayError> {
        let route_record = record.decode()?;
        crate::routing::replay_route_record(self, &route_record)
    }

    pub fn replay_canonical_bulk_plan_record(
        &self,
        record: &BridgeCanonicalBulkPlanRecord,
    ) -> Result<BridgeBulkWorkloadPlan, BridgeReplayError> {
        if !self.policy.allow_replay_artifacts() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::ReplayArtifactsDisabled,
                "Bridge replay artifacts are disabled by runtime policy.",
            ));
        }

        let record = record.decode()?;
        let replayed =
            self.plan_bulk_workload(record.request().clone())
                .map_err(|error| {
                    BridgeReplayError::new(
                BridgeReplayErrorKind::BulkPlanReplayMismatch,
                format!("Bridge bulk replay failed to reconstruct the planned workload: {error}"),
            )
                })?;

        if replayed.workload_identity() != record.workload_identity()
            || replayed.canonical_request().digest() != record.canonical_request_digest()
            || replayed.normalized_summary().digest() != record.normalized_summary_digest()
            || replayed.canonical_planning_identity() != record.canonical_planning_identity()
            || replayed.admission_profile_identity() != record.admission_profile_identity()
            || replayed.packet_set().digest() != record.packet_set_digest()
            || replayed.execution_plan().digest() != record.execution_plan_digest()
            || replayed.execution_plan().reduced_artifact().digest()
                != record.reduced_artifact_digest()
        {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::BulkPlanReplayMismatch,
                format!(
                    "Bridge bulk replay reconstructed workload `{}` / plan `{}` / execution `{}` but the canonical record expected `{}` / `{}` / `{}`.",
                    replayed.workload_identity().as_str(),
                    replayed.canonical_planning_identity().as_str(),
                    replayed.execution_plan().digest(),
                    record.workload_identity().as_str(),
                    record.canonical_planning_identity().as_str(),
                    record.execution_plan_digest()
                ),
            ));
        }

        Ok(replayed)
    }

    pub fn diagnostics(&self) -> &BridgeDiagnosticsFacade {
        &self.diagnostics
    }

    pub fn continuity_lineage_source(&self) -> Option<&dyn ContinuityLineageSource> {
        self.continuity_lineage_source.as_deref()
    }

    pub fn plan_continuity_requests(
        &self,
        prior_route_record: &BridgeRouteRecord,
    ) -> Result<BridgeEligibleContinuityRequestSet, BridgeContinuityError> {
        let planned = crate::continuity::BridgePlannedContinuityRequestSet::from_route_record(
            prior_route_record,
        )?;
        crate::continuity::BridgeEligibleContinuityRequestSet::from_planned(planned)
    }

    pub fn plan_historical_lineage_packet(
        &self,
        requests: &BridgeEligibleContinuityRequestSet,
    ) -> Result<BridgeHistoricalLineagePacket, BridgeContinuityError> {
        let source = self.continuity_lineage_source().ok_or_else(|| {
            BridgeContinuityError::new(
                BridgeContinuityErrorKind::MissingLineageSource,
                "Bridge historical lineage planning requires a configured continuity lineage source.",
            )
        })?;
        let mut entries = Vec::with_capacity(requests.requests().len());
        for request in requests.requests() {
            let lineage_authority = source
                .historical_lineage(BridgeHistoricalLineageRequest::new(
                    requests.authority_basis().clone(),
                    request.prior_slice().clone(),
                ))
                .map_err(|error| match error.kind() {
                    BridgeLineageSourceErrorKind::UnsupportedContinuityClass => {
                        BridgeContinuityError::new(
                            BridgeContinuityErrorKind::UnsupportedContinuityClass,
                            format!(
                                "Bridge continuity request `{}` targeted an unsupported continuity class: {error}",
                                request.request_key()
                            ),
                        )
                    }
                    BridgeLineageSourceErrorKind::HistoricalResolutionFailure => {
                        BridgeContinuityError::new(
                            BridgeContinuityErrorKind::HistoricalResolutionFailure,
                            format!(
                                "Bridge failed to resolve historical lineage for continuity request `{}`: {error}",
                                request.request_key()
                            ),
                        )
                    }
                })?;
            if lineage_authority.authority_basis() != requests.authority_basis() {
                return Err(BridgeContinuityError::new(
                    BridgeContinuityErrorKind::LineageAuthorityMismatch,
                    format!(
                        "Bridge historical lineage authority for continuity request `{}` did not match the planned branch/snapshot authority basis.",
                        request.request_key()
                    ),
                ));
            }
            entries.push(crate::continuity::BridgeHistoricalLineagePacketEntry::new(
                request.request_key(),
                request.prior_slice().clone(),
                lineage_authority,
            ));
        }
        Ok(crate::continuity::BridgeHistoricalLineagePacket::from_entries(requests, entries))
    }
}
