use crate::query_obligation_selection::public_facade::{
    IntoQueryGraphObligationSelectionRequest, QueryGraphObligationSelectionFacadeError,
    QueryGraphObligationSelectionRequest, WorthQuerySelectedGraphObligations,
};
use crate::query_obligation_selection::selection_substrate::QueryObligationSelectionSubstrate;
use worth_spatial::facade::workload_vocabulary::{
    CompleteWorkloadEvidenceLedger, SpatialEvidenceQueryTouchDescriptor,
};

use super::WorthWorkload;

impl WorthWorkload {
    pub fn select_query_graph_obligations<I>(
        &self,
        input: I,
    ) -> Result<WorthQuerySelectedGraphObligations, QueryGraphObligationSelectionFacadeError>
    where
        I: IntoQueryGraphObligationSelectionRequest,
    {
        let request = input.into_query_graph_obligation_selection_request()?;
        self.require_query_obligation_selection_request_matches_workload(&request)?;
        let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(
            request.into_selection_input(),
        )?;
        Ok(WorthQuerySelectedGraphObligations::from_selected(selected))
    }

    fn require_query_obligation_selection_request_matches_workload(
        &self,
        request: &QueryGraphObligationSelectionRequest,
    ) -> Result<(), QueryGraphObligationSelectionFacadeError> {
        let Some(descriptor) = request.spatial_descriptor() else {
            return Ok(());
        };

        require_spatial_query_descriptor_stage_index_matches_workload(
            self.evidence_ledger(),
            descriptor,
        )?;
        require_spatial_query_descriptor_evidence_link_matches_workload(
            self.evidence_ledger(),
            descriptor,
        )?;

        Ok(())
    }
}

fn require_spatial_query_descriptor_stage_index_matches_workload(
    ledger: &CompleteWorkloadEvidenceLedger,
    descriptor: &SpatialEvidenceQueryTouchDescriptor,
) -> Result<(), QueryGraphObligationSelectionFacadeError> {
    let stage_index_identity = ledger.stage_index().index_identity();
    if descriptor.stage_index_identity() == stage_index_identity {
        return Ok(());
    }

    Err(
        QueryGraphObligationSelectionFacadeError::workload_authority_mismatch(format!(
            "spatial Query descriptor belongs to stage index `{}`, not workload stage index `{}`",
            descriptor.stage_index_identity(),
            stage_index_identity
        )),
    )
}

fn require_spatial_query_descriptor_evidence_link_matches_workload(
    ledger: &CompleteWorkloadEvidenceLedger,
    descriptor: &SpatialEvidenceQueryTouchDescriptor,
) -> Result<(), QueryGraphObligationSelectionFacadeError> {
    let stage_links = ledger
        .link_required_stages(&[descriptor.evidence_stage()])
        .map_err(|error| {
            QueryGraphObligationSelectionFacadeError::workload_authority_mismatch(
                error.human_reason(),
            )
        })?;
    if stage_links.links_to_identity(descriptor.evidence_stage(), descriptor.evidence_identity()) {
        return Ok(());
    }

    Err(
        QueryGraphObligationSelectionFacadeError::workload_authority_mismatch(format!(
            "spatial Query descriptor evidence identity `{}` is not linked by this workload",
            descriptor.evidence_identity()
        )),
    )
}
