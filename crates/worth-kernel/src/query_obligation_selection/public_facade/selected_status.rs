use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionProof;

use super::super::selection_substrate::QuerySelectedGraphObligations;
use super::selected_closeout::WorthQuerySelectedGraphObligationCloseout;

#[derive(Clone, Debug)]
pub struct WorthQuerySelectedGraphObligations {
    selected: QuerySelectedGraphObligations,
}

impl WorthQuerySelectedGraphObligations {
    pub(crate) fn from_selected(selected: QuerySelectedGraphObligations) -> Self {
        Self { selected }
    }

    pub fn execution_proof(&self) -> &ForgeQueryGraphObligationExecutionProof {
        self.selected.execution_proof()
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selected.selected_obligation_count()
    }

    pub fn execution_row_count(&self) -> usize {
        self.selected.execution_row_count()
    }

    pub fn selected_registration_digests(&self) -> Vec<String> {
        self.selected.selected_registration_digests()
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        self.selected.touch_descriptor_digest()
    }

    pub fn authority_digest(&self) -> &str {
        self.selected.authority_digest()
    }

    pub fn spatial_touch_digest(&self) -> Option<&str> {
        self.selected.spatial_touch_digest()
    }

    pub fn spatial_lookup_product_digest(&self) -> Option<&str> {
        self.selected.spatial_lookup_product_digest()
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        self.selected.adoption_manifest_digest()
    }

    pub fn closeout(&self) -> WorthQuerySelectedGraphObligationCloseout {
        WorthQuerySelectedGraphObligationCloseout::from_closeout(self.selected.closeout())
    }
}
