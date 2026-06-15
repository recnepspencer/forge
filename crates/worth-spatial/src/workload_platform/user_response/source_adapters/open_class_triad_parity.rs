use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::open_class_triad_parity::{
    OpenClassTriadParityDenial, OpenClassTriadParityDenialKind, OpenClassTriadParityReceipt,
};
use crate::workload_platform::user_response::{
    source::WorthUserResponseSourceKind, WorthUserOutcomeCauseKind, WorthUserResponseSource,
};

impl WorthUserResponseSource {
    pub fn from_open_class_triad_parity(receipt: &OpenClassTriadParityReceipt) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "Open-class triad parity preserved wire, sheet, and NMT fan identity across {} receipt-backed lanes.",
                    receipt.counters().receipt_backed_lanes()
                ),
                evidence_digest: receipt.triad_digest().to_string(),
                source_identity: receipt.triad_digest().to_string(),
            },
        }
    }

    pub fn from_open_class_triad_parity_denial(denial: &OpenClassTriadParityDenial) -> Self {
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "open-class-triad-parity-denial".to_string(),
                format!("{:?}", denial.kind()),
                denial.human_reason().to_string(),
            ],
        );
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: cause_kind(denial.kind()),
                message: denial.human_reason().to_string(),
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }
}

fn cause_kind(kind: OpenClassTriadParityDenialKind) -> WorthUserOutcomeCauseKind {
    match kind {
        OpenClassTriadParityDenialKind::DeniedLaneUpgrade => {
            WorthUserOutcomeCauseKind::DeniedMovementOrRotation
        }
        OpenClassTriadParityDenialKind::CrossClassCheckpointReplay
        | OpenClassTriadParityDenialKind::ProjectionConsumptionMismatch
        | OpenClassTriadParityDenialKind::TopologyParityMismatch
        | OpenClassTriadParityDenialKind::BoundedConversionViolation => {
            WorthUserOutcomeCauseKind::IntegrityMismatch
        }
        OpenClassTriadParityDenialKind::StormExtractionUnsupported
        | OpenClassTriadParityDenialKind::UnsupportedOpenClass => {
            WorthUserOutcomeCauseKind::UnsupportedInput
        }
        OpenClassTriadParityDenialKind::MissingDeclaration
        | OpenClassTriadParityDenialKind::MissingOpenClass
        | OpenClassTriadParityDenialKind::DuplicateOpenClass
        | OpenClassTriadParityDenialKind::ParityReceiptRejected
        | OpenClassTriadParityDenialKind::MissingLaneEvidence => {
            WorthUserOutcomeCauseKind::MissingEvidence
        }
    }
}
