use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoopSet, PlanarBooleanBornLoopSet,
    PlanarBooleanLoopContainmentEvidencePostureSet, PlanarBooleanLoopRoleOutcomeSet,
    PlanarBooleanLoopSourceCarrierSet,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanDegenerateLoopOutcomeBoundaryInput<'a> {
    reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
    born_loops: &'a PlanarBooleanBornLoopSet,
    role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
    containment_postures: &'a PlanarBooleanLoopContainmentEvidencePostureSet,
    source_loop_carriers: &'a PlanarBooleanLoopSourceCarrierSet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
}

impl<'a> PlanarBooleanDegenerateLoopOutcomeBoundaryInput<'a> {
    pub fn from_reconstructed_products_and_role_evidence(
        reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
        born_loops: &'a PlanarBooleanBornLoopSet,
        role_outcomes: &'a PlanarBooleanLoopRoleOutcomeSet,
        containment_postures: &'a PlanarBooleanLoopContainmentEvidencePostureSet,
        source_loop_carriers: &'a PlanarBooleanLoopSourceCarrierSet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    ) -> Self {
        Self {
            reconstructed_loops,
            born_loops,
            role_outcomes,
            containment_postures,
            source_loop_carriers,
            split_fragments,
        }
    }

    pub fn reconstructed_loops(&self) -> &'a PlanarBooleanAdmittedReconstructedLoopSet {
        self.reconstructed_loops
    }

    pub fn born_loops(&self) -> &'a PlanarBooleanBornLoopSet {
        self.born_loops
    }

    pub fn role_outcomes(&self) -> &'a PlanarBooleanLoopRoleOutcomeSet {
        self.role_outcomes
    }

    pub fn containment_postures(&self) -> &'a PlanarBooleanLoopContainmentEvidencePostureSet {
        self.containment_postures
    }

    pub fn source_loop_carriers(&self) -> &'a PlanarBooleanLoopSourceCarrierSet {
        self.source_loop_carriers
    }

    pub fn split_fragments(&self) -> &'a PlanarBooleanSplitEdgeFragmentSet {
        self.split_fragments
    }
}
