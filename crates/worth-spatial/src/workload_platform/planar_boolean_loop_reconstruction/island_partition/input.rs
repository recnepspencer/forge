use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoopSet, PlanarBooleanBornLoopSet,
};

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanLoopIslandPartitionInput<'a> {
    reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
    born_loops: &'a PlanarBooleanBornLoopSet,
}

impl<'a> PlanarBooleanLoopIslandPartitionInput<'a> {
    pub fn from_reconstructed_loop_boundary(
        reconstructed_loops: &'a PlanarBooleanAdmittedReconstructedLoopSet,
        born_loops: &'a PlanarBooleanBornLoopSet,
    ) -> Self {
        Self {
            reconstructed_loops,
            born_loops,
        }
    }

    pub fn reconstructed_loops(self) -> &'a PlanarBooleanAdmittedReconstructedLoopSet {
        self.reconstructed_loops
    }

    pub fn born_loops(self) -> &'a PlanarBooleanBornLoopSet {
        self.born_loops
    }
}
