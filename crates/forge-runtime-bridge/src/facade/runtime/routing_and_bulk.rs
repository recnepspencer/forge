use super::*;

impl RuntimeBridge {
    /// Creates a new runtime bridge builder.
    pub fn builder() -> RuntimeBridgeBuilder {
        RuntimeBridgeBuilder::new()
    }

    /// Routes one authoritative truth change through the standard path.
    ///
    /// This is the everyday front door for:
    ///
    /// - ingesting committed truth change
    /// - planning invalidation
    /// - delivering invalidation to the bound compute sink
    ///
    /// Prefer this over the lower-level ingest/plan/deliver sequence unless the
    /// job explicitly needs advanced control.
    pub fn route(
        &self,
        request: impl Into<BridgeRouteRequest>,
    ) -> Result<BridgeRoute, BridgeStandardRouteError> {
        let planned = self.plan_committed_patch(request.into())?;
        let result = self.deliver_invalidation(planned.clone())?;
        Ok(BridgeRoute::new(planned, result))
    }

    /// Returns the runtime policy frozen into this bridge instance.
    ///
    /// Reach for this when you need to explain or verify runtime-wide replay,
    /// diagnostics, or execution guarantees.
    pub fn policy(&self) -> &BridgeRuntimePolicy {
        &self.policy
    }

    /// Evaluates the current bridge-visible result for a routed target.
    ///
    /// This is the standard answer to "what should the compute side see now for
    /// the thing that was just routed?"
    pub fn evaluate_current(
        &self,
        target: BridgeEvaluationTarget,
    ) -> Result<BridgeSignalEvaluationRequest, BridgeDeliveryError> {
        self.prepare_signal_evaluation(target.into_planned_route())
    }

    /// Evaluates an explicit truth view.
    ///
    /// Use this when branch head, branch snapshot, or historical commit basis
    /// is part of the job rather than an internal detail.
    pub fn evaluate(
        &self,
        request: BridgeTruthViewEvaluationRequest,
    ) -> Result<BridgeTruthViewEvaluation, BridgeDeliveryError> {
        let planned = self.plan_truth_view_packet(request.declaration(), request.read_packet())?;
        let observation = self.materialize_truth_view_observation(planned)?;
        let canonical_record = self.canonicalize_historical_evaluation_record(&observation);
        Ok(BridgeTruthViewEvaluation::new(
            observation,
            canonical_record,
        ))
    }

    /// Specialist ingress step that turns a route request into a committed-patch envelope.
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

    /// Plans one already-ingested committed-patch envelope with default mapping context.
    pub fn plan_envelope(
        &self,
        envelope: BridgeCommittedPatchEnvelope,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.plan_envelope_with_mapping_context(envelope, BridgeMappingContext::default())
    }

    /// Plans one already-ingested committed-patch envelope with explicit mapping context.
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

    /// Plans one already-ingested envelope under explicit mapping context and route policy.
    pub fn plan_envelope_with_mapping_context_and_route_policy(
        &self,
        envelope: BridgeCommittedPatchEnvelope,
        mapping_context: BridgeMappingContext,
        route_policy: &BridgeRoutePlanningPolicy,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.ensure_route_planning_policy_compatible(route_policy)?;
        crate::routing::planning::plan_ingested_patch(
            self,
            crate::routing::IngestedBridgePatch::new(
                envelope,
                mapping_context,
                crate::routing::scope::RouteScope::begin()
                    .with_route_planning_policy(route_policy.clone()),
            ),
        )
    }

    /// Plans one committed patch through the routing substrate without delivering it.
    ///
    /// Prefer [`RuntimeBridge::route`] for ordinary work. This is the advanced
    /// door for callers that need to inspect or stage the planned route.
    pub fn plan_committed_patch(
        &self,
        request: BridgeRouteRequest,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.plan_committed_patch_with_mapping_context(request, BridgeMappingContext::default())
    }

    /// Plans one committed patch with explicit mapping context.
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

