use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use crate::construction::certification::{
    TopologyConstructionCertificationPlan, TopologyConstructionCertificationReadSurface,
    TopologyConstructionInspectionSurface,
};
use crate::construction::lowering::TopologyConstructionLoweringPlan;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionFactKind {
    VertexBirth,
    EdgeBirth,
    LoopMembership,
    WireMembership,
    FaceMembership,
    ShellMembership,
    BodyMembership,
}

impl TopologyConstructionFactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VertexBirth => "vertex-birth",
            Self::EdgeBirth => "edge-birth",
            Self::LoopMembership => "loop-membership",
            Self::WireMembership => "wire-membership",
            Self::FaceMembership => "face-membership",
            Self::ShellMembership => "shell-membership",
            Self::BodyMembership => "body-membership",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConstructionFactProvenance {
    EquivalentProjectionConsumptionFacts,
}

impl TopologyConstructionFactProvenance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EquivalentProjectionConsumptionFacts => {
                "equivalent typed facts from inspection-backed projection consumption"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConstructionFactRow {
    kind: TopologyConstructionFactKind,
    topology_birth_class: String,
    source_birth_digest: String,
    fact_count: usize,
    row_digest: String,
}

impl TopologyConstructionFactRow {
    fn new(
        kind: TopologyConstructionFactKind,
        topology_birth_class: &str,
        source_birth_digest: &str,
        fact_count: usize,
    ) -> Self {
        let row_digest = digest_parts(&[
            kind.as_str().to_string(),
            topology_birth_class.to_string(),
            source_birth_digest.to_string(),
            fact_count.to_string(),
        ]);
        Self {
            kind,
            topology_birth_class: topology_birth_class.to_string(),
            source_birth_digest: source_birth_digest.to_string(),
            fact_count,
            row_digest,
        }
    }

    pub fn kind(&self) -> TopologyConstructionFactKind {
        self.kind
    }

