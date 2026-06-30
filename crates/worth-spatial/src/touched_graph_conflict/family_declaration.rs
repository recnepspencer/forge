use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::family_identity::SpatialConflictFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConflictLocalityAuthorityRequirement {
    SpatialTouchAuthorityRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConflictPriorProofPosture {
    NoPriorProofRequired,
    ReplayUndoOrTransactionRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConflictDiagnosticWitness {
    SpatialTouchDigest,
    EvidenceFamilyDigest,
    ReplayBoundaryScope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConflictSelectionProductPosture {
    DeclarationOnlySelectionRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConflictFamilyDeclarationDigest(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConflictFamilyDeclaration {
    identity: SpatialConflictFamilyIdentity,
    locality_authority_requirement: SpatialConflictLocalityAuthorityRequirement,
    primary_overlap_category: ConflictOverlapCategory,
    secondary_overlap_category: Option<ConflictOverlapCategory>,
    routing_posture: ConflictRoutingPosture,
    prior_proof_posture: SpatialConflictPriorProofPosture,
    diagnostic_witness: SpatialConflictDiagnosticWitness,
    selection_product_posture: SpatialConflictSelectionProductPosture,
    declaration_digest: SpatialConflictFamilyDeclarationDigest,
}

impl SpatialConflictFamilyDeclaration {
    pub(crate) fn new(
        identity: SpatialConflictFamilyIdentity,
        locality_authority_requirement: SpatialConflictLocalityAuthorityRequirement,
        primary_overlap_category: ConflictOverlapCategory,
        secondary_overlap_category: Option<ConflictOverlapCategory>,
        routing_posture: ConflictRoutingPosture,
        prior_proof_posture: SpatialConflictPriorProofPosture,
        diagnostic_witness: SpatialConflictDiagnosticWitness,
        selection_product_posture: SpatialConflictSelectionProductPosture,
    ) -> Self {
        let declaration_digest = SpatialConflictFamilyDeclarationDigest(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:conflict-family-declaration:v1".to_string(),
                identity.digest(),
                format!("locality:{locality_authority_requirement:?}"),
                format!("overlap:{primary_overlap_category:?}"),
                format!("overlap-secondary:{secondary_overlap_category:?}"),
                format!("routing:{routing_posture:?}"),
                format!("prior-proof:{prior_proof_posture:?}"),
                format!("diagnostic:{diagnostic_witness:?}"),
                format!("selection:{selection_product_posture:?}"),
            ],
        ));
        Self {
            identity,
            locality_authority_requirement,
            primary_overlap_category,
            secondary_overlap_category,
            routing_posture,
            prior_proof_posture,
            diagnostic_witness,
            selection_product_posture,
            declaration_digest,
        }
    }

    pub const fn identity(&self) -> SpatialConflictFamilyIdentity {
        self.identity
    }

    pub const fn locality_authority_requirement(
        &self,
    ) -> SpatialConflictLocalityAuthorityRequirement {
        self.locality_authority_requirement
    }

    pub const fn primary_overlap_category(&self) -> ConflictOverlapCategory {
        self.primary_overlap_category
    }

    pub const fn secondary_overlap_category(&self) -> Option<ConflictOverlapCategory> {
        self.secondary_overlap_category
    }

    pub const fn routing_posture(&self) -> ConflictRoutingPosture {
        self.routing_posture
    }

    pub const fn prior_proof_posture(&self) -> SpatialConflictPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn diagnostic_witness(&self) -> SpatialConflictDiagnosticWitness {
        self.diagnostic_witness
    }

    pub const fn selection_product_posture(&self) -> SpatialConflictSelectionProductPosture {
        self.selection_product_posture
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_digest.as_str()
    }

    pub(crate) const fn admits_evidence_selection(&self) -> bool {
        matches!(
            (
                self.locality_authority_requirement,
                self.primary_overlap_category,
                self.secondary_overlap_category,
                self.prior_proof_posture,
                self.diagnostic_witness,
                self.selection_product_posture,
            ),
            (
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
                ConflictOverlapCategory::Evidence,
                None,
                SpatialConflictPriorProofPosture::NoPriorProofRequired,
                SpatialConflictDiagnosticWitness::EvidenceFamilyDigest,
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        )
    }

    pub(crate) const fn admits_replay_boundary_selection(&self) -> bool {
        matches!(
            (
                self.locality_authority_requirement,
                self.primary_overlap_category,
                self.secondary_overlap_category,
                self.prior_proof_posture,
                self.diagnostic_witness,
                self.selection_product_posture,
            ),
            (
                SpatialConflictLocalityAuthorityRequirement::SpatialTouchAuthorityRequired,
                ConflictOverlapCategory::ReplayUndo,
                Some(ConflictOverlapCategory::Transaction),
                SpatialConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
                SpatialConflictDiagnosticWitness::ReplayBoundaryScope,
                SpatialConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        )
    }
}

impl SpatialConflictFamilyDeclarationDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
