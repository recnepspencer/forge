use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAdmissionError,
    ForgeQueryDomainOperatingContext,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::bindings::{BindingContinuityClass, RebindingOutcomeClass};

use crate::binding::rebinding::{
    primitive_rebinding_replay_parity, PrimitiveRebindingAuthoringError,
    PrimitiveRebindingBranchLocalInspection, PrimitiveRebindingDeclarationEntry,
    PrimitiveRebindingHistoricalInspection, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingReplayParityError, PrimitiveRebindingReplaySource,
};

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct BindingLayerCertificationBundle {
    report_digest: String,
    deterministic_outcome_class: RebindingOutcomeClass,
    deterministic_continuity_class: BindingContinuityClass,
    binding_identity: String,
    selected_candidate_identity: Option<String>,
    historical_digest: String,
    historical_inspection_digest: String,
    branch_local_digest: String,
    branch_local_inspection_digest: String,
    replay_digest: String,
    replay_ordinary_kind: String,
}

#[cfg_attr(not(test), allow(dead_code))]
impl BindingLayerCertificationBundle {
    pub(crate) fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub(crate) fn deterministic_outcome_class(&self) -> RebindingOutcomeClass {
        self.deterministic_outcome_class
    }

    pub(crate) fn deterministic_continuity_class(&self) -> BindingContinuityClass {
        self.deterministic_continuity_class
    }

    pub(crate) fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub(crate) fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub(crate) fn historical_digest(&self) -> &str {
        &self.historical_digest
    }

    pub(crate) fn historical_inspection_digest(&self) -> &str {
        &self.historical_inspection_digest
    }

    pub(crate) fn branch_local_digest(&self) -> &str {
        &self.branch_local_digest
    }

    pub(crate) fn branch_local_inspection_digest(&self) -> &str {
        &self.branch_local_inspection_digest
    }

    pub(crate) fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub(crate) fn replay_ordinary_kind(&self) -> &str {
        &self.replay_ordinary_kind
    }
}

#[allow(dead_code)]
pub(crate) enum BindingLayerCertificationBundleError {
    Declaration(
        ForgeQueryDeclarationAdmissionError<
            PrimitiveRebindingQueryDomain,
            PrimitiveRebindingDeclarationEntry,
        >,
    ),
    Spatial(PrimitiveRebindingAuthoringError),
    DeterminismMismatch {
        reason: &'static str,
    },
    HistoricalInspectionParityMismatch {
        reason: &'static str,
    },
    BranchLocalInspectionParityMismatch {
        reason: &'static str,
    },
    ReplayParity(PrimitiveRebindingReplayParityError),
}

impl BindingLayerCertificationBundleError {
    pub(crate) fn reason(&self) -> &'static str {
        match self {
            Self::Declaration(_) => {
                "binding-layer certification requires canonical rebinding declarations under the admitted handle"
            }
            Self::Spatial(_) => {
                "binding-layer certification requires the compared rebinding declarations to admit spatially"
            }
            Self::DeterminismMismatch { reason }
            | Self::HistoricalInspectionParityMismatch { reason }
            | Self::BranchLocalInspectionParityMismatch { reason } => reason,
            Self::ReplayParity(error) => error.reason(),
        }
    }
}

