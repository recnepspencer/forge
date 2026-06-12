use std::collections::BTreeSet;

use crate::planar_contracts::contract_bundle::PlanarM7ReadinessReceipt;

use super::denial::{M6PlanarCloseoutDenial, M6PlanarCloseoutDenialKind};
use super::fixture_fence::M6LegacyFixtureFence;
use super::platform_targets::{M6PremetabossEvidencePosture, M6PremetabossEvidenceRow};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum M6PremetabossFamily {
    CoplanarOverlapStorm,
    HighValencePlanarSingularity,
    ThinFeatureScaleSeparation,
    RetainedHistoryCancellationChain,
    DirtyPlanarInputCleanFail,
    UnboundedHalfSpacePosture,
    ProjectionConsumedPlanarFactParity,
    BooleanReadinessFinalBoss,
    NmtOpenRadialFan,
    NmtMixedSurfaceKillBox,
    NmtOpenClassTriadParity,
    NmtGrazingBasketStack,
}

impl M6PremetabossFamily {
    pub const ALL: [Self; 12] = [
        Self::CoplanarOverlapStorm,
        Self::HighValencePlanarSingularity,
        Self::ThinFeatureScaleSeparation,
        Self::RetainedHistoryCancellationChain,
        Self::DirtyPlanarInputCleanFail,
        Self::UnboundedHalfSpacePosture,
        Self::ProjectionConsumedPlanarFactParity,
        Self::BooleanReadinessFinalBoss,
        Self::NmtOpenRadialFan,
        Self::NmtMixedSurfaceKillBox,
        Self::NmtOpenClassTriadParity,
        Self::NmtGrazingBasketStack,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoplanarOverlapStorm => "mb-m6-1-coplanar-overlap-storm",
            Self::HighValencePlanarSingularity => "mb-m6-2-high-valence-planar-singularity",
            Self::ThinFeatureScaleSeparation => "mb-m6-3-thin-feature-scale-separation",
            Self::RetainedHistoryCancellationChain => "mb-m6-4-retained-history-cancellation",
            Self::DirtyPlanarInputCleanFail => "mb-m6-5-dirty-planar-clean-fail",
            Self::UnboundedHalfSpacePosture => "mb-m6-6-unbounded-half-space-posture",
            Self::ProjectionConsumedPlanarFactParity => "mb-m6-7-projection-consumed-parity",
            Self::BooleanReadinessFinalBoss => "mb-m6-8-boolean-readiness-final-boss",
            Self::NmtOpenRadialFan => "mb-m6-nmt-1-open-radial-fan",
            Self::NmtMixedSurfaceKillBox => "mb-m6-nmt-2-mixed-surface-kill-box",
            Self::NmtOpenClassTriadParity => "mb-m6-nmt-3-open-class-triad-parity",
            Self::NmtGrazingBasketStack => "mb-m6-nmt-4-grazing-basket-stack",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum M6ShortcutDeletionFamily {
    KernelLocalPredicateRuntime,
    KernelLocalStructuralIdentityRuntime,
    KernelLocalRetainedPlanarFactsRuntime,
    KernelLocalProjectionConsumptionRuntime,
    KernelLocalRecoveryRuntime,
    KernelLocalPseudoQueryRuntime,
}

impl M6ShortcutDeletionFamily {
    pub const ALL: [Self; 6] = [
        Self::KernelLocalPredicateRuntime,
        Self::KernelLocalStructuralIdentityRuntime,
        Self::KernelLocalRetainedPlanarFactsRuntime,
        Self::KernelLocalProjectionConsumptionRuntime,
        Self::KernelLocalRecoveryRuntime,
        Self::KernelLocalPseudoQueryRuntime,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KernelLocalPredicateRuntime => "kernel-local-predicate-runtime",
            Self::KernelLocalStructuralIdentityRuntime => {
                "kernel-local-structural-identity-runtime"
            }
            Self::KernelLocalRetainedPlanarFactsRuntime => {
                "kernel-local-retained-planar-facts-runtime"
            }
            Self::KernelLocalProjectionConsumptionRuntime => {
                "kernel-local-projection-consumption-runtime"
            }
            Self::KernelLocalRecoveryRuntime => "kernel-local-recovery-runtime",
            Self::KernelLocalPseudoQueryRuntime => "kernel-local-pseudo-query-runtime",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6LegacyDeletionEvidenceRow {
    family: M6ShortcutDeletionFamily,
    evidence_digest: String,
}

impl M6LegacyDeletionEvidenceRow {
    pub fn deleted(family: M6ShortcutDeletionFamily, evidence_digest: impl Into<String>) -> Self {
        Self {
            family,
            evidence_digest: evidence_digest.into(),
        }
    }

    pub fn family(&self) -> M6ShortcutDeletionFamily {
        self.family
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6QueryBoundaryEvidenceRow {
    declaration_digest: String,
    envelope_digest: String,
}

impl M6QueryBoundaryEvidenceRow {
    pub fn from_m7_readiness(readiness: &PlanarM7ReadinessReceipt) -> Self {
        Self {
            declaration_digest: readiness.declaration_digest().to_string(),
            envelope_digest: readiness.envelope_digest().to_string(),
        }
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct M6PlanarCloseoutCertification {
    readiness: PlanarM7ReadinessReceipt,
    premetaboss: Vec<M6PremetabossEvidenceRow>,
    legacy_deletion: Vec<M6LegacyDeletionEvidenceRow>,
    query_boundary: Option<M6QueryBoundaryEvidenceRow>,
    legacy_fixture_fence: Option<M6LegacyFixtureFence>,
}

impl M6PlanarCloseoutCertification {
    pub fn from_m7_readiness(readiness: PlanarM7ReadinessReceipt) -> Self {
        Self {
            readiness,
            premetaboss: Vec::new(),
            legacy_deletion: Vec::new(),
            query_boundary: None,
            legacy_fixture_fence: None,
        }
    }

    pub fn with_premetaboss_evidence(
        mut self,
        evidence: impl IntoIterator<Item = M6PremetabossEvidenceRow>,
    ) -> Self {
        self.premetaboss.extend(evidence);
        self
    }

    pub fn with_legacy_deletion_evidence(
        mut self,
        evidence: impl IntoIterator<Item = M6LegacyDeletionEvidenceRow>,
    ) -> Self {
        self.legacy_deletion.extend(evidence);
        self
    }

    pub fn with_query_boundary_evidence(mut self, evidence: M6QueryBoundaryEvidenceRow) -> Self {
        self.query_boundary = Some(evidence);
        self
    }

    pub fn with_legacy_fixture_fence(mut self, fence: M6LegacyFixtureFence) -> Self {
        self.legacy_fixture_fence = Some(fence);
        self
    }

    pub(crate) fn build(self) -> Result<M6PlanarCloseoutBasis, M6PlanarCloseoutDenial> {
        M6PlanarCloseoutBasis::from_builder(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct M6PlanarCloseoutBasis {
    readiness: PlanarM7ReadinessReceipt,
    premetaboss: Vec<M6PremetabossEvidenceRow>,
    legacy_deletion: Vec<M6LegacyDeletionEvidenceRow>,
    query_boundary: M6QueryBoundaryEvidenceRow,
    legacy_fixture_fence: M6LegacyFixtureFence,
}

impl M6PlanarCloseoutBasis {
    fn from_builder(
        builder: M6PlanarCloseoutCertification,
    ) -> Result<Self, M6PlanarCloseoutDenial> {
        let query_boundary = builder.query_boundary.ok_or_else(|| {
            M6PlanarCloseoutDenial::new(
                M6PlanarCloseoutDenialKind::MissingQueryBoundaryEvidence,
                "M6 closeout requires explicit Query boundary evidence",
            )
        })?;
        let basis = Self {
            readiness: builder.readiness,
            premetaboss: builder.premetaboss,
            legacy_deletion: builder.legacy_deletion,
            query_boundary,
            legacy_fixture_fence: builder.legacy_fixture_fence.ok_or_else(|| {
                M6PlanarCloseoutDenial::new(
                    M6PlanarCloseoutDenialKind::MissingLegacyFixtureFence,
                    "M6 closeout requires an explicit legacy fixture fence",
                )
            })?,
        };
        validate_m6_closeout_basis(&basis)?;
        Ok(basis)
    }

    pub fn readiness(&self) -> &PlanarM7ReadinessReceipt {
        &self.readiness
    }

    pub fn premetaboss_rows(&self) -> &[M6PremetabossEvidenceRow] {
        &self.premetaboss
    }

    pub fn legacy_deletion_rows(&self) -> &[M6LegacyDeletionEvidenceRow] {
        &self.legacy_deletion
    }

    pub fn query_boundary(&self) -> &M6QueryBoundaryEvidenceRow {
        &self.query_boundary
    }

    pub fn legacy_fixture_fence(&self) -> &M6LegacyFixtureFence {
        &self.legacy_fixture_fence
    }

    pub fn closeout_rows(&self) -> usize {
        self.premetaboss.len() + self.legacy_deletion.len() + 2
    }
}

fn validate_m6_closeout_basis(basis: &M6PlanarCloseoutBasis) -> Result<(), M6PlanarCloseoutDenial> {
    if basis.readiness.boolean_result().is_some() || basis.readiness.imprint_action().is_some() {
        return Err(denial(
            M6PlanarCloseoutDenialKind::BooleanExecutionAlreadyPresent,
            "M6 closeout must freeze pre-boolean readiness, not boolean execution",
        ));
    }
    if basis.query_boundary.declaration_digest() != basis.readiness.declaration_digest()
        || basis.query_boundary.envelope_digest() != basis.readiness.envelope_digest()
    {
        return Err(denial(
            M6PlanarCloseoutDenialKind::QueryBoundaryMismatch,
            "M6 closeout Query boundary evidence must match the M7 readiness receipt",
        ));
    }
    require_exact_premetaboss_families(&basis.premetaboss)?;
    require_exact_legacy_deletion_families(&basis.legacy_deletion)?;
    Ok(())
}

fn require_exact_premetaboss_families(
    rows: &[M6PremetabossEvidenceRow],
) -> Result<(), M6PlanarCloseoutDenial> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if row.posture() != M6PremetabossEvidencePosture::WorkloadPlatform {
            return Err(denial(
                M6PlanarCloseoutDenialKind::SyntheticEndToEndBlocked,
                format!(
                    "{} cannot register synthetic MB closeout evidence: {}",
                    row.family().as_str(),
                    row.human_reason()
                ),
            ));
        }
        if !seen.insert(row.family()) {
            return Err(denial(
                M6PlanarCloseoutDenialKind::DuplicatePremetabossFamily,
                format!(
                    "duplicate pre-MetaBoss closeout row: {}",
                    row.family().as_str()
                ),
            ));
        }
    }
    for family in M6PremetabossFamily::ALL {
        if !seen.contains(&family) {
            return Err(denial(
                M6PlanarCloseoutDenialKind::MissingPremetabossFamily,
                format!("missing pre-MetaBoss closeout row: {}", family.as_str()),
            ));
        }
    }
    Ok(())
}

fn require_exact_legacy_deletion_families(
    rows: &[M6LegacyDeletionEvidenceRow],
) -> Result<(), M6PlanarCloseoutDenial> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.family()) {
            return Err(denial(
                M6PlanarCloseoutDenialKind::DuplicateLegacyDeletionFamily,
                format!(
                    "duplicate legacy deletion closeout row: {}",
                    row.family().as_str()
                ),
            ));
        }
    }
    for family in M6ShortcutDeletionFamily::ALL {
        if !seen.contains(&family) {
            return Err(denial(
                M6PlanarCloseoutDenialKind::MissingLegacyDeletionFamily,
                format!("missing legacy deletion closeout row: {}", family.as_str()),
            ));
        }
    }
    Ok(())
}

fn denial(kind: M6PlanarCloseoutDenialKind, reason: impl Into<String>) -> M6PlanarCloseoutDenial {
    M6PlanarCloseoutDenial::new(kind, reason)
}
