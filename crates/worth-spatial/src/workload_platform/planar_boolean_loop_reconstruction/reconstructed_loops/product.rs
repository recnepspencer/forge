use super::construction::admit_reconstructed_loop_boundary;
use super::counters::PlanarBooleanReconstructedLoopBoundaryCounters;
use super::denial::PlanarBooleanReconstructedLoopBoundaryDenial;
use super::input::PlanarBooleanReconstructedLoopBoundaryInput;
use super::row::{PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanBornLoop};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanAdmittedReconstructedLoopSet {
    set_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanAdmittedReconstructedLoop>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBornLoopSet {
    set_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanBornLoop>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanReconstructedLoopBoundary {
    admitted_reconstructed_loops: PlanarBooleanAdmittedReconstructedLoopSet,
    born_loops: PlanarBooleanBornLoopSet,
    counters: PlanarBooleanReconstructedLoopBoundaryCounters,
}

impl PlanarBooleanReconstructedLoopBoundary {
    pub fn admit(
        input: PlanarBooleanReconstructedLoopBoundaryInput<'_>,
    ) -> Result<Self, PlanarBooleanReconstructedLoopBoundaryDenial> {
        admit_reconstructed_loop_boundary(input)
    }

    pub(crate) fn new(
        admitted_reconstructed_loops: PlanarBooleanAdmittedReconstructedLoopSet,
        born_loops: PlanarBooleanBornLoopSet,
        counters: PlanarBooleanReconstructedLoopBoundaryCounters,
    ) -> Self {
        Self {
            admitted_reconstructed_loops,
            born_loops,
            counters,
        }
    }

    pub fn reconstructed_loops(&self) -> &PlanarBooleanAdmittedReconstructedLoopSet {
        &self.admitted_reconstructed_loops
    }

    pub fn born_loops(&self) -> &PlanarBooleanBornLoopSet {
        &self.born_loops
    }

    pub fn counters(&self) -> PlanarBooleanReconstructedLoopBoundaryCounters {
        self.counters
    }
}

impl PlanarBooleanAdmittedReconstructedLoopSet {
    pub(crate) fn new(
        set_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanAdmittedReconstructedLoop>,
    ) -> Self {
        Self {
            set_identity,
            request_identity,
            rows,
        }
    }

    pub fn reconstructed_loop_set_identity(&self) -> &str {
        &self.set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanAdmittedReconstructedLoop] {
        &self.rows
    }
}

impl PlanarBooleanBornLoopSet {
    pub(crate) fn new(
        set_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanBornLoop>,
    ) -> Self {
        Self {
            set_identity,
            request_identity,
            rows,
        }
    }

    pub fn born_loop_set_identity(&self) -> &str {
        &self.set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanBornLoop] {
        &self.rows
    }
}
