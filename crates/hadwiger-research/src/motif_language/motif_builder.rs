use std::collections::BTreeSet;

use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerQueryDeclarationReference;

use super::motif_artifacts::{
    MotifArtifact, MotifArtifactCheckedInput, MotifGeometryTemplateReference,
    MotifProofSupportPosture,
};
use super::motif_errors::MotifLanguageError;
use super::motif_identity::{
    MotifForbiddenSameColorPair, MotifParameterBinding, MotifTerminal, MotifUnitEdge, MotifVertex,
};

#[derive(Clone, Debug)]
pub struct MotifArtifactBuilder {
    motif_id: String,
    source_declaration: HadwigerQueryDeclarationReference,
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

impl MotifArtifactBuilder {
    pub(crate) fn new(
        motif_id: impl Into<String>,
        source_declaration: HadwigerQueryDeclarationReference,
    ) -> Self {
        Self {
            motif_id: motif_id.into(),
            source_declaration,
            source_family: None,
            novelty_signature: None,
            geometry_template: None,
            proof_support_posture: MotifProofSupportPosture::Candidate,
            vertices: Vec::new(),
            parameters: Vec::new(),
            terminals: Vec::new(),
            unit_edges: Vec::new(),
            forbidden_same_color_pairs: Vec::new(),
        }
    }

    pub(crate) fn source_declaration(&self) -> &HadwigerQueryDeclarationReference {
        &self.source_declaration
    }

    pub fn with_source_family(
        mut self,
        source_family: impl Into<String>,
    ) -> Result<Self, MotifLanguageError> {
        self.source_family = Some(require_non_empty(source_family, "source_family")?);
        Ok(self)
    }

    pub fn with_novelty_signature(
        mut self,
        novelty_signature: impl Into<String>,
    ) -> Result<Self, MotifLanguageError> {
        self.novelty_signature = Some(require_non_empty(novelty_signature, "novelty_signature")?);
        Ok(self)
    }

    pub fn with_geometry_template(
        mut self,
        geometry_template: MotifGeometryTemplateReference,
    ) -> Self {
        self.geometry_template = Some(geometry_template);
        self
    }

    pub fn with_proof_support_posture(mut self, posture: MotifProofSupportPosture) -> Self {
        self.proof_support_posture = posture;
        self
    }

    pub fn with_terminal(mut self, terminal: MotifTerminal) -> Result<Self, MotifLanguageError> {
        reject_duplicate(
            self.terminals.iter().map(MotifTerminal::label),
            "terminal",
            terminal.label(),
        )?;
        self.terminals.push(terminal);
        self.terminals.sort();
        Ok(self)
    }

    pub fn with_vertex(mut self, vertex: MotifVertex) -> Result<Self, MotifLanguageError> {
        reject_duplicate(
            self.vertices.iter().map(MotifVertex::label),
            "vertex",
            vertex.label(),
        )?;
        self.vertices.push(vertex);
        self.vertices.sort();
        Ok(self)
    }

    pub fn with_parameter(
        mut self,
        parameter: MotifParameterBinding,
    ) -> Result<Self, MotifLanguageError> {
        reject_duplicate(
            self.parameters.iter().map(MotifParameterBinding::name),
            "parameter",
            parameter.name(),
        )?;
        self.parameters.push(parameter);
        self.parameters.sort();
        Ok(self)
    }

    pub fn with_unit_edge(mut self, unit_edge: MotifUnitEdge) -> Result<Self, MotifLanguageError> {
        self.unit_edges.push(unit_edge);
        self.unit_edges.sort();
        self.unit_edges.dedup();
        Ok(self)
    }

    pub fn with_forbidden_same_color_pair(
        mut self,
        pair: MotifForbiddenSameColorPair,
    ) -> Result<Self, MotifLanguageError> {
        self.forbidden_same_color_pairs.push(pair);
        self.forbidden_same_color_pairs.sort();
        self.forbidden_same_color_pairs.dedup();
        Ok(self)
    }

    pub fn finish(self) -> Result<MotifArtifact, MotifLanguageError> {
        let motif_id = require_non_empty(self.motif_id, "motif_id")?;
        let vertex_index = self
            .vertices
            .iter()
            .map(|vertex| vertex.label().to_string())
            .collect::<BTreeSet<_>>();
        let terminal_index = self
            .terminals
            .iter()
            .map(|terminal| terminal.label().to_string())
            .collect::<BTreeSet<_>>();
        for edge in &self.unit_edges {
            require_vertex(&vertex_index, edge.left_label())?;
            require_vertex(&vertex_index, edge.right_label())?;
        }
        for pair in &self.forbidden_same_color_pairs {
            require_terminal(&terminal_index, pair.left_label())?;
            require_terminal(&terminal_index, pair.right_label())?;
        }
        MotifArtifact::checked(MotifArtifactCheckedInput {
            motif_id,
            source_declaration: self.source_declaration,
            source_family: self.source_family,
            novelty_signature: self.novelty_signature,
            geometry_template: self.geometry_template,
            proof_support_posture: self.proof_support_posture,
            vertices: self.vertices,
            parameters: self.parameters,
            terminals: self.terminals,
            unit_edges: self.unit_edges,
            forbidden_same_color_pairs: self.forbidden_same_color_pairs,
        })
        .map_err(Into::into)
    }
}

fn reject_duplicate<'a>(
    mut existing: impl Iterator<Item = &'a str>,
    field: &'static str,
    candidate: &str,
) -> Result<(), MotifLanguageError> {
    if existing.any(|value| value == candidate) {
        return Err(MotifLanguageError::DuplicateIdentityField {
            field,
            value: candidate.to_string(),
        });
    }
    Ok(())
}

fn require_vertex(
    vertex_index: &BTreeSet<String>,
    vertex_label: &str,
) -> Result<(), MotifLanguageError> {
    if vertex_index.contains(vertex_label) {
        Ok(())
    } else {
        Err(MotifLanguageError::MissingMotifVertex {
            vertex_label: vertex_label.to_string(),
        })
    }
}

fn require_terminal(
    terminal_index: &BTreeSet<String>,
    terminal_label: &str,
) -> Result<(), MotifLanguageError> {
    if terminal_index.contains(terminal_label) {
        Ok(())
    } else {
        Err(MotifLanguageError::MissingMotifTerminal {
            terminal_label: terminal_label.to_string(),
        })
    }
}
