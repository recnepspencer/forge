use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerQueryDeclarationReference;

use super::equivalence_classification_requests::TilingCandidateEquivalenceRequest;
use super::equivalence_scopes::TilingEquivalenceScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TilingCandidateEquivalencePosture {
    BlocksDuplicateCheckerWork,
    BlocksDuplicateProofAdmission,
    EquivalentButNonBlocking,
    Unsupported,
}

impl TilingCandidateEquivalencePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlocksDuplicateCheckerWork => "blocks_duplicate_checker_work",
            Self::BlocksDuplicateProofAdmission => "blocks_duplicate_proof_admission",
            Self::EquivalentButNonBlocking => "equivalent_but_non_blocking",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingEquivalenceCounters {
    candidate_breadth_inspected: usize,
    equivalence_scopes_evaluated: usize,
    exact_equality_hits: usize,
    tile_equivalence_hits: usize,
    novelty_fingerprint_hits: usize,
    suppression_hits: usize,
    reactivation_hits: usize,
    query_declarations_performed: usize,
    hidden_broad_scan_refusals: usize,
}

impl TilingEquivalenceCounters {
    pub(crate) fn classification(
        scope: TilingEquivalenceScope,
        equivalent: bool,
        query_declarations: usize,
    ) -> Self {
        Self {
            candidate_breadth_inspected: 2,
            equivalence_scopes_evaluated: 1,
            exact_equality_hits: usize::from(equivalent),
            tile_equivalence_hits: usize::from(equivalent && scope.blocks_checker_work()),
            novelty_fingerprint_hits: 0,
            suppression_hits: 0,
            reactivation_hits: 0,
            query_declarations_performed: query_declarations,
            hidden_broad_scan_refusals: 0,
        }
    }

    pub(crate) fn suppression(query_declarations: usize) -> Self {
        Self {
            candidate_breadth_inspected: 1,
            equivalence_scopes_evaluated: 1,
            exact_equality_hits: 0,
            tile_equivalence_hits: 0,
            novelty_fingerprint_hits: 0,
            suppression_hits: 1,
            reactivation_hits: 0,
            query_declarations_performed: query_declarations,
            hidden_broad_scan_refusals: 0,
        }
    }

    pub(crate) fn reactivation(query_declarations: usize) -> Self {
        Self {
            candidate_breadth_inspected: 1,
            equivalence_scopes_evaluated: 1,
            exact_equality_hits: 0,
            tile_equivalence_hits: 0,
            novelty_fingerprint_hits: 0,
            suppression_hits: 0,
            reactivation_hits: 1,
            query_declarations_performed: query_declarations,
            hidden_broad_scan_refusals: 0,
        }
    }

    pub fn candidate_breadth_inspected(&self) -> usize {
        self.candidate_breadth_inspected
    }

    pub fn equivalence_scopes_evaluated(&self) -> usize {
        self.equivalence_scopes_evaluated
    }

    pub fn exact_equality_hits(&self) -> usize {
        self.exact_equality_hits
    }

    pub fn tile_equivalence_hits(&self) -> usize {
        self.tile_equivalence_hits
    }

    pub fn novelty_fingerprint_hits(&self) -> usize {
        self.novelty_fingerprint_hits
    }

    pub fn suppression_hits(&self) -> usize {
        self.suppression_hits
    }

    pub fn reactivation_hits(&self) -> usize {
        self.reactivation_hits
    }

    pub fn query_declarations_performed(&self) -> usize {
        self.query_declarations_performed
    }

    pub fn hidden_broad_scan_refusals(&self) -> usize {
        self.hidden_broad_scan_refusals
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.candidate_breadth_inspected,
            self.equivalence_scopes_evaluated,
            self.exact_equality_hits,
            self.tile_equivalence_hits,
            self.novelty_fingerprint_hits,
            self.suppression_hits,
            self.reactivation_hits,
            self.query_declarations_performed,
            self.hidden_broad_scan_refusals
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingCandidateEquivalenceProof {
    core: HadwigerArtifactCore,
    equivalence_id: String,
    scope: TilingEquivalenceScope,
    posture: TilingCandidateEquivalencePosture,
    equivalence_token: String,
    query_declaration_reference: HadwigerQueryDeclarationReference,
    counters: TilingEquivalenceCounters,
}

impl TilingCandidateEquivalenceProof {
    pub(crate) fn checked(
        request: TilingCandidateEquivalenceRequest,
        query_declaration_reference: HadwigerQueryDeclarationReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let equivalent = request.basis_tokens_match();
        let posture = classify_posture(request.scope(), equivalent);
        let counters = TilingEquivalenceCounters::classification(request.scope(), equivalent, 1);
        let equivalence_token = request.equivalence_token();
        let core = artifact_core(
            HadwigerArtifactKind::TilingCandidateEquivalenceProof,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::QueryDeclaration(query_declaration_reference.clone()),
            vec![
                request.left_reference().clone(),
                request.right_reference().clone(),
            ],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "schema",
                    "WORTH.hadwiger.tiling_equivalence.v1",
                ),
                HadwigerArtifactPayloadEntry::text("equivalence_id", request.equivalence_id()),
                HadwigerArtifactPayloadEntry::text("scope", request.scope().as_str()),
                HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
                HadwigerArtifactPayloadEntry::text("equivalence_token", &equivalence_token),
                HadwigerArtifactPayloadEntry::text(
                    "query_declaration",
                    query_declaration_reference.stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text("counters", counters.stable_token()),
            ],
        )?;
        Ok(Self {
            core,
            equivalence_id: request.equivalence_id().to_string(),
            scope: request.scope(),
            posture,
            equivalence_token,
            query_declaration_reference,
            counters,
        })
    }

    pub fn equivalence_id(&self) -> &str {
        &self.equivalence_id
    }

    pub fn equivalence_scope(&self) -> TilingEquivalenceScope {
        self.scope
    }

    pub fn posture(&self) -> TilingCandidateEquivalencePosture {
        self.posture
    }

    pub fn equivalence_token(&self) -> &str {
        &self.equivalence_token
    }

    pub fn query_declaration_reference(&self) -> &HadwigerQueryDeclarationReference {
        &self.query_declaration_reference
    }

    pub fn counters(&self) -> &TilingEquivalenceCounters {
        &self.counters
    }

    pub fn blocks_duplicate_checker_work(&self) -> bool {
        self.posture == TilingCandidateEquivalencePosture::BlocksDuplicateCheckerWork
    }

    pub fn blocks_duplicate_proof_admission(&self) -> bool {
        self.posture == TilingCandidateEquivalencePosture::BlocksDuplicateProofAdmission
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(TilingCandidateEquivalenceProof, core);

fn classify_posture(
    scope: TilingEquivalenceScope,
    equivalent: bool,
) -> TilingCandidateEquivalencePosture {
    if !equivalent {
        TilingCandidateEquivalencePosture::Unsupported
    } else if scope.blocks_checker_work() {
        TilingCandidateEquivalencePosture::BlocksDuplicateCheckerWork
    } else if scope.blocks_proof_admission() {
        TilingCandidateEquivalencePosture::BlocksDuplicateProofAdmission
    } else {
        TilingCandidateEquivalencePosture::EquivalentButNonBlocking
    }
}
