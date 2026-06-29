use serde::Serialize;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    ConflictAspectClass, ConflictLocalityIdentity, ConflictOverlapCategory,
    ConflictParticipantAuthority, ConflictParticipantIdentity, ConflictPriorProofIdentity,
    ConflictRoutingVocabularyError, ConflictTransactionProofInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictOverlapIdentityInput {
    category: ConflictOverlapCategory,
    participants: Vec<ConflictParticipantIdentity>,
    aspect_class: Option<ConflictAspectClass>,
    locality_identity: Option<ConflictLocalityIdentity>,
    prior_proof_identities: Vec<ConflictPriorProofIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConflictOverlapIdentity {
    category: ConflictOverlapCategory,
    participants: Vec<ConflictParticipantIdentity>,
    aspect_class: Option<ConflictAspectClass>,
    locality_identity: Option<ConflictLocalityIdentity>,
    prior_proof_identities: Vec<ConflictPriorProofIdentity>,
    overlap_identity_digest: String,
}

impl ConflictOverlapIdentityInput {
    pub fn entity(participants: Vec<ConflictParticipantIdentity>) -> Self {
        Self::new(ConflictOverlapCategory::Entity, participants)
    }

    pub fn relation(participants: Vec<ConflictParticipantIdentity>) -> Self {
        Self::new(ConflictOverlapCategory::Relation, participants)
    }

    pub fn aspect(
        aspect_class: ConflictAspectClass,
        locality_identity: ConflictLocalityIdentity,
        participants: Vec<ConflictParticipantIdentity>,
    ) -> Self {
        Self::new(ConflictOverlapCategory::Aspect, participants)
            .with_aspect_class(aspect_class)
            .with_locality_identity(locality_identity)
    }

    pub fn locality(locality_identity: ConflictLocalityIdentity) -> Self {
        Self::new(ConflictOverlapCategory::Locality, Vec::new())
            .with_locality_identity(locality_identity)
    }

    pub fn evidence(
        locality_identity: ConflictLocalityIdentity,
        participants: Vec<ConflictParticipantIdentity>,
    ) -> Self {
        Self::new(ConflictOverlapCategory::Evidence, participants)
            .with_locality_identity(locality_identity)
    }

    pub fn validator(
        locality_identity: ConflictLocalityIdentity,
        participants: Vec<ConflictParticipantIdentity>,
    ) -> Self {
        Self::new(ConflictOverlapCategory::Validator, participants)
            .with_locality_identity(locality_identity)
    }

    pub fn replay_undo(
        locality_identity: ConflictLocalityIdentity,
        prior_proof_identities: Vec<ConflictPriorProofIdentity>,
    ) -> Self {
        Self::new(ConflictOverlapCategory::ReplayUndo, Vec::new())
            .with_locality_identity(locality_identity)
            .with_prior_proof_identities(prior_proof_identities)
    }

    pub fn transaction(
        locality_identity: ConflictLocalityIdentity,
        transaction: ConflictTransactionProofInput,
    ) -> Self {
        Self::new(ConflictOverlapCategory::Transaction, Vec::new())
            .with_locality_identity(locality_identity)
            .with_prior_proof_identities(vec![transaction.claim().clone().into()])
    }

    fn new(
        category: ConflictOverlapCategory,
        participants: Vec<ConflictParticipantIdentity>,
    ) -> Self {
        Self {
            category,
            participants,
            aspect_class: None,
            locality_identity: None,
            prior_proof_identities: Vec::new(),
        }
    }

    fn with_aspect_class(mut self, aspect_class: ConflictAspectClass) -> Self {
        self.aspect_class = Some(aspect_class);
        self
    }

    fn with_locality_identity(mut self, locality_identity: ConflictLocalityIdentity) -> Self {
        self.locality_identity = Some(locality_identity);
        self
    }

    fn with_prior_proof_identities(
        mut self,
        prior_proof_identities: Vec<ConflictPriorProofIdentity>,
    ) -> Self {
        self.prior_proof_identities = prior_proof_identities;
        self
    }
}

impl ConflictOverlapIdentity {
    pub const fn category(&self) -> ConflictOverlapCategory {
        self.category
    }

    pub fn participants(&self) -> &[ConflictParticipantIdentity] {
        &self.participants
    }

    pub const fn aspect_class(&self) -> Option<ConflictAspectClass> {
        self.aspect_class
    }

    pub const fn locality_identity(&self) -> Option<&ConflictLocalityIdentity> {
        self.locality_identity.as_ref()
    }

    pub fn prior_proof_identities(&self) -> &[ConflictPriorProofIdentity] {
        &self.prior_proof_identities
    }

    pub fn overlap_identity_digest(&self) -> &str {
        &self.overlap_identity_digest
    }
}

pub fn admit_conflict_overlap_identity(
    input: ConflictOverlapIdentityInput,
) -> Result<ConflictOverlapIdentity, ConflictRoutingVocabularyError> {
    validate_input(&input)?;
    let overlap_identity_digest =
        truth_digest_parts(TruthDigestScope::ArtifactIdentity, &canonical_parts(&input));
    Ok(ConflictOverlapIdentity {
        category: input.category,
        participants: input.participants,
        aspect_class: input.aspect_class,
        locality_identity: input.locality_identity,
        prior_proof_identities: input.prior_proof_identities,
        overlap_identity_digest,
    })
}

fn validate_input(
    input: &ConflictOverlapIdentityInput,
) -> Result<(), ConflictRoutingVocabularyError> {
    match input.category {
        ConflictOverlapCategory::Entity => {
            validate_participants(input, ConflictParticipantAuthority::Entity)
        }
        ConflictOverlapCategory::Relation => {
            validate_participants(input, ConflictParticipantAuthority::Relation)
        }
        ConflictOverlapCategory::Aspect => {
            require_locality(input)?;
            validate_non_empty(input)?;
            if input.aspect_class.is_none() {
                return Err(ConflictRoutingVocabularyError::MissingAspectClass);
            }
            Ok(())
        }
        ConflictOverlapCategory::Locality => require_locality(input),
        ConflictOverlapCategory::Evidence => {
            require_locality(input)?;
            validate_participants(input, ConflictParticipantAuthority::Evidence)
        }
        ConflictOverlapCategory::Validator => {
            require_locality(input)?;
            validate_participants(input, ConflictParticipantAuthority::Validator)
        }
        ConflictOverlapCategory::ReplayUndo => {
            require_locality(input)?;
            if input.prior_proof_identities.is_empty() {
                return Err(ConflictRoutingVocabularyError::MissingPriorProof(
                    input.category,
                ));
            }
            if input
                .prior_proof_identities
                .iter()
                .all(ConflictPriorProofIdentity::is_replay_undo_or_execution)
            {
                Ok(())
            } else {
                Err(ConflictRoutingVocabularyError::WrongPriorProof(
                    "replay/undo overlap requires replay scope, undo scope, or prior execution proof",
                ))
            }
        }
        ConflictOverlapCategory::Transaction => {
            require_locality(input)?;
            if input.prior_proof_identities.len() != 1 {
                return Err(ConflictRoutingVocabularyError::MissingPriorProof(
                    input.category,
                ));
            }
            if input.prior_proof_identities[0].is_transaction_scope() {
                Ok(())
            } else {
                Err(ConflictRoutingVocabularyError::WrongPriorProof(
                    "transaction overlap requires an admitted transaction scope claim",
                ))
            }
        }
    }
}

fn validate_participants(
    input: &ConflictOverlapIdentityInput,
    expected: ConflictParticipantAuthority,
) -> Result<(), ConflictRoutingVocabularyError> {
    validate_non_empty(input)?;
    for participant in &input.participants {
        if participant.authority() != expected {
            return Err(ConflictRoutingVocabularyError::WrongParticipantAuthority {
                category: input.category,
                expected,
                found: participant.authority(),
            });
        }
    }
    Ok(())
}

fn validate_non_empty(
    input: &ConflictOverlapIdentityInput,
) -> Result<(), ConflictRoutingVocabularyError> {
    if input.participants.is_empty() {
        Err(ConflictRoutingVocabularyError::EmptyParticipantSet(
            input.category,
        ))
    } else {
        Ok(())
    }
}

fn require_locality(
    input: &ConflictOverlapIdentityInput,
) -> Result<(), ConflictRoutingVocabularyError> {
    if input.locality_identity.is_none() {
        Err(ConflictRoutingVocabularyError::MissingLocalityIdentity(
            input.category,
        ))
    } else {
        Ok(())
    }
}

fn canonical_parts(input: &ConflictOverlapIdentityInput) -> Vec<String> {
    let mut parts = vec![
        "worth-schema:touched-graph-conflict-overlap-identity:v1".to_string(),
        format!("category:{}", input.category.as_str()),
    ];
    if let Some(aspect_class) = input.aspect_class {
        parts.push(format!("aspect:{}", aspect_class.as_str()));
    }
    if let Some(locality_identity) = &input.locality_identity {
        parts.push(format!("locality:{}", locality_identity.canonical_part()));
    }
    let mut participants = input
        .participants
        .iter()
        .map(ConflictParticipantIdentity::canonical_part)
        .collect::<Vec<_>>();
    participants.sort();
    parts.extend(
        participants
            .into_iter()
            .map(|part| format!("participant:{part}")),
    );
    let mut proofs = input
        .prior_proof_identities
        .iter()
        .map(ConflictPriorProofIdentity::canonical_part)
        .collect::<Vec<_>>();
    proofs.sort();
    parts.extend(proofs.into_iter().map(|part| format!("proof:{part}")));
    parts
}
