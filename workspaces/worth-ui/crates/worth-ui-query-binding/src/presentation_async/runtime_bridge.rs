use worth_query::facade::runtime;

use super::{
    WorthUiPresentationAsyncDeclaration, WorthUiPresentationAsyncObservation,
    WorthUiPresentationAsyncPosture, WorthUiPresentationRequestBasis,
};

type PresentationLiveView = runtime::WorthQueryLiveView<runtime::WorthQueryUnrefinedLiveShape>;

#[path = "runtime_bridge/schema.rs"]
mod schema;
use schema::{presentation_live_request, presentation_schema_view, presentation_view_name};
#[path = "runtime_bridge/completion_progress.rs"]
mod completion_progress;
pub(super) use completion_progress::WorthUiPresentationCompletionProgress;

pub struct WorthUiPresentationRuntimeAdmission {
    declaration: WorthUiPresentationAsyncDeclaration,
    query_declaration: runtime::WorthQueryInstalledOwnedAsyncDeclaration,
    request: worth_runtime_bridge::facade::AdmittedBridgeAsyncRequestIdentity,
    effects_indeterminate_issuer:
        worth_runtime_bridge::facade::BridgeOwnedAsyncEffectsIndeterminateIssuer,
    view: PresentationLiveView,
    semantic_instances: Box<[runtime::WorthQueryInstalledOwnedConditionalInstance]>,
}

pub struct WorthUiPresentationCompletionAdvance {
    observation: WorthUiPresentationAsyncObservation,
}

#[derive(Debug)]
pub enum WorthUiPresentationCompletionDenial {
    QueryOwned(runtime::WorthQueryOwnedAsyncRuntimeDenial),
    QueryTransition(runtime::WorthQueryAsyncSourceBindingError),
    Observation(WorthUiPresentationRuntimeAdmissionDenial),
}

#[derive(Debug)]
pub(crate) enum WorthUiPresentationRuntimeAdmissionDenial {
    QueryOwned(runtime::WorthQueryOwnedAsyncRuntimeDenial),
    QueryLive(runtime::WorthQueryRuntimeError),
    MissingAsyncResultState,
    MissingSemanticRuntime,
    QueryDeclarationMismatch,
    SemanticInstallation(runtime::WorthQueryOwnedConditionalInstanceDenial),
    CleanupRequired {
        cause: Box<WorthUiPresentationRuntimeAdmissionDenial>,
        recovery: Box<WorthUiPresentationRuntimeCleanup>,
        last_denial: WorthUiPresentationRuntimeCleanupDenial,
    },
}

#[derive(Debug)]
pub(crate) enum WorthUiPresentationRuntimeCleanupDenial {
    Query(runtime::WorthQueryOwnedAsyncRuntimeDenial),
    Semantic(runtime::WorthQueryOwnedConditionalInstanceDenial),
}

#[derive(Debug)]
pub(crate) struct WorthUiPresentationRuntimeCleanup {
    request: Option<worth_runtime_bridge::facade::AdmittedBridgeAsyncRequestIdentity>,
    semantic_instances: Box<[runtime::WorthQueryInstalledOwnedConditionalInstance]>,
    request_retired: bool,
    next_semantic_retirement: usize,
}

