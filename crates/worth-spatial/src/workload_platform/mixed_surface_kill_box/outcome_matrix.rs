use super::denial::MixedSurfaceKillBoxDenial;
use super::family_run::MixedSurfaceFamilyRun;
use super::receipt::MixedSurfaceKillBoxReceipt;
use crate::workload_platform::surface_support::SurfaceFamily;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MixedSurfaceKillBoxOutcomeKind {
    Admitted,
    Unsupported,
    IntegrityMismatch,
    Denied,
    MissingEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MixedSurfaceKillBoxOutcomeRow {
    kind: MixedSurfaceKillBoxOutcomeKind,
    family: Option<SurfaceFamily>,
    human_reason: String,
    evidence_identity: String,
}

impl MixedSurfaceKillBoxOutcomeRow {
    pub fn from_family_run(
        kind: MixedSurfaceKillBoxOutcomeKind,
        family: SurfaceFamily,
        human_reason: impl Into<String>,
        evidence_identity: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            family: Some(family),
            human_reason: human_reason.into(),
            evidence_identity: evidence_identity.into(),
        }
    }

    pub fn from_denial(
        kind: MixedSurfaceKillBoxOutcomeKind,
        denial: &MixedSurfaceKillBoxDenial,
    ) -> Self {
        Self {
            kind,
            family: None,
            human_reason: denial.human_reason(),
            evidence_identity: denial_evidence_identity(denial),
        }
    }

    pub fn kind(&self) -> MixedSurfaceKillBoxOutcomeKind {
        self.kind
    }

    pub fn family(&self) -> Option<SurfaceFamily> {
        self.family
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
}

fn denial_evidence_identity(denial: &MixedSurfaceKillBoxDenial) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "mixed-surface-kill-box-matrix-denial".to_string(),
            format!("{denial:?}"),
            denial.human_reason(),
        ],
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MixedSurfaceKillBoxOutcomeMatrix {
    rows: Vec<MixedSurfaceKillBoxOutcomeRow>,
}

impl MixedSurfaceKillBoxOutcomeMatrix {
    pub fn from_receipt(
        receipt: &MixedSurfaceKillBoxReceipt,
    ) -> Result<Self, MixedSurfaceKillBoxDenial> {
        let mut rows = Vec::new();
        append_family_outcome_rows(&mut rows, receipt);
        append_receipt_bound_denial_rows(&mut rows, receipt)?;
        append_external_substitution_rows(&mut rows);
        let matrix = Self::from_required_rows(rows)?;
        Ok(matrix)
    }

    fn from_required_rows(
        rows: Vec<MixedSurfaceKillBoxOutcomeRow>,
    ) -> Result<Self, MixedSurfaceKillBoxDenial> {
        let matrix = Self { rows };
        matrix.require(MixedSurfaceKillBoxOutcomeKind::Admitted)?;
        matrix.require(MixedSurfaceKillBoxOutcomeKind::Unsupported)?;
        matrix.require(MixedSurfaceKillBoxOutcomeKind::IntegrityMismatch)?;
        matrix.require(MixedSurfaceKillBoxOutcomeKind::Denied)?;
        matrix.require(MixedSurfaceKillBoxOutcomeKind::MissingEvidence)?;
        Ok(matrix)
    }

    pub fn rows(&self) -> &[MixedSurfaceKillBoxOutcomeRow] {
        &self.rows
    }

    pub fn row_for_kind(
        &self,
        kind: MixedSurfaceKillBoxOutcomeKind,
    ) -> Option<&MixedSurfaceKillBoxOutcomeRow> {
        self.rows.iter().find(|row| row.kind == kind)
    }

    fn require(
        &self,
        kind: MixedSurfaceKillBoxOutcomeKind,
    ) -> Result<(), MixedSurfaceKillBoxDenial> {
        self.row_for_kind(kind).map(|_| ()).ok_or(
            MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence {
                family: SurfaceFamily::Unknown,
            },
        )
    }
}

fn append_family_outcome_rows(
    rows: &mut Vec<MixedSurfaceKillBoxOutcomeRow>,
    receipt: &MixedSurfaceKillBoxReceipt,
) {
    for run in receipt.runs() {
        let kind = if run.is_acceptable_m7_input() {
            MixedSurfaceKillBoxOutcomeKind::Admitted
        } else {
            MixedSurfaceKillBoxOutcomeKind::Unsupported
        };
        rows.push(MixedSurfaceKillBoxOutcomeRow::from_family_run(
            kind,
            run.family(),
            run.human_reason(),
            run.support_evidence_digest(),
        ));
    }
}

fn append_receipt_bound_denial_rows(
    rows: &mut Vec<MixedSurfaceKillBoxOutcomeRow>,
    receipt: &MixedSurfaceKillBoxReceipt,
) -> Result<(), MixedSurfaceKillBoxDenial> {
    let plane = required_run(receipt, SurfaceFamily::Plane)?;
    let generated = required_run(receipt, SurfaceFamily::GeneratedFeature)?;
    let freeform = required_run(receipt, SurfaceFamily::Freeform)?;
    let plane_receipt_smuggling_denial = receipt_bound_plane_smuggling_denial(generated, plane)?;
    let wrong_response_denial = receipt_bound_wrong_response_denial(generated, freeform)?;

    rows.push(MixedSurfaceKillBoxOutcomeRow::from_denial(
        MixedSurfaceKillBoxOutcomeKind::IntegrityMismatch,
        &plane_receipt_smuggling_denial,
    ));
    rows.push(MixedSurfaceKillBoxOutcomeRow::from_denial(
        MixedSurfaceKillBoxOutcomeKind::Denied,
        &wrong_response_denial,
    ));
    Ok(())
}

fn append_external_substitution_rows(rows: &mut Vec<MixedSurfaceKillBoxOutcomeRow>) {
    rows.push(MixedSurfaceKillBoxOutcomeRow::from_denial(
        MixedSurfaceKillBoxOutcomeKind::IntegrityMismatch,
        &MixedSurfaceKillBoxDenial::KernelSummarySubstitution,
    ));
    rows.push(MixedSurfaceKillBoxOutcomeRow::from_denial(
        MixedSurfaceKillBoxOutcomeKind::MissingEvidence,
        &MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence {
            family: SurfaceFamily::Freeform,
        },
    ));
}

fn required_run(
    receipt: &MixedSurfaceKillBoxReceipt,
    family: SurfaceFamily,
) -> Result<&MixedSurfaceFamilyRun, MixedSurfaceKillBoxDenial> {
    receipt
        .run_for_family(family)
        .ok_or(MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence { family })
}

fn receipt_bound_plane_smuggling_denial(
    generated: &MixedSurfaceFamilyRun,
    plane: &MixedSurfaceFamilyRun,
) -> Result<MixedSurfaceKillBoxDenial, MixedSurfaceKillBoxDenial> {
    match generated.attempt_with_plane_support_receipt(plane) {
        Ok(()) => Err(MixedSurfaceKillBoxDenial::GeneratedFeatureSmugglingAttempt),
        Err(denial) => Ok(denial),
    }
}

fn receipt_bound_wrong_response_denial(
    generated: &MixedSurfaceFamilyRun,
    freeform: &MixedSurfaceFamilyRun,
) -> Result<MixedSurfaceKillBoxDenial, MixedSurfaceKillBoxDenial> {
    match generated.attempt_with_user_response(freeform) {
        Ok(()) => Err(MixedSurfaceKillBoxDenial::GeneratedFeatureSmugglingAttempt),
        Err(denial) => Ok(denial),
    }
}
