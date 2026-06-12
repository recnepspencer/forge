use std::collections::BTreeSet;

use super::denial::{OpenClassTriadParityDenial, OpenClassTriadParityDenialKind};
use super::receipt::OpenClassTriadParityReceipt;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpenClassTriadOutcomeKind {
    Admitted,
    Denied,
    IntegrityMismatch,
    Unsupported,
    NoOptions,
}

impl OpenClassTriadOutcomeKind {
    pub const REQUIRED: [Self; 5] = [
        Self::Admitted,
        Self::Denied,
        Self::IntegrityMismatch,
        Self::Unsupported,
        Self::NoOptions,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenClassTriadOutcomeRow {
    kind: OpenClassTriadOutcomeKind,
    human_reason: String,
}

impl OpenClassTriadOutcomeRow {
    pub fn admitted(receipt: &OpenClassTriadParityReceipt) -> Self {
        Self {
            kind: OpenClassTriadOutcomeKind::Admitted,
            human_reason: format!(
                "Open-class triad parity compared {} open classes and {} receipt-backed lanes.",
                receipt.counters().open_classes_compared(),
                receipt.counters().receipt_backed_lanes()
            ),
        }
    }

    pub fn from_denial(denial: &OpenClassTriadParityDenial) -> Self {
        Self {
            kind: outcome_kind(denial.kind()),
            human_reason: denial.human_reason().to_string(),
        }
    }

    pub fn kind(&self) -> OpenClassTriadOutcomeKind {
        self.kind
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenClassTriadOutcomeMatrix {
    rows: Vec<OpenClassTriadOutcomeRow>,
}

impl OpenClassTriadOutcomeMatrix {
    pub fn from_rows(rows: Vec<OpenClassTriadOutcomeRow>) -> Result<Self, &'static str> {
        if rows.is_empty() {
            return Err("open-class triad outcome matrix requires rows");
        }
        require_every_outcome_kind_once(&rows)?;
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[OpenClassTriadOutcomeRow] {
        &self.rows
    }
}

fn require_every_outcome_kind_once(rows: &[OpenClassTriadOutcomeRow]) -> Result<(), &'static str> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.kind()) {
            return Err("open-class triad outcome matrix has duplicate outcome kind");
        }
    }
    for required in OpenClassTriadOutcomeKind::REQUIRED {
        if !seen.contains(&required) {
            return Err("open-class triad outcome matrix is missing a required outcome kind");
        }
    }
    Ok(())
}

fn outcome_kind(kind: OpenClassTriadParityDenialKind) -> OpenClassTriadOutcomeKind {
    match kind {
        OpenClassTriadParityDenialKind::DeniedLaneUpgrade => OpenClassTriadOutcomeKind::Denied,
        OpenClassTriadParityDenialKind::CrossClassCheckpointReplay
        | OpenClassTriadParityDenialKind::ProjectionConsumptionMismatch
        | OpenClassTriadParityDenialKind::TopologyParityMismatch
        | OpenClassTriadParityDenialKind::BoundedConversionViolation => {
            OpenClassTriadOutcomeKind::IntegrityMismatch
        }
        OpenClassTriadParityDenialKind::StormExtractionUnsupported
        | OpenClassTriadParityDenialKind::UnsupportedOpenClass => {
            OpenClassTriadOutcomeKind::Unsupported
        }
        OpenClassTriadParityDenialKind::MissingDeclaration
        | OpenClassTriadParityDenialKind::MissingOpenClass
        | OpenClassTriadParityDenialKind::DuplicateOpenClass
        | OpenClassTriadParityDenialKind::ParityReceiptRejected
        | OpenClassTriadParityDenialKind::MissingLaneEvidence => {
            OpenClassTriadOutcomeKind::NoOptions
        }
    }
}
