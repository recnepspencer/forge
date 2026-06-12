use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeBuildError, BridgeBuildErrorKind};

use super::{
    AdmittedStructuralComparisonContract, StructuralComparisonMode, StructuralIdentityDeclaration,
    StructuralTruthViewBasisKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedStructuralIdentityDeclaration {
    declaration: StructuralIdentityDeclaration,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ValidatedStructuralIdentityDeclaration {
    pub(crate) fn from_contract(contract: &AdmittedStructuralComparisonContract) -> Self {
        let declaration = contract.validated_declaration().declaration().clone();
        let canonical_basis = Arc::<str>::from(format!(
            "validated-structural-declaration|contract={}|declaration={}",
            contract.digest(),
            declaration.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration,
            canonical_basis,
            digest: Arc::from(format!(
                "validated-structural-declaration:sha256:{digest:x}"
            )),
        }
    }

    pub(crate) fn new(
        declaration: StructuralIdentityDeclaration,
    ) -> Result<Self, BridgeBuildError> {
        if declaration.schema_identity() != declaration.equivalence_contract().schema_identity() {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::StructuralComparisonModeMismatch,
                format!(
                    "Structural declaration `{}` used schema `{}` but equivalence contract was declared for schema `{}`.",
                    declaration.declaration_identity().as_str(),
                    declaration.schema_identity().as_str(),
                    declaration.equivalence_contract().schema_identity().as_str(),
                ),
            ));
        }

        match (declaration.comparison_mode(), declaration.truth_view_basis().basis_kind()) {
            (
                StructuralComparisonMode::AdvisoryRemap,
                StructuralTruthViewBasisKind::ExplicitBranchPairComparison,
            ) => {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::StructuralComparisonModeMismatch,
                    format!(
                        "Structural declaration `{}` used branch-pair comparison truth-view basis for advisory remap mode.",
                        declaration.declaration_identity().as_str(),
                    ),
                ))
            }
            (
                StructuralComparisonMode::BranchComparison,
                StructuralTruthViewBasisKind::ExplicitBranchPairComparison,
            )
            | (
                StructuralComparisonMode::AdvisoryRemap,
                StructuralTruthViewBasisKind::ExplicitSnapshot
                | StructuralTruthViewBasisKind::ExplicitHistoricalVersion
                | StructuralTruthViewBasisKind::ExplicitBranchHead,
            ) => {}
            (StructuralComparisonMode::BranchComparison, _) => {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::StructuralComparisonModeMismatch,
                    format!(
                        "Structural declaration `{}` must use an explicit branch-pair truth-view basis for branch comparison mode.",
                        declaration.declaration_identity().as_str(),
                    ),
                ))
            }
        }

        let canonical_basis = Arc::<str>::from(format!(
            "validated-structural-declaration|declaration={}",
            declaration.digest()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            declaration,
            canonical_basis,
            digest: Arc::from(format!(
                "validated-structural-declaration:sha256:{digest:x}"
            )),
        })
    }

    pub fn declaration(&self) -> &StructuralIdentityDeclaration {
        &self.declaration
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
    use super::ValidatedStructuralIdentityDeclaration;

    use crate::snapshot::BridgeTruthViewSelector;
    use crate::structural::{
        StructuralComparisonMode, StructuralFingerprintEquivalenceContract,
        StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
        StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
        StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity,
        StructuralSchemaIdentity, StructuralTruthViewBasis, StructuralTruthViewBasisKind,
    };

    fn contract(schema: &str) -> StructuralFingerprintEquivalenceContract {
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new(schema),
            StructuralFingerprintFamily::TopologyFingerprint,
            "topology-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        )
    }

    #[test]
    fn validation_rejects_branch_pair_basis_for_advisory_remap() {
        let declaration = StructuralIdentityDeclaration::advisory_remap(
            StructuralIdentityDeclarationIdentity::new("structural:bad-remap"),
            StructuralSchemaIdentity::new("schema:geometry"),
            contract("schema:geometry"),
            StructuralTruthViewBasis::explicit_branch_pair(
                BridgeTruthViewSelector::branch_head(
                    crate::truth_identity_fixtures::truth_branch_fixture("left"),
                ),
                BridgeTruthViewSelector::branch_head(
                    crate::truth_identity_fixtures::truth_branch_fixture("right"),
                ),
            ),
        );

        let error = ValidatedStructuralIdentityDeclaration::new(declaration)
            .expect_err("branch-pair basis should be rejected for advisory remap");
        assert_eq!(
            error.kind(),
            crate::error::BridgeBuildErrorKind::StructuralComparisonModeMismatch
        );
    }

    #[test]
    fn validation_accepts_branch_comparison_with_branch_pair_basis() {
        let declaration = StructuralIdentityDeclaration::branch_comparison(
            StructuralIdentityDeclarationIdentity::new("structural:branch-compare"),
            StructuralSchemaIdentity::new("schema:geometry"),
            contract("schema:geometry"),
            StructuralTruthViewBasis::explicit_branch_pair(
                BridgeTruthViewSelector::branch_snapshot(
                    crate::truth_identity_fixtures::truth_branch_fixture("left"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-left"),
                ),
                BridgeTruthViewSelector::branch_snapshot(
                    crate::truth_identity_fixtures::truth_branch_fixture("right"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-right"),
                ),
            ),
        );

        let validated = ValidatedStructuralIdentityDeclaration::new(declaration)
            .expect("branch comparison declaration should validate");
        assert_eq!(
            validated.declaration().comparison_mode(),
            StructuralComparisonMode::BranchComparison
        );
        assert_eq!(
            validated.declaration().truth_view_basis().basis_kind(),
            StructuralTruthViewBasisKind::ExplicitBranchPairComparison
        );
    }
}
