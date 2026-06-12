use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, MergeDeclarationIdentityTag};

use super::authority::BridgeMergeAuthorityBasis;
use super::ontology::BridgeMergeOntologyMappingSurface;
use super::taxonomy::{
    BridgeMergeAuthoritativeLineageDisposition, BridgeMergeCausalFrontierDisposition,
    BridgeMergeConsumptionClass, BridgeMergeSchemaPolicyDisposition,
    BridgeMergeStructuralAdvisoryDisposition,
};

pub type MergeHistoryDeclarationIdentity = BridgeIdentity<MergeDeclarationIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeHistoryDeclaration {
    declaration_identity: MergeHistoryDeclarationIdentity,
    bridge_class: BridgeMergeConsumptionClass,
    ontology_mapping: BridgeMergeOntologyMappingSurface,
    authority_basis: BridgeMergeAuthorityBasis,
    authoritative_lineage: BridgeMergeAuthoritativeLineageDisposition,
    causal_frontier: BridgeMergeCausalFrontierDisposition,
    schema_policy: BridgeMergeSchemaPolicyDisposition,
    structural_advisory: BridgeMergeStructuralAdvisoryDisposition,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl MergeHistoryDeclaration {
    pub fn new(
        declaration_identity: MergeHistoryDeclarationIdentity,
        bridge_class: BridgeMergeConsumptionClass,
        ontology_mapping: BridgeMergeOntologyMappingSurface,
        authority_basis: BridgeMergeAuthorityBasis,
    ) -> Self {
        Self::from_parts(
            declaration_identity,
            bridge_class,
            ontology_mapping,
            authority_basis,
            BridgeMergeAuthoritativeLineageDisposition::CanonicalSuccessor,
            BridgeMergeCausalFrontierDisposition::Admitted,
            BridgeMergeSchemaPolicyDisposition::Admitted,
            BridgeMergeStructuralAdvisoryDisposition::NotConsulted,
        )
    }

    fn from_parts(
        declaration_identity: MergeHistoryDeclarationIdentity,
        bridge_class: BridgeMergeConsumptionClass,
        ontology_mapping: BridgeMergeOntologyMappingSurface,
        authority_basis: BridgeMergeAuthorityBasis,
        authoritative_lineage: BridgeMergeAuthoritativeLineageDisposition,
        causal_frontier: BridgeMergeCausalFrontierDisposition,
        schema_policy: BridgeMergeSchemaPolicyDisposition,
        structural_advisory: BridgeMergeStructuralAdvisoryDisposition,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "merge-history-declaration|id={}|class:{bridge_class:?}|mapping={}|authority={}|lineage:{authoritative_lineage:?}|causal:{causal_frontier:?}|policy:{schema_policy:?}|structural:{structural_advisory:?}",
            declaration_identity.as_str(),
            ontology_mapping.digest(),
            authority_basis.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            declaration_identity,
            bridge_class,
            ontology_mapping,
            authority_basis,
            authoritative_lineage,
            causal_frontier,
            schema_policy,
            structural_advisory,
            canonical_basis,
            digest: Arc::from(format!("merge-history-declaration:sha256:{digest:x}")),
        }
    }

    pub fn with_authoritative_lineage(
        self,
        authoritative_lineage: BridgeMergeAuthoritativeLineageDisposition,
    ) -> Self {
        Self::from_parts(
            self.declaration_identity,
            self.bridge_class,
            self.ontology_mapping,
            self.authority_basis,
            authoritative_lineage,
            self.causal_frontier,
            self.schema_policy,
            self.structural_advisory,
        )
    }

    pub fn with_causal_frontier(
        self,
        causal_frontier: BridgeMergeCausalFrontierDisposition,
    ) -> Self {
        Self::from_parts(
            self.declaration_identity,
            self.bridge_class,
            self.ontology_mapping,
            self.authority_basis,
            self.authoritative_lineage,
            causal_frontier,
            self.schema_policy,
            self.structural_advisory,
        )
    }

    pub fn with_schema_policy(self, schema_policy: BridgeMergeSchemaPolicyDisposition) -> Self {
        Self::from_parts(
            self.declaration_identity,
            self.bridge_class,
            self.ontology_mapping,
            self.authority_basis,
            self.authoritative_lineage,
            self.causal_frontier,
            schema_policy,
            self.structural_advisory,
        )
    }

    pub fn with_structural_advisory(
        self,
        structural_advisory: BridgeMergeStructuralAdvisoryDisposition,
    ) -> Self {
        Self::from_parts(
            self.declaration_identity,
            self.bridge_class,
            self.ontology_mapping,
            self.authority_basis,
            self.authoritative_lineage,
            self.causal_frontier,
            self.schema_policy,
            structural_advisory,
        )
    }

    pub fn declaration_identity(&self) -> &MergeHistoryDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn bridge_class(&self) -> BridgeMergeConsumptionClass {
        self.bridge_class
    }

    pub fn ontology_mapping(&self) -> &BridgeMergeOntologyMappingSurface {
        &self.ontology_mapping
    }

    pub fn authority_basis(&self) -> &BridgeMergeAuthorityBasis {
        &self.authority_basis
    }

    pub fn authoritative_lineage(&self) -> BridgeMergeAuthoritativeLineageDisposition {
        self.authoritative_lineage
    }

    pub fn causal_frontier(&self) -> BridgeMergeCausalFrontierDisposition {
        self.causal_frontier
    }

    pub fn schema_policy(&self) -> BridgeMergeSchemaPolicyDisposition {
        self.schema_policy
    }

    pub fn structural_advisory(&self) -> BridgeMergeStructuralAdvisoryDisposition {
        self.structural_advisory
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

    use crate::merge::{
        BridgeMergeAuthoritativeLineageDisposition, BridgeMergeAuthorityBasis,
        BridgeMergeAuthorityBasisKind, BridgeMergeCausalFrontierDisposition,
        BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface,
        BridgeMergeParentOrderProof, BridgeMergeSchemaPolicyDisposition,
        BridgeMergeStructuralAdvisoryDisposition, MergeHistoryDeclaration,
        MergeHistoryDeclarationIdentity,
    };

    #[test]
    fn merge_history_declaration_is_canonical_for_same_inputs() {
        let mapping = BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1");
        let parent_order = BridgeMergeParentOrderProof::new(vec![
            crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
            crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
        ]);
        let authority = BridgeMergeAuthorityBasis::new(
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            "merge-artifact:commit-c",
            "rel-merge-v1",
            "schema-policy-v1",
            parent_order,
        );

        let left = MergeHistoryDeclaration::new(
            MergeHistoryDeclarationIdentity::new("merge:ordered-history"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            mapping.clone(),
            authority.clone(),
        );
        let right = MergeHistoryDeclaration::new(
            MergeHistoryDeclarationIdentity::new("merge:ordered-history"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            mapping,
            authority,
        );

        assert_eq!(left, right);
        assert_eq!(
            left.bridge_class(),
            BridgeMergeConsumptionClass::AspectReconciliationMerge
        );
        assert_eq!(
            left.authority_basis().parent_order_proof().parents(),
            &[
                crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
                crate::truth_identity_fixtures::truth_commit_fixture("parent-b")
            ]
        );
    }

    #[test]
    fn merge_history_declaration_carries_stage_inputs_in_digest() {
        let declaration = MergeHistoryDeclaration::new(
            MergeHistoryDeclarationIdentity::new("merge:ordered-history"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
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
        )
        .with_authoritative_lineage(BridgeMergeAuthoritativeLineageDisposition::CanonicalSuccessor)
        .with_causal_frontier(BridgeMergeCausalFrontierDisposition::Admitted)
        .with_schema_policy(BridgeMergeSchemaPolicyDisposition::Admitted)
        .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);

        assert_eq!(
            declaration.structural_advisory(),
            BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent
        );
        assert_eq!(
            declaration.authoritative_lineage(),
            BridgeMergeAuthoritativeLineageDisposition::CanonicalSuccessor
        );
        assert_eq!(
            declaration.causal_frontier(),
            BridgeMergeCausalFrontierDisposition::Admitted
        );
        assert_eq!(
            declaration.schema_policy(),
            BridgeMergeSchemaPolicyDisposition::Admitted
        );
    }
}
