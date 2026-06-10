use super::{
    DeclarativeLiveQueryRequest, ForgeQueryAspectApiFinalizationCloseout,
    ForgeQueryAspectMutationBuilder, ForgeQueryAuthoritativeMutationEvidenceCloseout,
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryBatchWriteReceipt,
    ForgeQueryBranchOptions, ForgeQueryBranchSession, ForgeQueryComputedBuilder,
    ForgeQueryDeleteMutationBuilder, ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryEffectBuilder, ForgeQueryEffectHandle, ForgeQueryEffectIntentReceipt,
    ForgeQueryExistingEntityTarget, ForgeQueryExistingRelationTarget,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryHandleContract, ForgeQueryIntentDeclaration,
    ForgeQueryIntentReceipt, ForgeQueryLiveView, ForgeQueryLiveViewBuilder,
    ForgeQueryMutationBatchBuilder, ForgeQueryMutationMetadata, ForgeQueryMutationSurfaceReport,
    ForgeQueryPreviewOptions, ForgeQueryPreviewSession, ForgeQueryRuntime, ForgeQueryRuntimeError,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimePublicApiContract,
    ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryRuntimePublicSupportMatrix,
    ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeStateTarget,
    ForgeQueryWorkspaceLiveViewDeclaration, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
    QuerySchemaView,
};
use crate::program::ForgeQueryDerivedView;
use crate::session_label::ForgeQuerySessionLabel;

pub struct ForgeQueryWorkspace {
    pub(super) name: String,
    pub(super) runtime: ForgeQueryRuntime,
}

impl ForgeQueryWorkspace {
    pub(super) fn new(
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

    pub fn snapshot_token(&self) -> String {
        self.runtime.snapshot_token()
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

    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.write_intent(command).execute()
    }

    pub fn write_intent(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> crate::intent_admission::ForgeQueryRuntimeWriteIntentAuthoring<'_> {
        self.runtime.write_intent(command)
    }

    pub fn write_batch_intent(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> crate::intent_admission::ForgeQueryRuntimeWriteBatchIntentAuthoring<'_> {
        self.runtime.write_batch_intent(commands)
    }

    pub fn insert(
        &mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_insert(collection)?;
        self.write(command)
    }

    pub fn update(
        &mut self,
        entity_identity: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update(entity_identity)?;
        self.write(command)
    }

    pub fn bind_existing_entity(
        &self,
        target: ForgeQueryExistingEntityTarget,
    ) -> Result<ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError> {
        Ok(ForgeQueryExistingTruthTargetBinding::from_entity_target(
            target,
        )?)
    }

    pub fn bind_existing_relation(
        &self,
        target: ForgeQueryExistingRelationTarget,
    ) -> Result<ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError> {
        Ok(ForgeQueryExistingTruthTargetBinding::from_relation_target(
            target,
        )?)
    }

    pub fn probe_existing<I, S>(
        &self,
        binding: ForgeQueryExistingTruthTargetBinding,
        aspect_paths: I,
    ) -> Result<super::ForgeQueryExistingTruthProbe, ForgeQueryRuntimeError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Ok(self
            .runtime
            .probe_existing_intent(super::ForgeQueryExistingTruthProbeRequest::new(
                binding,
                aspect_paths,
            )?)
            .execute()?
            .probe()
            .clone())
    }

    pub fn probe_existing_intent(
        &self,
        request: super::ForgeQueryExistingTruthProbeRequest,
    ) -> crate::intent_admission::ForgeQueryRuntimeExistingTruthProbeIntentAuthoring<'_> {
        self.runtime.probe_existing_intent(request)
    }

    pub fn update_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        self.write(command)
    }

    pub fn assert_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_assert_existing(binding)?;
        self.write(command)
    }

    pub fn verify_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_verify_existing(binding)?;
        self.write(command)
    }

    pub fn update_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        update: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let asserted_aspects = verify(ForgeQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth update")?;
        let command = update(ForgeQueryAspectMutationBuilder::new())
            .build_update_existing_verified(binding, asserted_aspects)?;
        self.write(command)
    }

    pub fn delete(
        &mut self,
        entity_identity: impl Into<String>,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.write(ForgeQueryWriteCommand::Delete {
            entity_identity: entity_identity.into(),
        })
    }

    pub fn delete_with(
        &mut self,
        entity_identity: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete(entity_identity)?;
        self.write(command)
    }

    pub fn delete_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.write(ForgeQueryWriteCommand::DeleteExistingAspects {
            binding,
            touched_aspect_paths: Vec::new(),
            metadata: ForgeQueryMutationMetadata::default(),
            naming_intent: None,
        })
    }

    pub fn delete_existing_with(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete_existing(binding)?;
        self.write(command)
    }

    pub fn delete_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        delete: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let asserted_aspects = verify(ForgeQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth delete")?;
        let command = delete(ForgeQueryDeleteMutationBuilder::new())
            .build_delete_existing_verified(binding, asserted_aspects)?;
        self.write(command)
    }

    pub fn batch(
        &mut self,
        declaration: impl FnOnce(ForgeQueryMutationBatchBuilder) -> ForgeQueryMutationBatchBuilder,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let commands = declaration(ForgeQueryMutationBatchBuilder::new()).finish()?;
        self.write_batch_intent(commands).execute()
    }
}
