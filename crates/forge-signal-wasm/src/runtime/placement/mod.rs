use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::core::RuntimeCore;

mod callback_placement_eligibility;
mod declaration_candidate;
pub(crate) mod declaration_classification;
pub(crate) mod lowering;
pub(crate) mod placement_category;
pub(crate) mod raw_declaration_proof;
mod worker_placement_summary_projection;

pub use callback_placement_eligibility::WorkerCallbackPlacementEligibilityPackage;
pub(crate) use declaration_candidate::{PlacementDeclarationCandidate, PlacementDeclarationOrigin};
use worker_placement_summary_projection::WorkerPlacementSummary;

impl RuntimeCore {
    pub fn worker_placement_summary(&self) -> Result<WorkerPlacementSummary, ForgeSignalJsError> {
        Ok(
            worker_placement_summary_projection::project_worker_placement_summary(
                self.collect_worker_placement_declaration_candidates()?,
            ),
        )
    }

    pub fn worker_callback_placement_eligibility(
        &self,
    ) -> Result<WorkerCallbackPlacementEligibilityPackage, ForgeSignalJsError> {
        callback_placement_eligibility::certify_callback_placement_eligibility(
            self.collect_worker_placement_declaration_candidates()?,
        )
    }
}
