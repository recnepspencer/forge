use super::{
    DeclarativeLiveQueryRequest, QuerySchemaView, WorthQueryAspectApiFinalizationCloseout,
    WorthQueryAuthoritativeMutationEvidenceCloseout,
    WorthQueryAuthoritativeMutationEvidenceSupport, WorthQueryBranchOptions,
    WorthQueryBranchSession, WorthQueryComputedBuilder, WorthQueryDerivedViewHandle,
    WorthQueryDerivedViewMaintainer, WorthQueryEffectBuilder, WorthQueryEffectHandle,
    WorthQueryEffectIntentReceipt, WorthQueryGraphIndexInventory,
    WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessAuthorityContext,
    WorthQueryGraphReadAccessShapeExplanationError, WorthQueryGraphReadMaterializationRuntime,
    WorthQueryHandleContract, WorthQueryIntentDeclaration, WorthQueryIntentReceipt,
    WorthQueryLiveView, WorthQueryLiveViewBuilder, WorthQueryManagedLiveWorkspaceCapability,
    WorthQueryMutationSurfaceReport, WorthQueryPreviewOptions, WorthQueryPreviewSession,
    WorthQueryReadFamily, WorthQueryRuntime, WorthQueryRuntimeError, WorthQueryRuntimeFacadeFamily,
    WorthQueryRuntimePublicApiContract, WorthQueryRuntimePublicApiFamilyContract,
    WorthQueryRuntimePublicSupportMatrix, WorthQueryRuntimeStateSnapshot,
    WorthQueryRuntimeStateTarget, WorthQueryWorkspaceLiveViewDeclaration,
};
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::program::WorthQueryDerivedView;
use crate::session_label::WorthQuerySessionLabel;
pub struct WorthQueryWorkspace {
    pub(super) name: String,
    pub(super) runtime: WorthQueryRuntime,
}

