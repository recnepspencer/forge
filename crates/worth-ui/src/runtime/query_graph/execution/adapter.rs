use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationExecutionProof,
    ForgeQueryGraphObligationInMemoryTestWorkspace,
};
use forge_query::facade::runtime::ForgeQueryGraphObligationRegistration;

use crate::runtime::{WorthUiQueryGraphOperatingWorld, WorthUiQueryGraphTouchDescriptor};

pub(crate) struct WorthUiQueryGraphExecutionAdapter {
    workspace: ForgeQueryGraphObligationInMemoryTestWorkspace,
}

impl WorthUiQueryGraphExecutionAdapter {
    pub(crate) fn from_registrations(
        registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
    ) -> Result<Self, ForgeQueryGraphObligationConsumerKitError> {
        Ok(Self {
            workspace: ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations(
                registrations,
            )?,
        })
    }

    pub(crate) fn execute(
        &self,
        touch_descriptor: &WorthUiQueryGraphTouchDescriptor,
        operating_world: &WorthUiQueryGraphOperatingWorld,
    ) -> ForgeQueryGraphObligationExecutionProof {
        self.workspace
            .prove_execution(touch_descriptor.descriptor(), operating_world.descriptor())
    }
}
