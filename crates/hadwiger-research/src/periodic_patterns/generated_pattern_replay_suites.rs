use crate::candidate_screening::{
    FinitePatchBoundaryExtensionCertificate, PeriodicQuotientConflictCertificate,
    PeriodicQuotientRectangleModel, SubstitutionConsistencyCertificate,
    TranslationRotationClosureCertificate,
};
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::GraphVersion;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerCanonicalArtifact};

use super::color_holonomy_certificates::ColorHolonomyLoopCertificate;
use super::periodic_quotient_cells::PeriodicQuotientCell;
use super::replay_errors::GeneratedPatternReplayShapeError;
use super::replay_errors::{require_replay_non_empty, GeneratedPatternReplayError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedPatternReplaySuite {
    core: HadwigerArtifactCore,
    replay_suite_id: String,
    quotient_reference: HadwigerArtifactReference,
    periodic_quotient_cell: Option<PeriodicQuotientCell>,
    periodic_quotient_conflicts: Vec<PeriodicQuotientConflictReplayCertificate>,
    color_holonomy_loops: Vec<ColorHolonomyLoopCertificate>,
    translation_rotation_certificates: Vec<TranslationRotationClosureReplayCertificate>,
    substitution_certificates: Vec<SubstitutionConsistencyCertificate>,
    finite_patch_extension_certificates: Vec<FinitePatchBoundaryExtensionCertificate>,
}

impl GeneratedPatternReplaySuite {
    pub fn builder(
        replay_suite_id: impl Into<String>,
        quotient_reference: HadwigerArtifactReference,
    ) -> GeneratedPatternReplaySuiteBuilder {
        GeneratedPatternReplaySuiteBuilder {
            replay_suite_id: replay_suite_id.into(),
            quotient_reference,
            periodic_quotient_cell: None,
            periodic_quotient_conflicts: Vec::new(),
            color_holonomy_loops: Vec::new(),
            translation_rotation_certificates: Vec::new(),
            substitution_certificates: Vec::new(),
            finite_patch_extension_certificates: Vec::new(),
        }
    }

    pub fn replay_suite_id(&self) -> &str {
        &self.replay_suite_id
    }

    pub fn quotient_reference(&self) -> &HadwigerArtifactReference {
        &self.quotient_reference
    }

    pub fn periodic_quotient_cell(&self) -> Option<&PeriodicQuotientCell> {
        self.periodic_quotient_cell.as_ref()
    }

    pub fn color_holonomy_loops(&self) -> &[ColorHolonomyLoopCertificate] {
        &self.color_holonomy_loops
    }

    pub fn periodic_quotient_conflicts(&self) -> &[PeriodicQuotientConflictReplayCertificate] {
        &self.periodic_quotient_conflicts
    }

    pub fn substitution_certificates(&self) -> &[SubstitutionConsistencyCertificate] {
        &self.substitution_certificates
    }

    pub fn translation_rotation_certificates(
        &self,
    ) -> &[TranslationRotationClosureReplayCertificate] {
        &self.translation_rotation_certificates
    }

    pub fn finite_patch_extension_certificates(
        &self,
    ) -> &[FinitePatchBoundaryExtensionCertificate] {
        &self.finite_patch_extension_certificates
    }

    pub fn stable_token(&self) -> String {
        replay_suite_stable_token(
            &self.replay_suite_id,
            &self.quotient_reference,
            self.periodic_quotient_cell.as_ref(),
            &self.periodic_quotient_conflicts,
            &self.color_holonomy_loops,
            &self.translation_rotation_certificates,
            &self.substitution_certificates,
            &self.finite_patch_extension_certificates,
        )
    }
}

impl_hadwiger_artifact!(GeneratedPatternReplaySuite, core);

#[derive(Clone, Debug)]
pub struct GeneratedPatternReplaySuiteBuilder {
    replay_suite_id: String,
    quotient_reference: HadwigerArtifactReference,
    periodic_quotient_cell: Option<PeriodicQuotientCell>,
    periodic_quotient_conflicts: Vec<PeriodicQuotientConflictReplayCertificate>,
    color_holonomy_loops: Vec<ColorHolonomyLoopCertificate>,
    translation_rotation_certificates: Vec<TranslationRotationClosureReplayCertificate>,
    substitution_certificates: Vec<SubstitutionConsistencyCertificate>,
    finite_patch_extension_certificates: Vec<FinitePatchBoundaryExtensionCertificate>,
}

impl GeneratedPatternReplaySuiteBuilder {
    pub fn with_periodic_quotient_cell(
        mut self,
        quotient_cell: PeriodicQuotientCell,
    ) -> Result<Self, GeneratedPatternReplayError> {
        if quotient_cell.reference() != self.quotient_reference {
            return Err(GeneratedPatternReplayShapeError::QuotientReferenceMismatch.into());
        }
        self.periodic_quotient_cell = Some(quotient_cell);
        Ok(self)
    }

    pub fn with_periodic_quotient_conflict_certificate(
        mut self,
        model: PeriodicQuotientRectangleModel,
        certificate: PeriodicQuotientConflictCertificate,
    ) -> Result<Self, GeneratedPatternReplayError> {
        self.periodic_quotient_conflicts
            .push(PeriodicQuotientConflictReplayCertificate::new(
                model,
                certificate,
            ));
        self.periodic_quotient_conflicts
            .sort_by_key(PeriodicQuotientConflictReplayCertificate::stable_token);
        Ok(self)
    }

    pub fn with_color_holonomy_loop(
        mut self,
        certificate: ColorHolonomyLoopCertificate,
    ) -> Result<Self, GeneratedPatternReplayError> {
        self.color_holonomy_loops.push(certificate);
        self.color_holonomy_loops
            .sort_by_key(ColorHolonomyLoopCertificate::stable_token);
        Ok(self)
    }

    pub fn with_substitution_certificate(
        mut self,
        certificate: SubstitutionConsistencyCertificate,
    ) -> Result<Self, GeneratedPatternReplayError> {
        self.substitution_certificates.push(certificate);
        self.substitution_certificates
            .sort_by_key(SubstitutionConsistencyCertificate::stable_token);
        Ok(self)
    }

    pub fn with_translation_rotation_closure_certificate(
        mut self,
        graph: GraphVersion,
        certificate: TranslationRotationClosureCertificate,
    ) -> Result<Self, GeneratedPatternReplayError> {
        self.translation_rotation_certificates.push(
            TranslationRotationClosureReplayCertificate::new(graph, certificate),
        );
        self.translation_rotation_certificates
            .sort_by_key(TranslationRotationClosureReplayCertificate::stable_token);
        Ok(self)
    }

    pub fn with_finite_patch_extension_certificate(
        mut self,
        certificate: FinitePatchBoundaryExtensionCertificate,
    ) -> Result<Self, GeneratedPatternReplayError> {
        self.finite_patch_extension_certificates.push(certificate);
        self.finite_patch_extension_certificates
            .sort_by_key(FinitePatchBoundaryExtensionCertificate::stable_token);
        Ok(self)
    }

    pub fn finish(self) -> Result<GeneratedPatternReplaySuite, GeneratedPatternReplayError> {
        let replay_suite_id =
            require_replay_non_empty(self.replay_suite_id, "generated_pattern_replay_suite_id")?;
        if self.periodic_quotient_cell.is_none()
            && self.color_holonomy_loops.is_empty()
            && self.periodic_quotient_conflicts.is_empty()
            && self.translation_rotation_certificates.is_empty()
            && self.substitution_certificates.is_empty()
            && self.finite_patch_extension_certificates.is_empty()
        {
            return Err(GeneratedPatternReplayShapeError::MissingReplayCertificate.into());
        }
        let stable_token = replay_suite_stable_token(
            &replay_suite_id,
            &self.quotient_reference,
            self.periodic_quotient_cell.as_ref(),
            &self.periodic_quotient_conflicts,
            &self.color_holonomy_loops,
            &self.translation_rotation_certificates,
            &self.substitution_certificates,
            &self.finite_patch_extension_certificates,
        );
        let mut parents = vec![self.quotient_reference.clone()];
        if let Some(quotient) = &self.periodic_quotient_cell {
            parents.push(quotient.reference());
        }
        let core = artifact_core(
            HadwigerArtifactKind::GeneratedPatternReplaySuite,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "generated_pattern_replay_suite".to_string(),
            },
            parents,
            vec![
                HadwigerArtifactPayloadEntry::text(
                    "schema",
                    "forge.hadwiger.generated_pattern_replay_suite.v1",
                ),
                HadwigerArtifactPayloadEntry::text("suite", stable_token),
            ],
        )?;
        Ok(GeneratedPatternReplaySuite {
            core,
            replay_suite_id,
            quotient_reference: self.quotient_reference,
            periodic_quotient_cell: self.periodic_quotient_cell,
            periodic_quotient_conflicts: self.periodic_quotient_conflicts,
            color_holonomy_loops: self.color_holonomy_loops,
            translation_rotation_certificates: self.translation_rotation_certificates,
            substitution_certificates: self.substitution_certificates,
            finite_patch_extension_certificates: self.finite_patch_extension_certificates,
        })
    }
}

