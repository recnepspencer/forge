use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    BridgeIdentity, MergeAuthorityBasisIdentityTag, MergeDeclarationIdentityTag,
    MergeOntologyMappingIdentityTag, MergeParentOrderIdentityTag,
};
use crate::input::envelope::TruthCommitIdentity;

use super::taxonomy::{
    BridgeMergeAuthoritativeLineageDisposition, BridgeMergeAuthorityBasisKind,
    BridgeMergeCausalFrontierDisposition, BridgeMergeConsumptionClass,
    BridgeMergeOntologyLoweringKind, BridgeMergeSchemaPolicyDisposition,
    BridgeMergeStructuralAdvisoryDisposition, CanonicalRelationalMergeClass,
};

pub type MergeHistoryDeclarationIdentity = BridgeIdentity<MergeDeclarationIdentityTag>;
pub type BridgeMergeAuthorityBasisIdentity = BridgeIdentity<MergeAuthorityBasisIdentityTag>;
pub type BridgeMergeOntologyMappingSurfaceIdentity =
    BridgeIdentity<MergeOntologyMappingIdentityTag>;
pub type BridgeMergeParentOrderProofIdentity = BridgeIdentity<MergeParentOrderIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeOntologyMappingEntry {
    canonical_relational_class: CanonicalRelationalMergeClass,
    bridge_class: BridgeMergeConsumptionClass,
    lowering_kind: BridgeMergeOntologyLoweringKind,
    canonical_basis: Arc<str>,
}

impl BridgeMergeOntologyMappingEntry {
    pub fn direct_wrapper(
        canonical_relational_class: CanonicalRelationalMergeClass,
        bridge_class: BridgeMergeConsumptionClass,
    ) -> Self {
        Self {
            canonical_relational_class,
            bridge_class,
            lowering_kind: BridgeMergeOntologyLoweringKind::DirectWrapper,
            canonical_basis: Arc::from(format!(
                "merge-ontology-entry|canonical:{canonical_relational_class:?}|bridge:{bridge_class:?}|lowering:{:?}",
                BridgeMergeOntologyLoweringKind::DirectWrapper
            )),
        }
    }

    pub fn canonical_relational_class(&self) -> CanonicalRelationalMergeClass {
        self.canonical_relational_class
    }

    pub fn bridge_class(&self) -> BridgeMergeConsumptionClass {
        self.bridge_class
    }

    pub fn lowering_kind(&self) -> BridgeMergeOntologyLoweringKind {
        self.lowering_kind
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeOntologyMappingSurface {
    mapping_identity: BridgeMergeOntologyMappingSurfaceIdentity,
    ontology_version: Arc<str>,
    entries: Arc<[BridgeMergeOntologyMappingEntry]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMergeOntologyMappingSurface {
    pub fn new(
        ontology_version: impl Into<Arc<str>>,
        mut entries: Vec<BridgeMergeOntologyMappingEntry>,
    ) -> Self {
        entries.sort_by(|left, right| {
            left.canonical_relational_class()
                .cmp(&right.canonical_relational_class())
                .then_with(|| left.bridge_class().cmp(&right.bridge_class()))
                .then_with(|| left.lowering_kind().cmp(&right.lowering_kind()))
        });
        let ontology_version = ontology_version.into();
        let canonical_basis = Arc::<str>::from(format!(
            "merge-ontology-mapping-surface|version={}|entries={}",
            ontology_version.as_ref(),
            entries
                .iter()
                .map(BridgeMergeOntologyMappingEntry::canonical_basis)
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            mapping_identity: BridgeMergeOntologyMappingSurfaceIdentity::new(format!(
                "merge-ontology-mapping-surface:sha256:{digest:x}"
            )),
            ontology_version,
            entries: Arc::from(entries),
            canonical_basis,
            digest: Arc::from(format!("merge-ontology-mapping-surface:sha256:{digest:x}")),
        }
    }

    pub fn direct_phase_m9_0(ontology_version: impl Into<Arc<str>>) -> Self {
        Self::new(
            ontology_version,
            vec![
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::AspectReconciliation,
                    BridgeMergeConsumptionClass::AspectReconciliationMerge,
                ),
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::Deletion,
                    BridgeMergeConsumptionClass::DeletionMerge,
                ),
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::TopologyRewire,
                    BridgeMergeConsumptionClass::TopologyRewireMerge,
                ),
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::PolicyResolvedConflict,
                    BridgeMergeConsumptionClass::PolicyResolvedConflictMerge,
                ),
            ],
        )
    }

    pub fn mapping_identity(&self) -> &BridgeMergeOntologyMappingSurfaceIdentity {
        &self.mapping_identity
    }

    pub fn ontology_version(&self) -> &str {
        self.ontology_version.as_ref()
    }

