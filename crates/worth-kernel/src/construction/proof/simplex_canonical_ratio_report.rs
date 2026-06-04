use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use worth_primitives::{canonical_simplex_vertices, CANONICAL_SIMPLEX_LATERAL_RATIO};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimplexCanonicalWitnessDefinition {
    scale: f64,
    lateral_ratio: f64,
}

impl SimplexCanonicalWitnessDefinition {
    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn lateral_ratio(&self) -> f64 {
        self.lateral_ratio
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimplexCanonicalRatioReport {
    definition: SimplexCanonicalWitnessDefinition,
    report_digest: String,
}

impl SimplexCanonicalRatioReport {
    pub fn definition(&self) -> SimplexCanonicalWitnessDefinition {
        self.definition
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_simplex_canonical_ratio_report() -> SimplexCanonicalRatioReport {
    let definition = SimplexCanonicalWitnessDefinition {
        scale: 2.0,
        lateral_ratio: CANONICAL_SIMPLEX_LATERAL_RATIO,
    };
    let vertices = canonical_simplex_vertices(definition.scale(), 0.0);
    let derived_ratio = vertices.local_vertices()[2][0].abs() / definition.scale();
    assert!(
        (derived_ratio - definition.lateral_ratio()).abs() <= f64::EPSILON,
        "shared simplex witness ratio drifted from the canonical ratio surface"
    );
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            definition.scale().to_bits().to_string(),
            definition.lateral_ratio().to_bits().to_string(),
            derived_ratio.to_bits().to_string(),
        ],
    );
    SimplexCanonicalRatioReport {
        definition,
        report_digest,
    }
}
