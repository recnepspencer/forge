use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::PrimitiveConstructionContinuityRow;
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionContinuityQueryInspectionSurface {
    ContinuitySurfaceReportReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionContinuityQueryReadSurface {
    ContinuityInspection,
    ProjectionConsumptionFromContinuitySurfaceReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionContinuityQueryFactProvenance {
    DirectContinuitySurfaceReport,
    EquivalentProjectionConsumptionFacts,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionQueryContinuityParityReport {
    case: crate::construction::certification::PrimitiveConstructionContinuityCase,
    profile_name: &'static str,
    source: crate::construction::certification::PrimitiveConstructionContinuityResolutionSource,
    continuity_class: worth_spatial::facade::arbitration::SpatialIdentityContinuityClass,
    explanation_class:
        worth_spatial::facade::arbitration::SpatialIdentityContinuityExplanationClass,
    candidate: Option<worth_spatial::facade::arbitration::SpatialIntentCandidate>,
    blocked_capability: Option<worth_spatial::facade::arbitration::SpatialBlockedCapability>,
    preserves_subject_identity: bool,
    preserves_anchor_identity: bool,
    query_contract_digest: String,
    read_surface: PrimitiveConstructionContinuityQueryReadSurface,
    inspection_surface: PrimitiveConstructionContinuityQueryInspectionSurface,
    fact_provenance: PrimitiveConstructionContinuityQueryFactProvenance,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryContinuityParityReport {
    fn new(
        query_contract_digest: String,
        row: PrimitiveConstructionContinuityRow,
        read_surface: PrimitiveConstructionContinuityQueryReadSurface,
        fact_provenance: PrimitiveConstructionContinuityQueryFactProvenance,
    ) -> Self {
        let inspection_surface =
            PrimitiveConstructionContinuityQueryInspectionSurface::ContinuitySurfaceReportReceipt;
        let parity_verified = !query_contract_digest.is_empty();
        let report_digest = digest_owned_parts(&[
            format!("{:?}", row.case()),
            row.profile_name().to_string(),
            format!("{:?}", row.source()),
            format!("{:?}", row.continuity_class()),
            format!("{:?}", row.explanation_class()),
            format!("{:?}", row.candidate()),
            format!("{:?}", row.blocked_capability()),
            row.preserves_subject_identity().to_string(),
            row.preserves_anchor_identity().to_string(),
            query_contract_digest.clone(),
            format!("{read_surface:?}"),
            format!("{inspection_surface:?}"),
            format!("{fact_provenance:?}"),
            parity_verified.to_string(),
        ]);
        Self {
            case: row.case(),
            profile_name: row.profile_name(),
            source: row.source(),
            continuity_class: row.continuity_class(),
            explanation_class: row.explanation_class(),
            candidate: row.candidate(),
            blocked_capability: row.blocked_capability(),
            preserves_subject_identity: row.preserves_subject_identity(),
            preserves_anchor_identity: row.preserves_anchor_identity(),
            query_contract_digest,
            read_surface,
            inspection_surface,
            fact_provenance,
            parity_verified,
            report_digest,
        }
    }

    pub fn case(&self) -> crate::construction::certification::PrimitiveConstructionContinuityCase {
        self.case
    }

    pub fn profile_name(&self) -> &'static str {
        self.profile_name
    }

    pub fn source(
        &self,
    ) -> crate::construction::certification::PrimitiveConstructionContinuityResolutionSource {
        self.source
    }

    pub fn continuity_class(
        &self,
    ) -> worth_spatial::facade::arbitration::SpatialIdentityContinuityClass {
        self.continuity_class
    }

    pub fn explanation_class(
        &self,
    ) -> worth_spatial::facade::arbitration::SpatialIdentityContinuityExplanationClass {
        self.explanation_class
    }

    pub fn candidate(&self) -> Option<worth_spatial::facade::arbitration::SpatialIntentCandidate> {
        self.candidate
    }

    pub fn blocked_capability(
        &self,
    ) -> Option<worth_spatial::facade::arbitration::SpatialBlockedCapability> {
        self.blocked_capability
    }

    pub fn preserves_subject_identity(&self) -> bool {
        self.preserves_subject_identity
    }

    pub fn preserves_anchor_identity(&self) -> bool {
        self.preserves_anchor_identity
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryContinuityParityError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionQueryContinuityParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryContinuityParityError {}

pub fn prepare_primitive_construction_query_continuity_inspection_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    row: PrimitiveConstructionContinuityRow,
) -> Result<
    PrimitiveConstructionQueryContinuityParityReport,
    PrimitiveConstructionQueryContinuityParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryContinuityParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryContinuityParityReport::new(
        query_contract_digest,
        row,
        PrimitiveConstructionContinuityQueryReadSurface::ContinuityInspection,
        PrimitiveConstructionContinuityQueryFactProvenance::DirectContinuitySurfaceReport,
    ))
}

pub fn prepare_primitive_construction_query_continuity_projection_consumption_receipt_report(
    workspace: &mut ForgeQueryWorkspace,
    row: PrimitiveConstructionContinuityRow,
) -> Result<
    PrimitiveConstructionQueryContinuityParityReport,
    PrimitiveConstructionQueryContinuityParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryContinuityParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryContinuityParityReport::new(
        query_contract_digest,
        row,
        PrimitiveConstructionContinuityQueryReadSurface::ProjectionConsumptionFromContinuitySurfaceReport,
        PrimitiveConstructionContinuityQueryFactProvenance::EquivalentProjectionConsumptionFacts,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_query_continuity_inspection_parity_report,
        prepare_primitive_construction_query_continuity_projection_consumption_receipt_report,
    };
    use crate::construction::{
        prepare_primitive_construction_continuity_surface_report,
        PrimitiveConstructionContinuityCase,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn query_continuity_parity_preserves_identity_classification() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-continuity".to_string(),
        )
        .expect("workspace");
        let report =
            prepare_primitive_construction_continuity_surface_report().expect("continuity report");
        let row = report
            .row(PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged)
            .expect("row")
            .clone();

        let inspection = prepare_primitive_construction_query_continuity_inspection_parity_report(
            &mut workspace,
            row.clone(),
        )
        .expect("inspection");
        let projection =
            prepare_primitive_construction_query_continuity_projection_consumption_receipt_report(
                &mut workspace,
                row,
            )
            .expect("projection");

        assert_eq!(inspection.continuity_class(), projection.continuity_class());
        assert_eq!(inspection.source(), projection.source());
        assert_eq!(inspection.candidate(), projection.candidate());
        assert_eq!(
            inspection.blocked_capability(),
            projection.blocked_capability()
        );
        assert_eq!(
            inspection.preserves_subject_identity(),
            projection.preserves_subject_identity()
        );
        assert_eq!(
            inspection.preserves_anchor_identity(),
            projection.preserves_anchor_identity()
        );
        assert_eq!(
            inspection.continuity_class(),
            worth_spatial::facade::arbitration::SpatialIdentityContinuityClass::IdentityMerged
        );
        assert!(inspection.parity_verified());
        assert!(projection.parity_verified());
    }
}
