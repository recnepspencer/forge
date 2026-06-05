use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::graph_memory::GraphResidentFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternSignature {
    core: HadwigerArtifactCore,
    signature_token: String,
}

impl PatternSignature {
    pub(crate) fn from_parts(
        pattern_family: &'static str,
        evidence_token: impl Into<String>,
        scope_token: impl Into<String>,
        parents: Vec<HadwigerArtifactReference>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let evidence_token = require_non_empty(evidence_token, "evidence_token")?;
        let scope_token = require_non_empty(scope_token, "scope_token")?;
        let signature_token = format!("{pattern_family}:{evidence_token}:{scope_token}");
        let core = artifact_core(
            HadwigerArtifactKind::PatternSignature,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "pattern_signature".to_string(),
            },
            parents,
            vec![
                HadwigerArtifactPayloadEntry::text("pattern_family", pattern_family),
                HadwigerArtifactPayloadEntry::text("signature_token", signature_token.clone()),
            ],
        )?;
        Ok(Self {
            core,
            signature_token,
        })
    }

    pub(crate) fn from_graph_resident_failure(
        failure: &GraphResidentFailure,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Self::from_parts(
            "graph_resident_failure",
            failure
                .failure_basis_fingerprint()
                .artifact_digest()
                .stable_token(),
            failure.failure_scope().stable_token(),
            vec![failure.reference()],
        )
    }

    pub fn stable_token(&self) -> &str {
        &self.signature_token
    }
}

impl_hadwiger_artifact!(PatternSignature, core);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotifObservation {
    core: HadwigerArtifactCore,
    pattern_signature: PatternSignature,
    source_evidence: HadwigerArtifactReference,
}

impl MotifObservation {
    pub(crate) fn from_graph_resident_failure(
        failure: &GraphResidentFailure,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let pattern_signature = PatternSignature::from_graph_resident_failure(failure)?;
        let source_evidence = failure.reference();
        let core = artifact_core(
            HadwigerArtifactKind::MotifObservation,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "motif_observation".to_string(),
            },
            vec![source_evidence.clone(), pattern_signature.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "pattern_signature",
                    pattern_signature.stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "source_evidence",
                    source_evidence.stable_token(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            pattern_signature,
            source_evidence,
        })
    }

    pub(crate) fn from_evidence_reference(
        source_evidence: HadwigerArtifactReference,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let pattern_signature = PatternSignature::from_parts(
            "retained_evidence",
            source_evidence.artifact_digest().stable_token(),
            source_evidence.artifact_kind().as_str(),
            vec![source_evidence.clone()],
        )?;
        let core = artifact_core(
            HadwigerArtifactKind::MotifObservation,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "retained_evidence_motif_observation".to_string(),
            },
            vec![source_evidence.clone(), pattern_signature.reference()],
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "pattern_signature",
                    pattern_signature.stable_token(),
                ),
                HadwigerArtifactPayloadEntry::text(
                    "source_evidence",
                    source_evidence.stable_token(),
                ),
            ],
        )?;
        Ok(Self {
            core,
            pattern_signature,
            source_evidence,
        })
    }

    pub fn pattern_signature(&self) -> &PatternSignature {
        &self.pattern_signature
    }

    pub fn source_evidence(&self) -> &HadwigerArtifactReference {
        &self.source_evidence
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(MotifObservation, core);
