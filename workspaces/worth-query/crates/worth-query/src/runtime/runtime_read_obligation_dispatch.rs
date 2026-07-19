use std::collections::BTreeSet;

use crate::intent_admission::{WorthQueryLiveReadExecutionHandoff, WorthQueryReadExecutionHandoff};
use crate::query_context::{QueryContextFamily, ScopedQueryBasisContext};
use worth_foundational::facade::CanonicalFieldPath;

use super::{
    WorthQueryAuthoritativeMutationObligationDispatch, WorthQueryGraphObligationDispatchContext,
    WorthQueryGraphObligationDispatchError, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphReadTouchShape, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchDescriptorDenial, WorthQueryGraphTouchReadVerb, WorthQueryRuntime,
    WorthQueryRuntimeError, WorthQueryWorkspaceError,
};

impl WorthQueryRuntime {
    pub(crate) fn read_family_obligation_dispatch(
        &self,
        handoff: &WorthQueryReadExecutionHandoff,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let read_graph = handoff.read_family().read_graph();
        let touch_descriptor =
            self.admit_read_touch_descriptor(WorthQueryGraphTouchDescriptor::read_family_shape(
                read_graph.declarative_request().target(),
                read_family_read_verbs(read_graph, handoff.basis_context()),
                read_family_touch_shape(read_graph),
            ))?;
        let operating_world = read_family_operating_world(handoff.basis_context());
        let context = WorthQueryGraphObligationDispatchContext::read_family(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_read_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn live_read_obligation_dispatch(
        &self,
        handoff: &WorthQueryLiveReadExecutionHandoff,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let touch_descriptor =
            self.admit_read_touch_descriptor(WorthQueryGraphTouchDescriptor::live_read_shape(
                handoff.installation().view_name(),
                [
                    WorthQueryGraphTouchReadVerb::ObservesCollection,
                    WorthQueryGraphTouchReadVerb::RetainsLiveSubscription,
                    WorthQueryGraphTouchReadVerb::RequiresPolicyBasis,
                ],
                WorthQueryGraphReadTouchShape::default(),
            ))?;
        let operating_world =
            WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
        let context = WorthQueryGraphObligationDispatchContext::live_read(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_read_obligations(touch_descriptor, operating_world, context)
    }

    fn dispatch_read_obligations(
        &self,
        touch_descriptor: WorthQueryGraphTouchDescriptor,
        operating_world: WorthQueryGraphObligationOperatingWorldDescriptor,
        context: WorthQueryGraphObligationDispatchContext,
    ) -> Result<Option<WorthQueryAuthoritativeMutationObligationDispatch>, WorthQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            WorthQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
                .map_err(graph_obligation_dispatch_error)?;
        if let Some(denial) =
            crate::runtime::WorthQueryGraphObligationDenial::from_dispatch(&dispatch)
        {
            return Err(WorthQueryRuntimeError::GraphObligationDenied(denial));
        }
        Ok(Some(dispatch))
    }

    fn admit_read_touch_descriptor(
        &self,
        descriptor: Result<WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchDescriptorDenial>,
    ) -> Result<WorthQueryGraphTouchDescriptor, WorthQueryRuntimeError> {
        descriptor.map_err(WorthQueryRuntimeError::GraphObligationTouchDescriptorDenied)
    }
}

fn read_family_read_verbs(
    read_graph: &crate::runtime::WorthQueryReadGraph,
    basis_context: Option<&ScopedQueryBasisContext>,
) -> Vec<WorthQueryGraphTouchReadVerb> {
    let mut verbs = BTreeSet::from([
        WorthQueryGraphTouchReadVerb::ObservesCollection,
        WorthQueryGraphTouchReadVerb::ExposesDerivedTopology,
    ]);
    if !read_family_touch_shape(read_graph)
        .aspect_touches()
        .is_empty()
    {
        verbs.insert(WorthQueryGraphTouchReadVerb::ObservesAspect);
    }
    if read_graph.declared_traversal_clause_count() > 0 {
        verbs.insert(WorthQueryGraphTouchReadVerb::ObservesRelationKind);
    }
    if read_graph.relationship_proof_admission().is_some() {
        verbs.insert(WorthQueryGraphTouchReadVerb::MaterializesDiagnostic);
    }
    if let Some(context) = basis_context {
        verbs.insert(WorthQueryGraphTouchReadVerb::RequiresPolicyBasis);
        if read_context_crosses_operating_world(context.family()) {
            verbs.insert(WorthQueryGraphTouchReadVerb::CrossesOperatingWorld);
        }
        if read_context_allows_stale_basis(context.family()) {
            verbs.insert(WorthQueryGraphTouchReadVerb::ReadsStaleBasisAllowed);
        }
    }
    verbs.into_iter().collect()
}

fn read_family_touch_shape(
    read_graph: &crate::runtime::WorthQueryReadGraph,
) -> WorthQueryGraphReadTouchShape {
    let request = read_graph.declarative_request();
    let mut aspect_touches = BTreeSet::new();
    for field in request
        .query_projection()
        .iter()
        .chain(request.result_fields().iter())
    {
        aspect_touches.insert(read_field_touch(field.source_field_key()));
    }
    for filter in request.predicate_filters() {
        aspect_touches.insert(read_field_touch(filter.source_field_key()));
    }
    for ordering in request.ordering() {
        aspect_touches.insert(read_field_touch(ordering.source_field_key()));
    }
    WorthQueryGraphReadTouchShape::new(aspect_touches)
}

fn read_field_touch(
    field: &crate::authoring::AspectFieldKey,
) -> crate::runtime::WorthQueryAspectTouch {
    crate::runtime::WorthQueryAspectTouch::from_native_parts(
        field.native_aspect_key(),
        Some(CanonicalFieldPath::single(field.native_field_key())),
    )
}

fn read_context_crosses_operating_world(family: &QueryContextFamily) -> bool {
    matches!(
        family,
        QueryContextFamily::BranchHead
            | QueryContextFamily::PreviewDerivedHistorical
            | QueryContextFamily::HistoricalSnapshot
            | QueryContextFamily::HistoricalCommit
            | QueryContextFamily::DiffComparison
    )
}

fn read_context_allows_stale_basis(family: &QueryContextFamily) -> bool {
    matches!(
        family,
        QueryContextFamily::PreviewDerivedHistorical
            | QueryContextFamily::HistoricalSnapshot
            | QueryContextFamily::HistoricalCommit
            | QueryContextFamily::DiffComparison
    )
}

fn graph_obligation_dispatch_error(
    error: WorthQueryGraphObligationDispatchError,
) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::Workspace(WorthQueryWorkspaceError::new(error.to_string()))
}

fn read_family_operating_world(
    basis_context: Option<&ScopedQueryBasisContext>,
) -> WorthQueryGraphObligationOperatingWorldDescriptor {
    match basis_context.map(ScopedQueryBasisContext::family) {
        Some(QueryContextFamily::BranchHead) => {
            WorthQueryGraphObligationOperatingWorldDescriptor::branch()
        }
        Some(QueryContextFamily::PreviewDerivedHistorical) => {
            WorthQueryGraphObligationOperatingWorldDescriptor::preview()
        }
        Some(
            QueryContextFamily::CurrentBranchHead
            | QueryContextFamily::HistoricalSnapshot
            | QueryContextFamily::HistoricalCommit
            | QueryContextFamily::DiffComparison,
        )
        | None => WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    }
}
