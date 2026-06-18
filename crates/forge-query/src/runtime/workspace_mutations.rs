use super::workspace::ForgeQueryWorkspace;
#[cfg(test)]
use super::ForgeQueryMutationMetadata;
use super::{
    ForgeQueryAspectMutationBuilder, ForgeQueryDeleteMutationBuilder,
    ForgeQueryExistingEntityTarget, ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};
use super::{
    ForgeQueryBatchWriteReceipt, ForgeQueryExistingRelationTarget, ForgeQueryMutationBatchBuilder,
};
use crate::memory_workspace::ForgeQueryEntityIdentity;

impl ForgeQueryWorkspace {
    pub(crate) fn write(
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
        entity_identity: ForgeQueryEntityIdentity,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update(entity_identity)?;
        self.write(command)
    }

    #[allow(dead_code)]
    pub(crate) fn bind_existing_entity(
        &self,
        target: ForgeQueryExistingEntityTarget,
    ) -> Result<ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError> {
        Ok(ForgeQueryExistingTruthTargetBinding::from_entity_target(
            target,
        )?)
    }

    #[allow(dead_code)]
    pub(crate) fn bind_existing_relation(
        &self,
        target: ForgeQueryExistingRelationTarget,
    ) -> Result<ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError> {
        Ok(ForgeQueryExistingTruthTargetBinding::from_relation_target(
            target,
        )?)
    }

    pub(crate) fn probe_existing<I, S>(
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

    #[cfg(test)]
    pub(crate) fn update_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        self.write(command)
    }

    #[cfg(test)]
    pub(crate) fn assert_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_assert_existing(binding)?;
        self.write(command)
    }

    #[cfg(test)]
    pub(crate) fn verify_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_verify_existing(binding)?;
        self.write(command)
    }

    #[cfg(test)]
    pub(crate) fn update_existing_verified(
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
        entity_identity: ForgeQueryEntityIdentity,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.write(ForgeQueryWriteCommand::Delete { entity_identity })
    }

    pub fn delete_with(
        &mut self,
        entity_identity: ForgeQueryEntityIdentity,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete(entity_identity)?;
        self.write(command)
    }

    #[cfg(test)]
    pub(crate) fn delete_existing(
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

    #[cfg(test)]
    pub(crate) fn delete_existing_with(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete_existing(binding)?;
        self.write(command)
    }

    #[cfg(test)]
    pub(crate) fn delete_existing_verified(
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

    #[allow(dead_code)]
    pub(crate) fn batch(
        &mut self,
        declaration: impl FnOnce(ForgeQueryMutationBatchBuilder) -> ForgeQueryMutationBatchBuilder,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let commands = declaration(ForgeQueryMutationBatchBuilder::new()).finish()?;
        self.write_batch_intent(commands).execute()
    }
}
