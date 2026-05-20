use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchBasisAdmission, ForgeQueryBranchOptions,
    ForgeQueryEffectPolicy, ForgeQueryPreviewBasisAdmission, ForgeQueryPreviewOptions,
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::authoring::{
    primitive_construction_authoring, PrimitiveConstructionAuthorityChainReport,
    WorthKernelAuthorityError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::outcome::{
    prepare_primitive_construction_outcome, PrimitiveConstructionPreparedOutcome,
};
use crate::construction::realization_truth::PrimitiveConstructionRuntimeRealizationTruth;
use crate::construction::{PrimitiveConstructionFamily, PrimitiveConstructionIntent};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionRuntimeBasisLaneReport {
    label: String,
    effect_policy: ForgeQueryEffectPolicy,
    authority_lane: ForgeQueryAuthorityLane,
    evidence: Vec<String>,
    admission_digest: String,
}

impl PrimitiveConstructionRuntimeBasisLaneReport {
    fn from_preview(admission: &ForgeQueryPreviewBasisAdmission) -> Self {
        Self::new(
            admission.label(),
            admission.effect_policy(),
            admission.authority_lane(),
            admission.evidence(),
        )
    }

    fn from_branch(admission: &ForgeQueryBranchBasisAdmission) -> Self {
        Self::new(
            admission.label(),
            admission.effect_policy(),
            admission.authority_lane(),
            admission.evidence(),
        )
    }

    fn new(
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        authority_lane: ForgeQueryAuthorityLane,
        evidence: &[String],
    ) -> Self {
        let admission_digest = digest_owned_parts(&[
            label.to_string(),
            effect_policy.to_string(),
            authority_lane.to_string(),
            evidence.join("|"),
        ]);
        Self {
            label: label.to_string(),
            effect_policy,
            authority_lane,
            evidence: evidence.to_vec(),
            admission_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionBranchPreviewRuntimeReport {
    family: PrimitiveConstructionFamily,
    authority_chain_report: PrimitiveConstructionAuthorityChainReport,
    branch_preview_contract_digest: String,
    outcome: PrimitiveConstructionPreparedOutcome,
    realization_truth: PrimitiveConstructionRuntimeRealizationTruth,
    preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    report_digest: String,
}

impl PrimitiveConstructionBranchPreviewRuntimeReport {
    fn new(
        family: PrimitiveConstructionFamily,
        authority_chain_report: PrimitiveConstructionAuthorityChainReport,
        branch_preview_contract_digest: String,
        outcome: PrimitiveConstructionPreparedOutcome,
        realization_truth: PrimitiveConstructionRuntimeRealizationTruth,
        preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
        branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    ) -> Self {
        let report_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            authority_chain_report.report_digest().to_string(),
            branch_preview_contract_digest.clone(),
            outcome.outcome_digest().to_string(),
            realization_truth.truth_digest().to_string(),
            preview_lane.admission_digest().to_string(),
            branch_lane.admission_digest().to_string(),
        ]);
        Self {
            family,
            authority_chain_report,
            branch_preview_contract_digest,
            outcome,
            realization_truth,
            preview_lane,
            branch_lane,
            report_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn authority_chain_report(&self) -> &PrimitiveConstructionAuthorityChainReport {
        &self.authority_chain_report
    }

    pub fn branch_preview_contract_digest(&self) -> &str {
        &self.branch_preview_contract_digest
    }

    pub fn outcome(&self) -> &PrimitiveConstructionPreparedOutcome {
        &self.outcome
    }

    pub fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        self.realization_truth.selected_strategy()
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.realization_truth.attempted_strategies()
    }

    pub fn attempted_realization_strategy_count(&self) -> usize {
        self.realization_truth.attempted_strategy_count()
    }

    pub fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        self.realization_truth.stability_class()
    }

    pub fn feature_conditioning_class(&self) -> Option<PrimitiveFeatureConditioningClass> {
        self.realization_truth.feature_conditioning_class()
    }

    pub fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        self.realization_truth.support_normal_class()
    }

    pub fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        self.realization_truth.normalization_disposition()
    }

    pub fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        self.realization_truth.exhaustion_reason()
    }

    pub fn preview_lane(&self) -> &PrimitiveConstructionRuntimeBasisLaneReport {
        &self.preview_lane
    }

    pub fn branch_lane(&self) -> &PrimitiveConstructionRuntimeBasisLaneReport {
        &self.branch_lane
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionRuntimeBasisError {
    Authority(WorthKernelAuthorityError),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionRuntimeBasisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => write!(f, "{error:?}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionRuntimeBasisError {}

pub fn prepare_primitive_construction_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: impl Into<PrimitiveConstructionIntent>,
) -> Result<PrimitiveConstructionBranchPreviewRuntimeReport, PrimitiveConstructionRuntimeBasisError>
{
    let intent = intent.into();
    let request = intent.request().clone();
    let authority_chain_report = {
        let session = primitive_construction_authoring(workspace)
            .map_err(PrimitiveConstructionRuntimeBasisError::Authority)?;
        session.authority_chain_report()
    };
    let branch_preview_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .map_err(PrimitiveConstructionRuntimeBasisError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let outcome = prepare_primitive_construction_outcome(intent);
    let realization_truth = PrimitiveConstructionRuntimeRealizationTruth::from_outcome(&outcome);
    let preview_lane = {
        let preview = workspace
            .preview_with_options(
                format!("worth-kernel.{}.preview", request.family().as_str()),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionRuntimeBasisError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_preview(preview.basis_admission())
    };
    let branch_lane = {
        let branch = workspace
            .branch_with_options(
                format!("worth-kernel.{}.branch", request.family().as_str()),
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionRuntimeBasisError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_branch(branch.basis_admission())
    };
    Ok(PrimitiveConstructionBranchPreviewRuntimeReport::new(
        request.family(),
        authority_chain_report,
        branch_preview_contract_digest,
        outcome,
        realization_truth,
        preview_lane,
        branch_lane,
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_branch_preview_runtime_report;
    use crate::construction::{
        PrimitiveConstructionFamily, PrimitiveConstructionIntent, RegularPyramidSpec,
    };
    use forge_query::facade::{ForgeQueryAuthorityLane, ForgeQueryEffectPolicy};
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_geom::facade::{
        PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
        PrimitiveSupportNormalClass,
    };

    #[test]
    fn branch_preview_runtime_report_opens_preview_and_branch_lanes() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.runtime-basis".to_string(),
        )
        .expect("workspace");

        let report = prepare_primitive_construction_branch_preview_runtime_report(
            &mut workspace,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 5,
                radius: 1.0,
                height: 2.0,
            }),
        )
        .expect("runtime basis report");

        assert_eq!(report.family(), PrimitiveConstructionFamily::RegularPyramid);
        assert!(report.authority_chain_report().query_gap_rows().is_empty());
        assert_eq!(
            report.preview_lane().authority_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );
        assert_eq!(
            report.branch_lane().authority_lane(),
            ForgeQueryAuthorityLane::BranchLocalTruth
        );
        assert_eq!(
            report.preview_lane().effect_policy(),
            ForgeQueryEffectPolicy::SandboxedWriteIntent
        );
        assert_eq!(
            report.branch_lane().effect_policy(),
            ForgeQueryEffectPolicy::SandboxedWriteIntent
        );
        assert_eq!(
            report.feature_conditioning_class(),
            Some(PrimitiveFeatureConditioningClass::Healthy)
        );
        assert_eq!(
            report.support_normal_class(),
            Some(PrimitiveSupportNormalClass::Robust)
        );
        assert_eq!(
            report.normalization_disposition(),
            Some(PrimitiveNormalizationDisposition::WorldSpaceSufficient)
        );
        assert!(!report.branch_preview_contract_digest().is_empty());
        assert!(!report.outcome().outcome_digest().is_empty());
        assert!(!report.report_digest().is_empty());
    }
}
