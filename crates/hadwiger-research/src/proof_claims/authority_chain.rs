use forge_foundational::facade::{
    derive_canonical_digest, prepare_canonical_basis_sequence, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue,
    CanonicalDigestAlgorithmId, CanonicalDigestFrontDoor, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;

use crate::aspect_authority::{HadwigerAspectKind, HadwigerAspectPosture};
use crate::domain_artifacts::{
    HadwigerArtifactReference, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
    HadwigerQueryDeclarationReference, ProofClaim,
};

const PROOF_CHAIN_DIGEST_VERSION: &str = "forge.hadwiger.proof_authority_chain.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HadwigerProofAuthorityStepKind {
    QueryDeclaration,
    CheckerArtifact,
    AspectRecord,
    ProofClaim,
    BackgroundTheorem,
    ProjectionConsumptionReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerProofAuthorityStep {
    step_kind: HadwigerProofAuthorityStepKind,
    label: String,
    stable_token: String,
    aspect_kind: Option<HadwigerAspectKind>,
    aspect_posture: Option<HadwigerAspectPosture>,
}

impl HadwigerProofAuthorityStep {
    pub(crate) fn new(
        step_kind: HadwigerProofAuthorityStepKind,
        label: impl Into<String>,
        stable_token: impl Into<String>,
    ) -> Self {
        Self {
            step_kind,
            label: label.into(),
            stable_token: stable_token.into(),
            aspect_kind: None,
            aspect_posture: None,
        }
    }

    pub(crate) fn aspect(
        label: impl Into<String>,
        stable_token: impl Into<String>,
        aspect_kind: HadwigerAspectKind,
        aspect_posture: HadwigerAspectPosture,
    ) -> Self {
        Self {
            step_kind: HadwigerProofAuthorityStepKind::AspectRecord,
            label: label.into(),
            stable_token: stable_token.into(),
            aspect_kind: Some(aspect_kind),
            aspect_posture: Some(aspect_posture),
        }
    }

    pub fn step_kind(&self) -> HadwigerProofAuthorityStepKind {
        self.step_kind
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn stable_token(&self) -> &str {
        &self.stable_token
    }

    pub fn aspect_kind(&self) -> Option<HadwigerAspectKind> {
        self.aspect_kind
    }

    pub fn aspect_posture(&self) -> Option<HadwigerAspectPosture> {
        self.aspect_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerProofAuthorityChain {
    claim_digest: String,
    chain_digest: String,
    weakest_posture: HadwigerAspectPosture,
    steps: Vec<HadwigerProofAuthorityStep>,
    query_declaration_references: Vec<HadwigerQueryDeclarationReference>,
    checker_artifact_references: Vec<HadwigerArtifactReference>,
    aspect_tokens: Vec<String>,
    background_theorem_reference: Option<HadwigerArtifactReference>,
    uses_checked_upper_bound: bool,
    uses_background_upper_bound: bool,
    projection_consumption_receipt_tokens: Vec<String>,
}

impl HadwigerProofAuthorityChain {
    pub(crate) fn new(
        proof_claim: &ProofClaim,
        weakest_posture: HadwigerAspectPosture,
        steps: Vec<HadwigerProofAuthorityStep>,
        query_declaration_references: Vec<HadwigerQueryDeclarationReference>,
        checker_artifact_references: Vec<HadwigerArtifactReference>,
        aspect_tokens: Vec<String>,
        background_theorem_reference: Option<HadwigerArtifactReference>,
        uses_checked_upper_bound: bool,
        uses_background_upper_bound: bool,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let claim_digest = proof_claim.artifact_digest().stable_token().to_string();
        let projection_consumption_receipt_tokens = Vec::new();
        let chain_digest = chain_digest(
            &claim_digest,
            weakest_posture,
            &steps,
            &query_declaration_references,
            &checker_artifact_references,
            &aspect_tokens,
            background_theorem_reference.as_ref(),
        )?;
        Ok(Self {
            claim_digest,
            chain_digest,
            weakest_posture,
            steps,
            query_declaration_references,
            checker_artifact_references,
            aspect_tokens,
            background_theorem_reference,
            uses_checked_upper_bound,
            uses_background_upper_bound,
            projection_consumption_receipt_tokens,
        })
    }

    pub fn claim_digest(&self) -> &str {
        &self.claim_digest
    }

    pub fn chain_digest(&self) -> &str {
        &self.chain_digest
    }

    pub fn weakest_posture(&self) -> HadwigerAspectPosture {
        self.weakest_posture
    }

    pub fn steps(&self) -> &[HadwigerProofAuthorityStep] {
        &self.steps
    }

    pub fn query_declaration_references(&self) -> &[HadwigerQueryDeclarationReference] {
        &self.query_declaration_references
    }

    pub fn checker_artifact_references(&self) -> &[HadwigerArtifactReference] {
        &self.checker_artifact_references
    }

    pub fn aspect_tokens(&self) -> &[String] {
        &self.aspect_tokens
    }

    pub fn background_theorem_reference(&self) -> Option<&HadwigerArtifactReference> {
        self.background_theorem_reference.as_ref()
    }

    pub fn uses_checked_upper_bound(&self) -> bool {
        self.uses_checked_upper_bound
    }

    pub fn uses_background_upper_bound(&self) -> bool {
        self.uses_background_upper_bound
    }

    pub fn projection_consumption_receipt_tokens(&self) -> &[String] {
        &self.projection_consumption_receipt_tokens
    }
}

fn chain_digest(
    claim_digest: &str,
    weakest_posture: HadwigerAspectPosture,
    steps: &[HadwigerProofAuthorityStep],
    query_declaration_references: &[HadwigerQueryDeclarationReference],
    checker_artifact_references: &[HadwigerArtifactReference],
    aspect_tokens: &[String],
    background_theorem_reference: Option<&HadwigerArtifactReference>,
) -> Result<String, HadwigerArtifactShapeError> {
    let domain = CanonicalBasisDomain::Future("forge.hadwiger.proof_authority_chain");
    let mut entries = vec![
        text_entry(domain, "digest_schema_version", PROOF_CHAIN_DIGEST_VERSION),
        text_entry(domain, "claim_digest", claim_digest),
        text_entry(domain, "weakest_posture", weakest_posture.as_str()),
    ];
    for (index, step) in steps.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("step.{index:04}"),
            format!(
                "{:?}:{}:{}",
                step.step_kind(),
                step.label(),
                step.stable_token()
            ),
        ));
    }
    for (index, reference) in query_declaration_references.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("query_declaration.{index:04}"),
            reference.stable_token(),
        ));
    }
    for (index, reference) in checker_artifact_references.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("checker_artifact.{index:04}"),
            reference.stable_token(),
        ));
    }
    for (index, token) in aspect_tokens.iter().enumerate() {
        entries.push(text_entry(domain, format!("aspect.{index:04}"), token));
    }
    if let Some(reference) = background_theorem_reference {
        entries.push(text_entry(
            domain,
            "background_theorem",
            reference.stable_token(),
        ));
    }
    let version = CanonicalizationRuleVersion::new(PROOF_CHAIN_DIGEST_VERSION)
        .expect("Hadwiger proof chain digest version is a stable literal");
    let sequence = match prepare_canonical_basis_sequence(version, domain, entries) {
        TransitionOutcome::Success(sequence) => sequence,
        _ => {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "canonical_basis",
            })
        }
    };
    let ready = match CanonicalDigestFrontDoor
        .for_sequence(sequence, CanonicalDigestAlgorithmId::test_stable_fixture())
    {
        TransitionOutcome::Success(ready) => ready,
        _ => {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "canonical_digest",
            })
        }
    };
    Ok(
        crate::domain_artifacts::core_artifact::canonical_digest_token(&derive_canonical_digest(
            ready,
        )),
    )
}

fn text_entry(
    domain: CanonicalBasisDomain,
    locus: impl Into<String>,
    value: impl Into<String>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Field,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}
