use crate::{OwnerEvidenceClass, OwnerObservationOmissionCause};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptLossClassification {
    NoOwnerTransition,
    DiagnosticOmissionDefect,
    AuthoritativeReceiptOmissionDefect,
    CrashLostNonAuthoritativeTrace,
    ProtocolMappingHole,
}

pub const fn classify_receipt_loss(
    evidence: OwnerEvidenceClass,
    cause: OwnerObservationOmissionCause,
) -> ReceiptLossClassification {
    match (cause, evidence) {
        (OwnerObservationOmissionCause::NoOwnerTransition, _) => {
            ReceiptLossClassification::NoOwnerTransition
        }
        (_, OwnerEvidenceClass::ForbiddenAuthoritySubstitute) => {
            ReceiptLossClassification::ProtocolMappingHole
        }
        (
            OwnerObservationOmissionCause::InstrumentationDidNotEmit,
            OwnerEvidenceClass::EphemeralDiagnosticTrace,
        ) => ReceiptLossClassification::DiagnosticOmissionDefect,
        (OwnerObservationOmissionCause::InstrumentationDidNotEmit, _) => {
            ReceiptLossClassification::AuthoritativeReceiptOmissionDefect
        }
        (
            OwnerObservationOmissionCause::LostAcrossCrash,
            OwnerEvidenceClass::EphemeralDiagnosticTrace,
        ) => ReceiptLossClassification::CrashLostNonAuthoritativeTrace,
        (OwnerObservationOmissionCause::LostAcrossCrash, _) => {
            ReceiptLossClassification::AuthoritativeReceiptOmissionDefect
        }
        (OwnerObservationOmissionCause::MissingFromProtocol, _) => {
            ReceiptLossClassification::ProtocolMappingHole
        }
    }
}
