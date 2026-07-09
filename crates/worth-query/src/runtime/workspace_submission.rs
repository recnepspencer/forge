use super::{
    WorthQueryEffectHandle, WorthQueryEffectIntentReceipt, WorthQueryIntentDeclaration,
    WorthQueryIntentReceipt, WorthQueryMutationBatchBuilder, WorthQueryRuntimeError,
    WorthQueryRuntimeFacadeFamily, WorthQueryWorkspace, WorthQueryWriteCommand,
    WorthQueryWriteReceipt,
};

pub struct WorthQueryWorkspaceSubmissionLane<'a> {
    runtime: &'a mut super::WorthQueryRuntime,
}

impl<'a> WorthQueryWorkspaceSubmissionLane<'a> {
    pub(crate) fn new(runtime: &'a mut super::WorthQueryRuntime) -> Self {
        Self { runtime }
    }

    pub fn submit(
        &mut self,
        command: WorthQueryWriteCommand,
    ) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError> {
        self.runtime.write(command)
    }

    pub fn submit_batch(
        &mut self,
        commands: Vec<WorthQueryWriteCommand>,
    ) -> Result<super::WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        self.runtime.write_batch_intent(commands).execute()
    }

    pub fn submit_batch_builder(
        &mut self,
        builder: WorthQueryMutationBatchBuilder,
    ) -> Result<super::WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
        self.submit_batch(builder.finish()?)
    }

    pub fn submit_intent(
        &mut self,
        declaration: WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError> {
        self.runtime.execute_intent(declaration)
    }

    pub fn submit_effect_intent<T>(
        &mut self,
        effect: &WorthQueryEffectHandle<T>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
    ) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError> {
        self.runtime
            .execute_next_effect_write_intent(effect, strategy_version, input_contract)
    }

    pub fn write_intent(
        &'a mut self,
        command: WorthQueryWriteCommand,
    ) -> crate::intent_admission::WorthQueryRuntimeWriteIntentAuthoring<'a> {
        self.runtime.write_intent(command)
    }

    pub fn write_batch_intent(
        &'a mut self,
        commands: Vec<WorthQueryWriteCommand>,
    ) -> crate::intent_admission::WorthQueryRuntimeWriteBatchIntentAuthoring<'a> {
        self.runtime.write_batch_intent(commands)
    }
}

impl WorthQueryWorkspace {
    pub fn submissions(
        &mut self,
    ) -> Result<WorthQueryWorkspaceSubmissionLane<'_>, WorthQueryRuntimeError> {
        self.runtime
            .admit_facade_family(WorthQueryRuntimeFacadeFamily::Submission)?;
        Ok(WorthQueryWorkspaceSubmissionLane::new(&mut self.runtime))
    }
}
