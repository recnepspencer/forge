use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumedPlanarFactsInspectionKind {
    RetainedSource,
    StructuralIdentity,
    MotionPosture,
    TopologyContract,
    ProjectionReceipt,
    MaterializationBasis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumedPlanarFactsInspectionRow {
    kind: ProjectionConsumedPlanarFactsInspectionKind,
    locus: String,
    value: String,
}

impl ProjectionConsumedPlanarFactsInspectionRow {
    pub(crate) fn from_basis(basis: &ProjectionConsumedPlanarFactsBasis) -> Vec<Self> {
        let mut rows = vec![
            row(
                ProjectionConsumedPlanarFactsInspectionKind::RetainedSource,
                "projection_consumed.retained_planar_fact",
                basis.retained_planar_fact_digest(),
            ),
            row(
                ProjectionConsumedPlanarFactsInspectionKind::StructuralIdentity,
                "projection_consumed.structural_identity",
                basis.structural_identity_digest(),
            ),
            row(
                ProjectionConsumedPlanarFactsInspectionKind::MotionPosture,
                "projection_consumed.motion_posture",
                basis.motion_posture_digest(),
            ),
            row(
                ProjectionConsumedPlanarFactsInspectionKind::TopologyContract,
                "projection_consumed.topology_contract",
                basis.topology_contract_digest(),
            ),
            row(
                ProjectionConsumedPlanarFactsInspectionKind::MaterializationBasis,
                "projection_consumed.materialization_basis",
                basis.materialization_basis_identity(),
            ),
        ];
        rows.extend(
            basis
                .projection_receipts()
                .iter()
                .enumerate()
                .map(|(index, receipt)| {
                    row(
                        ProjectionConsumedPlanarFactsInspectionKind::ProjectionReceipt,
                        format!("projection_consumed.projection.{index}"),
                        receipt.fact_digest(),
                    )
                }),
        );
        rows
    }

    pub fn kind(&self) -> ProjectionConsumedPlanarFactsInspectionKind {
        self.kind
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn row(
    kind: ProjectionConsumedPlanarFactsInspectionKind,
    locus: impl Into<String>,
    value: impl ToString,
) -> ProjectionConsumedPlanarFactsInspectionRow {
    ProjectionConsumedPlanarFactsInspectionRow {
        kind,
        locus: locus.into(),
        value: value.to_string(),
    }
}
