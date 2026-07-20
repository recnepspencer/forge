use crate::runtime::{
    WorthQueryGraphObligationIndex, WorthQueryGraphObligationMaterializedDispatch,
    WorthQueryGraphObligationOperatingWorldDescriptor, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryGraphTouchDescriptor,
};

use super::super::error::{
    WorthQueryGraphObligationConsumerKitError, WorthQueryGraphObligationConsumerKitErrorKind,
};
use super::execution::WorthQueryGraphObligationExecutionProof;
use super::selection::WorthQueryGraphObligationInMemoryProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationInMemoryTestWorkspace {
    catalog: WorthQueryGraphObligationRegistrationCatalog,
    index: WorthQueryGraphObligationIndex,
}

impl WorthQueryGraphObligationInMemoryTestWorkspace {
    pub fn from_registrations(
        registrations: impl IntoIterator<Item = WorthQueryGraphObligationRegistration>,
    ) -> Result<Self, WorthQueryGraphObligationConsumerKitError> {
        let catalog = WorthQueryGraphObligationRegistrationCatalog::from_registrations(
            registrations.into_iter().collect(),
        )
        .map_err(|error| {
            WorthQueryGraphObligationConsumerKitError::new(
                WorthQueryGraphObligationConsumerKitErrorKind::InMemoryWorkspaceBuildFailed,
                error.to_string(),
            )
        })?;
        Ok(Self::from_catalog(catalog))
    }

    pub fn from_catalog(catalog: WorthQueryGraphObligationRegistrationCatalog) -> Self {
        let index = WorthQueryGraphObligationIndex::from_catalog(&catalog);
        Self { catalog, index }
    }

    pub fn prove_selection(
        &self,
        touch_descriptor: &WorthQueryGraphTouchDescriptor,
        operating_world: &WorthQueryGraphObligationOperatingWorldDescriptor,
    ) -> WorthQueryGraphObligationInMemoryProof {
        WorthQueryGraphObligationInMemoryProof::from_selection(
            self.index
                .select_for_touch(touch_descriptor, operating_world),
        )
    }

    pub fn prove_execution(
        &self,
        touch_descriptor: &WorthQueryGraphTouchDescriptor,
        operating_world: &WorthQueryGraphObligationOperatingWorldDescriptor,
    ) -> WorthQueryGraphObligationExecutionProof {
        let selection = self
            .index
            .select_for_touch(touch_descriptor, operating_world);
        let selection_proof =
            WorthQueryGraphObligationInMemoryProof::from_selection(selection.clone());
        let envelope = WorthQueryGraphObligationMaterializedDispatch::from_selection(selection)
            .selected_result_envelope();
        WorthQueryGraphObligationExecutionProof::from_envelope(selection_proof, envelope)
    }

    pub fn catalog(&self) -> &WorthQueryGraphObligationRegistrationCatalog {
        &self.catalog
    }

    pub fn index(&self) -> &WorthQueryGraphObligationIndex {
        &self.index
    }
}