    /// Plans one committed patch with an explicit route policy.
    pub fn plan_committed_patch_with_route_policy(
        &self,
        request: BridgeRouteRequest,
        route_policy: &BridgeRoutePlanningPolicy,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.plan_committed_patch_with_mapping_context_and_route_policy(
            request,
            BridgeMappingContext::default(),
            route_policy,
        )
    }

    /// Plans one committed patch with both explicit mapping context and route policy.
    pub fn plan_committed_patch_with_mapping_context_and_route_policy(
        &self,
        request: BridgeRouteRequest,
        mapping_context: BridgeMappingContext,
        route_policy: &BridgeRoutePlanningPolicy,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        self.ensure_route_planning_policy_compatible(route_policy)?;
        let ingested = crate::input::ingress::ingest_committed_patch(self, request)?;
        crate::routing::planning::plan_ingested_patch(
            self,
            ingested
                .with_mapping_context(mapping_context)
                .with_route_scope(
                    crate::routing::scope::RouteScope::begin()
                        .with_route_planning_policy(route_policy.clone()),
                ),
        )
    }

    pub(crate) fn plan_committed_patch_with_mapping_context_and_route_policy_digest_for_replay(
        &self,
        request: BridgeRouteRequest,
        mapping_context: BridgeMappingContext,
        route_policy_digest: &str,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        let ingested = crate::input::ingress::ingest_committed_patch(self, request)?;
        crate::routing::planning::plan_ingested_patch(
            self,
            ingested
                .with_mapping_context(mapping_context)
                .with_route_scope(
                    crate::routing::scope::RouteScope::begin()
                        .with_route_planning_policy_digest(route_policy_digest.to_owned()),
                ),
        )
    }

    pub(crate) fn plan_committed_patch_with_mapping_context_and_route_policy_for_replay(
        &self,
        request: BridgeRouteRequest,
        mapping_context: BridgeMappingContext,
        route_policy: &BridgeRoutePlanningPolicy,
    ) -> Result<BridgePlannedRoute, BridgeRouteError> {
        let ingested = crate::input::ingress::ingest_committed_patch(self, request)?;
        crate::routing::planning::plan_ingested_patch(
            self,
            ingested
                .with_mapping_context(mapping_context)
                .with_route_scope(
                    crate::routing::scope::RouteScope::begin()
                        .with_route_planning_policy(route_policy.clone()),
                ),
        )
    }

    /// Plans a bulk bridge workload under the runtime's default route policy.
    ///
    /// This is an advanced execution-planning surface used by bulk and
    /// certification workflows.
    pub fn plan_bulk_workload(
        &self,
        request: BridgeBulkWorkloadRequest,
    ) -> Result<BridgeBulkWorkloadPlan, BridgeRouteError> {
        crate::routing::planning::plan_bulk_workload(self, request)
    }

    /// Plans a bulk bridge workload under an explicit route policy.
    pub fn plan_bulk_workload_with_route_policy(
        &self,
        request: BridgeBulkWorkloadRequest,
        route_policy: &BridgeRoutePlanningPolicy,
    ) -> Result<BridgeBulkWorkloadPlan, BridgeRouteError> {
        self.ensure_route_planning_policy_compatible(route_policy)?;
        crate::routing::planning::plan_bulk_workload_with_route_policy(self, request, route_policy)
    }

    /// Canonicalizes and records a bulk workload plan for replay and diagnostics.
    pub fn canonicalize_bulk_workload_plan(
        &self,
        plan: &BridgeBulkWorkloadPlan,
    ) -> BridgeCanonicalBulkPlanRecord {
        let record = crate::routing::BridgeCanonicalBulkPlanRecord::from_bulk_workload_plan(plan);
        self.diagnostics.record_bulk(record.clone());
        record
    }

    /// Delivers one previously planned route to the configured compute sink.
    pub fn deliver_invalidation(
        &self,
        route: BridgePlannedRoute,
    ) -> Result<BridgeRouteResult, BridgeDeliveryError> {
        crate::delivery::deliver_planned_route(self, route)
    }

    /// Prepares a planned route for later delivery.
    pub fn prepare_delivery(&self, route: BridgePlannedRoute) -> BridgePreparedDeliveryRequest {
        crate::delivery::prepare_planned_route_for_delivery(route)
    }

