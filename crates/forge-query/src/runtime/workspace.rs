use super::{
    DeclarativeLiveQueryRequest, ForgeQueryAspectApiFinalizationCloseout,
    ForgeQueryAuthoritativeMutationEvidenceCloseout,
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryBranchOptions,
    ForgeQueryBranchSession, ForgeQueryComputedBuilder, ForgeQueryDerivedViewHandle,
    ForgeQueryDerivedViewMaintainer, ForgeQueryEffectBuilder, ForgeQueryEffectHandle,
    ForgeQueryEffectIntentReceipt, ForgeQueryGraphIndexInventory,
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAuthorityContext,
    ForgeQueryGraphReadAccessShapeExplanationError, ForgeQueryGraphReadMaterializationRuntime,
    ForgeQueryHandleContract, ForgeQueryIntentDeclaration, ForgeQueryIntentReceipt,
    ForgeQueryLiveView, ForgeQueryLiveViewBuilder, ForgeQueryMutationSurfaceReport,
    ForgeQueryPreviewOptions, ForgeQueryPreviewSession, ForgeQueryReadFamily, ForgeQueryRuntime,
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimePublicApiContract,
    ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryRuntimePublicSupportMatrix,
    ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeStateTarget,
    ForgeQueryWorkspaceLiveViewDeclaration, QuerySchemaView,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::program::ForgeQueryDerivedView;
use crate::session_label::ForgeQuerySessionLabel;

pub struct ForgeQueryWorkspace {
    pub(super) name: String,
    pub(super) runtime: ForgeQueryRuntime,
}

impl ForgeQueryWorkspace {
    pub(crate) fn new(
        name: impl Into<String>,
        runtime: ForgeQueryRuntime,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "workspace name may not be empty",
                ),
            ));
        }
        Ok(Self { name, runtime })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        self.runtime.current_snapshot_identity()
    }

    pub fn into_runtime(self) -> ForgeQueryRuntime {
        self.runtime
    }

    pub fn public_api_contract(&self) -> ForgeQueryRuntimePublicApiContract {
        self.runtime.public_api_contract()
    }

    pub fn public_handle_contract(&self) -> ForgeQueryHandleContract {
        self.runtime.public_handle_contract()
    }

    pub fn public_support_matrix(&self) -> ForgeQueryRuntimePublicSupportMatrix {
        self.runtime.public_support_matrix()
    }

    pub fn graph_index_inventory(&self) -> ForgeQueryGraphIndexInventory {
        self.runtime.graph_index_inventory()
    }

    pub fn graph_read_materializations(&mut self) -> ForgeQueryGraphReadMaterializationRuntime<'_> {
        ForgeQueryGraphReadMaterializationRuntime::new(&mut self.runtime)
    }

    pub(crate) fn admit_graph_read_access_for_family(
        &self,
        family: &ForgeQueryReadFamily,
    ) -> Result<ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessShapeExplanationError>
    {
        self.runtime.admit_graph_read_access_for_family(family)
    }

    pub(crate) fn admit_graph_read_access_for_family_in_authority(
        &self,
        family: &ForgeQueryReadFamily,
        authority: &ForgeQueryGraphReadAccessAuthorityContext,
    ) -> Result<ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessShapeExplanationError>
    {
        self.runtime
            .admit_graph_read_access_for_family_in_authority(family, authority)
    }

    pub fn public_mutation_surface_report(&self) -> ForgeQueryMutationSurfaceReport {
        self.runtime.public_mutation_surface_report()
    }

    pub fn public_authoritative_mutation_evidence_support(
        &self,
    ) -> ForgeQueryAuthoritativeMutationEvidenceSupport {
        self.runtime
            .public_authoritative_mutation_evidence_support()
    }

    pub fn public_aspect_api_finalization_closeout(
        &self,
    ) -> ForgeQueryAspectApiFinalizationCloseout {
        self.runtime.public_aspect_api_finalization_closeout()
    }

    pub fn public_authoritative_mutation_evidence_closeout(
        &self,
    ) -> ForgeQueryAuthoritativeMutationEvidenceCloseout {
        self.runtime
            .public_authoritative_mutation_evidence_closeout()
    }

    pub fn admit_public_api_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryRuntimeError> {
        self.runtime.admit_public_api_family(family)
    }

    pub fn state<T>(
        &self,
        target: T,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError>
    where
        T: ForgeQueryRuntimeStateTarget,
    {
        target.into_state_snapshot(&self.runtime)
    }

    pub fn live_view<T>(
        &mut self,
        name: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryLiveViewBuilder) -> ForgeQueryLiveViewBuilder,
    ) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
        let name = name.into();
        let (request, schema_view) = declaration(ForgeQueryLiveViewBuilder::surface(&name))
            .build()?
            .into_parts();
        self.runtime.declare_live_view(name, request, schema_view)
    }

    pub fn live_view_request<T>(
        &mut self,
        name: impl Into<String>,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveView<T>, ForgeQueryRuntimeError> {
        let declaration =
            ForgeQueryWorkspaceLiveViewDeclaration::try_from_request(request, schema_view)?;
        let (request, schema_view) = declaration.into_parts();
        self.runtime.declare_live_view(name, request, schema_view)
    }

    pub fn computed<T>(
        &mut self,
        name: impl Into<String>,
        view: impl FnOnce(ForgeQueryComputedBuilder) -> ForgeQueryComputedBuilder,
        maintainer: impl ForgeQueryDerivedViewMaintainer + 'static,
    ) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
        let name = name.into();
        let view = view(ForgeQueryComputedBuilder::surface(&name)).build()?;
        self.runtime
            .declare_maintained_derived_view(view, maintainer)
    }

    pub fn computed_view<T>(
        &mut self,
        view: ForgeQueryDerivedView,
        maintainer: impl ForgeQueryDerivedViewMaintainer + 'static,
    ) -> Result<ForgeQueryDerivedViewHandle<T>, ForgeQueryRuntimeError> {
        self.runtime
            .declare_maintained_derived_view(view, maintainer)
    }

    pub fn computed_definition(
        &mut self,
        name: impl Into<String>,
        view: impl FnOnce(ForgeQueryComputedBuilder) -> ForgeQueryComputedBuilder,
    ) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
        let name = name.into();
        let view = view(ForgeQueryComputedBuilder::surface(&name)).build()?;
        self.runtime.declare_derived_view(view)
    }

    pub fn effect<T>(
        &mut self,
        name: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryEffectBuilder) -> ForgeQueryEffectBuilder,
    ) -> Result<ForgeQueryEffectHandle<T>, ForgeQueryRuntimeError> {
        let name = name.into();
        let declaration = declaration(ForgeQueryEffectBuilder::new(&name)).build()?;
        self.runtime.declare_effect(declaration)
    }

    pub fn effect_declaration<T>(
        &mut self,
        declaration: super::ForgeQueryEffectDeclaration,
    ) -> Result<ForgeQueryEffectHandle<T>, ForgeQueryRuntimeError> {
        self.runtime.declare_effect(declaration)
    }

    pub fn preview<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.runtime.preview(label)
    }

    pub fn preview_with_options<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
        options: ForgeQueryPreviewOptions,
    ) -> Result<ForgeQueryPreviewSession<'a>, ForgeQueryRuntimeError> {
        self.runtime.preview_with_options(label, options)
    }

    pub fn branch<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.runtime.branch(label)
    }

    pub fn branch_with_options<'a>(
        &'a mut self,
        label: ForgeQuerySessionLabel,
        options: ForgeQueryBranchOptions,
    ) -> Result<ForgeQueryBranchSession<'a>, ForgeQueryRuntimeError> {
        self.runtime.branch_with_options(label, options)
    }

    pub fn intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime.execute_intent(declaration)
    }

    pub fn next_effect_intent<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime
            .execute_next_effect_write_intent(effect, strategy_version, input_contract)
    }
}
