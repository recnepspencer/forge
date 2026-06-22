use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryEffectPolicy,
    ForgeQueryPreviewOptions, ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily,
    ForgeQuerySessionLabel, ForgeQueryWorkspace,
};
use worth_geom::facade::{
    PrimitiveNormalizationDisposition, PrimitiveRealizationExhaustionReason,
    PrimitiveRealizationStrategy, PrimitiveStabilityClass, PrimitiveSupportNormalClass,
};

use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::evidence_reports::sealed_report_identity;
use crate::construction::tests::support::runtime_truth::{
    prepare_primitive_construction_certification_runtime_truth,
    PrimitiveConstructionCertificationRuntimeTruth,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct BranchPreviewBasisCapture {
    branch_preview_contract_digest: String,
    preview_admission_digest: String,
    preview_effect_policy: ForgeQueryEffectPolicy,
    preview_authority_lane: ForgeQueryAuthorityLane,
    branch_admission_digest: String,
    branch_effect_policy: ForgeQueryEffectPolicy,
    branch_authority_lane: ForgeQueryAuthorityLane,
}

#[derive(Debug)]
pub(crate) enum BranchPreviewBasisError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for BranchPreviewBasisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for BranchPreviewBasisError {}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BranchPreviewBasisReport {
    basis_capture: BranchPreviewBasisCapture,
    runtime_truth: PrimitiveConstructionCertificationRuntimeTruth,
}

impl BranchPreviewBasisReport {
    fn new(
        basis_capture: BranchPreviewBasisCapture,
        runtime_truth: PrimitiveConstructionCertificationRuntimeTruth,
    ) -> Self {
        Self {
            basis_capture,
            runtime_truth,
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.runtime_truth.family()
    }

    pub(crate) fn branch_preview_contract_digest(&self) -> &str {
        &self.basis_capture.branch_preview_contract_digest
    }

    pub(crate) fn preview_admission_digest(&self) -> &str {
        &self.basis_capture.preview_admission_digest
    }

    pub(crate) fn branch_admission_digest(&self) -> &str {
        &self.basis_capture.branch_admission_digest
    }

    pub(crate) fn realization_strategy(&self) -> Option<PrimitiveRealizationStrategy> {
        match &self.runtime_truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
                Some(outcome.realization_strategy())
            }
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(_) => None,
        }
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        match &self.runtime_truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
                outcome.attempted_realization_strategies()
            }
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
                rejected.attempted_realization_strategies()
            }
        }
    }

    pub(crate) fn stability_class(&self) -> Option<PrimitiveStabilityClass> {
        match &self.runtime_truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
                Some(outcome.stability_class())
            }
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
                rejected.stability_class()
            }
        }
    }

    pub(crate) fn support_normal_class(&self) -> Option<PrimitiveSupportNormalClass> {
        match &self.runtime_truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
                Some(outcome.support_normal_class())
            }
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
                rejected.support_normal_class()
            }
        }
    }

    pub(crate) fn normalization_disposition(&self) -> Option<PrimitiveNormalizationDisposition> {
        match &self.runtime_truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => {
                Some(outcome.normalization_disposition())
            }
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
                rejected.normalization_disposition()
            }
        }
    }

    pub(crate) fn exhaustion_reason(&self) -> Option<PrimitiveRealizationExhaustionReason> {
        match &self.runtime_truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(_) => None,
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
                rejected.exhaustion_reason()
            }
        }
    }

    pub(crate) fn parity_verified(&self) -> bool {
        self.basis_capture.preview_effect_policy == self.basis_capture.branch_effect_policy
            && self.basis_capture.preview_authority_lane == ForgeQueryAuthorityLane::PreviewTruth
            && self.basis_capture.branch_authority_lane == ForgeQueryAuthorityLane::BranchLocalTruth
    }

    pub(crate) fn report_digest(&self) -> String {
        let (
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            support_normal_class,
            normalization_disposition,
            exhaustion_reason,
        ) = match &self.runtime_truth {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => (
                Some(outcome.realization_strategy().as_str().to_string()),
                outcome
                    .attempted_realization_strategies()
                    .iter()
                    .map(|value| value.as_str())
                    .collect::<Vec<_>>()
                    .join("->"),
                Some(outcome.stability_class().as_str().to_string()),
                Some(outcome.support_normal_class().as_str().to_string()),
                Some(outcome.normalization_disposition().as_str().to_string()),
                None,
            ),
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => (
                None,
                rejected
                    .attempted_realization_strategies()
                    .iter()
                    .map(|value| value.as_str())
                    .collect::<Vec<_>>()
                    .join("->"),
                rejected
                    .stability_class()
                    .map(|value| value.as_str().to_string()),
                rejected
                    .support_normal_class()
                    .map(|value| value.as_str().to_string()),
                rejected
                    .normalization_disposition()
                    .map(|value| value.as_str().to_string()),
                rejected
                    .exhaustion_reason()
                    .map(|value| value.as_str().to_string()),
            ),
        };
        sealed_report_identity(
            "worth-kernel.construction.branch-preview-basis",
            "branch-preview-basis",
            |report| {
                report
                    .shape_participating("family", self.runtime_truth.family().as_str())?
                    .value_participating(
                        "branch-preview-contract",
                        self.basis_capture.branch_preview_contract_digest.clone(),
                    )?
                    .value_participating(
                        "preview-admission",
                        self.basis_capture.preview_admission_digest.clone(),
                    )?
                    .value_participating(
                        "branch-admission",
                        self.basis_capture.branch_admission_digest.clone(),
                    )?
                    .optional_value_participating("realization-strategy", realization_strategy)?
                    .value_participating(
                        "attempted-realization-strategies",
                        attempted_realization_strategies,
                    )?
                    .optional_value_participating("stability-class", stability_class)?
                    .optional_value_participating("support-normal-class", support_normal_class)?
                    .optional_value_participating(
                        "normalization-disposition",
                        normalization_disposition,
                    )?
                    .optional_value_participating("exhaustion-reason", exhaustion_reason)?
                    .bool_participating("parity-verified", self.parity_verified())
            },
        )
    }
}

