use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryReadSurface,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

use crate::construction::authoring::{
    primitive_construction_authoring, PrimitiveConstructionQueryEntryError,
    WorthKernelAuthorityError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::realization_truth::PrimitiveConstructionRuntimeRealizationTruth;
use crate::construction::PrimitiveConstructionAuthoringInput;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryProjectionConsumptionReceiptReport {
    family: crate::construction::request::PrimitiveConstructionFamily,
    query_contract_digest: String,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    read_surface: TopologyConstructionQueryReadSurface,
    inspection_surface: TopologyConstructionQueryInspectionSurface,
    fact_provenance: TopologyConstructionQueryFactProvenance,
    realization_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
    support_normal_class: Option<PrimitiveSupportNormalClass>,
    normalization_disposition: Option<PrimitiveNormalizationDisposition>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryProjectionConsumptionReceiptReport {
    fn new(
        query_contract_digest: String,
        prepared: &crate::construction::result::PreparedPrimitiveConstructionResult,
    ) -> Self {
        let topology_query_envelope = prepared.topology_query_handoff().topology_query_envelope();
        let realization_truth =
            PrimitiveConstructionRuntimeRealizationTruth::from_prepared_result(prepared);
        let parity_verified = topology_query_envelope.required_query_families()
            == [
                ForgeQueryRuntimeFacadeFamily::Write,
                ForgeQueryRuntimeFacadeFamily::Inspect,
            ]
            && topology_query_envelope.read_surface()
                == TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
            && topology_query_envelope.inspection_surface()
                == TopologyConstructionQueryInspectionSurface::InspectReceipt
            && topology_query_envelope.fact_provenance()
                == TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
            && prepared.topology_fact_digest() == topology_query_envelope.fact_digest()
            && !query_contract_digest.is_empty()
            && realization_truth.selected_strategy().is_some()
            && realization_truth.stability_class().is_some();
        let report_digest = digest_owned_parts(&[
            prepared.family().as_str().to_string(),
            query_contract_digest.clone(),
            topology_query_envelope
                .required_query_families()
                .iter()
                .map(|family| format!("{family:?}"))
                .collect::<Vec<_>>()
                .join("|"),
            topology_query_envelope.read_surface().as_str().to_string(),
            topology_query_envelope
                .inspection_surface()
                .as_str()
                .to_string(),
            topology_query_envelope
                .fact_provenance()
                .as_str()
                .to_string(),
            realization_truth.truth_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            family: prepared.family(),
            query_contract_digest,
            required_query_families: topology_query_envelope.required_query_families().to_vec(),
            read_surface: topology_query_envelope.read_surface(),
            inspection_surface: topology_query_envelope.inspection_surface(),
            fact_provenance: topology_query_envelope.fact_provenance(),
            realization_strategy: realization_truth.selected_strategy(),
            attempted_realization_strategies: realization_truth.attempted_strategies().to_vec(),
            stability_class: realization_truth.stability_class(),
            feature_conditioning_class: realization_truth.feature_conditioning_class(),
            support_normal_class: realization_truth.support_normal_class(),
            normalization_disposition: realization_truth.normalization_disposition(),
            parity_verified,
            report_digest,
        }
    }

    pub fn family(&self) -> crate::construction::request::PrimitiveConstructionFamily {
        self.family
    }

    pub fn query_contract_digest(&self) -> &str {
        &self.query_contract_digest
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.read_surface
    }

    pub fn inspection_surface(&self) -> TopologyConstructionQueryInspectionSurface {
        self.inspection_surface
    }

    pub fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.fact_provenance
    }

    pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.realization_strategy
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.stability_class
    }

    pub fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        self.feature_conditioning_class
    }

    pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.support_normal_class
    }

    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.normalization_disposition
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryProjectionConsumptionReceiptError {
    Authority(WorthKernelAuthorityError),
    QueryEntry(PrimitiveConstructionQueryEntryError),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionQueryProjectionConsumptionReceiptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => write!(f, "{error:?}"),
            Self::QueryEntry(error) => write!(f, "{error}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryProjectionConsumptionReceiptError {}

pub fn prepare_primitive_construction_query_projection_consumption_receipt_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: impl PrimitiveConstructionAuthoringInput,
) -> Result<
    PrimitiveConstructionQueryProjectionConsumptionReceiptReport,
    PrimitiveConstructionQueryProjectionConsumptionReceiptError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryProjectionConsumptionReceiptError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let prepared = {
        let mut session = primitive_construction_authoring(workspace)
            .map_err(PrimitiveConstructionQueryProjectionConsumptionReceiptError::Authority)?;
        session
            .author(intent)
            .map_err(PrimitiveConstructionQueryProjectionConsumptionReceiptError::QueryEntry)?
            .prepare_result()
            .map_err(PrimitiveConstructionQueryProjectionConsumptionReceiptError::QueryEntry)?
    };
    Ok(
        PrimitiveConstructionQueryProjectionConsumptionReceiptReport::new(
            query_contract_digest,
            &prepared,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_query_projection_consumption_receipt_report;
    use crate::construction::{
        PrimitiveConstructionFamily, PrimitiveConstructionIntent, RegularPrismSpec,
        RegularPyramidSpec,
    };
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyConstructionQueryFactProvenance,
        TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryReadSurface,
        TopologyRuntimeAdapters,
    };
    use worth_geom::facade::{
        PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
        PrimitiveSupportNormalClass,
    };

    #[test]
    fn projection_consumption_receipt_report_proves_the_sanctioned_query_read_story() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-projection-receipt".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_projection_consumption_receipt_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 6,
                radius: 1.0,
                height: 2.0,
            }),
        )
        .expect("projection consumption receipt report");

        assert_eq!(report.family(), PrimitiveConstructionFamily::RegularPrism);
        assert!(!report.query_contract_digest().is_empty());
        assert_eq!(
            report.required_query_families(),
            &[
                ForgeQueryRuntimeFacadeFamily::Write,
                ForgeQueryRuntimeFacadeFamily::Inspect,
            ]
        );
        assert_eq!(
            report.read_surface(),
            TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
        );
        assert_eq!(
            report.inspection_surface(),
            TopologyConstructionQueryInspectionSurface::InspectReceipt
        );
        assert_eq!(
            report.fact_provenance(),
            TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
        );
        assert_eq!(
            report.realization_strategy(),
            Some(PrimitiveRealizationStrategy::DirectWorld)
        );
        assert_eq!(
            report.stability_class(),
            Some(PrimitiveStabilityClass::StableDirect)
        );
        assert!(report.parity_verified());
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn projection_consumption_receipt_report_preserves_escalated_realization_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-projection-realization".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_projection_consumption_receipt_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0e-200,
                height: 1.0e-200,
            }),
        )
        .expect("projection consumption receipt report");

        assert_eq!(
            report.realization_strategy(),
            Some(PrimitiveRealizationStrategy::ExactSupport)
        );
        assert_eq!(
            report.attempted_realization_strategies(),
            &[
                PrimitiveRealizationStrategy::DirectWorld,
                PrimitiveRealizationStrategy::ExactSupport,
            ]
        );
        assert_eq!(
            report.stability_class(),
            Some(PrimitiveStabilityClass::StableAfterEscalation)
        );
        assert_eq!(
            report.support_normal_class(),
            Some(PrimitiveSupportNormalClass::Degenerate)
        );
        assert_eq!(
            report.normalization_disposition(),
            Some(PrimitiveNormalizationDisposition::LocalTransformationApplied)
        );
    }
}