    pub fn entries(&self) -> &[BridgeMergeOntologyMappingEntry] {
        &self.entries
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeParentOrderProof {
    proof_identity: BridgeMergeParentOrderProofIdentity,
    parents: Arc<[TruthCommitIdentity]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMergeParentOrderProof {
    pub fn new(parents: Vec<TruthCommitIdentity>) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "merge-parent-order-proof|parents={}",
            parents
                .iter()
                .map(TruthCommitIdentity::as_str)
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            proof_identity: BridgeMergeParentOrderProofIdentity::new(format!(
                "merge-parent-order-proof:sha256:{digest:x}"
            )),
            parents: Arc::from(parents),
            canonical_basis,
            digest: Arc::from(format!("merge-parent-order-proof:sha256:{digest:x}")),
        }
    }

    pub fn proof_identity(&self) -> &BridgeMergeParentOrderProofIdentity {
        &self.proof_identity
    }

    pub fn parents(&self) -> &[TruthCommitIdentity] {
        &self.parents
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeAuthorityBasis {
    basis_identity: BridgeMergeAuthorityBasisIdentity,
    basis_kind: BridgeMergeAuthorityBasisKind,
    artifact_identity: Arc<str>,
    ontology_version: Arc<str>,
    schema_policy_descriptor_version: Arc<str>,
    parent_order_proof: BridgeMergeParentOrderProof,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMergeAuthorityBasis {
    pub fn new(
        basis_kind: BridgeMergeAuthorityBasisKind,
        artifact_identity: impl Into<Arc<str>>,
        ontology_version: impl Into<Arc<str>>,
        schema_policy_descriptor_version: impl Into<Arc<str>>,
        parent_order_proof: BridgeMergeParentOrderProof,
    ) -> Self {
        let artifact_identity = artifact_identity.into();
        let ontology_version = ontology_version.into();
        let schema_policy_descriptor_version = schema_policy_descriptor_version.into();
        let canonical_basis = Arc::<str>::from(format!(
            "merge-authority-basis|kind:{basis_kind:?}|artifact={}|ontology-version={}|policy-version={}|parent-order={}",
            artifact_identity.as_ref(),
            ontology_version.as_ref(),
            schema_policy_descriptor_version.as_ref(),
            parent_order_proof.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            basis_identity: BridgeMergeAuthorityBasisIdentity::new(format!(
                "merge-authority-basis:sha256:{digest:x}"
            )),
            basis_kind,
            artifact_identity,
            ontology_version,
            schema_policy_descriptor_version,
            parent_order_proof,
            canonical_basis,
            digest: Arc::from(format!("merge-authority-basis:sha256:{digest:x}")),
        }
    }

    pub fn basis_identity(&self) -> &BridgeMergeAuthorityBasisIdentity {
        &self.basis_identity
    }

    pub fn basis_kind(&self) -> BridgeMergeAuthorityBasisKind {
        self.basis_kind
    }

    pub fn artifact_identity(&self) -> &str {
        self.artifact_identity.as_ref()
    }

    pub fn ontology_version(&self) -> &str {
        self.ontology_version.as_ref()
    }

    pub fn schema_policy_descriptor_version(&self) -> &str {
        self.schema_policy_descriptor_version.as_ref()
    }

    pub fn parent_order_proof(&self) -> &BridgeMergeParentOrderProof {
        &self.parent_order_proof
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

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
    use super::{
        BridgeMergeAuthorityBasis, BridgeMergeOntologyMappingSurface, BridgeMergeParentOrderProof,
        MergeHistoryDeclaration, MergeHistoryDeclarationIdentity,
    };
    use crate::input::envelope::TruthCommitIdentity;
    use crate::merge::{
        BridgeMergeAuthoritativeLineageDisposition, BridgeMergeAuthorityBasisKind,
        BridgeMergeCausalFrontierDisposition, BridgeMergeConsumptionClass,
        BridgeMergeSchemaPolicyDisposition, BridgeMergeStructuralAdvisoryDisposition,
    };

    #[test]
    fn merge_ontology_mapping_surface_is_canonical_for_same_inputs() {
        let left = BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1");
        let right = BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1");

        assert_eq!(left, right);
        assert_eq!(left.entries().len(), 4);
    }

    #[test]
    fn merge_history_declaration_is_canonical_for_same_inputs() {
        let mapping = BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1");
        let parent_order = BridgeMergeParentOrderProof::new(vec![
            TruthCommitIdentity::new("parent-a"),
            TruthCommitIdentity::new("parent-b"),
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
        assert!(left
            .canonical_basis()
            .contains("class:AspectReconciliationMerge"));
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
                    TruthCommitIdentity::new("parent-a"),
                    TruthCommitIdentity::new("parent-b"),
                ]),
            ),
        )
        .with_authoritative_lineage(BridgeMergeAuthoritativeLineageDisposition::CanonicalSuccessor)
        .with_causal_frontier(BridgeMergeCausalFrontierDisposition::Admitted)
        .with_schema_policy(BridgeMergeSchemaPolicyDisposition::Admitted)
        .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);

        assert!(declaration
            .canonical_basis()
            .contains("structural:AdvisoryConsistent"));
    }
}