impl WorthQueryWorkspace {
    pub(crate) fn new(
        name: impl Into<String>,
        runtime: WorthQueryRuntime,
    ) -> Result<Self, WorthQueryRuntimeError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(WorthQueryRuntimeError::Workspace(
                crate::memory_workspace::WorthQueryWorkspaceError::new(
                    "workspace name may not be empty",
                ),
            ));
        }
        Ok(Self { name, runtime })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn managed_live_capability(
        &self,
    ) -> std::sync::Arc<WorthQueryManagedLiveWorkspaceCapability> {
        self.runtime.managed_live_capability()
    }

    pub(crate) fn admit_managed_live_capability(
        &self,
        capability: &std::sync::Arc<WorthQueryManagedLiveWorkspaceCapability>,
        resource_name: &str,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.runtime
            .admit_managed_live_capability(capability, resource_name)
    }

    pub fn snapshot_identity(&self) -> WorthQuerySnapshotIdentity {
        self.runtime.current_snapshot_identity()
    }

    pub(crate) fn capture_branch_comparison_basis(
        &self,
        label: WorthQuerySessionLabel,
    ) -> Result<super::WorthQueryRuntimeBranchComparisonBasis, WorthQueryRuntimeError> {
        self.runtime.capture_branch_comparison_basis(label)
    }

    pub fn into_runtime(self) -> WorthQueryRuntime {
        self.runtime
    }

    pub fn public_api_contract(&self) -> WorthQueryRuntimePublicApiContract {
        self.runtime.public_api_contract()
    }

    pub fn public_handle_contract(&self) -> WorthQueryHandleContract {
        self.runtime.public_handle_contract()
    }

    pub fn public_support_matrix(&self) -> WorthQueryRuntimePublicSupportMatrix {
        self.runtime.public_support_matrix()
    }

    pub fn graph_index_inventory(&self) -> WorthQueryGraphIndexInventory {
        self.runtime.graph_index_inventory()
    }

    pub fn graph_read_materializations(&mut self) -> WorthQueryGraphReadMaterializationRuntime<'_> {
        WorthQueryGraphReadMaterializationRuntime::new(&mut self.runtime)
    }

    pub(crate) fn admit_graph_read_access_for_family(
        &self,
        family: &WorthQueryReadFamily,
    ) -> Result<WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessShapeExplanationError>
    {
        self.runtime.admit_graph_read_access_for_family(family)
    }

    pub(crate) fn admit_graph_read_access_for_family_in_authority(
        &self,
        family: &WorthQueryReadFamily,
        authority: &WorthQueryGraphReadAccessAuthorityContext,
    ) -> Result<WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessShapeExplanationError>
    {
        self.runtime
            .admit_graph_read_access_for_family_in_authority(family, authority)
    }

    pub fn public_mutation_surface_report(&self) -> WorthQueryMutationSurfaceReport {
        self.runtime.public_mutation_surface_report()
    }

    pub fn public_authoritative_mutation_evidence_support(
        &self,
    ) -> WorthQueryAuthoritativeMutationEvidenceSupport {
        self.runtime
            .public_authoritative_mutation_evidence_support()
    }

    pub fn public_aspect_api_finalization_closeout(
        &self,
    ) -> WorthQueryAspectApiFinalizationCloseout {
        self.runtime.public_aspect_api_finalization_closeout()
    }

    pub fn public_authoritative_mutation_evidence_closeout(
        &self,
    ) -> WorthQueryAuthoritativeMutationEvidenceCloseout {
        self.runtime
            .public_authoritative_mutation_evidence_closeout()
    }

    pub fn admit_public_api_family(
        &self,
        family: WorthQueryRuntimeFacadeFamily,
    ) -> Result<WorthQueryRuntimePublicApiFamilyContract, WorthQueryRuntimeError> {
        self.runtime.admit_public_api_family(family)
    }

    pub fn state<T>(
        &self,
        target: T,
    ) -> Result<WorthQueryRuntimeStateSnapshot, WorthQueryRuntimeError>
    where
        T: WorthQueryRuntimeStateTarget,
    {
        target.into_state_snapshot(&self.runtime)
    }

    pub fn live_view<T>(
        &mut self,
        name: impl Into<String>,
        declaration: impl FnOnce(WorthQueryLiveViewBuilder) -> WorthQueryLiveViewBuilder,
    ) -> Result<WorthQueryLiveView<T>, WorthQueryRuntimeError> {
        let name = name.into();
        let (request, schema_view) = declaration(WorthQueryLiveViewBuilder::surface(&name))
            .build()?
            .into_parts();
        self.runtime.declare_live_view(name, request, schema_view)
    }

    pub fn live_view_request<T>(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<WorthQueryLiveView<T>, WorthQueryRuntimeError> {
        let declaration =
            WorthQueryWorkspaceLiveViewDeclaration::try_from_request(request, schema_view)?;
        let (request, schema_view) = declaration.into_parts();
        self.runtime.declare_live_view(name, request, schema_view)
    }

    pub fn computed<T>(
        &mut self,
        name: impl Into<String>,
        view: impl FnOnce(WorthQueryComputedBuilder) -> WorthQueryComputedBuilder,
        maintainer: impl WorthQueryDerivedViewMaintainer + 'static,
    ) -> Result<WorthQueryDerivedViewHandle<T>, WorthQueryRuntimeError> {
        let name = name.into();
        let view = view(WorthQueryComputedBuilder::surface(&name)).build()?;
        self.runtime
            .declare_maintained_derived_view(view, maintainer)
    }

    pub fn computed_view<T>(
        &mut self,
        view: WorthQueryDerivedView,
        maintainer: impl WorthQueryDerivedViewMaintainer + 'static,
    ) -> Result<WorthQueryDerivedViewHandle<T>, WorthQueryRuntimeError> {
        self.runtime
            .declare_maintained_derived_view(view, maintainer)
    }

    pub fn computed_definition(
        &mut self,
        name: impl Into<String>,
        view: impl FnOnce(WorthQueryComputedBuilder) -> WorthQueryComputedBuilder,
    ) -> Result<WorthQueryDerivedView, WorthQueryRuntimeError> {
        let name = name.into();
        let view = view(WorthQueryComputedBuilder::surface(&name)).build()?;
        self.runtime.declare_derived_view(view)
    }

    pub fn effect<T>(
        &mut self,
        name: impl Into<String>,
        declaration: impl FnOnce(WorthQueryEffectBuilder) -> WorthQueryEffectBuilder,
    ) -> Result<WorthQueryEffectHandle<T>, WorthQueryRuntimeError> {
        let name = name.into();
        let declaration = declaration(WorthQueryEffectBuilder::new(&name)).build()?;
        self.runtime.declare_effect(declaration)
    }

    pub fn effect_declaration<T>(
        &mut self,
        declaration: super::WorthQueryEffectDeclaration,
    ) -> Result<WorthQueryEffectHandle<T>, WorthQueryRuntimeError> {
        self.runtime.declare_effect(declaration)
    }

    pub fn preview<'a>(
        &'a mut self,
        label: WorthQuerySessionLabel,
    ) -> Result<WorthQueryPreviewSession<'a>, WorthQueryRuntimeError> {
        self.runtime.preview(label)
    }

    pub fn preview_with_options<'a>(
        &'a mut self,
        label: WorthQuerySessionLabel,
        options: WorthQueryPreviewOptions,
    ) -> Result<WorthQueryPreviewSession<'a>, WorthQueryRuntimeError> {
        self.runtime.preview_with_options(label, options)
    }

    pub fn branch<'a>(
        &'a mut self,
        label: WorthQuerySessionLabel,
    ) -> Result<WorthQueryBranchSession<'a>, WorthQueryRuntimeError> {
        self.runtime.branch(label)
    }

    pub fn branch_with_options<'a>(
        &'a mut self,
        label: WorthQuerySessionLabel,
        options: WorthQueryBranchOptions,
    ) -> Result<WorthQueryBranchSession<'a>, WorthQueryRuntimeError> {
        self.runtime.branch_with_options(label, options)
    }

    pub fn intent(
        &mut self,
        declaration: WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError> {
        self.runtime.execute_intent(declaration)
    }

    pub fn next_effect_intent<T>(
        &mut self,
        effect: &WorthQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError> {
        self.runtime
            .execute_next_effect_write_intent(effect, strategy_version, input_contract)
    }
}
