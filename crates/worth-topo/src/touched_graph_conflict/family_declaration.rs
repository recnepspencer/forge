use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictOverlapCategory, ConflictRoutingPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::family_identity::TopologyConflictFamilyIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConflictLocalityAuthorityRequirement {
    DerivedInvalidationTouchedClosureRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConflictPriorProofPosture {
    NoPriorProofRequired,
    ReplayUndoOrTransactionRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConflictDiagnosticWitness {
    TouchedClosureDigest,
    ValidatorFamilyDigest,
    ReplayBoundaryScope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyConflictSelectionProductPosture {
    DeclarationOnlySelectionRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConflictFamilyDeclarationDigest(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConflictFamilyDeclaration {
    identity: TopologyConflictFamilyIdentity,
    locality_authority_requirement: TopologyConflictLocalityAuthorityRequirement,
    primary_overlap_category: ConflictOverlapCategory,
    secondary_overlap_category: Option<ConflictOverlapCategory>,
    routing_posture: ConflictRoutingPosture,
    prior_proof_posture: TopologyConflictPriorProofPosture,
    diagnostic_witness: TopologyConflictDiagnosticWitness,
    selection_product_posture: TopologyConflictSelectionProductPosture,
    declaration_digest: TopologyConflictFamilyDeclarationDigest,
}

impl TopologyConflictFamilyDeclaration {
    pub(crate) fn new(
        identity: TopologyConflictFamilyIdentity,
        locality_authority_requirement: TopologyConflictLocalityAuthorityRequirement,
        primary_overlap_category: ConflictOverlapCategory,
        secondary_overlap_category: Option<ConflictOverlapCategory>,
        routing_posture: ConflictRoutingPosture,
        prior_proof_posture: TopologyConflictPriorProofPosture,
        diagnostic_witness: TopologyConflictDiagnosticWitness,
        selection_product_posture: TopologyConflictSelectionProductPosture,
    ) -> Self {
        let declaration_digest = TopologyConflictFamilyDeclarationDigest(truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:conflict-family-declaration:v1".to_string(),
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

    pub const fn identity(&self) -> TopologyConflictFamilyIdentity {
        self.identity
    }

    pub const fn locality_authority_requirement(
        &self,
    ) -> TopologyConflictLocalityAuthorityRequirement {
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

    pub const fn prior_proof_posture(&self) -> TopologyConflictPriorProofPosture {
        self.prior_proof_posture
    }

    pub const fn diagnostic_witness(&self) -> TopologyConflictDiagnosticWitness {
        self.diagnostic_witness
    }

    pub const fn selection_product_posture(&self) -> TopologyConflictSelectionProductPosture {
        self.selection_product_posture
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_digest.as_str()
    }

    pub(crate) const fn admits_aspect_locality_selection(&self) -> bool {
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
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
                ConflictOverlapCategory::Aspect,
                Some(ConflictOverlapCategory::Locality),
                TopologyConflictPriorProofPosture::NoPriorProofRequired,
                TopologyConflictDiagnosticWitness::TouchedClosureDigest,
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        )
    }

    pub(crate) const fn admits_validator_selection(&self) -> bool {
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
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
                ConflictOverlapCategory::Validator,
                None,
                TopologyConflictPriorProofPosture::NoPriorProofRequired,
                TopologyConflictDiagnosticWitness::ValidatorFamilyDigest,
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
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
                TopologyConflictLocalityAuthorityRequirement::DerivedInvalidationTouchedClosureRequired,
                ConflictOverlapCategory::ReplayUndo,
                Some(ConflictOverlapCategory::Transaction),
                TopologyConflictPriorProofPosture::ReplayUndoOrTransactionRequired,
                TopologyConflictDiagnosticWitness::ReplayBoundaryScope,
                TopologyConflictSelectionProductPosture::DeclarationOnlySelectionRequired,
            )
        )
    }
}

impl TopologyConflictFamilyDeclarationDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
