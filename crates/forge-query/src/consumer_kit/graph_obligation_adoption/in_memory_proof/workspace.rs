use crate::runtime::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationMaterializedDispatch,
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphTouchDescriptor,
};

use super::super::error::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationConsumerKitErrorKind,
};
use super::execution::ForgeQueryGraphObligationExecutionProof;
use super::selection::ForgeQueryGraphObligationInMemoryProof;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationInMemoryTestWorkspace {
    catalog: ForgeQueryGraphObligationRegistrationCatalog,
    index: ForgeQueryGraphObligationIndex,
}

impl ForgeQueryGraphObligationInMemoryTestWorkspace {
    pub fn from_registrations(
        registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
    ) -> Result<Self, ForgeQueryGraphObligationConsumerKitError> {
        let catalog = ForgeQueryGraphObligationRegistrationCatalog::from_registrations(
            registrations.into_iter().collect(),
        )
        .map_err(|error| {
            ForgeQueryGraphObligationConsumerKitError::new(
                ForgeQueryGraphObligationConsumerKitErrorKind::InMemoryWorkspaceBuildFailed,
                error.to_string(),
            )
        })?;
        Ok(Self::from_catalog(catalog))
    }

    pub fn from_catalog(catalog: ForgeQueryGraphObligationRegistrationCatalog) -> Self {
        let index = ForgeQueryGraphObligationIndex::from_catalog(&catalog);
        Self { catalog, index }
    }

    pub fn prove_selection(
        &self,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
    ) -> ForgeQueryGraphObligationInMemoryProof {
        ForgeQueryGraphObligationInMemoryProof::from_selection(
            self.index
                .select_for_touch(touch_descriptor, operating_world),
        )
    }

    pub fn prove_execution(
        &self,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
    ) -> ForgeQueryGraphObligationExecutionProof {
        let selection = self
            .index
            .select_for_touch(touch_descriptor, operating_world);
        let selection_proof =
            ForgeQueryGraphObligationInMemoryProof::from_selection(selection.clone());
        let envelope = ForgeQueryGraphObligationMaterializedDispatch::from_selection(selection)
            .selected_result_envelope();
        ForgeQueryGraphObligationExecutionProof::from_envelope(selection_proof, envelope)
    }

    pub fn catalog(&self) -> &ForgeQueryGraphObligationRegistrationCatalog {
        &self.catalog
    }

    pub fn index(&self) -> &ForgeQueryGraphObligationIndex {
        &self.index
    }
}
