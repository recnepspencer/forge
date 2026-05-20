use crate::basis::ExecutionBasisIntent;
use crate::binding::{
    derive_binding_requirements, resolve_bindings, BindingResolution, BoundBindings,
    NonIdentityBindingMetadata,
};
use crate::collection::{CollectionPlanBundle, CollectionPlanningMode, CollectionResultFamily};
use crate::identity::{
    BindingFulfillmentDigest, CanonicalQueryDigest, CanonicalResultShapeDigest,
    CollectionPlanDigest, PlanDigest, ValidatedQueryDigest, ValidatedResultShapeDigest,
};
use crate::live::LivePromotionDescriptor;
use crate::validation::ValidatedQueryBundle;

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use crate::frontier_planning::{
    BoundedMaterializationFrontierPreflight, BundleResolvedBasisDigest, FrontierAwarePlan,
    FrontierBreadthPrediction, FrontierBundlePlan, FrontierBundleRoutePlanningError,
    FrontierComplexityContract, FrontierCounterSnapshot, FrontierDisjointnessClass,
    FrontierParityBundle, FrontierParityBundleError, FrontierPerformanceStatus, FrontierPlanFamily,
    FrontierPlanningCounters, FrontierPlanningError, FrontierPlanningInput, FrontierPlanningReport,
    FrontierPostureDigest, FrontierPredictionDriftOutcome, FrontierPreflightAdmissionError,
    FrontierRouteCounters, FrontierRoutePlanningError, FrontierRouteReport, FrontierSurfaceDigest,
    OrderedCollectionFrontierPreflight, PacketEquivalenceContract, PacketMergeBoundary,
    PacketMergeContract, ParallelAdmissionBundleEvidence, ParallelAdmissionDecision,
    ParallelAdmissionEvidence, ParallelAdmissionRoute, ParallelAdmissionRouteSet,
    PlannedRouteFamily, PlannedWorkPacket, PlannedWorkPacketDigest, PlannedWorkPacketFamily,
    PlannedWorkPacketSet, SerialFallbackBundleEvidence, SerialFallbackBundleRoutes,
    SerialFallbackEvidence, SerialFallbackReason, SerialFallbackRoute,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlannedExecutionRoute {
    RuntimeSnapshotRead,
    RuntimeExpandedSnapshotRead,
    StoreSnapshotRead,
}

impl PlannedExecutionRoute {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeSnapshotRead => "runtime_snapshot_read",
            Self::RuntimeExpandedSnapshotRead => "runtime_expanded_snapshot_read",
            Self::StoreSnapshotRead => "store_snapshot_read",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FallbackDisposition {
    Forbidden,
    AdmittedButUnused,
    AdmittedAndSelected,
}

impl FallbackDisposition {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Forbidden => "forbidden",
            Self::AdmittedButUnused => "admitted_but_unused",
            Self::AdmittedAndSelected => "admitted_and_selected",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExecutionCostMarker(String);

impl ExecutionCostMarker {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionMechanics {
    cost_markers: Vec<ExecutionCostMarker>,
}

impl ExecutionMechanics {
    pub fn cost_markers(&self) -> &[ExecutionCostMarker] {
        &self.cost_markers
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningSemanticInputs {
    binding_resolution: Option<BindingResolution>,
    basis_intent: ExecutionBasisIntent,
}

impl PlanningSemanticInputs {
    pub(crate) fn new(
        binding_resolution: Option<BindingResolution>,
        basis_intent: ExecutionBasisIntent,
    ) -> Self {
        Self {
            binding_resolution,
            basis_intent,
        }
    }

    pub fn binding_resolution(&self) -> Option<&BindingResolution> {
        self.binding_resolution.as_ref()
    }

    pub fn basis_intent(&self) -> &ExecutionBasisIntent {
        &self.basis_intent
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningAmbientContext {
    metadata: Vec<NonIdentityBindingMetadata>,
}

impl PlanningAmbientContext {
    pub(crate) fn new(metadata: Vec<NonIdentityBindingMetadata>) -> Self {
        Self { metadata }
    }

    pub fn metadata(&self) -> &[NonIdentityBindingMetadata] {
        &self.metadata
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningRequestContext {
    semantic: PlanningSemanticInputs,
    ambient: PlanningAmbientContext,
}

impl PlanningRequestContext {
    pub(crate) fn new(semantic: PlanningSemanticInputs, ambient: PlanningAmbientContext) -> Self {
        Self { semantic, ambient }
    }

    pub fn semantic(&self) -> &PlanningSemanticInputs {
        &self.semantic
    }

    pub fn ambient(&self) -> &PlanningAmbientContext {
        &self.ambient
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannedQueryArtifact {
    validated_query_digest: ValidatedQueryDigest,
    canonical_query_digest: CanonicalQueryDigest,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
    projection_count: usize,
    traversal_count: usize,
    predicate_count: usize,
    ordering_count: usize,
    plan_digest: PlanDigest,
}

impl PlannedQueryArtifact {
    pub fn validated_query_digest(&self) -> &ValidatedQueryDigest {
        &self.validated_query_digest
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn route(&self) -> &PlannedExecutionRoute {
        &self.route
    }

    pub fn fallback(&self) -> &FallbackDisposition {
        &self.fallback
    }

    pub fn plan_digest(&self) -> &PlanDigest {
        &self.plan_digest
    }

    pub fn projection_count(&self) -> usize {
        self.projection_count
    }

    pub fn traversal_count(&self) -> usize {
        self.traversal_count
    }

    pub fn predicate_count(&self) -> usize {
        self.predicate_count
    }

    pub fn ordering_count(&self) -> usize {
        self.ordering_count
    }

    pub(crate) fn new(
        validated_query_digest: ValidatedQueryDigest,
        canonical_query_digest: CanonicalQueryDigest,
        validated_result_shape_digest: &ValidatedResultShapeDigest,
        route: PlannedExecutionRoute,
        fallback: FallbackDisposition,
        projection_count: usize,
        traversal_count: usize,
        predicate_count: usize,
        ordering_count: usize,
        collection_digest: Option<&CollectionPlanDigest>,
        binding_digest: Option<&BindingFulfillmentDigest>,
    ) -> Self {
        let mut parts = vec![
            format!("validated_query:{}", validated_query_digest.as_str()),
            format!(
                "validated_result_shape:{}",
                validated_result_shape_digest.as_str()
            ),
            format!("route:{}", route.as_str()),
            format!("fallback:{}", fallback.as_str()),
            format!("projection_count:{projection_count}"),
            format!("traversal_count:{traversal_count}"),
            format!("predicate_count:{predicate_count}"),
            format!("ordering_count:{ordering_count}"),
        ];
        if let Some(collection_digest) = collection_digest {
            parts.push(format!("collection:{}", collection_digest.as_str()));
        }
        if let Some(binding_digest) = binding_digest {
            parts.push(format!("binding:{}", binding_digest.as_str()));
        }

        Self {
            validated_query_digest,
            canonical_query_digest,
            route,
            fallback,
            projection_count,
            traversal_count,
            predicate_count,
            ordering_count,
            plan_digest: PlanDigest::from_parts(&parts),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannedResultShapeArtifact {
    validated_result_shape_digest: ValidatedResultShapeDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    binding_count: usize,
}

impl PlannedResultShapeArtifact {
    pub fn validated_result_shape_digest(&self) -> &ValidatedResultShapeDigest {
        &self.validated_result_shape_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn binding_count(&self) -> usize {
        self.binding_count
    }

    pub(crate) fn new(
        validated_result_shape_digest: ValidatedResultShapeDigest,
        canonical_result_shape_digest: CanonicalResultShapeDigest,
        binding_count: usize,
    ) -> Self {
        Self {
            validated_result_shape_digest,
            canonical_result_shape_digest,
            binding_count,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PlanningCounters {
    planned_projection_entry_count: usize,
    planned_traversal_clause_count: usize,
    route_candidate_count: usize,
    planned_read_surface_count: usize,
    planned_fallback_option_count: usize,
    fallback_denial_count: usize,
    planned_materialization_edge_class_count: usize,
    planned_traversal_depth_limit: usize,
    planned_aggregate_input_breadth: usize,
    planned_cdc_family_count: usize,
}

impl PlanningCounters {
    pub fn planned_projection_entry_count(&self) -> usize {
        self.planned_projection_entry_count
    }

    pub fn planned_traversal_clause_count(&self) -> usize {
        self.planned_traversal_clause_count
    }

    pub fn route_candidate_count(&self) -> usize {
        self.route_candidate_count
    }

    pub fn planned_read_surface_count(&self) -> usize {
        self.planned_read_surface_count
    }

    pub fn planned_fallback_option_count(&self) -> usize {
        self.planned_fallback_option_count
    }

    pub fn fallback_denial_count(&self) -> usize {
        self.fallback_denial_count
    }

    pub fn planned_materialization_edge_class_count(&self) -> usize {
        self.planned_materialization_edge_class_count
    }

    pub fn planned_traversal_depth_limit(&self) -> usize {
        self.planned_traversal_depth_limit
    }

    pub fn planned_aggregate_input_breadth(&self) -> usize {
        self.planned_aggregate_input_breadth
    }

    pub fn planned_cdc_family_count(&self) -> usize {
        self.planned_cdc_family_count
    }

    pub(crate) fn new(
        planned_projection_entry_count: usize,
        planned_traversal_clause_count: usize,
        route_candidate_count: usize,
        planned_read_surface_count: usize,
        planned_fallback_option_count: usize,
        fallback_denial_count: usize,
        planned_materialization_edge_class_count: usize,
        planned_traversal_depth_limit: usize,
        planned_aggregate_input_breadth: usize,
        planned_cdc_family_count: usize,
    ) -> Self {
        Self {
            planned_projection_entry_count,
            planned_traversal_clause_count,
            route_candidate_count,
            planned_read_surface_count,
            planned_fallback_option_count,
            fallback_denial_count,
            planned_materialization_edge_class_count,
            planned_traversal_depth_limit,
            planned_aggregate_input_breadth,
            planned_cdc_family_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningReport {
    plan_digest: PlanDigest,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
    projection_count: usize,
    traversal_count: usize,
    predicate_count: usize,
    ordering_count: usize,
    result_shape_binding_count: usize,
}

impl PlanningReport {
    pub fn plan_digest(&self) -> &PlanDigest {
        &self.plan_digest
    }

    pub fn route(&self) -> &PlannedExecutionRoute {
        &self.route
    }

    pub fn fallback(&self) -> &FallbackDisposition {
        &self.fallback
    }

    pub fn projection_count(&self) -> usize {
        self.projection_count
    }

    pub fn traversal_count(&self) -> usize {
        self.traversal_count
    }

    pub fn predicate_count(&self) -> usize {
        self.predicate_count
    }

    pub fn ordering_count(&self) -> usize {
        self.ordering_count
    }

    pub fn result_shape_binding_count(&self) -> usize {
        self.result_shape_binding_count
    }

    pub(crate) fn new(
        plan_digest: PlanDigest,
        route: PlannedExecutionRoute,
        fallback: FallbackDisposition,
        projection_count: usize,
        traversal_count: usize,
        predicate_count: usize,
        ordering_count: usize,
        result_shape_binding_count: usize,
    ) -> Self {
        Self {
            plan_digest,
            route,
            fallback,
            projection_count,
            traversal_count,
            predicate_count,
            ordering_count,
            result_shape_binding_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanningFailureClass {
    UnsupportedPlanShape,
    IncompletePlanningInputs,
    InternalInvariantBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanningError {
    MissingBindingResolutionForIdentityBoundQuery,
    UnsupportedBackendParityRequest,
    UnsupportedFallbackShape,
    UnsupportedOrderingFamily,
    UnsupportedCursorShape,
    UnsupportedTraversalBound,
    UnsupportedAggregateFamily,
    UnsupportedCollectionResultFamily,
    BindingResolutionFailed { failure_digest: String },
    PlanningInvariantViolation { message: &'static str },
}

impl PlanningError {
    pub fn failure_class(&self) -> PlanningFailureClass {
        match self {
            Self::MissingBindingResolutionForIdentityBoundQuery => {
                PlanningFailureClass::IncompletePlanningInputs
            }
            Self::UnsupportedBackendParityRequest
            | Self::UnsupportedFallbackShape
            | Self::UnsupportedOrderingFamily
            | Self::UnsupportedCursorShape
            | Self::UnsupportedTraversalBound
            | Self::UnsupportedAggregateFamily
            | Self::UnsupportedCollectionResultFamily => PlanningFailureClass::UnsupportedPlanShape,
            Self::BindingResolutionFailed { .. } => PlanningFailureClass::IncompletePlanningInputs,
            Self::PlanningInvariantViolation { .. } => PlanningFailureClass::InternalInvariantBreak,
        }
    }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RequestedTraversalBound {
    UnboundedExpansion,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RequestedAggregateFamily {
    CountRows,
    GroupedIntegerSum,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RequestedDerivedFieldFamily {
    DisplayLabel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPlanBundle {
    query: PlannedQueryArtifact,
    result_shape: PlannedResultShapeArtifact,
    collection: Option<CollectionPlanBundle>,
    live_promotion: LivePromotionDescriptor,
    request_context: PlanningRequestContext,
    report: PlanningReport,
    counters: PlanningCounters,
}

impl ExecutionPlanBundle {
    pub fn query(&self) -> &PlannedQueryArtifact {
        &self.query
    }

    pub fn result_shape(&self) -> &PlannedResultShapeArtifact {
        &self.result_shape
    }

    pub fn collection(&self) -> Option<&CollectionPlanBundle> {
        self.collection.as_ref()
    }

    pub fn live_promotion(&self) -> &LivePromotionDescriptor {
        &self.live_promotion
    }

    pub fn request_context(&self) -> &PlanningRequestContext {
        &self.request_context
    }

    pub fn report(&self) -> &PlanningReport {
        &self.report
    }

    pub fn counters(&self) -> &PlanningCounters {
        &self.counters
    }

    pub fn check_invariants(&self) -> Result<(), PlanningError> {
        if self.report.plan_digest() != self.query.plan_digest() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report digest does not match planned query digest",
            });
        }

        if self.report.route() != self.query.route() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report route does not match planned query route",
            });
        }

        if self.report.fallback() != self.query.fallback() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report fallback does not match planned query fallback",
            });
        }

        if self.report.projection_count() != self.query.projection_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report projection count does not match planned query projection count",
            });
        }

        if self.report.traversal_count() != self.query.traversal_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report traversal count does not match planned query traversal count",
            });
        }

        if self.report.predicate_count() != self.query.predicate_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report predicate count does not match planned query predicate count",
            });
        }

        if self.report.ordering_count() != self.query.ordering_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report ordering count does not match planned query ordering count",
            });
        }

        if self.report.result_shape_binding_count() != self.result_shape.binding_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report binding count does not match planned result-shape binding count",
            });
        }

        Ok(())
    }

    pub(crate) fn new(
        bundle: &ValidatedQueryBundle,
        query: PlannedQueryArtifact,
        result_shape: PlannedResultShapeArtifact,
        collection: Option<CollectionPlanBundle>,
        request_context: PlanningRequestContext,
        counters: PlanningCounters,
    ) -> Result<Self, PlanningError> {
        if query.plan_digest().as_str().is_empty() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planned query digest must not be empty",
            });
        }

        let report = PlanningReport::new(
            query.plan_digest().clone(),
            query.route().clone(),
            query.fallback().clone(),
            query.projection_count(),
            query.traversal_count(),
            query.predicate_count(),
            query.ordering_count(),
            result_shape.binding_count(),
        );
        let live_promotion = LivePromotionDescriptor::for_plan(
            bundle,
            query.plan_digest().clone(),
            collection.as_ref(),
        );
        let bundle = Self {
            query,
            result_shape,
            collection,
            live_promotion,
            request_context,
            report,
            counters,
        };
        bundle.check_invariants()?;
        Ok(bundle)
    }
}

pub fn planning_request_context_for_direct(
    bundle: &ValidatedQueryBundle,
    basis_intent: ExecutionBasisIntent,
) -> Result<PlanningRequestContext, PlanningError> {
    if !bundle.query().identity_bindings().is_empty() {
        return Err(PlanningError::MissingBindingResolutionForIdentityBoundQuery);
    }

    Ok(PlanningRequestContext::new(
        PlanningSemanticInputs::new(None, basis_intent),
        PlanningAmbientContext::new(Vec::new()),
    ))
}

pub fn planning_request_context_for_bound(
    bundle: &ValidatedQueryBundle,
    basis_intent: ExecutionBasisIntent,
    bindings: BoundBindings,
    ambient_metadata: Vec<NonIdentityBindingMetadata>,
) -> Result<PlanningRequestContext, PlanningError> {
    let requirements = derive_binding_requirements(bundle);
    let resolution = resolve_bindings(requirements, bindings).map_err(|error| {
        PlanningError::BindingResolutionFailed {
            failure_digest: error.failure_digest(),
        }
    })?;

    Ok(PlanningRequestContext::new(
        PlanningSemanticInputs::new(Some(resolution), basis_intent),
        PlanningAmbientContext::new(ambient_metadata),
    ))
}

pub fn seed_execution_plan(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
) -> Result<ExecutionPlanBundle, PlanningError> {
    seed_execution_plan_for_collection_mode(
        bundle,
        request_context,
        route,
        fallback,
        CollectionPlanningMode::Ordinary,
    )
}

fn seed_execution_plan_for_collection_mode(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
    collection_mode: CollectionPlanningMode,
) -> Result<ExecutionPlanBundle, PlanningError> {
    if !bundle.query().identity_bindings().is_empty()
        && request_context.semantic().binding_resolution().is_none()
    {
        return Err(PlanningError::MissingBindingResolutionForIdentityBoundQuery);
    }

    let binding_digest = request_context
        .semantic()
        .binding_resolution()
        .and_then(|resolution| {
            if resolution.requirements().requirements().is_empty() {
                None
            } else {
                Some(resolution.digest())
            }
        });
    reject_unsupported_collection_shape(bundle, &collection_mode)?;
    let collection = CollectionPlanBundle::from_validated_bundle_for_mode(bundle, collection_mode);
    let query = PlannedQueryArtifact::new(
        bundle.query().digest().clone(),
        bundle.query().canonical_query_digest().clone(),
        bundle.result_shape().digest(),
        route.clone(),
        fallback.clone(),
        bundle.query().projection().len(),
        bundle.query().traversal().len(),
        bundle.query().predicates().entries().len(),
        bundle.query().ordering().entries().len(),
        collection.as_ref().map(CollectionPlanBundle::digest),
        binding_digest,
    );
    let result_shape = PlannedResultShapeArtifact::new(
        bundle.result_shape().digest().clone(),
        bundle
            .result_shape()
            .canonical_result_shape_digest()
            .clone(),
        bundle.result_shape().bindings().len(),
    );
    let counters = PlanningCounters::new(
        bundle.query().projection().len(),
        bundle.query().traversal().len(),
        route_candidate_count(&route),
        planned_read_surface_count(
            &route,
            bundle.query().projection().len(),
            bundle.query().traversal().len(),
            bundle.query().predicates().entries().len(),
            bundle.query().ordering().entries().len(),
        ),
        usize::from(fallback != FallbackDisposition::Forbidden),
        0,
        collection
            .as_ref()
            .map(|collection| collection.traversal_bound().edge_classes().len())
            .unwrap_or(0),
        collection
            .as_ref()
            .map(|collection| collection.traversal_bound().depth_limit().value() as usize)
            .unwrap_or(0),
        collection
            .as_ref()
            .map(|collection| {
                collection
                    .post_read_shaping()
                    .aggregate_shape()
                    .input_breadth()
                    .value()
            })
            .unwrap_or(0),
        collection
            .as_ref()
            .map(|collection| {
                usize::from(matches!(
                    collection.post_read_shaping().result_family(),
                    crate::collection::CollectionResultFamily::CdcCollection
                ))
            })
            .unwrap_or(0),
    );

    ExecutionPlanBundle::new(
        bundle,
        query,
        result_shape,
        collection,
        request_context,
        counters,
    )
}

fn reject_unsupported_collection_shape(
    bundle: &ValidatedQueryBundle,
    collection_mode: &CollectionPlanningMode,
) -> Result<(), PlanningError> {
    match bundle.query().family() {
        crate::authoring::QueryFamily::Detail => {
            if !matches!(collection_mode, CollectionPlanningMode::Ordinary) {
                return Err(PlanningError::UnsupportedCollectionResultFamily);
            }
        }
        crate::authoring::QueryFamily::Collection => {
            if bundle.query().ordering().entries().is_empty() {
                return Err(PlanningError::UnsupportedCursorShape);
            }
            if bundle.query().ordering().entries().len() > 1 {
                return Err(PlanningError::UnsupportedOrderingFamily);
            }
        }
    }

    Ok(())
}

pub fn plan_validated_bundle(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
) -> Result<ExecutionPlanBundle, PlanningError> {
    plan_validated_bundle_for_collection_family(
        bundle,
        request_context,
        CollectionResultFamily::OrdinaryCollection,
    )
}

pub fn plan_validated_bundle_for_collection_family(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    collection_result_family: CollectionResultFamily,
) -> Result<ExecutionPlanBundle, PlanningError> {
    if request_context.semantic().basis_intent().fallback_allowed() {
        return Err(PlanningError::UnsupportedFallbackShape);
    }

    if matches!(
        request_context.semantic().basis_intent().authority_family(),
        crate::basis::BasisAuthorityFamily::Store
    ) {
        return Err(PlanningError::UnsupportedBackendParityRequest);
    }

    let route = select_route(bundle, &request_context);
    let fallback = if request_context.semantic().basis_intent().fallback_allowed() {
        FallbackDisposition::AdmittedButUnused
    } else {
        FallbackDisposition::Forbidden
    };

    let collection_mode = match collection_result_family {
        CollectionResultFamily::OrdinaryCollection => CollectionPlanningMode::Ordinary,
        CollectionResultFamily::CdcCollection => CollectionPlanningMode::Cdc,
    };
    seed_execution_plan_for_collection_mode(
        bundle,
        request_context,
        route,
        fallback,
        collection_mode,
    )
}

#[cfg(test)]
pub(crate) fn lower_execution_preflight_to_frontier_plan(
    preflight: &crate::basis::ExecutionPreflightBundle,
) -> Result<FrontierAwarePlan, FrontierPlanningError> {
    crate::frontier_planning::lower_preflight_to_frontier_plan(preflight)
}

#[cfg(test)]
pub(crate) fn lower_live_plan_to_frontier_plan(
    live: &crate::live::LiveQueryPlan,
) -> Result<FrontierAwarePlan, FrontierPlanningError> {
    crate::frontier_planning::lower_live_plan_to_frontier_plan(live)
}

#[cfg(test)]
pub(crate) fn lower_frontier_planning_bundle(
    inputs: &[FrontierPlanningInput],
) -> Result<FrontierBundlePlan, FrontierPlanningError> {
    crate::frontier_planning::lower_frontier_bundle(inputs)
}

#[cfg(test)]
pub fn lower_preflight_to_parallel_admission_route(
    preflight: &OrderedCollectionFrontierPreflight,
    evidence: &ParallelAdmissionEvidence,
) -> Result<ParallelAdmissionRoute, FrontierRoutePlanningError> {
    crate::frontier_planning::lower_preflight_to_parallel_admission_route(preflight, evidence)
}

#[cfg(test)]
pub fn lower_preflight_to_serial_fallback_route(
    preflight: &BoundedMaterializationFrontierPreflight,
    evidence: &SerialFallbackEvidence,
) -> Result<SerialFallbackRoute, FrontierRoutePlanningError> {
    crate::frontier_planning::lower_preflight_to_serial_fallback_route(preflight, evidence)
}

#[cfg(test)]
pub fn lower_preflight_bundle_to_parallel_admission_routes(
    preflights: &[OrderedCollectionFrontierPreflight],
    evidences: &crate::frontier_planning::ParallelAdmissionBundleEvidence,
) -> Result<ParallelAdmissionRouteSet, FrontierBundleRoutePlanningError> {
    crate::frontier_planning::lower_preflight_bundle_to_parallel_admission_routes(
        preflights, evidences,
    )
}

#[cfg(test)]
pub fn lower_preflight_bundle_to_serial_fallback_routes(
    preflights: &[BoundedMaterializationFrontierPreflight],
    evidences: &crate::frontier_planning::SerialFallbackBundleEvidence,
) -> Result<SerialFallbackBundleRoutes, FrontierBundleRoutePlanningError> {
    crate::frontier_planning::lower_preflight_bundle_to_serial_fallback_routes(
        preflights, evidences,
    )
}

#[cfg(test)]
pub fn admit_ordered_collection_frontier_preflight(
    preflight: crate::basis::ExecutionPreflightBundle,
) -> Result<OrderedCollectionFrontierPreflight, FrontierPreflightAdmissionError> {
    crate::frontier_planning::admit_ordered_collection_frontier_preflight(preflight)
}

#[cfg(test)]
pub fn admit_bounded_materialization_frontier_preflight(
    preflight: crate::basis::ExecutionPreflightBundle,
) -> Result<BoundedMaterializationFrontierPreflight, FrontierPreflightAdmissionError> {
    crate::frontier_planning::admit_bounded_materialization_frontier_preflight(preflight)
}

#[cfg(test)]
pub(crate) fn plan_validated_bundle_for_requested_traversal_bound(
    _bundle: &ValidatedQueryBundle,
    _request_context: PlanningRequestContext,
    _requested_bound: RequestedTraversalBound,
) -> Result<ExecutionPlanBundle, PlanningError> {
    Err(PlanningError::UnsupportedTraversalBound)
}

#[cfg(test)]
pub(crate) fn plan_validated_bundle_for_requested_aggregate_family(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    requested_aggregate: RequestedAggregateFamily,
) -> Result<ExecutionPlanBundle, PlanningError> {
    match requested_aggregate {
        RequestedAggregateFamily::CountRows => {
            if request_context.semantic().basis_intent().fallback_allowed() {
                return Err(PlanningError::UnsupportedFallbackShape);
            }
            if matches!(
                request_context.semantic().basis_intent().authority_family(),
                crate::basis::BasisAuthorityFamily::Store
            ) {
                return Err(PlanningError::UnsupportedBackendParityRequest);
            }
            let route = select_route(bundle, &request_context);
            seed_execution_plan_for_collection_mode(
                bundle,
                request_context,
                route,
                FallbackDisposition::Forbidden,
                CollectionPlanningMode::AggregateRollupCount,
            )
        }
        RequestedAggregateFamily::GroupedIntegerSum => {
            Err(PlanningError::UnsupportedAggregateFamily)
        }
    }
}

#[cfg(test)]
pub(crate) fn plan_validated_bundle_for_requested_derived_field_family(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    requested_derived_field: RequestedDerivedFieldFamily,
) -> Result<ExecutionPlanBundle, PlanningError> {
    match requested_derived_field {
        RequestedDerivedFieldFamily::DisplayLabel => {
            if request_context.semantic().basis_intent().fallback_allowed() {
                return Err(PlanningError::UnsupportedFallbackShape);
            }
            if matches!(
                request_context.semantic().basis_intent().authority_family(),
                crate::basis::BasisAuthorityFamily::Store
            ) {
                return Err(PlanningError::UnsupportedBackendParityRequest);
            }
            let route = select_route(bundle, &request_context);
            seed_execution_plan_for_collection_mode(
                bundle,
                request_context,
                route,
                FallbackDisposition::Forbidden,
                CollectionPlanningMode::DerivedDisplayLabel,
            )
        }
    }
}

fn select_route(
    bundle: &ValidatedQueryBundle,
    request_context: &PlanningRequestContext,
) -> PlannedExecutionRoute {
    match request_context.semantic().basis_intent().authority_family() {
        crate::basis::BasisAuthorityFamily::Store => PlannedExecutionRoute::StoreSnapshotRead,
        crate::basis::BasisAuthorityFamily::Runtime => {
            if bundle.query().traversal().is_empty()
                && bundle.query().predicates().entries().is_empty()
                && bundle.query().ordering().entries().is_empty()
            {
                PlannedExecutionRoute::RuntimeSnapshotRead
            } else {
                PlannedExecutionRoute::RuntimeExpandedSnapshotRead
            }
        }
    }
}

fn route_candidate_count(route: &PlannedExecutionRoute) -> usize {
    match route {
        PlannedExecutionRoute::StoreSnapshotRead => 1,
        PlannedExecutionRoute::RuntimeSnapshotRead
        | PlannedExecutionRoute::RuntimeExpandedSnapshotRead => 2,
    }
}

fn planned_read_surface_count(
    route: &PlannedExecutionRoute,
    projection_count: usize,
    traversal_count: usize,
    predicate_count: usize,
    ordering_count: usize,
) -> usize {
    match route {
        PlannedExecutionRoute::RuntimeSnapshotRead | PlannedExecutionRoute::StoreSnapshotRead => {
            projection_count.max(1)
        }
        PlannedExecutionRoute::RuntimeExpandedSnapshotRead => {
            (projection_count + traversal_count + predicate_count + ordering_count).max(1)
        }
    }
}
