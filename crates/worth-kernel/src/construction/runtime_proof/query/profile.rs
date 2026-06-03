use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::PrimitiveConstructionPolicyProfileRow;
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyProfileQueryInspectionSurface {
    PolicyProfileSurfaceReportReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyProfileQueryReadSurface {
    PolicyProfileInspection,
    ProjectionConsumptionFromPolicyProfileSurfaceReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyProfileQueryFactProvenance {
    DirectPolicyProfileSurfaceReport,
    EquivalentProjectionConsumptionFacts,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionQueryPolicyProfileParityReport {
    case: crate::construction::certification::PrimitiveConstructionPolicyProfileCase,
    profile_name: &'static str,
    proximity_posture: worth_spatial::facade::arbitration::SpatialThresholdPosture,
    alignment_posture: worth_spatial::facade::arbitration::SpatialThresholdPosture,
    arbitration_posture: worth_spatial::facade::arbitration::SpatialArbitrationPosture,
    preview_richness: worth_spatial::facade::arbitration::SpatialPreviewRichness,
    representative_preview_case: crate::construction::PrimitiveConstructionPreviewCase,
    representative_continuity_case:
        Option<crate::construction::PrimitiveConstructionContinuityCase>,
    query_contract_digest: String,
    read_surface: PrimitiveConstructionPolicyProfileQueryReadSurface,
    inspection_surface: PrimitiveConstructionPolicyProfileQueryInspectionSurface,
    fact_provenance: PrimitiveConstructionPolicyProfileQueryFactProvenance,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryPolicyProfileParityReport {
    fn new(
        query_contract_digest: String,
        row: PrimitiveConstructionPolicyProfileRow,
        read_surface: PrimitiveConstructionPolicyProfileQueryReadSurface,
        fact_provenance: PrimitiveConstructionPolicyProfileQueryFactProvenance,
    ) -> Self {
        let inspection_surface =
            PrimitiveConstructionPolicyProfileQueryInspectionSurface::PolicyProfileSurfaceReportReceipt;
        let parity_verified = !query_contract_digest.is_empty();
        let report_digest = digest_owned_parts(&[
            format!("{:?}", row.case()),
            row.profile_name().to_string(),
            format!("{:?}", row.proximity_posture()),
            format!("{:?}", row.alignment_posture()),
            format!("{:?}", row.arbitration_posture()),
            format!("{:?}", row.preview_richness()),
            format!("{:?}", row.representative_preview_case()),
            format!("{:?}", row.representative_continuity_case()),
            query_contract_digest.clone(),
            format!("{read_surface:?}"),
            format!("{inspection_surface:?}"),
            format!("{fact_provenance:?}"),
            parity_verified.to_string(),
        ]);
        Self {
            case: row.case(),
            profile_name: row.profile_name(),
            proximity_posture: row.proximity_posture(),
            alignment_posture: row.alignment_posture(),
            arbitration_posture: row.arbitration_posture(),
            preview_richness: row.preview_richness(),
            representative_preview_case: row.representative_preview_case(),
            representative_continuity_case: row.representative_continuity_case(),
            query_contract_digest,
            read_surface,
            inspection_surface,
            fact_provenance,
            parity_verified,
            report_digest,
        }
    }

    pub fn case(
        &self,
    ) -> crate::construction::certification::PrimitiveConstructionPolicyProfileCase {
        self.case
    }

    pub fn profile_name(&self) -> &'static str {
        self.profile_name
    }

    pub fn proximity_posture(&self) -> worth_spatial::facade::arbitration::SpatialThresholdPosture {
        self.proximity_posture
    }

    pub fn alignment_posture(&self) -> worth_spatial::facade::arbitration::SpatialThresholdPosture {
        self.alignment_posture
    }

    pub fn arbitration_posture(
        &self,
    ) -> worth_spatial::facade::arbitration::SpatialArbitrationPosture {
        self.arbitration_posture
    }

    pub fn preview_richness(&self) -> worth_spatial::facade::arbitration::SpatialPreviewRichness {
        self.preview_richness
    }

    pub fn representative_preview_case(
        &self,
    ) -> crate::construction::PrimitiveConstructionPreviewCase {
        self.representative_preview_case
    }

    pub fn representative_continuity_case(
        &self,
    ) -> Option<crate::construction::PrimitiveConstructionContinuityCase> {
        self.representative_continuity_case
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryPolicyProfileParityError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionQueryPolicyProfileParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryPolicyProfileParityError {}

pub fn prepare_primitive_construction_query_policy_profile_inspection_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    row: PrimitiveConstructionPolicyProfileRow,
) -> Result<
    PrimitiveConstructionQueryPolicyProfileParityReport,
    PrimitiveConstructionQueryPolicyProfileParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryPolicyProfileParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryPolicyProfileParityReport::new(
        query_contract_digest,
        row,
        PrimitiveConstructionPolicyProfileQueryReadSurface::PolicyProfileInspection,
        PrimitiveConstructionPolicyProfileQueryFactProvenance::DirectPolicyProfileSurfaceReport,
    ))
}

pub fn prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report(
    workspace: &mut ForgeQueryWorkspace,
    row: PrimitiveConstructionPolicyProfileRow,
) -> Result<
    PrimitiveConstructionQueryPolicyProfileParityReport,
    PrimitiveConstructionQueryPolicyProfileParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryPolicyProfileParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryPolicyProfileParityReport::new(
        query_contract_digest,
        row,
        PrimitiveConstructionPolicyProfileQueryReadSurface::ProjectionConsumptionFromPolicyProfileSurfaceReport,
        PrimitiveConstructionPolicyProfileQueryFactProvenance::EquivalentProjectionConsumptionFacts,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_query_policy_profile_inspection_parity_report,
        prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report,
    };
    use crate::construction::{
        prepare_primitive_construction_policy_profile_report,
        PrimitiveConstructionPolicyProfileCase,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn query_policy_profile_parity_preserves_profile_posture_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-policy-profile".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_policy_profile_report();
        let row = report
            .row(PrimitiveConstructionPolicyProfileCase::BimHostFriendly)
            .expect("row")
            .clone();

        let inspection =
            prepare_primitive_construction_query_policy_profile_inspection_parity_report(
                &mut workspace,
                row.clone(),
            )
            .expect("inspection");
        let projection =
            prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report(
                &mut workspace,
                row,
            )
            .expect("projection");

        assert_eq!(inspection.profile_name(), projection.profile_name());
        assert_eq!(
            inspection.arbitration_posture(),
            projection.arbitration_posture()
        );
        assert_eq!(
            inspection.representative_continuity_case(),
            projection.representative_continuity_case()
        );
        assert!(inspection.parity_verified());
        assert!(projection.parity_verified());
    }
}
