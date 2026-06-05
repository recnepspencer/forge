use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::proof_grade::PrimitiveConstructionProofSubject;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionTruthProjectionRow {
    subject: PrimitiveConstructionProofSubject,
    canonical_truth_type: &'static str,
    projections: &'static [&'static str],
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
    let rows = vec![
        PrimitiveConstructionTruthProjectionRow {
            subject: PrimitiveConstructionProofSubject::CompoundParity,
            canonical_truth_type: "PrimitiveConstructionCompoundParityCanonicalTruth",
            projections: &[
                "ordering_report",
                "motion_parity_report",
                "grazing_boundary_report",
                "exhaustion_witness_parity_report",
            ],
        },
        PrimitiveConstructionTruthProjectionRow {
            subject: PrimitiveConstructionProofSubject::GeometryDigestSensitivity,
            canonical_truth_type: "PrimitiveGeometryIdentityBundle",
            projections: &[
                "PrimitiveRealizationReport.geometry_digest",
                "SpatialConstructionBirthPlan.realization_geometry_digest",
                "TopologyPrimitiveConstructionQueryBirthSynopsis.source_birth_digest",
            ],
        },
        PrimitiveConstructionTruthProjectionRow {
            subject: PrimitiveConstructionProofSubject::CanonicalWitnessParity,
            canonical_truth_type: "PrimitiveCanonicalWitnessGeometry",
            projections: &[
                "kernel family birth witness geometry",
                "geom simplex realization geometry",
                "shared canonical witness substrate",
            ],
        },
        PrimitiveConstructionTruthProjectionRow {
            subject: PrimitiveConstructionProofSubject::ShellWithHoleLayoutHostility,
            canonical_truth_type: "ShellWithHoleWitnessLayout",
            projections: &["planar containment proof", "planar non-overlap proof"],
        },
        PrimitiveConstructionTruthProjectionRow {
            subject: PrimitiveConstructionProofSubject::SimplexCanonicalRatio,
            canonical_truth_type: "SimplexCanonicalWitnessDefinition",
            projections: &["shared canonical simplex ratio surface"],
        },
    ];
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
