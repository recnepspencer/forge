use crate::basis_lifecycle::BasisFamily;
use crate::identity::hash_parts;

use super::counters::EffectLifecycleCounters;
use super::inventory::{EffectLoweredArtifactKind, EffectReceiptArtifactKind};
use super::planning::EffectAuthorityOwner;
use super::support_contract::{
    deferred_support_contract, support_deferred_neighbors, support_denial_kinds,
    EffectDeferredNeighborFamily, EffectDeferredSupportContract,
};
use super::support_matrix_rows::support_rows;
use super::taxonomy::DeniedEffectEligibilityKind;
use super::taxonomy::EffectFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectSupportPosture {
    Admitted,
    Advisory,
    Denied,
    RebindRequired,
    Deferred,
    Unsupported,
}

impl EffectSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
            Self::RebindRequired => "rebind_required",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectSupportCause {
    Supported,
    AdvisoryOnlyExecution,
    PreviewRebindRequired,
    BranchAuthorityRequired,
    StoreBackedExecutionDeferred,
    DurableReplayDeferred,
    UnsupportedForBasisFamily,
}

impl EffectSupportCause {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::AdvisoryOnlyExecution => "advisory_only_execution",
            Self::PreviewRebindRequired => "preview_rebind_required",
            Self::BranchAuthorityRequired => "branch_authority_required",
            Self::StoreBackedExecutionDeferred => "store_backed_execution_deferred",
            Self::DurableReplayDeferred => "durable_replay_deferred",
            Self::UnsupportedForBasisFamily => "unsupported_for_basis_family",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportRow {
    basis_family: BasisFamily,
    effect_family: EffectFamily,
    authority_owner: EffectAuthorityOwner,
    lowered_artifact_kind: EffectLoweredArtifactKind,
    receipt_artifact_kind: EffectReceiptArtifactKind,
    posture: EffectSupportPosture,
    cause: EffectSupportCause,
    row_digest: String,
}

impl EffectLifecycleSupportRow {
    pub(crate) fn new(
        basis_family: BasisFamily,
        effect_family: EffectFamily,
        authority_owner: EffectAuthorityOwner,
        lowered_artifact_kind: EffectLoweredArtifactKind,
        receipt_artifact_kind: EffectReceiptArtifactKind,
        posture: EffectSupportPosture,
        cause: EffectSupportCause,
    ) -> Self {
        let row_digest = hash_parts(&[
            format!("basis_family:{}", basis_family.as_str()),
            format!("effect_family:{}", effect_family.as_str()),
            format!("authority_owner:{}", authority_owner.as_str()),
            format!("lowered_artifact:{}", lowered_artifact_kind.as_str()),
            format!("receipt_artifact:{}", receipt_artifact_kind.as_str()),
            format!("posture:{}", posture.as_str()),
            format!("cause:{}", cause.as_str()),
        ]);
        Self {
            basis_family,
            effect_family,
            authority_owner,
            lowered_artifact_kind,
            receipt_artifact_kind,
            posture,
            cause,
            row_digest,
        }
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn effect_family(&self) -> EffectFamily {
        self.effect_family
    }

    pub fn authority_owner(&self) -> EffectAuthorityOwner {
        self.authority_owner
    }

    pub fn lowered_artifact_kind(&self) -> EffectLoweredArtifactKind {
        self.lowered_artifact_kind
    }

    pub fn receipt_artifact_kind(&self) -> EffectReceiptArtifactKind {
        self.receipt_artifact_kind
    }

    pub fn posture(&self) -> EffectSupportPosture {
        self.posture
    }

    pub fn cause(&self) -> EffectSupportCause {
        self.cause
    }

    pub fn requires_rebind(&self) -> bool {
        self.posture == EffectSupportPosture::RebindRequired
    }

    pub fn denial_kinds(&self) -> &'static [DeniedEffectEligibilityKind] {
        support_denial_kinds(self.posture, self.cause)
    }

    pub fn deferred_neighbors(&self) -> &'static [EffectDeferredNeighborFamily] {
        support_deferred_neighbors(self.effect_family)
    }