pub(crate) fn prepare_branch_preview_basis_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: PrimitiveConstructionIntent,
) -> Result<BranchPreviewBasisReport, BranchPreviewBasisError> {
    let runtime_truth =
        prepare_primitive_construction_certification_runtime_truth(intent.into_request());
    let basis_capture = capture_branch_preview_basis(workspace, runtime_truth.family())?;
    Ok(BranchPreviewBasisReport::new(basis_capture, runtime_truth))
}

fn capture_branch_preview_basis(
    workspace: &mut ForgeQueryWorkspace,
    family: PrimitiveConstructionFamily,
) -> Result<BranchPreviewBasisCapture, BranchPreviewBasisError> {
    let branch_preview_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .map_err(BranchPreviewBasisError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let (preview_admission_digest, preview_effect_policy, preview_authority_lane) = {
        let preview = workspace
            .preview_with_options(
                ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "preview"])
                    .expect("preview label"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .map_err(BranchPreviewBasisError::QueryRuntime)?;
        let preview_basis = preview.basis_admission();
        (
            preview_basis
                .admission_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            preview_basis.effect_policy(),
            preview_basis.authority_lane(),
        )
    };
    let (branch_admission_digest, branch_effect_policy, branch_authority_lane) = {
        let branch = workspace
            .branch_with_options(
                ForgeQuerySessionLabel::scoped_strs("worth-kernel", [family.as_str(), "branch"])
                    .expect("branch label"),
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .map_err(BranchPreviewBasisError::QueryRuntime)?;
        let branch_basis = branch.basis_admission();
        (
            branch_basis
                .admission_identity()
                .terminal_projection_for_reporting()
                .to_string(),
            branch_basis.effect_policy(),
            branch_basis.authority_lane(),
        )
    };

    Ok(BranchPreviewBasisCapture {
        branch_preview_contract_digest,
        preview_admission_digest,
        preview_effect_policy,
        preview_authority_lane,
        branch_admission_digest,
        branch_effect_policy,
        branch_authority_lane,
    })
}
