use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::{
    GraphVersion, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
};

use super::ScreeningSolverTranscript;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymmetryOrbitReductionCertificate {
    certificate_id: String,
    permutations: Vec<Vec<(String, String)>>,
    solver_transcript: ScreeningSolverTranscript,
}

impl SymmetryOrbitReductionCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        mut permutations: Vec<Vec<(String, String)>>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if permutations.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "symmetry_permutations",
            });
        }
        for permutation in &mut permutations {
            normalize_mapping(permutation)?;
        }
        permutations.sort();
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            permutations,
            solver_transcript,
        })
    }

    pub(crate) fn permutations(&self) -> &[Vec<(String, String)>] {
        &self.permutations
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{:?}:{}",
            self.certificate_id,
            self.permutations,
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExhaustiveLocalNeighborhoodCertificate {
    certificate_id: String,
    root_vertex: String,
    radius: usize,
    expected_vertices: Vec<String>,
    solver_transcript: ScreeningSolverTranscript,
}

impl ExhaustiveLocalNeighborhoodCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        root_vertex: impl Into<String>,
        radius: usize,
        mut expected_vertices: Vec<String>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if expected_vertices.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "expected_neighborhood_vertices",
            });
        }
        for vertex in &expected_vertices {
            require_non_empty(vertex.clone(), "expected_neighborhood_vertex")?;
        }
        expected_vertices.sort();
        expected_vertices.dedup();
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            root_vertex: require_non_empty(root_vertex, "root_vertex")?,
            radius,
            expected_vertices,
            solver_transcript,
        })
    }

    pub(crate) fn root_vertex(&self) -> &str {
        &self.root_vertex
    }

    pub(crate) fn radius(&self) -> usize {
        self.radius
    }

    pub(crate) fn expected_vertices(&self) -> &[String] {
        &self.expected_vertices
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{:?}:{}",
            self.certificate_id,
            self.root_vertex,
            self.radius,
            self.expected_vertices,
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KnownObstructionContainmentCertificate {
    certificate_id: String,
    obstruction_graph: GraphVersion,
    vertex_mapping: Vec<(String, String)>,
    solver_transcript: ScreeningSolverTranscript,
}

impl KnownObstructionContainmentCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        obstruction_graph: GraphVersion,
        mut vertex_mapping: Vec<(String, String)>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        normalize_mapping(&mut vertex_mapping)?;
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            obstruction_graph,
            vertex_mapping,
            solver_transcript,
        })
    }

    pub(crate) fn obstruction_graph(&self) -> &GraphVersion {
        &self.obstruction_graph
    }

    pub(crate) fn vertex_mapping(&self) -> &[(String, String)] {
        &self.vertex_mapping
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{:?}:{}",
            self.certificate_id,
            self.obstruction_graph.reference().stable_token(),
            self.vertex_mapping,
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateNoveltyCertificate {
    certificate_id: String,
    known_fingerprint: String,
    wl_rounds: usize,
    solver_transcript: ScreeningSolverTranscript,
}

impl CandidateNoveltyCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        known_fingerprint: impl Into<String>,
        wl_rounds: usize,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            known_fingerprint: require_non_empty(known_fingerprint, "known_fingerprint")?,
            wl_rounds,
            solver_transcript,
        })
    }

    pub(crate) fn known_fingerprint(&self) -> &str {
        &self.known_fingerprint
    }

    pub(crate) fn wl_rounds(&self) -> usize {
        self.wl_rounds
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.certificate_id,
            self.known_fingerprint,
            self.wl_rounds,
            self.solver_transcript.stable_token()
        )
    }
}

fn normalize_mapping(
    mapping: &mut Vec<(String, String)>,
) -> Result<(), HadwigerArtifactShapeError> {
    for (left, right) in mapping.iter() {
        require_non_empty(left.clone(), "mapping_left_vertex")?;
        require_non_empty(right.clone(), "mapping_right_vertex")?;
    }
    mapping.sort();
    mapping.dedup();
    Ok(())
}
