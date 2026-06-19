use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoopSet, PlanarBooleanBornLoopSet,
    PlanarBooleanDegenerateLoopOutcomeSet, PlanarBooleanDeniedLoopCandidateSet,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanSourceLoopSplitAttribution,
};

use super::support::PlanarBooleanLoopNamingAuthoritySupport;

#[derive(Clone, Copy)]
pub struct PlanarBooleanLoopIdentityMintingInput<'a> {
    reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
    born_loops: &'a PlanarBooleanBornLoopSet,
    role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
    degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
    denied_loop_candidates: &'a PlanarBooleanDeniedLoopCandidateSet,
    naming_support: &'a PlanarBooleanLoopNamingAuthoritySupport,
    split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
}

impl<'a> PlanarBooleanLoopIdentityMintingInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_phase_twelve_products_and_naming_support(
        reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
        born_loops: &'a PlanarBooleanBornLoopSet,
        role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
        degenerate_outcomes: &'a PlanarBooleanDegenerateLoopOutcomeSet,
        denied_loop_candidates: &'a PlanarBooleanDeniedLoopCandidateSet,
        naming_support: &'a PlanarBooleanLoopNamingAuthoritySupport,
        split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
    ) -> Self {
        Self {
            reconstructed_loops,
            born_loops,
            role_outcomes,
            degenerate_outcomes,
            denied_loop_candidates,
            naming_support,
            split_attribution,
        }
    }

    pub fn reconstructed_loops(self) -> &'a PlanarBooleanAdmittedReconstructedLoopSet {
        self.reconstructed_loops
    }

    pub fn born_loops(self) -> &'a PlanarBooleanBornLoopSet {
        self.born_loops
    }

    pub fn role_outcomes(self) -> &'a PlanarBooleanLoopRoleOutcomeSet {
        self.role_outcomes
    }

    pub fn degenerate_outcomes(self) -> &'a PlanarBooleanDegenerateLoopOutcomeSet {
        self.degenerate_outcomes
    }

    pub fn denied_loop_candidates(self) -> &'a PlanarBooleanDeniedLoopCandidateSet {
        self.denied_loop_candidates
    }

    pub fn naming_support(self) -> &'a PlanarBooleanLoopNamingAuthoritySupport {
        self.naming_support
    }

    pub fn split_attribution(self) -> &'a PlanarBooleanSourceLoopSplitAttribution {
        self.split_attribution
    }
}
