use crate::basis_lifecycle::BasisFamily;
use crate::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

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

const EFFECT_LIFECYCLE_IDENTITY_SCOPE: ForgeQueryEvidenceScope =
    ForgeQueryEvidenceScope::WorkflowMutationLowering;

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
    row_identity: ForgeQueryEvidenceIdentity,
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
        let row_identity = ForgeQueryEvidenceIdentity::compose(EFFECT_LIFECYCLE_IDENTITY_SCOPE)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "effect_lifecycle_support_row_v1",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("basis_family"),
                basis_family.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("effect_family"),
                effect_family.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("authority_owner"),
                authority_owner.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("lowered_artifact"),
                lowered_artifact_kind.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("receipt_artifact"),
                receipt_artifact_kind.as_str(),
            )
            .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
            .field_shape(ForgeQueryEvidenceTag::new("cause"), cause.as_str())
            .seal();
        Self {
            basis_family,
            effect_family,
            authority_owner,
            lowered_artifact_kind,
            receipt_artifact_kind,
            posture,
            cause,
            row_identity,
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

    pub fn row_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.row_identity
    }

    pub fn row_for_reporting(&self) -> &str {
        self.row_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportMatrix {
    rows: Vec<EffectLifecycleSupportRow>,
    matrix_identity: ForgeQueryEvidenceIdentity,
}

impl EffectLifecycleSupportMatrix {
    pub fn rows(&self) -> &[EffectLifecycleSupportRow] {
        &self.rows
    }

    pub fn matrix_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.matrix_identity
    }

    pub fn matrix_for_reporting(&self) -> &str {
        self.matrix_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectLifecycleSupportDiscovery {
    requested_basis_family: BasisFamily,
    requested_effect_family: EffectFamily,
    posture: EffectSupportPosture,
    cause: EffectSupportCause,
    matched_row: Option<EffectLifecycleSupportRow>,
    matched_row_identity: Option<ForgeQueryEvidenceIdentity>,
    support_matrix_identity: ForgeQueryEvidenceIdentity,
    discovery_identity: ForgeQueryEvidenceIdentity,
    counters: EffectLifecycleCounters,
}

impl EffectLifecycleSupportDiscovery {
    fn new(
        requested_basis_family: BasisFamily,
        requested_effect_family: EffectFamily,
        decision: EffectSupportDecision,
        support_matrix_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        let EffectSupportDecision {
            posture,
            cause,
            matched_row,
            rows_consulted,
        } = decision;
        let matched_row_identity = matched_row
            .as_ref()
            .map(|row| row.row_identity().clone());
        let counters = EffectLifecycleCounters::support_lookup(rows_consulted);
        let mut discovery = ForgeQueryEvidenceIdentity::compose(EFFECT_LIFECYCLE_IDENTITY_SCOPE)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "effect_lifecycle_support_discovery_v1",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("basis_family"),
                requested_basis_family.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("effect_family"),
                requested_effect_family.as_str(),
            )
            .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
            .field_shape(ForgeQueryEvidenceTag::new("cause"), cause.as_str())
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("matrix"),
                &support_matrix_identity,
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("counters"),
                &counters.evidence_identity(),
            );
        discovery = match matched_row_identity.as_ref() {
            Some(row_identity) => {
                discovery.field_evidence_identity(ForgeQueryEvidenceTag::new("matched_row"), row_identity)
            }
            None => discovery.field_shape(ForgeQueryEvidenceTag::new("matched_row"), "unsupported"),
        };
        let discovery_identity = discovery.seal();
        Self {
            requested_basis_family,
            requested_effect_family,
            posture,
            cause,
            matched_row,
            matched_row_identity,
            support_matrix_identity,
            discovery_identity,
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

    pub fn matched_row_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.matched_row_identity.as_ref()
    }

    pub fn matched_row_for_reporting(&self) -> Option<&str> {
        self.matched_row_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn support_matrix_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.support_matrix_identity
    }

    pub fn support_matrix_for_reporting(&self) -> &str {
        self.support_matrix_identity.as_str()
    }

    pub fn discovery_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.discovery_identity
    }

    pub fn discovery_for_reporting(&self) -> &str {
        self.discovery_identity.as_str()
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
    let row_identities = rows
        .iter()
        .map(|row| row.row_identity().clone())
        .collect::<Vec<_>>();
    let matrix_identity = ForgeQueryEvidenceIdentity::compose(EFFECT_LIFECYCLE_IDENTITY_SCOPE)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "effect_lifecycle_support_matrix_v1",
        )
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rows"), &row_identities)
        .seal();
    EffectLifecycleSupportMatrix {
        rows,
        matrix_identity,
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
        support_matrix.matrix_identity().clone(),
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
