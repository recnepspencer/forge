use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::artifact_core;
use crate::domain_artifacts::HadwigerQueryDeclarationReference;

use super::motif_builder::MotifArtifactBuilder;
use super::motif_digest_basis::{motif_payload_entries, MotifCanonicalIndex};
use super::motif_identity::{
    MotifForbiddenSameColorPair, MotifParameterBinding, MotifTerminal, MotifUnitEdge, MotifVertex,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotifProofSupportPosture {
    Candidate,
    Advisory,
    Blocked,
    CheckerSupported,
}

impl MotifProofSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::Advisory => "advisory",
            Self::Blocked => "blocked",
            Self::CheckerSupported => "checker_supported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotifGeometryTemplateReference {
    template_reference: String,
}

impl MotifGeometryTemplateReference {
    pub fn new(template_reference: impl Into<String>) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            template_reference: require_non_empty(template_reference, "geometry_template_ref")?,
        })
    }

    pub fn stable_token(&self) -> String {
        self.template_reference.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotifArtifact {
    core: HadwigerArtifactCore,
    motif_id: String,
    source_family: Option<String>,
    novelty_signature: Option<String>,
    geometry_template: Option<MotifGeometryTemplateReference>,
    proof_support_posture: MotifProofSupportPosture,
    vertices: Vec<MotifVertex>,
    parameters: Vec<MotifParameterBinding>,
    terminals: Vec<MotifTerminal>,
    unit_edges: Vec<MotifUnitEdge>,
    forbidden_same_color_pairs: Vec<MotifForbiddenSameColorPair>,
}

impl MotifArtifact {
    pub fn builder(
        motif_id: impl Into<String>,
        source_declaration: HadwigerQueryDeclarationReference,
    ) -> MotifArtifactBuilder {
        MotifArtifactBuilder::new(motif_id, source_declaration)
    }

    pub(crate) fn checked(
        input: MotifArtifactCheckedInput,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let motif_index = MotifCanonicalIndex {
            vertices: &input.vertices,
            parameters: &input.parameters,
            terminals: &input.terminals,
            unit_edges: &input.unit_edges,
            forbidden_pairs: &input.forbidden_same_color_pairs,
        };
        let payload_entries = motif_payload_entries(
            &input.motif_id,
            input.source_family.as_deref(),
            input.novelty_signature.as_deref(),
            input.geometry_template.as_ref(),
            input.proof_support_posture,
            &motif_index,
        );
        let core = artifact_core(
            HadwigerArtifactKind::MotifArtifact,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::QueryDeclaration(input.source_declaration),
            Vec::new(),
            payload_entries,
        )?;
        Ok(Self {
            core,
            motif_id: input.motif_id,
            source_family: input.source_family,
            novelty_signature: input.novelty_signature,
            geometry_template: input.geometry_template,
            proof_support_posture: input.proof_support_posture,
            vertices: input.vertices,
            parameters: input.parameters,
            terminals: input.terminals,
            unit_edges: input.unit_edges,
            forbidden_same_color_pairs: input.forbidden_same_color_pairs,
        })
    }

    pub fn motif_id(&self) -> &str {
        &self.motif_id
    }

    pub fn terminals(&self) -> &[MotifTerminal] {
        &self.terminals
    }

    pub fn vertices(&self) -> &[MotifVertex] {
        &self.vertices
    }

    pub fn unit_edges(&self) -> &[MotifUnitEdge] {
        &self.unit_edges
    }

    pub fn forbidden_same_color_pairs(&self) -> &[MotifForbiddenSameColorPair] {
        &self.forbidden_same_color_pairs
    }

    pub fn parameters(&self) -> &[MotifParameterBinding] {
        &self.parameters
    }

    pub fn proof_support_posture(&self) -> MotifProofSupportPosture {
        self.proof_support_posture
    }

    pub fn source_family(&self) -> Option<&str> {
        self.source_family.as_deref()
    }

    pub fn novelty_signature(&self) -> Option<&str> {
        self.novelty_signature.as_deref()
    }

    pub fn geometry_template(&self) -> Option<&MotifGeometryTemplateReference> {
        self.geometry_template.as_ref()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn claims_terminal_forcing_authority(&self) -> bool {
        false
    }
}

pub(crate) struct MotifArtifactCheckedInput {
    pub(crate) motif_id: String,
    pub(crate) source_declaration: HadwigerQueryDeclarationReference,
    pub(crate) source_family: Option<String>,
    pub(crate) novelty_signature: Option<String>,
    pub(crate) geometry_template: Option<MotifGeometryTemplateReference>,
    pub(crate) proof_support_posture: MotifProofSupportPosture,
    pub(crate) vertices: Vec<MotifVertex>,
    pub(crate) parameters: Vec<MotifParameterBinding>,
    pub(crate) terminals: Vec<MotifTerminal>,
    pub(crate) unit_edges: Vec<MotifUnitEdge>,
    pub(crate) forbidden_same_color_pairs: Vec<MotifForbiddenSameColorPair>,
}

impl_hadwiger_artifact!(MotifArtifact, core);
