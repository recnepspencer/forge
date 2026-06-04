use forge_query::facade::ForgeQueryAuthorityLane;
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

use crate::construction::authoring_input::PrimitiveConstructionAuthoringInput;
use crate::construction::digest::digest_owned_parts;
use crate::construction::runtime_basis::{
    prepare_primitive_construction_branch_preview_runtime_report,
    PrimitiveConstructionBranchPreviewRuntimeReport, PrimitiveConstructionRuntimeBasisError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryBasisPreviewParityReport {
    family: crate::construction::request::PrimitiveConstructionFamily,
    authority_chain_digest: String,
    query_gap_free: bool,
    branch_preview_contract_digest: String,
    preview_admission_digest: String,
    branch_admission_digest: String,
    realization_strategy: Option<PrimitiveRealizationStrategy>,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: Option<PrimitiveStabilityClass>,
    feature_conditioning_class: Option<PrimitiveFeatureConditioningClass>,
    support_normal_class: Option<PrimitiveSupportNormalClass>,
    normalization_disposition: Option<PrimitiveNormalizationDisposition>,
    exhaustion_reason: Option<PrimitiveRealizationExhaustionReason>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryBasisPreviewParityReport {
    fn new(runtime_report: &PrimitiveConstructionBranchPreviewRuntimeReport) -> Self {
        let query_gap_free = runtime_report
            .authority_chain_report()
            .query_gap_rows()
            .is_empty();
        let parity_verified = query_gap_free
            && runtime_report.preview_lane().effect_policy()
                == runtime_report.branch_lane().effect_policy()
            && runtime_report.preview_lane().authority_lane()
                == ForgeQueryAuthorityLane::PreviewTruth
            && runtime_report.branch_lane().authority_lane()
                == ForgeQueryAuthorityLane::BranchLocalTruth
            && runtime_report.realization_strategy().is_some()
            && runtime_report.stability_class().is_some();
        let report_digest = digest_owned_parts(&[
            runtime_report.family().as_str().to_string(),
            runtime_report
                .authority_chain_report()
                .report_digest()
                .to_string(),
            query_gap_free.to_string(),
            runtime_report.branch_preview_contract_digest().to_string(),
            runtime_report.preview_lane().admission_digest().to_string(),
            runtime_report.branch_lane().admission_digest().to_string(),
            runtime_report
                .realization_strategy()
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            runtime_report
                .attempted_realization_strategies()
                .iter()
                .map(|value| value.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            runtime_report
                .stability_class()
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            runtime_report
                .feature_conditioning_class()
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            runtime_report
                .support_normal_class()
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            runtime_report
                .normalization_disposition()
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            runtime_report
                .exhaustion_reason()
                .map(|value| value.as_str().to_string())
                .unwrap_or_default(),
            parity_verified.to_string(),
        ]);
        Self {
            family: runtime_report.family(),
            authority_chain_digest: runtime_report
                .authority_chain_report()
                .report_digest()
                .to_string(),
            query_gap_free,
            branch_preview_contract_digest: runtime_report
                .branch_preview_contract_digest()
                .to_string(),
            preview_admission_digest: runtime_report.preview_lane().admission_digest().to_string(),
            branch_admission_digest: runtime_report.branch_lane().admission_digest().to_string(),
            realization_strategy: runtime_report.realization_strategy(),
            attempted_realization_strategies: runtime_report
                .attempted_realization_strategies()
                .to_vec(),
            stability_class: runtime_report.stability_class(),
            feature_conditioning_class: runtime_report.feature_conditioning_class(),
            support_normal_class: runtime_report.support_normal_class(),
            normalization_disposition: runtime_report.normalization_disposition(),
            exhaustion_reason: runtime_report.exhaustion_reason(),
            parity_verified,
            report_digest,
        }
    }

    pub fn family(&self) -> crate::construction::request::PrimitiveConstructionFamily {
        self.family
    }

    pub fn branch_preview_contract_digest(&self) -> &str {
        &self.branch_preview_contract_digest
    }

    pub fn query_gap_free(&self) -> bool {
        self.query_gap_free
    }

    pub fn preview_admission_digest(&self) -> &str {
        &self.preview_admission_digest
    }

    pub fn branch_admission_digest(&self) -> &str {
        &self.branch_admission_digest
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

    pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.support_normal_class
    }

    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.normalization_disposition
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.exhaustion_reason
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_query_basis_preview_parity_report(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    intent: impl PrimitiveConstructionAuthoringInput,
) -> Result<
    PrimitiveConstructionQueryBasisPreviewParityReport,
    PrimitiveConstructionRuntimeBasisError,
> {
    let runtime_report =
        prepare_primitive_construction_branch_preview_runtime_report(workspace, intent)?;
    Ok(PrimitiveConstructionQueryBasisPreviewParityReport::new(
        &runtime_report,
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_query_basis_preview_parity_report;
    use crate::construction::intent::PrimitiveConstructionIntent;
    use crate::construction::request::PrimitiveConstructionFamily;
    use crate::construction::specs::{RegularPrismSpec, RegularPyramidSpec};
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_geom::facade::{
        PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
        PrimitiveSupportNormalClass,
    };

    #[test]
    fn query_basis_preview_parity_report_tracks_preview_and_branch_lane_alignment() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-basis-parity".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_basis_preview_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 5,
                radius: 1.0,
                height: 2.0,
            }),
        )
        .expect("basis preview parity report");

        assert_eq!(report.family(), PrimitiveConstructionFamily::RegularPyramid);
        assert!(report.parity_verified());
        assert_eq!(
            report.realization_strategy(),
            Some(PrimitiveRealizationStrategy::DirectWorld)
        );
        assert_eq!(
            report.stability_class(),
            Some(PrimitiveStabilityClass::StableDirect)
        );
        assert!(!report.branch_preview_contract_digest().is_empty());
        assert!(!report.preview_admission_digest().is_empty());
        assert!(!report.branch_admission_digest().is_empty());
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn query_basis_preview_parity_report_changes_digest_when_request_family_changes() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-basis-report-digest-drift".to_string(),
        )
        .expect("workspace");
        let prism = prepare_primitive_construction_query_basis_preview_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 6,
                radius: 1.0,
                height: 2.0,
            }),
        )
        .expect("prism basis report");
        let pyramid = prepare_primitive_construction_query_basis_preview_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 5,
                radius: 1.0,
                height: 2.0,
            }),
        )
        .expect("pyramid basis report");

        assert_ne!(prism.report_digest(), pyramid.report_digest());
    }

    #[test]
    fn query_basis_preview_parity_report_preserves_escalated_realization_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-basis-realization-truth".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_basis_preview_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0e-200,
                height: 1.0e-200,
            }),
        )
        .expect("basis preview parity report");

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
        assert_eq!(report.exhaustion_reason(), None);
    }

    #[test]
    fn query_basis_preview_parity_report_preserves_world_collapsed_salvage_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-basis-exhaustion-truth".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_basis_preview_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            })
            .at([1.0e308, 1.0e308, 1.0e308]),
        )
        .expect("basis preview parity report");

        assert_eq!(
            report.realization_strategy(),
            Some(PrimitiveRealizationStrategy::DirectWorld)
        );
        assert_eq!(
            report.attempted_realization_strategies(),
            &[PrimitiveRealizationStrategy::DirectWorld]
        );
        assert_eq!(
            report.stability_class(),
            Some(PrimitiveStabilityClass::StableDirect)
        );
        assert_eq!(
            report.normalization_disposition(),
            Some(PrimitiveNormalizationDisposition::WorldSpaceSufficient)
        );
        assert_eq!(report.exhaustion_reason(), None);
    }
}
