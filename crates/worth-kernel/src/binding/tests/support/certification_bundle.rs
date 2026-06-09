use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::bindings::{
    BindingContinuityClass, PrimitiveRebindingFactReceipt, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingRetainedFactSource, RebindingOutcomeClass,
};
use worth_spatial::facade::inspection::{
    geometry_replay_parity_entry, PrimitiveRebindingBranchLocalInspection,
    PrimitiveRebindingHistoricalInspection, PrimitiveRebindingReplayParityError,
    PrimitiveRebindingReplaySource, PrimitiveRebindingRetainedViewPayload,
};

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

pub(crate) enum BindingLayerCertificationBundleError {
    DeterminismMismatch { reason: &'static str },
    HistoricalInspectionParityMismatch { reason: &'static str },
    BranchLocalInspectionParityMismatch { reason: &'static str },
    ReplayParity(PrimitiveRebindingReplayParityError),
}

impl BindingLayerCertificationBundleError {
    pub(crate) fn reason(&self) -> &'static str {
        match self {
            Self::DeterminismMismatch { reason }
            | Self::HistoricalInspectionParityMismatch { reason }
            | Self::BranchLocalInspectionParityMismatch { reason } => reason,
            Self::ReplayParity(error) => error.reason(),
        }
    }
}

impl std::fmt::Debug for BindingLayerCertificationBundleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("BindingLayerCertificationBundleError");
        debug.field("reason", &self.reason());
        match self {
            Self::ReplayParity(error) => {
                debug.field("replay_error", error);
            }
            Self::DeterminismMismatch { .. }
            | Self::HistoricalInspectionParityMismatch { .. }
            | Self::BranchLocalInspectionParityMismatch { .. } => {}
        }
        debug.finish()
    }
}

pub(crate) fn primitive_rebinding_certification_bundle<C>(
    left_source: PrimitiveRebindingRetainedFactSource,
    right_source: PrimitiveRebindingRetainedFactSource,
    left_historical: PrimitiveRebindingHistoricalInspection,
    right_historical: PrimitiveRebindingHistoricalInspection,
    left_branch_local: PrimitiveRebindingBranchLocalInspection,
    right_branch_local: PrimitiveRebindingBranchLocalInspection,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<BindingLayerCertificationBundle, BindingLayerCertificationBundleError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    ensure_receipt_parity(left_source.receipt(), right_source.receipt(), |reason| {
        BindingLayerCertificationBundleError::DeterminismMismatch { reason }
    })?;
    ensure_receipt_parity(left_source.receipt(), left_historical.receipt(), |reason| {
        BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch { reason }
    })?;
    ensure_receipt_parity(
        right_source.receipt(),
        right_historical.receipt(),
        |reason| BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_receipt_parity(
        left_historical.receipt(),
        right_historical.receipt(),
        |reason| BindingLayerCertificationBundleError::HistoricalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_receipt_parity(
        left_source.receipt(),
        left_branch_local.receipt(),
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_receipt_parity(
        right_source.receipt(),
        right_branch_local.receipt(),
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
            reason,
        },
    )?;
    ensure_receipt_parity(
        left_branch_local.receipt(),
        right_branch_local.receipt(),
        |reason| BindingLayerCertificationBundleError::BranchLocalInspectionParityMismatch {
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

    let replay = geometry_replay_parity_entry(
        PrimitiveRebindingReplaySource::Historical(left_historical.retained_fact_receipt()),
        PrimitiveRebindingReplaySource::BranchLocal(right_branch_local.retained_fact_receipt()),
    )
    .compare(handle)
    .map_err(BindingLayerCertificationBundleError::ReplayParity)?;

    Ok(BindingLayerCertificationBundle {
        report_digest: truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("outcome:{:?}", left_source.receipt().outcome_class()),
                format!("continuity:{:?}", left_source.receipt().continuity_class()),
                format!("prior:{}", left_source.receipt().prior_binding_identity()),
                format!(
                    "selected:{}",
                    left_source
                        .receipt()
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
        deterministic_outcome_class: left_source.receipt().outcome_class(),
        deterministic_continuity_class: left_source.receipt().continuity_class(),
        binding_identity: left_source.receipt().prior_binding_identity().to_string(),
        selected_candidate_identity: left_source
            .receipt()
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

fn ensure_receipt_parity(
    left: &impl RebindingReceiptLike,
    right: &impl RebindingReceiptLike,
    error: impl Fn(&'static str) -> BindingLayerCertificationBundleError,
) -> Result<(), BindingLayerCertificationBundleError> {
    ensure_equal(
        format!("{:?}", left.outcome_class()),
        format!("{:?}", right.outcome_class()),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same outcome class",
        &error,
    )?;
    ensure_equal(
        format!("{:?}", left.continuity_class()),
        format!("{:?}", right.continuity_class()),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same continuity class",
        &error,
    )?;
    ensure_equal(
        left.prior_binding_identity(),
        right.prior_binding_identity(),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same binding identity",
        &error,
    )?;
    ensure_equal(
        left.prior_site_identity(),
        right.prior_site_identity(),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same anchor identity",
        &error,
    )?;
    ensure_equal(
        left.selected_candidate_identity().unwrap_or("none"),
        right.selected_candidate_identity().unwrap_or("none"),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same selected binding identity",
        &error,
    )?;
    ensure_equal(
        sorted_join(left.candidate_identities()),
        sorted_join(right.candidate_identities()),
        "binding-layer certification requires host-order-equivalent rebinding declarations to preserve the same candidate identity inventory",
        &error,
    )
}

trait RebindingReceiptLike {
    fn outcome_class(&self) -> RebindingOutcomeClass;
    fn continuity_class(&self) -> BindingContinuityClass;
    fn prior_binding_identity(&self) -> &str;
    fn prior_site_identity(&self) -> &str;
    fn selected_candidate_identity(&self) -> Option<&str>;
    fn candidate_identities(&self) -> &[String];
}

impl RebindingReceiptLike for PrimitiveRebindingFactReceipt {
    fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class()
    }

    fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class()
    }

    fn prior_binding_identity(&self) -> &str {
        self.prior_binding_identity()
    }

    fn prior_site_identity(&self) -> &str {
        self.prior_site_identity()
    }

    fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity()
    }

    fn candidate_identities(&self) -> &[String] {
        self.candidate_identities()
    }
}

impl RebindingReceiptLike for PrimitiveRebindingRetainedViewPayload {
    fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class()
    }

    fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class()
    }

    fn prior_binding_identity(&self) -> &str {
        self.prior_binding_identity()
    }

    fn prior_site_identity(&self) -> &str {
        self.prior_site_identity()
    }

    fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity()
    }

    fn candidate_identities(&self) -> &[String] {
        self.candidate_identities()
    }
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
