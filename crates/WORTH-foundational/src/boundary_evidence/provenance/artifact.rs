use crate::canonicalization::{CanonicalDerivedDigest, CanonicalEquivalenceBasis};
use crate::diagnostics::{FoundationalDiagnosticCodeId, FoundationalDiagnosticScopeId};
use crate::locators::FoundationalTransitionLocator;
use crate::profiles::FoundationalProfileIdentity;
use crate::transitions::FoundationalTransitionStrategyIdentity;

use super::super::primitives::{
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
};
use super::layers::FoundationalBoundaryEvidenceProvenanceLayerKind;
use super::source_basis::FoundationalBoundaryEvidenceSourceBasis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceAuthorityPath(FoundationalTransitionLocator);

impl FoundationalBoundaryEvidenceAuthorityPath {
    pub fn transition(locator: FoundationalTransitionLocator) -> Self {
        Self(locator)
    }

    pub fn locator(&self) -> &FoundationalTransitionLocator {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceStrategyBasis(FoundationalTransitionStrategyIdentity);

impl FoundationalBoundaryEvidenceStrategyBasis {
    pub fn strategy(identity: FoundationalTransitionStrategyIdentity) -> Self {
        Self(identity)
    }

    pub fn identity(&self) -> &FoundationalTransitionStrategyIdentity {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceProfileBasis(FoundationalProfileIdentity);

impl FoundationalBoundaryEvidenceProfileBasis {
    pub fn profile(identity: FoundationalProfileIdentity) -> Self {
        Self(identity)
    }

    pub fn identity(&self) -> &FoundationalProfileIdentity {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceComparisonBasis(CanonicalEquivalenceBasis);

impl FoundationalBoundaryEvidenceComparisonBasis {
    pub const fn comparison(basis: CanonicalEquivalenceBasis) -> Self {
        Self(basis)
    }

    pub const fn basis(&self) -> CanonicalEquivalenceBasis {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceCanonicalDigestBasis(CanonicalDerivedDigest);

impl FoundationalBoundaryEvidenceCanonicalDigestBasis {
    pub fn digest(digest: CanonicalDerivedDigest) -> Self {
        Self(digest)
    }

    pub fn digest_value(&self) -> &CanonicalDerivedDigest {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FoundationalBoundaryEvidenceSupportContextAttachment {
    DiagnosticCode(FoundationalDiagnosticCodeId),
    DiagnosticScope(FoundationalDiagnosticScopeId),
}

impl FoundationalBoundaryEvidenceSupportContextAttachment {
    pub const fn layer_kind(&self) -> FoundationalBoundaryEvidenceProvenanceLayerKind {
        FoundationalBoundaryEvidenceProvenanceLayerKind::SupportContextAttachment
    }

    pub fn diagnostic_code(code: FoundationalDiagnosticCodeId) -> Self {
        Self::DiagnosticCode(code)
    }

    pub fn diagnostic_scope(scope: FoundationalDiagnosticScopeId) -> Self {
        Self::DiagnosticScope(scope)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceProvenanceArtifact {
    locality: FoundationalBoundaryEvidenceLocality,
    freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
    source_basis: FoundationalBoundaryEvidenceSourceBasis,
    authority_path: Option<FoundationalBoundaryEvidenceAuthorityPath>,
    strategy_basis: Option<FoundationalBoundaryEvidenceStrategyBasis>,
    profile_basis: Option<FoundationalBoundaryEvidenceProfileBasis>,
    comparison_basis: Option<FoundationalBoundaryEvidenceComparisonBasis>,
    canonical_digest_basis: Option<FoundationalBoundaryEvidenceCanonicalDigestBasis>,
    support_context_attachments: Vec<FoundationalBoundaryEvidenceSupportContextAttachment>,
}

impl FoundationalBoundaryEvidenceProvenanceArtifact {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        locality: FoundationalBoundaryEvidenceLocality,
        freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
        source_basis: FoundationalBoundaryEvidenceSourceBasis,
        authority_path: Option<FoundationalBoundaryEvidenceAuthorityPath>,
        strategy_basis: Option<FoundationalBoundaryEvidenceStrategyBasis>,
        profile_basis: Option<FoundationalBoundaryEvidenceProfileBasis>,
        comparison_basis: Option<FoundationalBoundaryEvidenceComparisonBasis>,
        canonical_digest_basis: Option<FoundationalBoundaryEvidenceCanonicalDigestBasis>,
        support_context_attachments: Vec<FoundationalBoundaryEvidenceSupportContextAttachment>,
    ) -> Self {
        Self {
            locality,
            freshness_posture,
            source_basis,
            authority_path,
            strategy_basis,
            profile_basis,
            comparison_basis,
            canonical_digest_basis,
            support_context_attachments,
        }
    }

    pub const fn locality(&self) -> FoundationalBoundaryEvidenceLocality {
        self.locality
    }

    pub const fn freshness_posture(&self) -> FoundationalBoundaryEvidenceFreshnessPosture {
        self.freshness_posture
    }

    pub const fn source_basis(&self) -> &FoundationalBoundaryEvidenceSourceBasis {
        &self.source_basis
    }

    pub fn authority_path(&self) -> Option<&FoundationalBoundaryEvidenceAuthorityPath> {
        self.authority_path.as_ref()
    }

    pub fn strategy_basis(&self) -> Option<&FoundationalBoundaryEvidenceStrategyBasis> {
        self.strategy_basis.as_ref()
    }

    pub fn profile_basis(&self) -> Option<&FoundationalBoundaryEvidenceProfileBasis> {
        self.profile_basis.as_ref()
    }

    pub fn comparison_basis(&self) -> Option<&FoundationalBoundaryEvidenceComparisonBasis> {
        self.comparison_basis.as_ref()
    }

    pub fn canonical_digest_basis(
        &self,
    ) -> Option<&FoundationalBoundaryEvidenceCanonicalDigestBasis> {
        self.canonical_digest_basis.as_ref()
    }

    pub fn support_context_attachments(
        &self,
    ) -> &[FoundationalBoundaryEvidenceSupportContextAttachment] {
        &self.support_context_attachments
    }

    pub fn has_layer(&self, layer: FoundationalBoundaryEvidenceProvenanceLayerKind) -> bool {
        match layer {
            FoundationalBoundaryEvidenceProvenanceLayerKind::SourceBasis => true,
            FoundationalBoundaryEvidenceProvenanceLayerKind::AuthorityPath => {
                self.authority_path.is_some()
            }
            FoundationalBoundaryEvidenceProvenanceLayerKind::StrategyBasis => {
                self.strategy_basis.is_some()
            }
            FoundationalBoundaryEvidenceProvenanceLayerKind::ProfileBasis => {
                self.profile_basis.is_some()
            }
            FoundationalBoundaryEvidenceProvenanceLayerKind::ComparisonBasis => {
                self.comparison_basis.is_some()
            }
            FoundationalBoundaryEvidenceProvenanceLayerKind::CanonicalDigestBasis => {
                self.canonical_digest_basis.is_some()
            }
            FoundationalBoundaryEvidenceProvenanceLayerKind::SupportContextAttachment => {
                !self.support_context_attachments.is_empty()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceProvenanceConstructionDenial {
    ReplayDerivedLocalityRequiresReplayFreshness,
    RestoredReadmittedLocalityRequiresRestoredFreshness,
    CurrentOrBranchLocalLocalityMustNotUseReplayFreshness,
    CurrentOrBranchLocalLocalityMustNotUseRestoredFreshness,
}
