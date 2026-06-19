use std::collections::BTreeSet;

use crate::intent_admission::{ForgeQueryLiveReadExecutionHandoff, ForgeQueryReadExecutionHandoff};
use crate::query_context::{AdmittedQueryBasisContext, QueryContextFamily};

use super::{
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryGraphObligationDispatchContext,
    ForgeQueryGraphObligationDispatchError, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphReadTouchShape, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchReadVerb, ForgeQueryRuntime,
    ForgeQueryRuntimeError, ForgeQueryWorkspaceError,
};

impl ForgeQueryRuntime {
    pub(crate) fn read_family_obligation_dispatch(
        &self,
        handoff: &ForgeQueryReadExecutionHandoff,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let read_graph = handoff.read_family().read_graph();
        let touch_descriptor =
            self.admit_read_touch_descriptor(ForgeQueryGraphTouchDescriptor::read_family_shape(
                read_graph.declarative_request().target(),
                read_family_read_verbs(read_graph, handoff.basis_context()),
                read_family_touch_shape(read_graph),
            ))?;
        let operating_world = read_family_operating_world(handoff.basis_context());
        let context = ForgeQueryGraphObligationDispatchContext::read_family(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_read_obligations(touch_descriptor, operating_world, context)
    }

    pub(crate) fn live_read_obligation_dispatch(
        &self,
        handoff: &ForgeQueryLiveReadExecutionHandoff,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let touch_descriptor =
            self.admit_read_touch_descriptor(ForgeQueryGraphTouchDescriptor::live_read_shape(
                handoff.installation().view_name(),
                [
                    ForgeQueryGraphTouchReadVerb::ObservesCollection,
                    ForgeQueryGraphTouchReadVerb::RetainsLiveSubscription,
                    ForgeQueryGraphTouchReadVerb::RequiresPolicyBasis,
                ],
                ForgeQueryGraphReadTouchShape::default(),
            ))?;
        let operating_world =
            ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority();
        let context = ForgeQueryGraphObligationDispatchContext::live_read(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .map_err(graph_obligation_dispatch_error)?;
        self.dispatch_read_obligations(touch_descriptor, operating_world, context)
    }

    fn dispatch_read_obligations(
        &self,
        touch_descriptor: ForgeQueryGraphTouchDescriptor,
        operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
        context: ForgeQueryGraphObligationDispatchContext,
    ) -> Result<Option<ForgeQueryAuthoritativeMutationObligationDispatch>, ForgeQueryRuntimeError>
    {
        let selection =
            self.select_graph_obligations_for_touch(&touch_descriptor, &operating_world);
        let dispatch =
            ForgeQueryAuthoritativeMutationObligationDispatch::from_selection(context, selection)
                .map_err(graph_obligation_dispatch_error)?;
        if let Some(denial) =
            crate::runtime::ForgeQueryGraphObligationDenial::from_dispatch(&dispatch)
        {
            return Err(ForgeQueryRuntimeError::GraphObligationDenied(denial));
        }
        Ok(Some(dispatch))
    }

    fn admit_read_touch_descriptor(
        &self,
        descriptor: Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial>,
    ) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryRuntimeError> {
        descriptor.map_err(ForgeQueryRuntimeError::GraphObligationTouchDescriptorDenied)
    }
}

fn read_family_read_verbs(
    read_graph: &crate::runtime::ForgeQueryReadGraph,
    basis_context: Option<&AdmittedQueryBasisContext>,
) -> Vec<ForgeQueryGraphTouchReadVerb> {
    let mut verbs = BTreeSet::from([
        ForgeQueryGraphTouchReadVerb::ObservesCollection,
        ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology,
    ]);
    if !read_family_touch_shape(read_graph)
        .aspect_paths()
        .is_empty()
    {
        verbs.insert(ForgeQueryGraphTouchReadVerb::ObservesAspectPath);
    }
    if read_graph.declared_traversal_clause_count() > 0 {
        verbs.insert(ForgeQueryGraphTouchReadVerb::ObservesRelationKind);
    }
    if read_graph.relationship_proof_admission().is_some() {
        verbs.insert(ForgeQueryGraphTouchReadVerb::MaterializesDiagnostic);
    }
    if let Some(context) = basis_context {
        verbs.insert(ForgeQueryGraphTouchReadVerb::RequiresPolicyBasis);
        if read_context_crosses_operating_world(context.family()) {
            verbs.insert(ForgeQueryGraphTouchReadVerb::CrossesOperatingWorld);
        }
        if read_context_allows_stale_basis(context.family()) {
            verbs.insert(ForgeQueryGraphTouchReadVerb::ReadsStaleBasisAllowed);
        }
    }
    verbs.into_iter().collect()
}

fn read_family_touch_shape(
    read_graph: &crate::runtime::ForgeQueryReadGraph,
) -> ForgeQueryGraphReadTouchShape {
    let request = read_graph.declarative_request();
    let mut aspect_paths = BTreeSet::new();
    for field in request
        .query_projection()
        .iter()
        .chain(request.result_fields().iter())
    {
        aspect_paths.insert(format!("{}.{}", field.aspect(), field.field()));
    }
    for filter in request.predicate_filters() {
        aspect_paths.insert(format!("{}.{}", filter.aspect(), filter.field()));
    }
    for ordering in request.ordering() {
        aspect_paths.insert(format!("{}.{}", ordering.aspect(), ordering.field()));
    }
    ForgeQueryGraphReadTouchShape::new(aspect_paths)
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
    error: ForgeQueryGraphObligationDispatchError,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(error.to_string()))
}

fn read_family_operating_world(
    basis_context: Option<&AdmittedQueryBasisContext>,
) -> ForgeQueryGraphObligationOperatingWorldDescriptor {
    match basis_context.map(AdmittedQueryBasisContext::family) {
        Some(QueryContextFamily::BranchHead) => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::branch()
        }
        Some(QueryContextFamily::PreviewDerivedHistorical) => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::preview()
        }
        Some(
            QueryContextFamily::CurrentBranchHead
            | QueryContextFamily::HistoricalSnapshot
            | QueryContextFamily::HistoricalCommit
            | QueryContextFamily::DiffComparison,
        )
        | None => ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority(),
    }
}
