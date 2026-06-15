use crate::planar_contracts::planar_recovery::PlanarRecoveryPostureBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarRecoveryPostureInspectionKind {
    Source,
    Blocker,
    RecoveryAction,
    TruthEffect,
    BasisReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarRecoveryPostureInspectionRow {
    kind: PlanarRecoveryPostureInspectionKind,
    locus: String,
    value: String,
}

impl PlanarRecoveryPostureInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarRecoveryPostureBasis) -> Vec<Self> {
        let mut rows = vec![
            row(
                PlanarRecoveryPostureInspectionKind::Source,
                "planar_recovery.source",
                basis.source().source_digest(),
            ),
            row(
                PlanarRecoveryPostureInspectionKind::Blocker,
                "planar_recovery.blocker",
                format!("{:?}", basis.blocker_kind()),
            ),
            row(
                PlanarRecoveryPostureInspectionKind::RecoveryAction,
                "planar_recovery.action",
                format!("{:?}", basis.recovery_action()),
            ),
            row(
                PlanarRecoveryPostureInspectionKind::TruthEffect,
                "planar_recovery.truth_effect",
                format!("{:?}", basis.truth_effect()),
            ),
        ];
        if let Some(retained) = basis.retained_planar_facts() {
            rows.push(row(
                PlanarRecoveryPostureInspectionKind::BasisReceipt,
                "planar_recovery.retained_planar_fact",
                retained.retained_fact_digest(),
            ));
        }
        if let Some(projected) = basis.projection_consumed_facts() {
            rows.push(row(
                PlanarRecoveryPostureInspectionKind::BasisReceipt,
                "planar_recovery.projection_consumed_fact",
                projected.projection_consumption_digest(),
            ));
        }
        rows
    }
}

fn row(
    kind: PlanarRecoveryPostureInspectionKind,
    locus: impl Into<String>,
    value: impl ToString,
) -> PlanarRecoveryPostureInspectionRow {
    PlanarRecoveryPostureInspectionRow {
        kind,
        locus: locus.into(),
        value: value.to_string(),
    }
}
