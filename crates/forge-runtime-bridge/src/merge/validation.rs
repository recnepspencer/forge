use std::collections::BTreeSet;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeBuildError, BridgeBuildErrorKind};

use super::MergeHistoryDeclaration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedMergeHistoryDeclaration {
    declaration: MergeHistoryDeclaration,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ValidatedMergeHistoryDeclaration {
    pub(crate) fn new(declaration: MergeHistoryDeclaration) -> Result<Self, BridgeBuildError> {
        let parent_count = declaration
            .authority_basis()
            .parent_order_proof()
            .parents()
            .len();
        if parent_count < 2 {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::MergeAuthorityBasisMismatch,
                format!(
                    "Merge declaration `{}` requires at least two ordered parents, but authority basis `{}` supplied {}.",
                    declaration.declaration_identity().as_str(),
                    declaration.authority_basis().basis_identity().as_str(),
                    parent_count,
                ),
            ));
        }

        if declaration.ontology_mapping().ontology_version()
            != declaration.authority_basis().ontology_version()
        {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::MergeAuthorityBasisMismatch,
                format!(
                    "Merge declaration `{}` used ontology mapping version `{}` but authority basis `{}` carried `{}`.",
                    declaration.declaration_identity().as_str(),
                    declaration.ontology_mapping().ontology_version(),
                    declaration.authority_basis().basis_identity().as_str(),
                    declaration.authority_basis().ontology_version(),
                ),
            ));
        }

        let mut canonical_classes = BTreeSet::new();
        for entry in declaration.ontology_mapping().entries() {
            if !canonical_classes.insert(entry.canonical_relational_class()) {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::MergeOntologyLoweringMismatch,
                    format!(
                        "Merge declaration `{}` contained duplicate canonical relational merge class entries in mapping `{}`.",
                        declaration.declaration_identity().as_str(),
                        declaration.ontology_mapping().mapping_identity().as_str(),
                    ),
                ));
            }
        }

        if !declaration
            .ontology_mapping()
            .entries()
            .iter()
            .any(|entry| entry.bridge_class() == declaration.bridge_class())
        {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::MergeOntologyLoweringMismatch,
                format!(
                    "Merge declaration `{}` requested bridge class `{:?}` not present in ontology mapping `{}`.",
                    declaration.declaration_identity().as_str(),
                    declaration.bridge_class(),
                    declaration.ontology_mapping().mapping_identity().as_str(),
                ),
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "validated-merge-history-declaration|declaration={}",
            declaration.digest()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Ok(Self {
            declaration,
            canonical_basis,
            digest: Arc::from(format!(
                "validated-merge-history-declaration:sha256:{digest:x}"
            )),
        })
    }

    pub fn declaration(&self) -> &MergeHistoryDeclaration {
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
    use super::ValidatedMergeHistoryDeclaration;

    use crate::merge::{
        BridgeMergeAuthorityBasis, BridgeMergeAuthorityBasisKind, BridgeMergeConsumptionClass,
        BridgeMergeOntologyMappingEntry, BridgeMergeOntologyMappingSurface,
        BridgeMergeParentOrderProof, CanonicalRelationalMergeClass, MergeHistoryDeclaration,
        MergeHistoryDeclarationIdentity,
    };

    #[test]
    fn validation_rejects_single_parent_authority_basis() {
        let declaration = MergeHistoryDeclaration::new(
            MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:bad-parent-count"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
            BridgeMergeAuthorityBasis::new(
                BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
                "merge-artifact:commit-c",
                "rel-merge-v1",
                "schema-policy-v1",
                BridgeMergeParentOrderProof::new(vec![
                    crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
                ]),
            ),
        );

        let error = ValidatedMergeHistoryDeclaration::new(declaration)
            .expect_err("single parent merge authority should be rejected");
        assert_eq!(
            error.kind(),
            crate::error::BridgeBuildErrorKind::MergeAuthorityBasisMismatch
        );
    }

    #[test]
    fn validation_accepts_lossless_many_to_one_bridge_class_lowering() {
        let declaration = MergeHistoryDeclaration::new(
            MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:many-to-one-bridge-class"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            BridgeMergeOntologyMappingSurface::new(
                "rel-merge-v1",
                vec![
                    BridgeMergeOntologyMappingEntry::direct_wrapper(
                        CanonicalRelationalMergeClass::AspectReconciliation,
                        BridgeMergeConsumptionClass::AspectReconciliationMerge,
                    ),
                    BridgeMergeOntologyMappingEntry::direct_wrapper(
                        CanonicalRelationalMergeClass::Deletion,
                        BridgeMergeConsumptionClass::AspectReconciliationMerge,
                    ),
                ],
            ),
            BridgeMergeAuthorityBasis::new(
                BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
                "merge-artifact:commit-c",
                "rel-merge-v1",
                "schema-policy-v1",
                BridgeMergeParentOrderProof::new(vec![
                    crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
                    crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
                ]),
            ),
        );

        let validated = ValidatedMergeHistoryDeclaration::new(declaration)
            .expect("lossless many-to-one bridge class lowering should remain admissible");
        assert_eq!(
            validated.declaration().ontology_mapping().entries().len(),
            2
        );
    }
}
