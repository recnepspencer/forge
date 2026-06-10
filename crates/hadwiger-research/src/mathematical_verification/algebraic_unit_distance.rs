use std::collections::{BTreeMap, BTreeSet};

use crate::aspect_authority::{HadwigerAspectAuthorityError, UnitDistanceAspectRecord};
use crate::domain_artifacts::{
    GraphVersion, HadwigerArtifactReference, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
    HadwigerCheckerPosture, UnitDistanceVerification,
};
use crate::domain_declarations::UnitDistanceVerificationDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::algebraic_numbers::{parse_algebraic_scalar, AlgebraicPoint2};
use super::{admitted_declaration_reference, checker_evidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerAlgebraicGeometryError {
    EmptyField { field: &'static str },
    ZeroDenominator,
    NonSquarefreeRadicand { radicand: i128 },
    UnsupportedAlgebraicScalar { value: String },
    DuplicateCoordinate { vertex_label: String },
    MissingCoordinate { vertex_label: String },
    DuplicatePoint,
    NonUnitEdge { left: String, right: String },
    QueryDeclarationNotAdmitted,
    Artifact(HadwigerArtifactShapeError),
    Aspect(HadwigerAspectAuthorityError),
}

impl From<HadwigerArtifactShapeError> for HadwigerAlgebraicGeometryError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(value)
    }
}

