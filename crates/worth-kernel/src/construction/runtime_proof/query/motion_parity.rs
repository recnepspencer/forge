use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::motion::{
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionReport,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionRequestedMotionWitness, PrimitiveConstructionResolvedMotionWitness,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionQueryInspectionSurface {
    MotionWitnessReportReceipt,
}

impl PrimitiveConstructionMotionQueryInspectionSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MotionWitnessReportReceipt => "motion_witness_report_receipt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionQueryReadSurface {
    MotionWitnessReportInspection,
    ProjectionConsumptionFromMotionWitnessReport,
}

impl PrimitiveConstructionMotionQueryReadSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MotionWitnessReportInspection => "motion_witness_report_inspection",
            Self::ProjectionConsumptionFromMotionWitnessReport => {
                "projection_consumption_from_motion_witness_report"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionQueryFactProvenance {
    DirectMotionWitnessReport,
    EquivalentProjectionConsumptionFacts,
}

impl PrimitiveConstructionMotionQueryFactProvenance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectMotionWitnessReport => "direct_motion_witness_report",
            Self::EquivalentProjectionConsumptionFacts => "equivalent_projection_consumption_facts",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionQueryMotionWitnessParityReport {
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    subject_family: crate::construction::PrimitiveConstructionFamily,
    anchor: worth_spatial::facade::refs::SpatialAnchorRef,
    requested_witness: PrimitiveConstructionRequestedMotionWitness,
    status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolved_witness: Option<PrimitiveConstructionResolvedMotionWitness>,
    resolution_class:
        Option<worth_spatial::facade::witness_resolution::SpatialWitnessResolutionClass>,
    failure_kind: Option<PrimitiveConstructionMotionWitnessResolutionFailureKind>,
    query_contract_digest: String,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    read_surface: PrimitiveConstructionMotionQueryReadSurface,
    inspection_surface: PrimitiveConstructionMotionQueryInspectionSurface,
    fact_provenance: PrimitiveConstructionMotionQueryFactProvenance,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryMotionWitnessParityReport {
    fn new(
        query_contract_digest: String,
        motion_report: PrimitiveConstructionMotionWitnessResolutionReport,
        read_surface: PrimitiveConstructionMotionQueryReadSurface,
        fact_provenance: PrimitiveConstructionMotionQueryFactProvenance,
    ) -> Self {
        let required_query_families = vec![ForgeQueryRuntimeFacadeFamily::Inspect];
        let inspection_surface =
            PrimitiveConstructionMotionQueryInspectionSurface::MotionWitnessReportReceipt;
        let parity_verified = !query_contract_digest.is_empty()
            && required_query_families == [ForgeQueryRuntimeFacadeFamily::Inspect]
            && matches!(
                (read_surface, fact_provenance),
                (
                    PrimitiveConstructionMotionQueryReadSurface::MotionWitnessReportInspection,
                    PrimitiveConstructionMotionQueryFactProvenance::DirectMotionWitnessReport
                ) | (
                    PrimitiveConstructionMotionQueryReadSurface::ProjectionConsumptionFromMotionWitnessReport,
                    PrimitiveConstructionMotionQueryFactProvenance::EquivalentProjectionConsumptionFacts
                )
            )
            && match motion_report.status() {
                PrimitiveConstructionMotionWitnessResolutionStatus::Admitted => {
                    motion_report.resolved_witness().is_some()
                        && motion_report.resolution_class().is_some()
                        && motion_report.failure_kind().is_none()
                }
                PrimitiveConstructionMotionWitnessResolutionStatus::Rejected => {
                    motion_report.resolved_witness().is_none()
                        && motion_report.resolution_class().is_none()
                        && motion_report.failure_kind().is_some()
                }
            };
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &[
                format!("{:?}", motion_report.kind()),
                motion_report.subject_family().as_str().to_string(),
                format!("{:?}", motion_report.anchor()),
                format!("{:?}", motion_report.requested_witness()),
                format!("{:?}", motion_report.status()),
                format!("{:?}", motion_report.resolved_witness()),
                format!("{:?}", motion_report.resolution_class()),
                format!("{:?}", motion_report.failure_kind()),
                query_contract_digest.clone(),
                required_query_families
                    .iter()
                    .map(|family| format!("{family:?}"))
                    .collect::<Vec<_>>()
                    .join("|"),
                read_surface.as_str().to_string(),
                inspection_surface.as_str().to_string(),
                fact_provenance.as_str().to_string(),
                parity_verified.to_string(),
            ],
        );
        Self {
            kind: motion_report.kind(),
            subject_family: motion_report.subject_family(),
            anchor: motion_report.anchor().clone(),
            requested_witness: motion_report.requested_witness().clone(),
            status: motion_report.status(),
            resolved_witness: motion_report.resolved_witness(),
            resolution_class: motion_report.resolution_class(),
            failure_kind: motion_report.failure_kind(),
            query_contract_digest,
            required_query_families,
            read_surface,
            inspection_surface,
            fact_provenance,
            parity_verified,
            report_digest,
        }
    }

    pub fn kind(&self) -> PrimitiveConstructionMotionWitnessResolutionKind {
        self.kind
    }

    pub fn subject_family(&self) -> crate::construction::PrimitiveConstructionFamily {
        self.subject_family
    }

    pub fn anchor(&self) -> &worth_spatial::facade::refs::SpatialAnchorRef {
        &self.anchor
    }

    pub fn requested_witness(&self) -> &PrimitiveConstructionRequestedMotionWitness {
        &self.requested_witness
    }

    pub fn status(&self) -> PrimitiveConstructionMotionWitnessResolutionStatus {
        self.status
    }

    pub fn resolved_witness(&self) -> Option<PrimitiveConstructionResolvedMotionWitness> {
        self.resolved_witness
    }

    pub fn resolution_class(
        &self,
    ) -> Option<worth_spatial::facade::witness_resolution::SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<PrimitiveConstructionMotionWitnessResolutionFailureKind> {
        self.failure_kind
    }

    #[cfg(test)]
    pub fn query_contract_digest(&self) -> &str {
        &self.query_contract_digest
    }

    #[cfg(test)]
    pub fn read_surface(&self) -> PrimitiveConstructionMotionQueryReadSurface {
        self.read_surface
    }

    #[cfg(test)]
    pub fn inspection_surface(&self) -> PrimitiveConstructionMotionQueryInspectionSurface {
        self.inspection_surface
    }

    #[cfg(test)]
    pub fn fact_provenance(&self) -> PrimitiveConstructionMotionQueryFactProvenance {
        self.fact_provenance
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryMotionWitnessParityError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionQueryMotionWitnessParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryMotionWitnessParityError {}

pub fn prepare_primitive_construction_query_motion_inspection_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    motion_report: PrimitiveConstructionMotionWitnessResolutionReport,
) -> Result<
    PrimitiveConstructionQueryMotionWitnessParityReport,
    PrimitiveConstructionQueryMotionWitnessParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryMotionWitnessParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryMotionWitnessParityReport::new(
        query_contract_digest,
        motion_report,
        PrimitiveConstructionMotionQueryReadSurface::MotionWitnessReportInspection,
        PrimitiveConstructionMotionQueryFactProvenance::DirectMotionWitnessReport,
    ))
}

