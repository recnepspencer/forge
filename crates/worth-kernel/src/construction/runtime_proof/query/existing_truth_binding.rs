use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;

const AUDITED_FILES: [(&str, &str); 5] = [
    (
        "worth-kernel.authoring",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/authoring.rs"
        )),
    ),
    (
        "worth-kernel.runtime-basis",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/runtime_basis.rs"
        )),
    ),
    (
        "worth-topo.lowering",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/lowering.rs"
        )),
    ),
    (
        "worth-topo.execution",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/execution.rs"
        )),
    ),
    (
        "worth-topo.certification",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-topo/src/construction/certification.rs"
        )),
    ),
];

const EXISTING_TRUTH_PATTERNS: [&str; 9] = [
    "bind_existing_entity(",
    "bind_existing_relation(",
    "update_existing(",
    "assert_existing(",
    "verify_existing(",
    "update_existing_verified(",
    "delete_existing(",
    "delete_existing_verified(",
    "probe_existing(",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionExistingTruthBindingPosture {
    NotRequiredForFreshPrimitiveBirth,
}

impl PrimitiveConstructionExistingTruthBindingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotRequiredForFreshPrimitiveBirth => "not_required_for_fresh_primitive_birth",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryExistingTruthBindingReport {
    family: PrimitiveConstructionFamily,
    posture: PrimitiveConstructionExistingTruthBindingPosture,
    audited_file_count: usize,
    forbidden_pattern_count: usize,
    report_digest: String,
}

impl PrimitiveConstructionQueryExistingTruthBindingReport {
    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn posture(&self) -> PrimitiveConstructionExistingTruthBindingPosture {
        self.posture
    }

    pub fn audited_file_count(&self) -> usize {
        self.audited_file_count
    }

    pub fn forbidden_pattern_count(&self) -> usize {
        self.forbidden_pattern_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_query_existing_truth_binding_report(
    intent: impl Into<PrimitiveConstructionIntent>,
) -> PrimitiveConstructionQueryExistingTruthBindingReport {
    let family = intent.into().family();
    let forbidden_pattern_count = AUDITED_FILES
        .iter()
        .flat_map(|(_, source)| {
            EXISTING_TRUTH_PATTERNS
                .iter()
                .map(|pattern| source.contains(pattern))
        })
        .filter(|found| *found)
        .count();
    let posture =
        PrimitiveConstructionExistingTruthBindingPosture::NotRequiredForFreshPrimitiveBirth;
    let report_digest = digest_owned_parts(&[
        family.as_str().to_string(),
        posture.as_str().to_string(),
        AUDITED_FILES.len().to_string(),
        forbidden_pattern_count.to_string(),
    ]);
    PrimitiveConstructionQueryExistingTruthBindingReport {
        family,
        posture,
        audited_file_count: AUDITED_FILES.len(),
        forbidden_pattern_count,
        report_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_query_existing_truth_binding_report,
        PrimitiveConstructionExistingTruthBindingPosture,
    };
    use crate::construction::{
        PrimitiveConstructionFamily, PrimitiveConstructionIntent, SimplexSolidSpec,
    };

    #[test]
    fn existing_truth_binding_report_proves_fresh_primitive_birth_avoids_existing_truth_flows() {
        let report = prepare_primitive_construction_query_existing_truth_binding_report(
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.0)),
        );

        assert_eq!(report.family(), PrimitiveConstructionFamily::SimplexSolid);
        assert_eq!(
            report.posture(),
            PrimitiveConstructionExistingTruthBindingPosture::NotRequiredForFreshPrimitiveBirth
        );
        assert_eq!(report.audited_file_count(), 5);
        assert_eq!(report.forbidden_pattern_count(), 0);
        assert!(!report.report_digest().is_empty());
    }
}
