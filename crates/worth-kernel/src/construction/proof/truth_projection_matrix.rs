use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::PrimitiveConstructionProofSubject;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionTruthProjectionRow {
    subject: PrimitiveConstructionProofSubject,
    canonical_truth_type: &'static str,
    projections: &'static [&'static str],
}

impl PrimitiveConstructionTruthProjectionRow {
    pub fn subject(&self) -> PrimitiveConstructionProofSubject {
        self.subject
    }
    pub fn canonical_truth_type(&self) -> &'static str {
        self.canonical_truth_type
    }
    pub fn projections(&self) -> &'static [&'static str] {
        self.projections
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionTruthProjectionMatrix {
    rows: Vec<PrimitiveConstructionTruthProjectionRow>,
    report_digest: String,
}

impl PrimitiveConstructionTruthProjectionMatrix {
    pub fn rows(&self) -> &[PrimitiveConstructionTruthProjectionRow] {
        &self.rows
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_truth_projection_matrix(
) -> PrimitiveConstructionTruthProjectionMatrix {
    let rows = vec![PrimitiveConstructionTruthProjectionRow {
        subject: PrimitiveConstructionProofSubject::CompoundParity,
        canonical_truth_type: "PrimitiveConstructionCompoundParityCanonicalTruth",
        projections: &[
            "ordering_report",
            "motion_parity_report",
            "grazing_boundary_report",
            "exhaustion_witness_parity_report",
        ],
    }];
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .flat_map(|row| {
                std::iter::once(row.subject.as_str().to_string())
                    .chain(std::iter::once(row.canonical_truth_type.to_string()))
                    .chain(
                        row.projections
                            .iter()
                            .map(|projection| (*projection).to_string()),
                    )
            })
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionTruthProjectionMatrix {
        rows,
        report_digest,
    }
}
