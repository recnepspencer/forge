use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedPlanarFactsInspectionKind {
    BooleanReadiness,
    StructuralIdentity,
    MotionPosture,
    TopologyContract,
    RetainedFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedPlanarFactsInspectionRow {
    kind: RetainedPlanarFactsInspectionKind,
    locus: String,
    value: String,
}

impl RetainedPlanarFactsInspectionRow {
    pub(crate) fn from_basis(basis: &RetainedPlanarFactsBasis) -> Vec<Self> {
        let mut rows = vec![
            row(
                RetainedPlanarFactsInspectionKind::BooleanReadiness,
                "retained.boolean_readiness.fact",
                basis.boolean_readiness_receipt().fact_digest(),
            ),
            row(
                RetainedPlanarFactsInspectionKind::StructuralIdentity,
                "retained.structural_identity.fact",
                basis
                    .structural_identity_receipt()
                    .structural_identity_digest(),
            ),
            row(
                RetainedPlanarFactsInspectionKind::MotionPosture,
                "retained.motion_posture.fact",
                basis.motion_posture_receipt().retained_motion_digest(),
            ),
            row(
                RetainedPlanarFactsInspectionKind::TopologyContract,
                "retained.topology_contract.fact",
                basis.topology_contract_receipt().fact_digest(),
            ),
        ];
        rows.extend(
            basis
                .boolean_readiness_receipt()
                .basis()
                .family_rows()
                .iter()
                .map(|family| {
                    row(
                        RetainedPlanarFactsInspectionKind::RetainedFamily,
                        format!("retained.family.{}", family.family().as_str()),
                        family.receipt_count().to_string(),
                    )
                }),
        );
        rows
    }

    pub fn kind(&self) -> RetainedPlanarFactsInspectionKind {
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
    kind: RetainedPlanarFactsInspectionKind,
    locus: impl Into<String>,
    value: impl ToString,
) -> RetainedPlanarFactsInspectionRow {
    RetainedPlanarFactsInspectionRow {
        kind,
        locus: locus.into(),
        value: value.to_string(),
    }
}
