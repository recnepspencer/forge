use super::{
    ForgeQueryEffectHandle, ForgeQueryEffectIntentReceipt, ForgeQueryIntentDeclaration,
    ForgeQueryIntentReceipt, ForgeQueryMutationBatchBuilder, ForgeQueryRuntimeError,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace, ForgeQueryWriteCommand,
    ForgeQueryWriteReceipt,
};

pub struct ForgeQueryWorkspaceSubmissionLane<'a> {
    runtime: &'a mut super::ForgeQueryRuntime,
}

impl<'a> ForgeQueryWorkspaceSubmissionLane<'a> {
    pub(crate) fn new(runtime: &'a mut super::ForgeQueryRuntime) -> Self {
        Self { runtime }
    }

    pub fn submit(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.runtime.write(command)
    }

    pub fn submit_batch(
        &mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> Result<super::ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.runtime.write_batch_intent(commands).execute()
    }

    pub fn submit_batch_builder(
        &mut self,
        builder: ForgeQueryMutationBatchBuilder,
    ) -> Result<super::ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        self.submit_batch(builder.finish()?)
    }

    pub fn submit_intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime.execute_intent(declaration)
    }

    pub fn submit_effect_intent<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime
            .execute_next_effect_write_intent(effect, strategy_version, input_contract)
    }

    pub fn write_intent(
        &'a mut self,
        command: ForgeQueryWriteCommand,
    ) -> crate::intent_admission::ForgeQueryRuntimeWriteIntentAuthoring<'a> {
        self.runtime.write_intent(command)
    }

    pub fn write_batch_intent(
        &'a mut self,
        commands: Vec<ForgeQueryWriteCommand>,
    ) -> crate::intent_admission::ForgeQueryRuntimeWriteBatchIntentAuthoring<'a> {
        self.runtime.write_batch_intent(commands)
    }
}

impl ForgeQueryWorkspace {
    pub fn submissions(
        &mut self,
    ) -> Result<ForgeQueryWorkspaceSubmissionLane<'_>, ForgeQueryRuntimeError> {
        self.runtime
            .admit_facade_family(ForgeQueryRuntimeFacadeFamily::Submission)?;
        Ok(ForgeQueryWorkspaceSubmissionLane::new(&mut self.runtime))
    }
}
