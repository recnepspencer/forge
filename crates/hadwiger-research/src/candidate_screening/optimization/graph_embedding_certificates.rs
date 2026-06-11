use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;
use crate::mathematical_verification::ExactGraphEmbedding;

use super::{ScreeningRectangularRegion, ScreeningSolverTranscript};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RigidityRealizationPosture {
    Impossible,
    Flexible,
    LocallyRigid,
    GloballyRigidUnsupported,
}

impl RigidityRealizationPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Impossible => "impossible",
            Self::Flexible => "flexible",
            Self::LocallyRigid => "locally_rigid",
            Self::GloballyRigidUnsupported => "globally_rigid_unsupported",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitDistanceEmbeddabilityCertificate {
    certificate_id: String,
    embedding: ExactGraphEmbedding,
    non_edge_exclusions: Vec<(String, String)>,
    solver_transcript: ScreeningSolverTranscript,
}

impl UnitDistanceEmbeddabilityCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        embedding: ExactGraphEmbedding,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            embedding,
            non_edge_exclusions: Vec::new(),
            solver_transcript,
        })
    }

    pub fn with_non_edge_exclusion(
        mut self,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let left = require_non_empty(left, "left_vertex_label")?;
        let right = require_non_empty(right, "right_vertex_label")?;
        self.non_edge_exclusions.push(normalized_pair(left, right));
        self.non_edge_exclusions.sort();
        self.non_edge_exclusions.dedup();
        Ok(self)
    }

    pub(crate) fn embedding(&self) -> &ExactGraphEmbedding {
        &self.embedding
    }

    pub(crate) fn non_edge_exclusions(&self) -> &[(String, String)] {
        &self.non_edge_exclusions
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{:?}:{}",
            self.certificate_id,
            embedding_token(&self.embedding),
            self.non_edge_exclusions,
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RigidityRealizationCertificate {
    certificate_id: String,
    embedding: ExactGraphEmbedding,
    expected_posture: RigidityRealizationPosture,
    solver_transcript: ScreeningSolverTranscript,
}

impl RigidityRealizationCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        embedding: ExactGraphEmbedding,
        expected_posture: RigidityRealizationPosture,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            embedding,
            expected_posture,
            solver_transcript,
        })
    }

    pub(crate) fn embedding(&self) -> &ExactGraphEmbedding {
        &self.embedding
    }

    pub fn expected_posture(&self) -> RigidityRealizationPosture {
        self.expected_posture
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.certificate_id,
            embedding_token(&self.embedding),
            self.expected_posture.as_str(),
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactArithmeticIntervalExpectation {
    UnitContained,
    UnitExcluded,
}

impl ExactArithmeticIntervalExpectation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnitContained => "unit_contained",
            Self::UnitExcluded => "unit_excluded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactArithmeticIntervalCertificate {
    PointPair {
        certificate_id: String,
        embedding: ExactGraphEmbedding,
        left_vertex: String,
        right_vertex: String,
        expectation: ExactArithmeticIntervalExpectation,
        solver_transcript: ScreeningSolverTranscript,
    },
    RectanglePair {
        certificate_id: String,
        left_region: ScreeningRectangularRegion,
        right_region: ScreeningRectangularRegion,
        expectation: ExactArithmeticIntervalExpectation,
        solver_transcript: ScreeningSolverTranscript,
    },
}

impl ExactArithmeticIntervalCertificate {
    pub fn point_pair(
        certificate_id: impl Into<String>,
        embedding: ExactGraphEmbedding,
        left_vertex: impl Into<String>,
        right_vertex: impl Into<String>,
        expectation: ExactArithmeticIntervalExpectation,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self::PointPair {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            embedding,
            left_vertex: require_non_empty(left_vertex, "left_vertex")?,
            right_vertex: require_non_empty(right_vertex, "right_vertex")?,
            expectation,
            solver_transcript,
        })
    }

    pub fn rectangle_pair(
        certificate_id: impl Into<String>,
        left_region: ScreeningRectangularRegion,
        right_region: ScreeningRectangularRegion,
        expectation: ExactArithmeticIntervalExpectation,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self::RectanglePair {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            left_region,
            right_region,
            expectation,
            solver_transcript,
        })
    }

    pub fn stable_token(&self) -> String {
        match self {
            Self::PointPair {
                certificate_id,
                embedding,
                left_vertex,
                right_vertex,
                expectation,
                solver_transcript,
            } => format!(
                "{certificate_id}:point_pair:{}:{left_vertex}:{right_vertex}:{}:{}",
                embedding_token(embedding),
                expectation.as_str(),
                solver_transcript.stable_token()
            ),
            Self::RectanglePair {
                certificate_id,
                left_region,
                right_region,
                expectation,
                solver_transcript,
            } => format!(
                "{certificate_id}:rectangle_pair:{}:{}:{}:{}",
                left_region.stable_token(),
                right_region.stable_token(),
                expectation.as_str(),
                solver_transcript.stable_token()
            ),
        }
    }
}

fn normalized_pair(left: String, right: String) -> (String, String) {
    if left <= right {
        (left, right)
    } else {
        (right, left)
    }
}

fn embedding_token(embedding: &ExactGraphEmbedding) -> String {
    let mut token = embedding.embedding_id().to_string();
    for (vertex, point) in embedding.coordinates() {
        token.push_str(&format!(":{vertex}@{}", point.stable_token()));
    }
    token
}