    /// Delivers a previously prepared route.
    pub fn deliver_prepared(
        &self,
        prepared: BridgePreparedDeliveryRequest,
    ) -> Result<BridgeRouteResult, BridgeDeliveryError> {
        crate::delivery::deliver_prepared_route(self, prepared)
    }

    /// Delivers a previously planned bulk workload.
    pub fn deliver_bulk_workload_plan(
        &self,
        plan: BridgeBulkWorkloadPlan,
    ) -> Result<BridgeBulkWorkloadResult, BridgeDeliveryError> {
        crate::delivery::deliver_bulk_workload_plan(self, plan)
    }

    /// Prepares a signal evaluation request from a planned route.
    pub fn prepare_signal_evaluation(
        &self,
        route: BridgePlannedRoute,
    ) -> Result<BridgeSignalEvaluationRequest, BridgeDeliveryError> {
        crate::delivery::prepare_signal_evaluation(self, route)
    }

    /// Replays and verifies a canonical route record.
    pub fn replay_canonical_record(
        &self,
        record: &BridgeCanonicalRouteRecord,
    ) -> Result<BridgeReplaySummary, BridgeReplayError> {
        let route_record = record.decode()?;
        crate::routing::replay_route_record(self, &route_record)
    }

    /// Replays and verifies a canonical bulk workload record.
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

    /// Opens the standard diagnostics door for this bridge.
    ///
    /// The returned wrapper keeps the everyday, job-shaped helpers in front
    /// while still allowing access to the raw diagnostics facade when needed.
    pub fn diagnostics(&self) -> BridgeDiagnostics<'_> {
        BridgeDiagnostics::new(&self.diagnostics)
    }

    /// Projects a route planning policy from a lowered execution policy.
    pub fn project_route_planning_policy(
        &self,
        lowered: &LoweredBridgeExecutionPolicy,
    ) -> Result<BridgeRoutePlanningPolicy, BridgeRouteError> {
        let route_policy = lowered.route_planning_policy();
        self.ensure_route_planning_policy_compatible(&route_policy)?;
        Ok(route_policy)
    }

    /// Returns the configured continuity lineage source, if one is bound.
    pub fn continuity_lineage_source(&self) -> Option<&dyn ContinuityLineageSource> {
        self.continuity_lineage_source.as_deref()
    }

    /// Plans continuity requests from one retained route record.
    pub fn plan_continuity_requests(
        &self,
        prior_route_record: &BridgeRouteRecord,
    ) -> Result<BridgeEligibleContinuityRequestSet, BridgeContinuityError> {
        let planned = crate::continuity::BridgePlannedContinuityRequestSet::from_route_record(
            prior_route_record,
        )?;
        crate::continuity::BridgeEligibleContinuityRequestSet::from_planned(planned)
    }

    /// Materializes one historical lineage packet from eligible continuity requests.
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

impl RuntimeBridge {
    fn ensure_route_planning_policy_compatible(
        &self,
        route_policy: &BridgeRoutePlanningPolicy,
    ) -> Result<(), BridgeRouteError> {
        if route_policy.diagnostics_tier() > self.policy.diagnostics_tier() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::RoutePolicyMismatch,
                format!(
                    "Route planning policy `{}` requires diagnostics tier `{:?}` but runtime baseline admits only `{:?}`.",
                    route_policy.digest(),
                    route_policy.diagnostics_tier(),
                    self.policy.diagnostics_tier()
                ),
            ));
        }
        if route_policy.route_artifacts() && !self.policy.record_route_artifacts() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::RoutePolicyMismatch,
                format!(
                    "Route planning policy `{}` requires route artifacts but runtime baseline disables route artifact retention.",
                    route_policy.digest()
                ),
            ));
        }
        if route_policy.replay_artifacts() && !self.policy.allow_replay_artifacts() {
            return Err(BridgeRouteError::new(
                BridgeRouteErrorKind::RoutePolicyMismatch,
                format!(
                    "Route planning policy `{}` requires replay artifacts but runtime baseline disables replay retention.",
                    route_policy.digest()
                ),
            ));
        }
        Ok(())
    }
}