    pub fn fact_count(&self) -> usize {
        self.fact_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConstructionFactReport {
    source_lowering_digest: String,
    source_execution_digest: String,
    source_certification_digest: String,
    source_birth_digest: String,
    topology_birth_class: String,
    provenance: TopologyConstructionFactProvenance,
    certification_scope: String,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    read_surface: TopologyConstructionCertificationReadSurface,
    inspection_surface: TopologyConstructionInspectionSurface,
    rows: Vec<TopologyConstructionFactRow>,
    report_digest: String,
}

impl TopologyConstructionFactReport {
    fn new(
        lowering: &TopologyConstructionLoweringPlan,
        certification: &TopologyConstructionCertificationPlan,
    ) -> Self {
        let provenance = TopologyConstructionFactProvenance::EquivalentProjectionConsumptionFacts;
        let rows = vec![
            TopologyConstructionFactRow::new(
                TopologyConstructionFactKind::VertexBirth,
                lowering.topology_birth_class(),
                lowering.source_birth_digest(),
                lowering.expected_vertex_births(),
            ),
            TopologyConstructionFactRow::new(
                TopologyConstructionFactKind::EdgeBirth,
                lowering.topology_birth_class(),
                lowering.source_birth_digest(),
                lowering.expected_edge_births(),
            ),
            TopologyConstructionFactRow::new(
                TopologyConstructionFactKind::LoopMembership,
                lowering.topology_birth_class(),
                lowering.source_birth_digest(),
                lowering.expected_loop_births(),
            ),
            TopologyConstructionFactRow::new(
                TopologyConstructionFactKind::WireMembership,
                lowering.topology_birth_class(),
                lowering.source_birth_digest(),
                lowering.expected_wire_births(),
            ),
            TopologyConstructionFactRow::new(
                TopologyConstructionFactKind::FaceMembership,
                lowering.topology_birth_class(),
                lowering.source_birth_digest(),
                lowering.expected_face_births(),
            ),
            TopologyConstructionFactRow::new(
                TopologyConstructionFactKind::ShellMembership,
                lowering.topology_birth_class(),
                lowering.source_birth_digest(),
                lowering.expected_shell_births(),
            ),
            TopologyConstructionFactRow::new(
                TopologyConstructionFactKind::BodyMembership,
                lowering.topology_birth_class(),
                lowering.source_birth_digest(),
                lowering.expected_body_births(),
            ),
        ];
        let mut parts = vec![
            lowering.lowering_digest().to_string(),
            certification.source_execution_digest().to_string(),
            certification.certification_digest().to_string(),
            lowering.source_birth_digest().to_string(),
            lowering.topology_birth_class().to_string(),
            provenance.as_str().to_string(),
            certification.certification_scope().to_string(),
        ];
        parts.extend(
            certification
                .required_query_families()
                .iter()
                .map(|family| format!("required-query-family:{family:?}")),
        );
        parts.extend([
            certification.read_surface().as_str().to_string(),
            certification.inspection_surface().as_str().to_string(),
        ]);
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        Self {
            source_lowering_digest: lowering.lowering_digest().to_string(),
            source_execution_digest: certification.source_execution_digest().to_string(),
            source_certification_digest: certification.certification_digest().to_string(),
            source_birth_digest: lowering.source_birth_digest().to_string(),
            topology_birth_class: lowering.topology_birth_class().to_string(),
            provenance,
            certification_scope: certification.certification_scope().to_string(),
            required_query_families: certification.required_query_families().to_vec(),
            read_surface: certification.read_surface(),
            inspection_surface: certification.inspection_surface(),
            rows,
            report_digest: digest_parts(&parts),
        }
    }

    pub fn provenance(&self) -> TopologyConstructionFactProvenance {
        self.provenance
    }

    pub fn certification_scope(&self) -> &str {
        &self.certification_scope
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn read_surface(&self) -> TopologyConstructionCertificationReadSurface {
        self.read_surface
    }

    pub fn inspection_surface(&self) -> TopologyConstructionInspectionSurface {
        self.inspection_surface
    }

    pub fn rows(&self) -> &[TopologyConstructionFactRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        kind: TopologyConstructionFactKind,
    ) -> Option<&TopologyConstructionFactRow> {
        self.rows.iter().find(|row| row.kind() == kind)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn build_topology_construction_fact_report(
    lowering: &TopologyConstructionLoweringPlan,
    certification: &TopologyConstructionCertificationPlan,
) -> TopologyConstructionFactReport {
    TopologyConstructionFactReport::new(lowering, certification)
}

fn digest_parts(parts: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::{
        build_topology_construction_fact_report, TopologyConstructionFactKind,
        TopologyConstructionFactProvenance,
    };
    use crate::construction::{
        prepare_primitive_construction_certification, TopologyConstructionExecutionPlan,
        TopologyConstructionLoweringPlan,
    };
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

    #[test]
    fn fact_report_binds_body_membership_to_projection_consumption_surface() {
        let lowering = TopologyConstructionLoweringPlan::new_for_tests(
            "shell-birth",
            "planar_shell_with_hole_body",
            7,
            7,
            2,
            0,
            1,
            1,
            1,
        );
        let execution = TopologyConstructionExecutionPlan::new_for_tests("shell-lowering");
        let certification = prepare_primitive_construction_certification(&execution);
        let report = build_topology_construction_fact_report(&lowering, &certification);

        assert_eq!(
            report
                .row_for(TopologyConstructionFactKind::BodyMembership)
                .expect("body row")
                .fact_count(),
            1
        );
        assert_eq!(
            report.provenance(),
            TopologyConstructionFactProvenance::EquivalentProjectionConsumptionFacts
        );
        assert_eq!(
            report.certification_scope(),
            "worth-topo.construction-certification"
        );
        assert_eq!(
            report.required_query_families(),
            &[ForgeQueryRuntimeFacadeFamily::Inspect]
        );
        assert!(!report.report_digest().is_empty());
    }
}
