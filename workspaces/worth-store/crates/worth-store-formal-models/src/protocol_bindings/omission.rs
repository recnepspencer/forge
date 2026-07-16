use super::OwnerEvidenceClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerObservationOmissionCause {
    NoOwnerTransition,
    InstrumentationDidNotEmit,
    LostAcrossCrash,
    MissingFromProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OwnerObservationOmissionVerdict {
    ImpossibleNoTransition,
    InstrumentationDefect,
    CrashLostEphemeralDiagnostic,
    IllegalProtocolHole,
}

pub const fn classify_owner_observation_omission(
    evidence_class: OwnerEvidenceClass,
    cause: OwnerObservationOmissionCause,
) -> OwnerObservationOmissionVerdict {
    use OwnerEvidenceClass::{
        DurableAuthoritativeReceipt, EphemeralDiagnosticTrace, ReopenedObservedReceipt,
    };
    use OwnerObservationOmissionCause::{
        InstrumentationDidNotEmit, LostAcrossCrash, MissingFromProtocol, NoOwnerTransition,
    };
    use OwnerObservationOmissionVerdict::{
        CrashLostEphemeralDiagnostic, IllegalProtocolHole, ImpossibleNoTransition,
        InstrumentationDefect,
    };

    match (cause, evidence_class) {
        (NoOwnerTransition, _) => ImpossibleNoTransition,
        (InstrumentationDidNotEmit, _) => InstrumentationDefect,
        (LostAcrossCrash, EphemeralDiagnosticTrace) => CrashLostEphemeralDiagnostic,
        (LostAcrossCrash, ReopenedObservedReceipt) => InstrumentationDefect,
        (LostAcrossCrash, DurableAuthoritativeReceipt) | (MissingFromProtocol, _) => {
            IllegalProtocolHole
        }
        (LostAcrossCrash, OwnerEvidenceClass::ForbiddenAuthoritySubstitute) => IllegalProtocolHole,
    }
}
