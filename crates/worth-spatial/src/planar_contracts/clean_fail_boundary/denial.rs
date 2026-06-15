use super::PlanarCleanFailBoundaryCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarCleanFailBoundaryDenialKind {
    MissingSourceDigest,
    MissingAdmissionRow,
    MismatchedAdmissionFamily,
    AdmissionRowAdmitsRuntime,
    MissingTransformPosture,
    MissingRecoveryPosture,
    MismatchedRecoveryPosture,
    MissingDiagnostics,
    MismatchedDiagnostics,
    DiagnosticChangedTruth,
    HeuristicRepairAttempted,
    BoundedConversionAttempted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarCleanFailBoundaryDenial {
    kind: PlanarCleanFailBoundaryDenialKind,
    reason: &'static str,
    counters: PlanarCleanFailBoundaryCounters,
}

impl PlanarCleanFailBoundaryDenial {
    pub(crate) fn new(kind: PlanarCleanFailBoundaryDenialKind, reason: &'static str) -> Self {
        let counters = match kind {
            PlanarCleanFailBoundaryDenialKind::HeuristicRepairAttempted => {
                PlanarCleanFailBoundaryCounters::denied_repair()
            }
            PlanarCleanFailBoundaryDenialKind::BoundedConversionAttempted => {
                PlanarCleanFailBoundaryCounters::denied_bounded_conversion()
            }
            _ => PlanarCleanFailBoundaryCounters::certified(0, 0, 0, 0),
        };
        Self {
            kind,
            reason,
            counters,
        }
    }

    pub fn kind(&self) -> PlanarCleanFailBoundaryDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        self.reason
    }

    pub fn counters(&self) -> PlanarCleanFailBoundaryCounters {
        self.counters
    }
}
