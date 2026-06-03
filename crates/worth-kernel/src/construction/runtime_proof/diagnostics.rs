use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::outcome::{
    prepare_primitive_construction_outcome, PrimitiveConstructionPreparedOutcome,
    PrimitiveConstructionRejectedOutcome,
};
use crate::construction::request::PrimitiveConstructionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionBlockingBoundary {
    KernelIntent,
    SpatialBirth,
    TopologyLegality,
    PrimitiveClassAdmission,
}

impl PrimitiveConstructionBlockingBoundary {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KernelIntent => "kernel_intent",
            Self::SpatialBirth => "spatial_birth",
            Self::TopologyLegality => "topology_legality",
            Self::PrimitiveClassAdmission => "primitive_class_admission",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRejectionLocalityRow {
    family: PrimitiveConstructionFamily,
    rejection_class: crate::construction::outcome::PrimitiveConstructionRejectionClass,
    rejection_locality: crate::construction::outcome::PrimitiveConstructionRejectionLocality,
    blocking_boundary: PrimitiveConstructionBlockingBoundary,
    topology_scope: String,
    spatial_birth_scope: String,
    failure_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionRejectionLocalityRow {
    fn from_rejected_outcome(rejected: &PrimitiveConstructionRejectedOutcome) -> Self {
        let blocking_boundary = match rejected.rejection_locality() {
            crate::construction::outcome::PrimitiveConstructionRejectionLocality::Admission => {
                PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission
            }
            crate::construction::outcome::PrimitiveConstructionRejectionLocality::Scaffold => {
                PrimitiveConstructionBlockingBoundary::KernelIntent
            }
            crate::construction::outcome::PrimitiveConstructionRejectionLocality::SpatialBirth => {
                PrimitiveConstructionBlockingBoundary::SpatialBirth
            }
            crate::construction::outcome::PrimitiveConstructionRejectionLocality::Execution => {
                PrimitiveConstructionBlockingBoundary::TopologyLegality
            }
        };
        let topology_scope = rejected.family().topology_birth_class().to_string();
        let spatial_birth_scope = match blocking_boundary {
            PrimitiveConstructionBlockingBoundary::SpatialBirth => {
                "worth-spatial.construction-birth-authority".to_string()
            }
            _ => "not_reached".to_string(),
        };
        let row_digest = digest_owned_parts(&[
            rejected.family().as_str().to_string(),
            rejected.rejection_class().as_str().to_string(),
            rejected.rejection_locality().as_str().to_string(),
            blocking_boundary.as_str().to_string(),
            topology_scope.clone(),
            spatial_birth_scope.clone(),
            rejected.failure_digest().to_string(),
        ]);
        Self {
            family: rejected.family(),
            rejection_class: rejected.rejection_class(),
            rejection_locality: rejected.rejection_locality(),
            blocking_boundary,
            topology_scope,
            spatial_birth_scope,
            failure_digest: rejected.failure_digest().to_string(),
            row_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn rejection_class(
        &self,
    ) -> crate::construction::outcome::PrimitiveConstructionRejectionClass {
        self.rejection_class
    }

    pub fn rejection_locality(
        &self,
    ) -> crate::construction::outcome::PrimitiveConstructionRejectionLocality {
        self.rejection_locality
    }

    pub fn blocking_boundary(&self) -> PrimitiveConstructionBlockingBoundary {
        self.blocking_boundary
    }

    pub fn topology_scope(&self) -> &str {
        &self.topology_scope
    }

    pub fn spatial_birth_scope(&self) -> &str {
        &self.spatial_birth_scope
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRejectionLocalityReport {
    accepted_count: usize,
    rejected_count: usize,
    rows: Vec<PrimitiveConstructionRejectionLocalityRow>,
    report_digest: String,
}

impl PrimitiveConstructionRejectionLocalityReport {
    pub fn accepted_count(&self) -> usize {
        self.accepted_count
    }

    pub fn rejected_count(&self) -> usize {
        self.rejected_count
    }

    pub fn rows(&self) -> &[PrimitiveConstructionRejectionLocalityRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_rejection_locality_report(
    intents: impl IntoIterator<Item = PrimitiveConstructionIntent>,
) -> PrimitiveConstructionRejectionLocalityReport {
    let mut accepted_count = 0;
    let mut rows = Vec::new();
    for intent in intents {
        match prepare_primitive_construction_outcome(intent) {
            PrimitiveConstructionPreparedOutcome::Accepted(_) => accepted_count += 1,
            PrimitiveConstructionPreparedOutcome::Rejected(rejected) => {
                rows.push(
                    PrimitiveConstructionRejectionLocalityRow::from_rejected_outcome(&rejected),
                );
            }
        }
    }
    let mut digest_parts = vec![
        format!("accepted-count:{accepted_count}"),
        format!("rejected-count:{}", rows.len()),
    ];
    digest_parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
    let report_digest = digest_owned_parts(&digest_parts);
    PrimitiveConstructionRejectionLocalityReport {
        accepted_count,
        rejected_count: rows.len(),
        rows,
        report_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_rejection_locality_report,
        PrimitiveConstructionBlockingBoundary,
    };
    use crate::construction::{
        OrthotopeSpec, PrimitiveConstructionFamily, PrimitiveConstructionIntent, ShellWithHoleSpec,
        WireBodySpec,
    };

    #[test]
    fn rejection_locality_report_tracks_mixed_accepted_and_rejected_workloads() {
        let report = prepare_primitive_construction_rejection_locality_report(vec![
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            }),
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 2,
                hole_loop_edge_counts: vec![3],
            }),
        ]);

        assert_eq!(report.accepted_count(), 1);
        assert_eq!(report.rejected_count(), 2);
        assert_eq!(
            report.rows()[0].family(),
            PrimitiveConstructionFamily::WireBody
        );
        assert_eq!(
            report.rows()[0].blocking_boundary(),
            PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission
        );
        assert_eq!(
            report.rows()[0].topology_scope(),
            PrimitiveConstructionFamily::WireBody.topology_birth_class()
        );
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn rejection_locality_digest_changes_when_accepted_count_changes() {
        let mixed = prepare_primitive_construction_rejection_locality_report(vec![
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            }),
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
        ]);
        let rejected_only = prepare_primitive_construction_rejection_locality_report(vec![
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
        ]);

        assert_eq!(mixed.rejected_count(), rejected_only.rejected_count());
        assert_ne!(mixed.accepted_count(), rejected_only.accepted_count());
        assert_ne!(mixed.report_digest(), rejected_only.report_digest());
    }

    #[test]
    fn rejection_locality_report_preserves_distinct_rows_for_same_boundary() {
        let report = prepare_primitive_construction_rejection_locality_report(vec![
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 2,
                hole_loop_edge_counts: vec![3],
            }),
        ]);

        assert_eq!(report.accepted_count(), 0);
        assert_eq!(report.rejected_count(), 2);
        assert_eq!(
            report.rows()[0].blocking_boundary(),
            PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission
        );
        assert_eq!(
            report.rows()[1].blocking_boundary(),
            PrimitiveConstructionBlockingBoundary::PrimitiveClassAdmission
        );
        assert_ne!(report.rows()[0].family(), report.rows()[1].family());
        assert_ne!(report.rows()[0].row_digest(), report.rows()[1].row_digest());
    }
}