impl From<HadwigerAspectAuthorityError> for HadwigerAlgebraicGeometryError {
    fn from(value: HadwigerAspectAuthorityError) -> Self {
        Self::Aspect(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlgebraicGraphEmbedding {
    graph_version_reference: HadwigerArtifactReference,
    embedding_id: String,
    coordinates: BTreeMap<String, AlgebraicPoint2>,
}

impl AlgebraicGraphEmbedding {
    pub fn builder(
        graph_version_reference: HadwigerArtifactReference,
        embedding_id: impl Into<String>,
    ) -> AlgebraicGraphEmbeddingBuilder {
        AlgebraicGraphEmbeddingBuilder {
            graph_version_reference,
            embedding_id: embedding_id.into(),
            coordinates: BTreeMap::new(),
        }
    }

    pub fn from_seed_certificate(
        seed: &crate::frontier_seeds::FrontierGraphSeedArtifact,
    ) -> Result<Self, HadwigerAlgebraicGeometryError> {
        let certificate = seed.algebraic_embedding_certificate().ok_or(
            HadwigerAlgebraicGeometryError::EmptyField {
                field: "algebraic_seed_certificate",
            },
        )?;
        let graph_version_reference = seed.parent_artifacts().first().cloned().ok_or(
            HadwigerAlgebraicGeometryError::EmptyField {
                field: "seed_graph_version_reference",
            },
        )?;
        parse_seed_embedding_certificate(graph_version_reference, certificate)
    }

    pub fn coordinate(&self, vertex_label: &str) -> Option<&AlgebraicPoint2> {
        self.coordinates.get(vertex_label)
    }

    fn embedding_id(&self) -> &str {
        &self.embedding_id
    }

    fn reference(&self) -> HadwigerArtifactReference {
        self.graph_version_reference.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlgebraicGraphEmbeddingBuilder {
    graph_version_reference: HadwigerArtifactReference,
    embedding_id: String,
    coordinates: BTreeMap<String, AlgebraicPoint2>,
}

impl AlgebraicGraphEmbeddingBuilder {
    pub fn with_vertex(
        mut self,
        vertex_label: impl Into<String>,
        point: AlgebraicPoint2,
    ) -> Result<Self, HadwigerAlgebraicGeometryError> {
        let vertex_label = non_empty(vertex_label, "vertex_label")?;
        if self
            .coordinates
            .insert(vertex_label.clone(), point)
            .is_some()
        {
            return Err(HadwigerAlgebraicGeometryError::DuplicateCoordinate { vertex_label });
        }
        Ok(self)
    }

    pub fn finish(self) -> Result<AlgebraicGraphEmbedding, HadwigerAlgebraicGeometryError> {
        let embedding_id = non_empty(self.embedding_id, "embedding_id")?;
        let mut seen_points = BTreeSet::new();
        for point in self.coordinates.values() {
            if !seen_points.insert(point.stable_token()) {
                return Err(HadwigerAlgebraicGeometryError::DuplicatePoint);
            }
        }
        Ok(AlgebraicGraphEmbedding {
            graph_version_reference: self.graph_version_reference,
            embedding_id,
            coordinates: self.coordinates,
        })
    }
}

pub type AlgebraicUnitDistanceCertificate = AlgebraicGraphEmbedding;

pub fn verify_algebraic_unit_distance_embedding_checked(
    handle: &HadwigerResearchHandle,
    graph_version: &GraphVersion,
    embedding: AlgebraicGraphEmbedding,
) -> Result<super::UnitDistanceVerificationChecked, HadwigerAlgebraicGeometryError> {
    let declared = handle.declare_checked(UnitDistanceVerificationDeclaration::new(
        graph_version.version_id(),
        embedding.embedding_id(),
    ));
    let query_declaration_reference = admitted_declaration_reference(declared)
        .ok_or(HadwigerAlgebraicGeometryError::QueryDeclarationNotAdmitted)?;
    let query_identity = format!(
        "{}:{}",
        handle.handle_identity_digest(),
        query_declaration_reference.declaration_digest()
    );
    let mut rejection = None;
    for edge in graph_version.edges() {
        let (left, right) = edge.endpoints();
        let left_point = embedding.coordinate(left).ok_or_else(|| {
            HadwigerAlgebraicGeometryError::MissingCoordinate {
                vertex_label: left.to_string(),
            }
        })?;
        let right_point = embedding.coordinate(right).ok_or_else(|| {
            HadwigerAlgebraicGeometryError::MissingCoordinate {
                vertex_label: right.to_string(),
            }
        })?;
        if !left_point.squared_distance(right_point).is_one() {
            rejection = Some((left.to_string(), right.to_string()));
            break;
        }
    }
    let posture = if rejection.is_some() {
        HadwigerCheckerPosture::Rejected
    } else {
        HadwigerCheckerPosture::Admitted
    };
    let verification = UnitDistanceVerification::checked(
        embedding.reference(),
        query_declaration_reference,
        "hadwiger.algebraic_unit_distance",
        "0.1.0",
        posture,
        checker_evidence("algebraic-unit-distance", &query_identity)?,
    )?;
    let unit_distance_aspect = if let Some((left, right)) = rejection {
        UnitDistanceAspectRecord::rejected(
            graph_version.reference(),
            format!("algebraic checker rejected edge {left}:{right}"),
        )?
    } else {
        UnitDistanceAspectRecord::admitted_checked(
            graph_version.reference(),
            "algebraic checker admitted all graph edges",
        )?
    };
    Ok(super::UnitDistanceVerificationChecked::new(
        verification,
        unit_distance_aspect,
    ))
}

fn non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, HadwigerAlgebraicGeometryError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(HadwigerAlgebraicGeometryError::EmptyField { field })
    } else {
        Ok(value)
    }
}

fn empty(field: &'static str) -> HadwigerAlgebraicGeometryError {
    HadwigerAlgebraicGeometryError::EmptyField { field }
}

fn parse_seed_embedding_certificate(
    graph_version_reference: HadwigerArtifactReference,
    certificate: &str,
) -> Result<AlgebraicGraphEmbedding, HadwigerAlgebraicGeometryError> {
    let mut embedding_id = None;
    let mut builder = None;
    for line in certificate.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.is_empty() || parts[0] == "c" {
            continue;
        }
        match parts.as_slice() {
            ["embedding", id] => {
                embedding_id = Some(non_empty(*id, "embedding_id")?);
                builder = Some(AlgebraicGraphEmbedding::builder(
                    graph_version_reference.clone(),
                    *id,
                ));
            }
            ["v", label, x, y] => {
                let current = builder.take().ok_or(empty("embedding_header"))?;
                builder = Some(current.with_vertex(
                    *label,
                    AlgebraicPoint2::new(parse_algebraic_scalar(x)?, parse_algebraic_scalar(y)?),
                )?);
            }
            _ => return Err(empty("algebraic_seed_certificate_row")),
        }
    }
    if embedding_id.is_none() {
        return Err(empty("embedding_id"));
    }
    builder.ok_or(empty("embedding_builder"))?.finish()
}
