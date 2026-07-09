use worth_store_offline_verifier::OfflineVerifierBoundarySeam;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndependentVerifierObservationKind {
    Agreement,
    Disagreement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndependentVerifierObservation {
    seam: OfflineVerifierBoundarySeam,
    kind: IndependentVerifierObservationKind,
}

impl IndependentVerifierObservation {
    pub const fn agreement(seam: OfflineVerifierBoundarySeam) -> Self {
        Self {
            seam,
            kind: IndependentVerifierObservationKind::Agreement,
        }
    }

    pub const fn disagreement(seam: OfflineVerifierBoundarySeam) -> Self {
        Self {
            seam,
            kind: IndependentVerifierObservationKind::Disagreement,
        }
    }

    pub const fn seam(&self) -> OfflineVerifierBoundarySeam {
        self.seam
    }

    pub const fn kind(&self) -> IndependentVerifierObservationKind {
        self.kind
    }
}