pub fn prepare_primitive_construction_query_motion_projection_consumption_receipt_report(
    workspace: &mut ForgeQueryWorkspace,
    motion_report: PrimitiveConstructionMotionWitnessResolutionReport,
) -> Result<
    PrimitiveConstructionQueryMotionWitnessParityReport,
    PrimitiveConstructionQueryMotionWitnessParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryMotionWitnessParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryMotionWitnessParityReport::new(
        query_contract_digest,
        motion_report,
        PrimitiveConstructionMotionQueryReadSurface::ProjectionConsumptionFromMotionWitnessReport,
        PrimitiveConstructionMotionQueryFactProvenance::EquivalentProjectionConsumptionFacts,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_query_motion_inspection_parity_report,
        prepare_primitive_construction_query_motion_projection_consumption_receipt_report,
        PrimitiveConstructionMotionQueryFactProvenance,
        PrimitiveConstructionMotionQueryInspectionSurface,
        PrimitiveConstructionMotionQueryReadSurface,
    };
    use crate::construction::certification::motion::{
        prepare_primitive_construction_move_witness_resolution_report,
        prepare_primitive_construction_rotate_witness_resolution_report,
        PrimitiveConstructionMotionWitnessResolutionFailureKind,
        PrimitiveConstructionMotionWitnessResolutionKind,
        PrimitiveConstructionMotionWitnessResolutionStatus,
    };
    use crate::construction::intent::PrimitiveConstructionIntent;
    use crate::construction::specs::WireBodySpec;
    use crate::facade::authoring::intents::{MoveSpatialIntent, RotateSpatialIntent};
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_spatial::facade::refs::SpatialPointWitnessRef;

    #[test]
    fn query_motion_inspection_parity_report_preserves_admitted_witness_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-motion-inspection".to_string(),
        )
        .expect("workspace");
        let motion_report = prepare_primitive_construction_move_witness_resolution_report(
            MoveSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 6,
            }))
            .to_witness(SpatialPointWitnessRef::world_point([10.0, 0.0, 3.0])),
        );
        let report = prepare_primitive_construction_query_motion_inspection_parity_report(
            &mut workspace,
            motion_report,
        )
        .expect("query motion inspection parity");

        assert_eq!(
            report.kind(),
            PrimitiveConstructionMotionWitnessResolutionKind::Move
        );
        assert_eq!(
            report.status(),
            PrimitiveConstructionMotionWitnessResolutionStatus::Admitted
        );
        assert_eq!(
            report.read_surface(),
            PrimitiveConstructionMotionQueryReadSurface::MotionWitnessReportInspection
        );
        assert_eq!(
            report.inspection_surface(),
            PrimitiveConstructionMotionQueryInspectionSurface::MotionWitnessReportReceipt
        );
        assert_eq!(
            report.fact_provenance(),
            PrimitiveConstructionMotionQueryFactProvenance::DirectMotionWitnessReport
        );
        assert!(report.parity_verified());
        assert!(!report.query_contract_digest().is_empty());
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn query_motion_projection_receipt_report_preserves_rejected_failure_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-motion-projection".to_string(),
        )
        .expect("workspace");
        let motion_report = prepare_primitive_construction_rotate_witness_resolution_report(
            RotateSpatialIntent::shape(PrimitiveConstructionIntent::wire_body(WireBodySpec {
                edge_count: 4,
            }))
            .around([0.0, 0.0, 1.0])
            .by_radians(f64::NAN),
        );
        let report =
            prepare_primitive_construction_query_motion_projection_consumption_receipt_report(
                &mut workspace,
                motion_report,
            )
            .expect("query motion projection parity");

        assert_eq!(
            report.kind(),
            PrimitiveConstructionMotionWitnessResolutionKind::Rotate
        );
        assert_eq!(
            report.status(),
            PrimitiveConstructionMotionWitnessResolutionStatus::Rejected
        );
        assert_eq!(
            report.failure_kind(),
            Some(PrimitiveConstructionMotionWitnessResolutionFailureKind::NonFiniteRotationAngle)
        );
        assert_eq!(
            report.read_surface(),
            PrimitiveConstructionMotionQueryReadSurface::ProjectionConsumptionFromMotionWitnessReport
        );
        assert_eq!(
            report.fact_provenance(),
            PrimitiveConstructionMotionQueryFactProvenance::EquivalentProjectionConsumptionFacts
        );
        assert!(report.parity_verified());
    }
}