fn replay_suite_stable_token(
    replay_suite_id: &str,
    quotient_reference: &HadwigerArtifactReference,
    quotient_cell: Option<&PeriodicQuotientCell>,
    periodic_quotient_conflicts: &[PeriodicQuotientConflictReplayCertificate],
    loops: &[ColorHolonomyLoopCertificate],
    translation_rotation_certificates: &[TranslationRotationClosureReplayCertificate],
    substitution_certificates: &[SubstitutionConsistencyCertificate],
    finite_patch_extension_certificates: &[FinitePatchBoundaryExtensionCertificate],
) -> String {
    let loop_tokens = loops
        .iter()
        .map(ColorHolonomyLoopCertificate::stable_token)
        .collect::<Vec<_>>()
        .join("|");
    let quotient_conflict_tokens = periodic_quotient_conflicts
        .iter()
        .map(PeriodicQuotientConflictReplayCertificate::stable_token)
        .collect::<Vec<_>>()
        .join("|");
    let substitution_tokens = substitution_certificates
        .iter()
        .map(SubstitutionConsistencyCertificate::stable_token)
        .collect::<Vec<_>>()
        .join("|");
    let translation_rotation_tokens = translation_rotation_certificates
        .iter()
        .map(TranslationRotationClosureReplayCertificate::stable_token)
        .collect::<Vec<_>>()
        .join("|");
    let extension_tokens = finite_patch_extension_certificates
        .iter()
        .map(FinitePatchBoundaryExtensionCertificate::stable_token)
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        replay_suite_id,
        quotient_reference.stable_token(),
        quotient_cell
            .map(PeriodicQuotientCell::stable_token)
            .unwrap_or_else(|| "no_periodic_quotient_cell".to_string()),
        quotient_conflict_tokens,
        loop_tokens,
        translation_rotation_tokens,
        substitution_tokens,
        extension_tokens
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientConflictReplayCertificate {
    model: PeriodicQuotientRectangleModel,
    certificate: PeriodicQuotientConflictCertificate,
}

impl PeriodicQuotientConflictReplayCertificate {
    pub(crate) fn new(
        model: PeriodicQuotientRectangleModel,
        certificate: PeriodicQuotientConflictCertificate,
    ) -> Self {
        Self { model, certificate }
    }

    pub(crate) fn model(&self) -> PeriodicQuotientRectangleModel {
        self.model.clone()
    }

    pub(crate) fn certificate(&self) -> PeriodicQuotientConflictCertificate {
        self.certificate.clone()
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}",
            self.model.stable_token(),
            self.certificate.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationRotationClosureReplayCertificate {
    graph: GraphVersion,
    certificate: TranslationRotationClosureCertificate,
}

impl TranslationRotationClosureReplayCertificate {
    pub(crate) fn new(
        graph: GraphVersion,
        certificate: TranslationRotationClosureCertificate,
    ) -> Self {
        Self { graph, certificate }
    }

    pub(crate) fn graph(&self) -> &GraphVersion {
        &self.graph
    }

    pub(crate) fn certificate(&self) -> TranslationRotationClosureCertificate {
        self.certificate.clone()
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}",
            self.graph.reference().stable_token(),
            self.certificate.stable_token()
        )
    }
}