impl WorthUiPresentationRuntimeAdmission {
    pub(super) fn admit_in_workspace(
        workspace: &mut runtime::WorthQueryWorkspace,
        declaration: WorthUiPresentationAsyncDeclaration,
        truth_basis: worth_runtime_bridge::facade::BridgeAsyncRequestTruthViewBasis,
        semantic_installations: Vec<(
            [worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts; 8],
            u64,
        )>,
    ) -> Result<Self, WorthUiPresentationRuntimeAdmissionDenial> {
        let query_declaration =
            runtime::WorthQueryOwnedAsyncRequestDeclaration::from_async_resource_identity(
                declaration.request_identity().clone(),
                0x5755_4950_5245_5345,
                16 * 1024 * 1024,
                3,
            );
        let mut semantic_instances = Vec::with_capacity(semantic_installations.len());
        for (semantic_records, semantic_version) in semantic_installations {
            match super::semantic_invalidation::install_presentation_semantic_instance(
                workspace,
                semantic_records,
                semantic_version,
            ) {
                Ok(instance) => {
                    semantic_instances.push(instance);
                }
                Err(denial) if semantic_instances.is_empty() => {
                    return Err(
                        WorthUiPresentationRuntimeAdmissionDenial::SemanticInstallation(denial),
                    );
                }
                Err(denial) => {
                    return Err(cleanup_after_admission_failure(
                        workspace,
                        semantic_instances.into_boxed_slice(),
                        None,
                        WorthUiPresentationRuntimeAdmissionDenial::SemanticInstallation(denial),
                    ));
                }
            }
        }
        let query_declaration =
            match workspace.install_owned_bridge_async_declaration(query_declaration) {
                Ok(declaration) => declaration,
                Err(denial) => {
                    return Err(cleanup_after_admission_failure(
                        workspace,
                        semantic_instances.into_boxed_slice(),
                        None,
                        WorthUiPresentationRuntimeAdmissionDenial::QueryOwned(denial),
                    ));
                }
            };
        if query_declaration.identity() != declaration.request_identity()
            || query_declaration.clause() != declaration.clause()
        {
            return Err(cleanup_after_admission_failure(
                workspace,
                semantic_instances.into_boxed_slice(),
                None,
                WorthUiPresentationRuntimeAdmissionDenial::QueryDeclarationMismatch,
            ));
        }
        let request_admission =
            match workspace.admit_owned_bridge_async_request(&query_declaration, truth_basis) {
                Ok(request) => request,
                Err(denial) => {
                    return Err(cleanup_after_admission_failure(
                        workspace,
                        semantic_instances.into_boxed_slice(),
                        None,
                        WorthUiPresentationRuntimeAdmissionDenial::QueryOwned(denial),
                    ));
                }
            };
        let (request, effects_indeterminate_issuer) = request_admission.into_parts();
        let view = match workspace.declare_bridge_async_live_view_with_typed_identity(
            presentation_view_name(&declaration),
            presentation_live_request(),
            presentation_schema_view(),
            query_declaration.identity().request_identity(),
            &request,
        ) {
            Ok(view) => view,
            Err(denial) => {
                return Err(cleanup_after_admission_failure(
                    workspace,
                    semantic_instances.into_boxed_slice(),
                    Some(request),
                    WorthUiPresentationRuntimeAdmissionDenial::QueryLive(denial),
                ));
            }
        };
        Ok(Self {
            declaration,
            query_declaration,
            request,
            effects_indeterminate_issuer,
            view,
            semantic_instances: semantic_instances.into_boxed_slice(),
        })
    }

    pub fn basis(&self) -> &WorthUiPresentationRequestBasis {
        self.declaration.basis()
    }

    pub(super) fn semantic_instances(
        &self,
    ) -> &[runtime::WorthQueryInstalledOwnedConditionalInstance] {
        &self.semantic_instances
    }

    pub(crate) fn admit_transitions(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
        ordering: &worth_runtime_bridge::facade::BridgeMixedCauseOrdering,
    ) -> Result<
        runtime::WorthQueryAsyncResultTransitionBatch,
        runtime::WorthQueryAsyncSourceBindingError,
    > {
        workspace.admit_bridge_async_result_transitions(&self.view, ordering)
    }

    pub(super) fn admit_supersession(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
        displacing: &Self,
    ) -> Result<(), WorthUiPresentationCompletionDenial> {
        let _supersession = workspace
            .supersede_owned_bridge_async_live_view(
                &self.view,
                &self.query_declaration,
                &displacing.query_declaration,
            )
            .map_err(WorthUiPresentationCompletionDenial::QueryTransition)?;
        Ok(())
    }

