use crate::bindings::primitive_birth_completeness::SpatialConstructionBirthCompletenessReport;
use crate::facade::PrimitiveConstructionBirthFamily;

use super::primitive_birth::digest_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConstructionBirthMappingKind {
    Vertex,
    Edge,
    Loop,
    Wire,
    Face,
    Shell,
    Body,
}

impl SpatialConstructionBirthMappingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Edge => "edge",
            Self::Loop => "loop",
            Self::Wire => "wire",
            Self::Face => "face",
            Self::Shell => "shell",
            Self::Body => "body",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConstructionBirthMappingRow {
    kind: SpatialConstructionBirthMappingKind,
    topology_birth_class: String,
    birth_digest: String,
    mapped_count: usize,
    support_plane_count: usize,
    row_digest: String,
}

impl SpatialConstructionBirthMappingRow {
    fn new(
        kind: SpatialConstructionBirthMappingKind,
        topology_birth_class: &str,
        birth_digest: &str,
        mapped_count: usize,
        support_plane_count: usize,
    ) -> Self {
        let row_digest = digest_parts(&[
            kind.as_str().to_string(),
            topology_birth_class.to_string(),
            birth_digest.to_string(),
            mapped_count.to_string(),
            support_plane_count.to_string(),
        ]);
        Self {
            kind,
            topology_birth_class: topology_birth_class.to_string(),
            birth_digest: birth_digest.to_string(),
            mapped_count,
            support_plane_count,
            row_digest,
        }
    }

    pub fn kind(&self) -> SpatialConstructionBirthMappingKind {
        self.kind
    }

    pub fn mapped_count(&self) -> usize {
        self.mapped_count
    }

    pub fn birth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConstructionBirthMappingReport {
    family: PrimitiveConstructionBirthFamily,
    topology_birth_class: String,
    birth_digest: String,
    completeness_digest: String,
    rows: Vec<SpatialConstructionBirthMappingRow>,
    report_digest: String,
}

impl SpatialConstructionBirthMappingReport {
    fn new(completeness: &SpatialConstructionBirthCompletenessReport) -> Self {
        let rows = vec![
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Vertex,
                completeness.topology_birth_class(),
                completeness.birth_digest(),
                completeness.supported_vertex_count(),
                completeness.support_plane_count(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Edge,
                completeness.topology_birth_class(),
                completeness.birth_digest(),
                completeness.supported_edge_count(),
                completeness.support_plane_count(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Loop,
                completeness.topology_birth_class(),
                completeness.birth_digest(),
                completeness.supported_loop_count(),
                completeness.support_plane_count(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Wire,
                completeness.topology_birth_class(),
                completeness.birth_digest(),
                completeness.supported_wire_count(),
                completeness.support_plane_count(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Face,
                completeness.topology_birth_class(),
                completeness.birth_digest(),
                completeness.supported_face_count(),
                completeness.support_plane_count(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Shell,
                completeness.topology_birth_class(),
                completeness.birth_digest(),
                completeness.supported_shell_count(),
                completeness.support_plane_count(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Body,
                completeness.topology_birth_class(),
                completeness.birth_digest(),
                completeness.supported_body_count(),
                completeness.support_plane_count(),
            ),
        ];
        let mut parts = vec![
            completeness.family().as_str().to_string(),
            completeness.topology_birth_class().to_string(),
            completeness.birth_digest().to_string(),
            completeness.completeness_digest().to_string(),
        ];
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        Self {
            family: completeness.family(),
            topology_birth_class: completeness.topology_birth_class().to_string(),
            birth_digest: completeness.birth_digest().to_string(),
            completeness_digest: completeness.completeness_digest().to_string(),
            rows,
            report_digest: digest_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionBirthFamily {
        self.family
    }

    pub fn birth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub fn completeness_digest(&self) -> &str {
        &self.completeness_digest
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn rows(&self) -> &[SpatialConstructionBirthMappingRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        kind: SpatialConstructionBirthMappingKind,
    ) -> Option<&SpatialConstructionBirthMappingRow> {
        self.rows.iter().find(|row| row.kind() == kind)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn build_primitive_construction_birth_mapping_report(
    completeness: &SpatialConstructionBirthCompletenessReport,
) -> SpatialConstructionBirthMappingReport {
    SpatialConstructionBirthMappingReport::new(completeness)
}

#[cfg(test)]
mod tests {
    use crate::facade::{
        certify_primitive_construction_birth_completeness, plan_primitive_construction_birth,
        PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
    };
    use worth_geom::facade::Plane;

    use super::{
        build_primitive_construction_birth_mapping_report, SpatialConstructionBirthMappingKind,
    };

    #[test]
    fn birth_mapping_report_binds_shell_counts_to_birth_digest() {
        let input = PrimitiveConstructionBirthScaffoldInput::new(
            PrimitiveConstructionBirthFamily::ShellWithHole,
            "planar_shell_with_hole_body",
            "shell-scaffold".to_string(),
            vec![plane()],
            vec![
                [2.0, 0.0, 0.0],
                [0.0, 2.0, 0.0],
                [-2.0, 0.0, 0.0],
                [0.0, -2.0, 0.0],
                [0.5, 0.0, 0.0],
                [0.0, 0.5, 0.0],
                [-0.5, 0.0, 0.0],
            ],
            7,
            7,
            2,
            0,
            1,
            1,
            1,
        );
        let plan = plan_primitive_construction_birth(input.clone()).expect("birth plan");
        let completeness =
            certify_primitive_construction_birth_completeness(&input, &plan).expect("complete");
        let report = build_primitive_construction_birth_mapping_report(&completeness);

        assert_eq!(
            report.family(),
            PrimitiveConstructionBirthFamily::ShellWithHole
        );
        assert_eq!(report.topology_birth_class(), "planar_shell_with_hole_body");
        assert_eq!(report.birth_digest(), plan.birth_digest());
        assert_eq!(report.rows().len(), 7);
        assert_eq!(
            report
                .row_for(SpatialConstructionBirthMappingKind::Loop)
                .expect("loop row")
                .mapped_count(),
            2
        );
        assert_eq!(
            report
                .row_for(SpatialConstructionBirthMappingKind::Face)
                .expect("face row")
                .birth_digest(),
            plan.birth_digest()
        );
        assert!(!report.report_digest().is_empty());
    }

    fn plane() -> Plane {
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
    }
}