    pub fn deferred_contract(&self) -> Option<EffectDeferredSupportContract> {
        deferred_support_contract(self.cause)
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportMatrix {
    rows: Vec<EffectLifecycleSupportRow>,
    matrix_digest: String,
}

impl EffectLifecycleSupportMatrix {
    pub fn rows(&self) -> &[EffectLifecycleSupportRow] {
        &self.rows
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportDiscovery {
    requested_basis_family: BasisFamily,
    requested_effect_family: EffectFamily,
    posture: EffectSupportPosture,
    cause: EffectSupportCause,
    matched_row: Option<EffectLifecycleSupportRow>,
    matched_row_digest: Option<String>,
    support_matrix_digest: String,
    discovery_digest: String,
    counters: EffectLifecycleCounters,
}

impl EffectLifecycleSupportDiscovery {
    fn new(
        requested_basis_family: BasisFamily,
        requested_effect_family: EffectFamily,
        decision: EffectSupportDecision,
        support_matrix_digest: String,
    ) -> Self {
        let EffectSupportDecision {
            posture,
            cause,
            matched_row,
            rows_consulted,
        } = decision;
        let matched_row_digest = matched_row.as_ref().map(|row| row.row_digest().to_string());
        let counters = EffectLifecycleCounters::support_lookup(rows_consulted);
        let discovery_digest = hash_parts(&[
            "effect_lifecycle_support_discovery_v1".to_string(),
            format!("basis_family:{}", requested_basis_family.as_str()),
            format!("effect_family:{}", requested_effect_family.as_str()),
            format!("posture:{}", posture.as_str()),
            format!("cause:{}", cause.as_str()),
            format!(
                "matched_row:{}",
                matched_row_digest.as_deref().unwrap_or("unsupported")
            ),
            format!("matrix:{support_matrix_digest}"),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            requested_basis_family,
            requested_effect_family,
            posture,
            cause,
            matched_row,
            matched_row_digest,
            support_matrix_digest,
            discovery_digest,
            counters,
        }
    }

    pub fn requested_basis_family(&self) -> BasisFamily {
        self.requested_basis_family
    }

    pub fn requested_effect_family(&self) -> EffectFamily {
        self.requested_effect_family
    }

    pub fn posture(&self) -> EffectSupportPosture {
        self.posture
    }

    pub fn cause(&self) -> EffectSupportCause {
        self.cause
    }

    pub fn authority_owner(&self) -> Option<EffectAuthorityOwner> {
        self.matched_row.as_ref().map(|row| row.authority_owner())
    }

    pub fn requires_rebind(&self) -> bool {
        self.posture == EffectSupportPosture::RebindRequired
    }

    pub fn supported_lowering(&self) -> Option<EffectLoweredArtifactKind> {
        self.matched_row
            .as_ref()
            .map(|row| row.lowered_artifact_kind())
    }

    pub fn receipt_family(&self) -> Option<EffectReceiptArtifactKind> {
        self.matched_row
            .as_ref()
            .map(|row| row.receipt_artifact_kind())
    }

    pub fn denial_kinds(&self) -> &'static [DeniedEffectEligibilityKind] {
        support_denial_kinds(self.posture, self.cause)
    }

    pub fn deferred_neighbors(&self) -> &'static [EffectDeferredNeighborFamily] {
        support_deferred_neighbors(self.requested_effect_family)
    }

    pub fn deferred_contract(&self) -> Option<EffectDeferredSupportContract> {
        deferred_support_contract(self.cause)
    }

    pub fn matched_row_digest(&self) -> Option<&str> {
        self.matched_row_digest.as_deref()
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn discovery_digest(&self) -> &str {
        &self.discovery_digest
    }

    pub fn counters(&self) -> &EffectLifecycleCounters {
        &self.counters
    }
}

pub fn effect_lifecycle_support_matrix() -> EffectLifecycleSupportMatrix {
    let rows = support_rows()
        .iter()
        .map(|row| {
            EffectLifecycleSupportRow::new(
                row.basis_family,
                row.effect_family,
                row.authority_owner,
                row.lowered_artifact_kind,
                row.receipt_artifact_kind,
                row.posture,
                row.cause,
            )
        })
        .collect::<Vec<_>>();
    let matrix_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    );
    EffectLifecycleSupportMatrix {
        rows,
        matrix_digest,
    }
}

pub fn discover_effect_lifecycle_support(
    basis_family: BasisFamily,
    effect_family: EffectFamily,
) -> EffectLifecycleSupportDiscovery {
    let support_matrix = effect_lifecycle_support_matrix();
    EffectLifecycleSupportDiscovery::new(
        basis_family,
        effect_family,
        support_decision_for(basis_family, effect_family),
        support_matrix.matrix_digest().to_string(),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectSupportDecision {
    posture: EffectSupportPosture,
    cause: EffectSupportCause,
    matched_row: Option<EffectLifecycleSupportRow>,
    rows_consulted: usize,
}

impl EffectSupportDecision {
    pub(crate) fn posture(&self) -> EffectSupportPosture {
        self.posture
    }

    pub(crate) fn rows_consulted(&self) -> usize {
        self.rows_consulted
    }

    pub(crate) fn cause(&self) -> EffectSupportCause {
        self.cause
    }
}

pub(crate) fn support_decision_for(
    basis_family: BasisFamily,
    effect_family: EffectFamily,
) -> EffectSupportDecision {
    let mut rows_consulted = 0;
    for row in support_rows() {
        rows_consulted += 1;
        if row.basis_family == basis_family && row.effect_family == effect_family {
            return EffectSupportDecision {
                posture: row.posture,
                matched_row: Some(EffectLifecycleSupportRow::new(
                    row.basis_family,
                    row.effect_family,
                    row.authority_owner,
                    row.lowered_artifact_kind,
                    row.receipt_artifact_kind,
                    row.posture,
                    row.cause,
                )),
                cause: row.cause,
                rows_consulted,
            };
        }
    }

    EffectSupportDecision {
        posture: EffectSupportPosture::Unsupported,
        cause: EffectSupportCause::UnsupportedForBasisFamily,
        matched_row: None,
        rows_consulted,
    }
}
