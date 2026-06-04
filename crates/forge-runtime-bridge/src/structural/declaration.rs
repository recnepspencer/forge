use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    BridgeIdentity, StructuralDeclarationIdentityTag, StructuralEquivalenceContractIdentityTag,
    StructuralSchemaIdentityTag, StructuralTruthViewBasisIdentityTag,
};
use crate::snapshot::BridgeTruthViewSelector;

use super::taxonomy::{
    StructuralCandidateSearchScope, StructuralComparisonMode, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralTruthViewBasisKind,
};

pub type StructuralIdentityDeclarationIdentity = BridgeIdentity<StructuralDeclarationIdentityTag>;
pub type StructuralSchemaIdentity = BridgeIdentity<StructuralSchemaIdentityTag>;
pub type StructuralEquivalenceContractIdentity =
    BridgeIdentity<StructuralEquivalenceContractIdentityTag>;
pub type StructuralTruthViewBasisIdentity = BridgeIdentity<StructuralTruthViewBasisIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralFingerprintEquivalenceContract {
    contract_identity: StructuralEquivalenceContractIdentity,
    schema_identity: StructuralSchemaIdentity,
    fingerprint_family: StructuralFingerprintFamily,
    semantics_version: Arc<str>,
    normalization_rule: StructuralFingerprintNormalizationRule,
    ordering_rule: StructuralFingerprintOrderingRule,
    omission_policy: StructuralFingerprintOmissionPolicy,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl StructuralFingerprintEquivalenceContract {
    pub fn new(
        schema_identity: StructuralSchemaIdentity,
        fingerprint_family: StructuralFingerprintFamily,
        semantics_version: impl Into<Arc<str>>,
        normalization_rule: StructuralFingerprintNormalizationRule,
        ordering_rule: StructuralFingerprintOrderingRule,
        omission_policy: StructuralFingerprintOmissionPolicy,
    ) -> Self {
        let semantics_version = semantics_version.into();
        let canonical_basis = Arc::<str>::from(format!(
            "structural-equivalence-contract|schema={}|family:{fingerprint_family:?}|version={}|normalization:{normalization_rule:?}|ordering:{ordering_rule:?}|omission:{omission_policy:?}",
            schema_identity.as_str(),
            semantics_version.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let contract_identity = StructuralEquivalenceContractIdentity::new(format!(
            "structural-equivalence-contract:sha256:{digest:x}"
        ));

        Self {
            contract_identity,
            schema_identity,
            fingerprint_family,
            semantics_version,
            normalization_rule,
            ordering_rule,
            omission_policy,
            canonical_basis,
            digest: Arc::from(format!("structural-equivalence-contract:sha256:{digest:x}")),
        }
    }

    pub fn contract_identity(&self) -> &StructuralEquivalenceContractIdentity {
        &self.contract_identity
    }

    pub fn schema_identity(&self) -> &StructuralSchemaIdentity {
        &self.schema_identity
    }

    pub fn fingerprint_family(&self) -> StructuralFingerprintFamily {
        self.fingerprint_family
    }

    pub fn semantics_version(&self) -> &str {
        self.semantics_version.as_ref()
    }

    pub fn normalization_rule(&self) -> StructuralFingerprintNormalizationRule {
        self.normalization_rule
    }

    pub fn ordering_rule(&self) -> StructuralFingerprintOrderingRule {
        self.ordering_rule
    }

    pub fn omission_policy(&self) -> StructuralFingerprintOmissionPolicy {
        self.omission_policy
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralTruthViewBasis {
    Single {
        basis_identity: StructuralTruthViewBasisIdentity,
        basis_kind: StructuralTruthViewBasisKind,
        selector: BridgeTruthViewSelector,
        canonical_basis: Arc<str>,
        digest: Arc<str>,
    },
    BranchPair {
        basis_identity: StructuralTruthViewBasisIdentity,
        left_selector: BridgeTruthViewSelector,
        right_selector: BridgeTruthViewSelector,
        canonical_basis: Arc<str>,
        digest: Arc<str>,
    },
}

impl StructuralTruthViewBasis {
    pub fn explicit_snapshot(selector: BridgeTruthViewSelector) -> Self {
        Self::single(StructuralTruthViewBasisKind::ExplicitSnapshot, selector)
    }

    pub fn explicit_historical_version(selector: BridgeTruthViewSelector) -> Self {
        Self::single(
            StructuralTruthViewBasisKind::ExplicitHistoricalVersion,
            selector,
        )
    }

    pub fn explicit_branch_head(selector: BridgeTruthViewSelector) -> Self {
        Self::single(StructuralTruthViewBasisKind::ExplicitBranchHead, selector)
    }

    pub fn explicit_branch_pair(
        left_selector: BridgeTruthViewSelector,
        right_selector: BridgeTruthViewSelector,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "structural-truth-view-basis|kind:{:?}|left={}|right={}",
            StructuralTruthViewBasisKind::ExplicitBranchPairComparison,
            left_selector.canonical_basis(),
            right_selector.canonical_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self::BranchPair {
            basis_identity: StructuralTruthViewBasisIdentity::new(format!(
                "structural-truth-view-basis:sha256:{digest:x}"
            )),
            left_selector,
            right_selector,
            canonical_basis,
            digest: Arc::from(format!("structural-truth-view-basis:sha256:{digest:x}")),
        }
    }

    fn single(basis_kind: StructuralTruthViewBasisKind, selector: BridgeTruthViewSelector) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "structural-truth-view-basis|kind:{basis_kind:?}|selector={}",
            selector.canonical_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self::Single {
            basis_identity: StructuralTruthViewBasisIdentity::new(format!(
                "structural-truth-view-basis:sha256:{digest:x}"
            )),
            basis_kind,
            selector,
            canonical_basis,
            digest: Arc::from(format!("structural-truth-view-basis:sha256:{digest:x}")),
        }
    }

    pub fn basis_identity(&self) -> &StructuralTruthViewBasisIdentity {
        match self {
            Self::Single { basis_identity, .. } | Self::BranchPair { basis_identity, .. } => {
                basis_identity
            }
        }
    }

    pub fn basis_kind(&self) -> StructuralTruthViewBasisKind {
        match self {
            Self::Single { basis_kind, .. } => *basis_kind,
            Self::BranchPair { .. } => StructuralTruthViewBasisKind::ExplicitBranchPairComparison,
        }
    }

    pub fn canonical_basis(&self) -> &str {
        match self {
            Self::Single {
                canonical_basis, ..
            }
            | Self::BranchPair {
                canonical_basis, ..
            } => canonical_basis.as_ref(),
        }
    }

    pub fn digest(&self) -> &str {
        match self {
            Self::Single { digest, .. } | Self::BranchPair { digest, .. } => digest.as_ref(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralIdentityDeclaration {
    declaration_identity: StructuralIdentityDeclarationIdentity,
    schema_identity: StructuralSchemaIdentity,
    equivalence_contract: StructuralFingerprintEquivalenceContract,
    comparison_mode: StructuralComparisonMode,
    truth_view_basis: StructuralTruthViewBasis,
    candidate_scope: StructuralCandidateSearchScope,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl StructuralIdentityDeclaration {
    pub fn new(
        declaration_identity: StructuralIdentityDeclarationIdentity,
        schema_identity: StructuralSchemaIdentity,
        equivalence_contract: StructuralFingerprintEquivalenceContract,
        comparison_mode: StructuralComparisonMode,
        truth_view_basis: StructuralTruthViewBasis,
        candidate_scope: StructuralCandidateSearchScope,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "structural-declaration|id={}|schema={}|equivalence={}|mode:{comparison_mode:?}|truth-view={}|scope:{candidate_scope:?}",
            declaration_identity.as_str(),
            schema_identity.as_str(),
            equivalence_contract.digest(),
            truth_view_basis.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            declaration_identity,
            schema_identity,
            equivalence_contract,
            comparison_mode,
            truth_view_basis,
            candidate_scope,
            canonical_basis,
            digest: Arc::from(format!("structural-declaration:sha256:{digest:x}")),
        }
    }

    pub fn advisory_remap(
        declaration_identity: StructuralIdentityDeclarationIdentity,
        schema_identity: StructuralSchemaIdentity,
        equivalence_contract: StructuralFingerprintEquivalenceContract,
        truth_view_basis: StructuralTruthViewBasis,
    ) -> Self {
        Self::new(
            declaration_identity,
            schema_identity,
            equivalence_contract,
            StructuralComparisonMode::AdvisoryRemap,
            truth_view_basis,
            StructuralCandidateSearchScope::DeclaredStructuralIndexCohort,
        )
    }

    pub fn branch_comparison(
        declaration_identity: StructuralIdentityDeclarationIdentity,
        schema_identity: StructuralSchemaIdentity,
        equivalence_contract: StructuralFingerprintEquivalenceContract,
        truth_view_basis: StructuralTruthViewBasis,
    ) -> Self {
        Self::new(
            declaration_identity,
            schema_identity,
            equivalence_contract,
            StructuralComparisonMode::BranchComparison,
            truth_view_basis,
            StructuralCandidateSearchScope::BranchLocalCohort,
        )
    }

    pub fn declaration_identity(&self) -> &StructuralIdentityDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn schema_identity(&self) -> &StructuralSchemaIdentity {
        &self.schema_identity
    }

    pub fn equivalence_contract(&self) -> &StructuralFingerprintEquivalenceContract {
        &self.equivalence_contract
    }

    pub fn comparison_mode(&self) -> StructuralComparisonMode {
        self.comparison_mode
    }

    pub fn truth_view_basis(&self) -> &StructuralTruthViewBasis {
        &self.truth_view_basis
    }

    pub fn candidate_scope(&self) -> StructuralCandidateSearchScope {
        self.candidate_scope
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        StructuralFingerprintEquivalenceContract, StructuralIdentityDeclaration,
        StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
    };
    use crate::input::envelope::TruthBranchIdentity;
    use crate::snapshot::{BridgeTruthViewSelector, TruthSnapshotIdentity};
    use crate::structural::{
        StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
        StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
    };

    #[test]
    fn structural_equivalence_contract_is_canonical_for_same_inputs() {
        let left = StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::TopologyFingerprint,
            "topology-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        );
        let right = StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::TopologyFingerprint,
            "topology-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        );

        assert_eq!(left, right);
        assert_eq!(left.semantics_version(), "topology-v1");
    }

    #[test]
    fn advisory_remap_uses_declared_structural_index_scope_by_default() {
        let declaration = StructuralIdentityDeclaration::advisory_remap(
            StructuralIdentityDeclarationIdentity::new("structural:geometry-remap"),
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintEquivalenceContract::new(
                StructuralSchemaIdentity::new("schema:geometry"),
                StructuralFingerprintFamily::TopologyFingerprint,
                "topology-v1",
                StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
                StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
                StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
            ),
            StructuralTruthViewBasis::explicit_snapshot(
                BridgeTruthViewSelector::committed_snapshot(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ),
        );

        assert_eq!(
            declaration.canonical_basis(),
            format!(
                "structural-declaration|id=structural:geometry-remap|schema=schema:geometry|equivalence={}|mode:AdvisoryRemap|truth-view={}|scope:DeclaredStructuralIndexCohort",
                declaration.equivalence_contract().digest(),
                declaration.truth_view_basis().digest(),
            )
        );
    }
}
