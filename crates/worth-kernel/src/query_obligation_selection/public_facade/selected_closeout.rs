use super::super::selection_substrate::QuerySelectedGraphObligationCloseout;
use super::kinds::QueryGraphObligationSelectionAuthorityKind;
use super::selected_precision::WorthQuerySelectorPrecisionReport;

#[derive(Clone, Debug)]
pub struct WorthQuerySelectedGraphObligationCloseout {
    closeout: QuerySelectedGraphObligationCloseout,
}

impl WorthQuerySelectedGraphObligationCloseout {
    pub(crate) fn from_closeout(closeout: QuerySelectedGraphObligationCloseout) -> Self {
        Self { closeout }
    }

    pub fn authority_kind(&self) -> QueryGraphObligationSelectionAuthorityKind {
        self.closeout.authority_kind().into()
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        self.closeout.touch_descriptor_digest()
    }

    pub fn authority_digest(&self) -> &str {
        self.closeout.authority_digest()
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.closeout.selected_obligation_count()
    }

    pub fn selected_registration_count(&self) -> usize {
        self.closeout.selected_registration_digests().len()
    }

    pub fn execution_row_count(&self) -> usize {
        self.closeout.execution_row_count()
    }

    pub fn execution_proof_digest(&self) -> &str {
        self.closeout.execution_proof_digest()
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        self.closeout.adoption_manifest_digest()
    }

    pub fn residue_manifest_digest(&self) -> &str {
        self.closeout.residue_manifest_digest()
    }

    pub fn graph_read_access_planning_claimed(&self) -> bool {
        self.closeout.graph_read_access_planning_claimed()
    }

    pub fn spatial_query_gap_rows(&self) -> usize {
        self.closeout.spatial_query_gap_rows().len()
    }

    pub fn selector_precision_report(&self) -> WorthQuerySelectorPrecisionReport {
        WorthQuerySelectorPrecisionReport::from_report(
            self.closeout.selector_precision_report().clone(),
        )
    }

    pub fn local_ceremony_is_clean(&self) -> bool {
        self.closeout.local_ceremony_closeout().is_clean()
    }

    pub(crate) fn into_closeout(self) -> QuerySelectedGraphObligationCloseout {
        self.closeout
    }
}