    pub(super) fn admit_denial_before_effects(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<(), WorthUiPresentationCompletionDenial> {
        let _denial = workspace
            .deny_owned_bridge_async_live_view(&self.view, &self.query_declaration)
            .map_err(WorthUiPresentationCompletionDenial::QueryTransition)?;
        Ok(())
    }

    pub(super) fn admit_cancellation_before_effects(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<(), WorthUiPresentationCompletionDenial> {
        let _cancellation = workspace
            .cancel_owned_bridge_async_live_view(&self.view, &self.query_declaration)
            .map_err(WorthUiPresentationCompletionDenial::QueryTransition)?;
        Ok(())
    }

    pub(super) fn close_query_live_view(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<runtime::WorthQueryLiveViewCloseReceipt, WorthUiPresentationRuntimeAdmissionDenial>
    {
        workspace
            .retire_owned_bridge_async_request(&self.request)
            .map_err(WorthUiPresentationRuntimeAdmissionDenial::QueryOwned)?;
        workspace
            .close_owned_bridge_async_live_view(&self.view)
            .map_err(WorthUiPresentationRuntimeAdmissionDenial::QueryLive)
    }

    pub fn observation(
        &self,
        workspace: &runtime::WorthQueryWorkspace,
    ) -> Result<WorthUiPresentationAsyncObservation, WorthUiPresentationRuntimeAdmissionDenial>
    {
        let posture = workspace
            .state_live(&self.view)
            .map_err(WorthUiPresentationRuntimeAdmissionDenial::QueryLive)?
            .async_result_state()
            .ok_or(WorthUiPresentationRuntimeAdmissionDenial::MissingAsyncResultState)?
            .kind();
        let graph = workspace
            .owned_async_runtime_topology()
            .ok_or(WorthUiPresentationRuntimeAdmissionDenial::MissingSemanticRuntime)?;
        Ok(WorthUiPresentationAsyncObservation::new(
            WorthUiPresentationAsyncPosture::from_query(posture),
            graph.signal_graph_instance(),
        ))
    }

    pub(super) fn retire_semantic_at(
        &self,
        workspace: &mut runtime::WorthQueryWorkspace,
        index: usize,
    ) -> Result<(), runtime::WorthQueryOwnedConditionalInstanceDenial> {
        super::semantic_invalidation::retire_presentation_semantic_instance(
            workspace,
            &self.semantic_instances[index],
        )
    }
}

fn cleanup_after_admission_failure(
    workspace: &mut runtime::WorthQueryWorkspace,
    semantic_instances: Box<[runtime::WorthQueryInstalledOwnedConditionalInstance]>,
    request: Option<worth_runtime_bridge::facade::AdmittedBridgeAsyncRequestIdentity>,
    denial: WorthUiPresentationRuntimeAdmissionDenial,
) -> WorthUiPresentationRuntimeAdmissionDenial {
    let mut recovery = WorthUiPresentationRuntimeCleanup {
        request,
        semantic_instances,
        request_retired: false,
        next_semantic_retirement: 0,
    };
    match recovery.resume(workspace) {
        Ok(()) => denial,
        Err(last_denial) => WorthUiPresentationRuntimeAdmissionDenial::CleanupRequired {
            cause: Box::new(denial),
            recovery: Box::new(recovery),
            last_denial,
        },
    }
}

impl WorthUiPresentationRuntimeCleanup {
    pub(super) fn resume(
        &mut self,
        workspace: &mut runtime::WorthQueryWorkspace,
    ) -> Result<(), WorthUiPresentationRuntimeCleanupDenial> {
        if !self.request_retired {
            if let Some(request) = self.request.as_ref() {
                workspace
                    .retire_owned_bridge_async_request(request)
                    .map_err(WorthUiPresentationRuntimeCleanupDenial::Query)?;
            }
            self.request_retired = true;
        }
        while self.next_semantic_retirement < self.semantic_instances.len() {
            super::semantic_invalidation::retire_presentation_semantic_instance(
                workspace,
                &self.semantic_instances[self.next_semantic_retirement],
            )
            .map_err(WorthUiPresentationRuntimeCleanupDenial::Semantic)?;
            self.next_semantic_retirement += 1;
        }
        Ok(())
    }
}

impl WorthUiPresentationRuntimeAdmissionDenial {
    pub(super) fn into_cleanup_required(
        self,
    ) -> Result<
        (
            WorthUiPresentationRuntimeCleanup,
            WorthUiPresentationRuntimeAdmissionDenial,
            WorthUiPresentationRuntimeCleanupDenial,
        ),
        Self,
    > {
        match self {
            Self::CleanupRequired {
                cause,
                recovery,
                last_denial,
            } => Ok((*recovery, *cause, last_denial)),
            denial => Err(denial),
        }
    }
}

impl std::fmt::Display for WorthUiPresentationCompletionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryOwned(denial) => write!(formatter, "Query-owned completion: {denial:?}"),
            Self::QueryTransition(denial) => {
                write!(formatter, "Query completion transition: {denial:?}")
            }
            Self::Observation(denial) => write!(formatter, "completion observation: {denial}"),
        }
    }
}

impl std::fmt::Display for WorthUiPresentationRuntimeAdmissionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryOwned(denial) => write!(formatter, "Query-owned admission: {denial:?}"),
            Self::QueryLive(denial) => write!(formatter, "Query live-view admission: {denial:?}"),
            Self::MissingAsyncResultState => formatter.write_str("missing async result state"),
            Self::MissingSemanticRuntime => formatter.write_str("missing semantic runtime"),
            Self::QueryDeclarationMismatch => formatter.write_str("Query declaration mismatch"),
            Self::SemanticInstallation(denial) => {
                write!(formatter, "semantic installation: {denial:?}")
            }
            Self::CleanupRequired {
                cause,
                recovery,
                last_denial,
            } => write!(
                formatter,
                "admission cleanup required after {cause}; next semantic retirement {}, request retained: {}, last denial: {last_denial}",
                recovery.next_semantic_retirement,
                recovery.request.is_some(),
            ),
        }
    }
}

impl std::fmt::Display for WorthUiPresentationRuntimeCleanupDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Query(denial) => write!(formatter, "Query cleanup: {denial:?}"),
            Self::Semantic(denial) => write!(formatter, "semantic cleanup: {denial:?}"),
        }
    }
}
