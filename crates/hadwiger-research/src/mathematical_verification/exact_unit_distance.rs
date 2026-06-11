use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::ForgeQueryDeclaredFamilyChecked;

use crate::aspect_authority::{HadwigerAspectAuthorityError, UnitDistanceAspectRecord};
use crate::domain_artifacts::{
    GraphVersion, HadwigerArtifactReference, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
    HadwigerCheckerCausalEvidence, HadwigerCheckerPosture, HadwigerQueryDeclarationReference,
    UnitDistanceVerification,
};
use crate::domain_declarations::UnitDistanceVerificationDeclaration;
use crate::query_entry::HadwigerResearchHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerExactGeometryError {
    EmptyField { field: &'static str },
    ZeroDenominator,
    DuplicateCoordinate { vertex_label: String },
    MissingCoordinate { vertex_label: String },
    DuplicatePoint,
    NonUnitEdge { left: String, right: String },
    QueryDeclarationNotAdmitted,
    Artifact(HadwigerArtifactShapeError),
    Aspect(HadwigerAspectAuthorityError),
}

impl From<HadwigerArtifactShapeError> for HadwigerExactGeometryError {
    fn from(value: HadwigerArtifactShapeError) -> Self {
        Self::Artifact(value)
    }
}

impl From<HadwigerAspectAuthorityError> for HadwigerExactGeometryError {
    fn from(value: HadwigerAspectAuthorityError) -> Self {
        Self::Aspect(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExactRational {
    numerator: i128,
    denominator: i128,
}

impl ExactRational {
    pub fn integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    pub fn fraction(
        numerator: i128,
        denominator: i128,
    ) -> Result<Self, HadwigerExactGeometryError> {
        if denominator == 0 {
            return Err(HadwigerExactGeometryError::ZeroDenominator);
        }
        let mut numerator = numerator;
        let mut denominator = denominator;
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator as u128) as i128;
        Ok(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }

    pub(crate) fn zero() -> Self {
        Self::integer(0)
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        Self::fraction(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
        .expect("normalized rational subtraction keeps non-zero denominator")
    }

    pub(crate) fn square(&self) -> Self {
        Self::fraction(
            self.numerator * self.numerator,
            self.denominator * self.denominator,
        )
        .expect("normalized rational square keeps non-zero denominator")
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        Self::fraction(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
        .expect("normalized rational addition keeps non-zero denominator")
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        Self::fraction(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
        .expect("normalized rational multiplication keeps non-zero denominator")
    }

    pub(crate) fn div(&self, other: &Self) -> Option<Self> {
        if other.is_zero() {
            None
        } else {
            Self::fraction(
                self.numerator * other.denominator,
                self.denominator * other.numerator,
            )
            .ok()
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    pub(crate) fn stable_token(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExactPoint2 {
    x: ExactRational,
    y: ExactRational,
}

impl ExactPoint2 {
    pub fn integer(x: i128, y: i128) -> Self {
        Self {
            x: ExactRational::integer(x),
            y: ExactRational::integer(y),
        }
    }

    pub fn fraction(
        x_numerator: i128,
        x_denominator: i128,
        y_numerator: i128,
        y_denominator: i128,
    ) -> Result<Self, HadwigerExactGeometryError> {
        Ok(Self {
            x: ExactRational::fraction(x_numerator, x_denominator)?,
            y: ExactRational::fraction(y_numerator, y_denominator)?,
        })
    }

    pub(crate) fn squared_distance(&self, other: &Self) -> ExactRational {
        self.x
            .sub(&other.x)
            .square()
            .add(&self.y.sub(&other.y).square())
    }

    pub(crate) fn x(&self) -> &ExactRational {
        &self.x
    }

    pub(crate) fn y(&self) -> &ExactRational {
        &self.y
    }

    pub(crate) fn stable_token(&self) -> String {
        format!("{},{}", self.x.stable_token(), self.y.stable_token())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactGraphEmbedding {
    graph_version_reference: HadwigerArtifactReference,
    embedding_id: String,
    coordinates: BTreeMap<String, ExactPoint2>,
}

impl ExactGraphEmbedding {
    pub fn builder(
        graph_version_reference: HadwigerArtifactReference,
        embedding_id: impl Into<String>,
    ) -> ExactGraphEmbeddingBuilder {
        ExactGraphEmbeddingBuilder {
            graph_version_reference,
            embedding_id: embedding_id.into(),
            coordinates: BTreeMap::new(),
        }
    }

    pub fn coordinate(&self, vertex_label: &str) -> Option<&ExactPoint2> {
        self.coordinates.get(vertex_label)
    }

    pub(crate) fn coordinates(&self) -> &BTreeMap<String, ExactPoint2> {
        &self.coordinates
    }

    pub(crate) fn embedding_id(&self) -> &str {
        &self.embedding_id
    }

    pub fn reference(&self) -> HadwigerArtifactReference {
        self.graph_version_reference.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactGraphEmbeddingBuilder {
    graph_version_reference: HadwigerArtifactReference,
    embedding_id: String,
    coordinates: BTreeMap<String, ExactPoint2>,
}

impl ExactGraphEmbeddingBuilder {
    pub fn with_vertex(
        mut self,
        vertex_label: impl Into<String>,
        point: ExactPoint2,
    ) -> Result<Self, HadwigerExactGeometryError> {
        let vertex_label = non_empty(vertex_label, "vertex_label")?;
        if self
            .coordinates
            .insert(vertex_label.clone(), point)
            .is_some()
        {
            return Err(HadwigerExactGeometryError::DuplicateCoordinate { vertex_label });
        }
        Ok(self)
    }

    pub fn finish(self) -> Result<ExactGraphEmbedding, HadwigerExactGeometryError> {
        let embedding_id = non_empty(self.embedding_id, "embedding_id")?;
        let mut seen_points = BTreeSet::new();
        for point in self.coordinates.values() {
            if !seen_points.insert(point.stable_token()) {
                return Err(HadwigerExactGeometryError::DuplicatePoint);
            }
        }
        Ok(ExactGraphEmbedding {
            graph_version_reference: self.graph_version_reference,
            embedding_id,
            coordinates: self.coordinates,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitDistanceVerificationChecked {
    verification: UnitDistanceVerification,
    unit_distance_aspect: UnitDistanceAspectRecord,
}

impl UnitDistanceVerificationChecked {
    pub fn verification(&self) -> &UnitDistanceVerification {
        &self.verification
    }

    pub fn unit_distance_aspect(&self) -> &UnitDistanceAspectRecord {
        &self.unit_distance_aspect
    }
}

pub fn verify_unit_distance_embedding_checked(
    handle: &HadwigerResearchHandle,
    graph_version: &GraphVersion,
    embedding: ExactGraphEmbedding,
) -> Result<UnitDistanceVerificationChecked, HadwigerExactGeometryError> {
    let declared = handle.declare_checked(UnitDistanceVerificationDeclaration::new(
        graph_version.version_id(),
        embedding.embedding_id.clone(),
    ));
    let query_declaration_reference = admitted_declaration_reference(declared)
        .ok_or(HadwigerExactGeometryError::QueryDeclarationNotAdmitted)?;
    let query_identity = format!(
        "{}:{}",
        handle.handle_identity_digest(),
        query_declaration_reference.declaration_digest()
    );
    let mut rejection = None;
    for edge in graph_version.edges() {
        let (left, right) = edge.endpoints();
        let left_point = embedding.coordinate(left).ok_or_else(|| {
            HadwigerExactGeometryError::MissingCoordinate {
                vertex_label: left.to_string(),
            }
        })?;
        let right_point = embedding.coordinate(right).ok_or_else(|| {
            HadwigerExactGeometryError::MissingCoordinate {
                vertex_label: right.to_string(),
            }
        })?;
        if left_point.squared_distance(right_point) != ExactRational::integer(1) {
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
        "hadwiger.exact_unit_distance",
        "0.1.0",
        posture,
        checker_evidence("unit-distance", &query_identity)?,
    )?;
    let unit_distance_aspect = if let Some((left, right)) = rejection {
        UnitDistanceAspectRecord::rejected(
            graph_version.reference(),
            format!("real exact unit-distance checker rejected edge {left}:{right}"),
        )?
    } else {
        UnitDistanceAspectRecord::admitted_checked(
            graph_version.reference(),
            "real exact unit-distance checker admitted all graph edges",
        )?
    };
    Ok(UnitDistanceVerificationChecked {
        verification,
        unit_distance_aspect,
    })
}

pub(crate) fn checker_evidence(
    lane: &str,
    identity: &str,
) -> Result<HadwigerCheckerCausalEvidence, HadwigerArtifactShapeError> {
    HadwigerCheckerCausalEvidence::new(
        format!("truth-view:{lane}:{identity}"),
        format!("route:{lane}:{identity}"),
        format!("evaluation:{lane}:{identity}"),
        format!("diagnostics:{lane}:{identity}"),
        format!("replay:{lane}:{identity}"),
    )
}

fn non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, HadwigerExactGeometryError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(HadwigerExactGeometryError::EmptyField { field })
    } else {
        Ok(value)
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

pub(crate) fn admitted_declaration_reference<I>(
    checked: ForgeQueryDeclaredFamilyChecked<crate::query_entry::HadwigerResearchDomainEntry, I>,
) -> Option<HadwigerQueryDeclarationReference>
where
    I: crate::domain_declarations::HadwigerResearchDeclarationInput,
{
    match checked {
        ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => Some(declaration.into()),
        _ => None,
    }
}