impl std::fmt::Debug for BindingLayerCertificationBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BindingLayerCertificationBundleError")
            .field("reason", &self.reason())
            .finish()
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn primitive_rebinding_certification_bundle<C>(
    left_entry: &PrimitiveRebindingDeclarationEntry,
    right_entry: &PrimitiveRebindingDeclarationEntry,
    left_historical: PrimitiveRebindingHistoricalInspection,
    right_historical: PrimitiveRebindingHistoricalInspection,
    left_branch_local: PrimitiveRebindingBranchLocalInspection,
    right_branch_local: PrimitiveRebindingBranchLocalInspection,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<BindingLayerCertificationBundle, BindingLayerCertificationBundleError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let left_decision = left_entry
        .clone()
        .admit()
        .map_err(BindingLayerCertificationBundleError::Spatial)?;
    let right_decision = right_entry
        .clone()
        .admit()
        .map_err(BindingLayerCertificationBundleError::Spatial)?;
    let left_declaration = handle
        .declare(left_entry.clone())
        .map_err(BindingLayerCertificationBundleError::Declaration)?;
    let right_declaration = handle
        .declare(right_entry.clone())
        .map_err(BindingLayerCertificationBundleError::Declaration)?;

    ensure_decision_parity(&left_decision, &right_decision, |reason| {
        BindingLayerCertificationBundleError::DeterminismMismatch { reason }
    })?;
    ensure_decision_parity(&left_decision, left_historical.decision(), |reason| {
        BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch { reason }
    })?;
    ensure_decision_parity(&right_decision, right_historical.decision(), |reason| {
        BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch { reason }
    })?;
    ensure_decision_parity(
        left_historical.decision(),
        right_historical.decision(),
        |reason| BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_decision_parity(&left_decision, left_branch_local.decision(), |reason| {
        BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch { reason }
    })?;
    ensure_decision_parity(&right_decision, right_branch_local.decision(), |reason| {
        BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch { reason }
    })?;
    ensure_decision_parity(
        left_branch_local.decision(),
        right_branch_local.decision(),
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        format!("{:?}", left_declaration.declaration_digest()),
        left_historical.inspection().declaration_digest().to_string(),
        "binding-layer certification requires left historical inspection to come from the left rebinding declaration",
        |reason| BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        format!("{:?}", right_declaration.declaration_digest()),
        right_historical.inspection().declaration_digest().to_string(),
        "binding-layer certification requires right historical inspection to come from the right rebinding declaration",
        |reason| BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        left_historical.historical_digest(),
        right_historical.historical_digest(),
        "binding-layer certification requires host-order-equivalent retained histories to preserve the same historical rebinding digest",
        |reason| BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        left_historical.inspection().inspection_digest(),
        right_historical.inspection().inspection_digest(),
        "binding-layer certification requires host-order-equivalent retained histories to preserve the same historical inspection digest",
        |reason| BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        format!("{:?}", left_declaration.declaration_digest()),
        left_branch_local.inspection().declaration_digest().to_string(),
        "binding-layer certification requires left branch-local inspection to come from the left rebinding declaration",
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        format!("{:?}", right_declaration.declaration_digest()),
        right_branch_local.inspection().declaration_digest().to_string(),
        "binding-layer certification requires right branch-local inspection to come from the right rebinding declaration",
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        left_branch_local.branch_basis_digest(),
        right_branch_local.branch_basis_digest(),
        "binding-layer certification requires equivalent branch-local proofs to preserve the same branch basis identity",
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        left_branch_local.branch_local_digest(),
        right_branch_local.branch_local_digest(),
        "binding-layer certification requires host-order-equivalent branch-local histories to preserve the same branch-local rebinding digest",
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_equal(
        left_branch_local.inspection().inspection_digest(),
        right_branch_local.inspection().inspection_digest(),
        "binding-layer certification requires host-order-equivalent branch-local histories to preserve the same branch-local inspection digest",
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;

    let replay = primitive_rebinding_replay_parity(
        left_entry,
        PrimitiveRebindingReplaySource::Historical(left_historical),
        right_entry,
        PrimitiveRebindingReplaySource::BranchLocal(right_branch_local),
        handle,
    )
    .map_err(BindingLayerCertificationBundleError::ReplayParity)?;

    Ok(BindingLayerCertificationBundle {
        report_digest: truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("outcome:{:?}", left_decision.outcome_class()),
                format!(
                    "continuity:{:?}",
                    left_decision.explanation().continuity_class()
                ),
                format!("prior:{}", left_decision.explanation().prior_identity()),
                format!(
                    "selected:{}",
                    left_decision
                        .explanation()
                        .selected_candidate_identity()
                        .unwrap_or("none")
                ),
                format!("historical:{}", right_historical.historical_digest()),
                format!(
                    "historical_inspection:{}",
                    right_historical.inspection().inspection_digest()
                ),
                format!("branch_local:{}", left_branch_local.branch_local_digest()),
                format!(
                    "branch_local_inspection:{}",
                    left_branch_local.inspection().inspection_digest()
                ),
                format!("replay:{}", replay.replay_digest()),
                format!("ordinary_kind:{}", replay.ordinary_kind()),
            ],
        ),
        deterministic_outcome_class: left_decision.outcome_class(),
        deterministic_continuity_class: left_decision.explanation().continuity_class().clone(),
        binding_identity: left_decision.explanation().prior_identity().to_string(),
        selected_candidate_identity: left_decision
            .explanation()
            .selected_candidate_identity()
            .map(ToOwned::to_owned),
        historical_digest: right_historical.historical_digest().to_string(),
        historical_inspection_digest: right_historical
            .inspection()
            .inspection_digest()
            .to_string(),
        branch_local_digest: left_branch_local.branch_local_digest().to_string(),
        branch_local_inspection_digest: left_branch_local
            .inspection()
            .inspection_digest()
            .to_string(),
        replay_digest: replay.replay_digest().to_string(),
        replay_ordinary_kind: replay.ordinary_kind().to_string(),
    })
}

fn ensure_decision_parity(
    left: &worth_spatial::facade::bindings::AdmittedRebindingDecision,
    right: &worth_spatial::facade::bindings::AdmittedRebindingDecision,
    error: impl Fn(&'static str) -> BindingLayerCertificationBundleError,
) -> Result<(), BindingLayerCertificationBundleError> {
    let left_explanation = left.explanation();
    let right_explanation = right.explanation();
    ensure_equal(
        format!("{:?}", left.outcome_class()),
        format!("{:?}", right.outcome_class()),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same outcome class",
        &error,
    )?;
    ensure_equal(
        format!("{:?}", left_explanation.continuity_class()),
        format!("{:?}", right_explanation.continuity_class()),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same continuity class",
        &error,
    )?;
    ensure_equal(
        left_explanation.prior_identity(),
        right_explanation.prior_identity(),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same binding identity",
        &error,
    )?;
    ensure_equal(
        left_explanation.prior_site_identity(),
        right_explanation.prior_site_identity(),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same anchor identity",
        &error,
    )?;
    ensure_equal(
        left_explanation.selected_candidate_identity().unwrap_or("none"),
        right_explanation.selected_candidate_identity().unwrap_or("none"),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same selected binding identity",
        &error,
    )?;
    ensure_equal(
        sorted_join(left_explanation.candidate_identities()),
        sorted_join(right_explanation.candidate_identities()),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same candidate identity inventory",
        &error,
    )
}

fn ensure_equal<T: Eq>(
    left: T,
    right: T,
    reason: &'static str,
    error: impl Fn(&'static str) -> BindingLayerCertificationBundleError,
) -> Result<(), BindingLayerCertificationBundleError> {
    if left == right {
        Ok(())
    } else {
        Err(error(reason))
    }
}

fn sorted_join(values: &[String]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.join("|")
}
