use crate::domain_artifacts::{HadwigerArtifactAuthorityOwner, HadwigerArtifactReference};

use super::aspect_kinds::{
    authority_token, require_non_empty, HadwigerAspectAuthorityError, HadwigerAspectKind,
    HadwigerAspectPosture, HadwigerAspectScope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerAspectRecord {
    aspect_kind: HadwigerAspectKind,
    aspect_posture: HadwigerAspectPosture,
    artifact_reference: HadwigerArtifactReference,
    authority_owner: HadwigerArtifactAuthorityOwner,
    aspect_scope: HadwigerAspectScope,
    source_metadata: String,
}

impl HadwigerAspectRecord {
    pub(crate) fn new(
        aspect_kind: HadwigerAspectKind,
        aspect_posture: HadwigerAspectPosture,
        artifact_reference: HadwigerArtifactReference,
        authority_owner: HadwigerArtifactAuthorityOwner,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        if aspect_posture == HadwigerAspectPosture::Admitted
            && aspect_kind.requires_external_math_authority()
        {
            return Err(
                HadwigerAspectAuthorityError::MathematicalAuthorityNotAdmitted { aspect_kind },
            );
        }
        let aspect_scope = HadwigerAspectScope::artifact(artifact_reference.stable_token())?;
        Ok(Self {
            aspect_kind,
            aspect_posture,
            artifact_reference,
            authority_owner,
            aspect_scope,
            source_metadata: require_non_empty(source_metadata, "source_metadata")?,
        })
    }

    pub fn aspect_kind(&self) -> HadwigerAspectKind {
        self.aspect_kind
    }

    pub fn aspect_posture(&self) -> HadwigerAspectPosture {
        self.aspect_posture
    }

    pub fn artifact_reference(&self) -> &HadwigerArtifactReference {
        &self.artifact_reference
    }

    pub fn authority_owner(&self) -> HadwigerArtifactAuthorityOwner {
        self.authority_owner
    }

    pub fn aspect_scope(&self) -> &HadwigerAspectScope {
        &self.aspect_scope
    }

    pub fn source_metadata(&self) -> &str {
        &self.source_metadata
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.aspect_kind.as_str(),
            self.aspect_posture.as_str(),
            authority_token(self.authority_owner),
            self.aspect_scope.stable_token(),
            self.source_metadata
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphShapeAspectRecord(HadwigerAspectRecord);

impl GraphShapeAspectRecord {
    pub fn admitted_shape(
        graph_version_reference: HadwigerArtifactReference,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        HadwigerAspectRecord::new(
            HadwigerAspectKind::GraphVersionShape,
            HadwigerAspectPosture::Admitted,
            graph_version_reference,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            "phase-3 graph version shape admission",
        )
        .map(Self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitDistanceAspectRecord(HadwigerAspectRecord);

impl UnitDistanceAspectRecord {
    pub(crate) fn admitted_checked(
        embedding_reference: HadwigerArtifactReference,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        checker_math_record(
            HadwigerAspectKind::UnitDistanceEmbedding,
            HadwigerAspectPosture::Admitted,
            embedding_reference,
            source_metadata,
        )
        .map(Self)
    }

    pub fn deferred(
        embedding_reference: HadwigerArtifactReference,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        math_record(
            HadwigerAspectKind::UnitDistanceEmbedding,
            HadwigerAspectPosture::Deferred,
            embedding_reference,
            source_metadata,
        )
        .map(Self)
    }

    pub fn rejected(
        embedding_reference: HadwigerArtifactReference,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        math_record(
            HadwigerAspectKind::UnitDistanceEmbedding,
            HadwigerAspectPosture::Rejected,
            embedding_reference,
            source_metadata,
        )
        .map(Self)
    }

    pub fn stale(
        embedding_reference: HadwigerArtifactReference,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        math_record(
            HadwigerAspectKind::UnitDistanceEmbedding,
            HadwigerAspectPosture::Stale,
            embedding_reference,
            source_metadata,
        )
        .map(Self)
    }

    pub fn conflict(
        embedding_reference: HadwigerArtifactReference,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        math_record(
            HadwigerAspectKind::UnitDistanceEmbedding,
            HadwigerAspectPosture::Conflict,
            embedding_reference,
            source_metadata,
        )
        .map(Self)
    }

    pub fn satisfies_mathematical_dependency(&self) -> bool {
        self.0.aspect_posture().satisfies_mathematical_dependency()
    }

    pub fn aspect_posture(&self) -> HadwigerAspectPosture {
        self.0.aspect_posture()
    }

    pub fn artifact_reference(&self) -> &HadwigerArtifactReference {
        self.0.artifact_reference()
    }

    pub fn stable_token(&self) -> String {
        self.0.stable_token()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorabilityAspectRecord {
    record: HadwigerAspectRecord,
    color_count: u32,
}

impl ColorabilityAspectRecord {
    pub(crate) fn admitted_checked(
        graph_version_reference: HadwigerArtifactReference,
        color_count: u32,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        colorability_checker_record(
            HadwigerAspectPosture::Admitted,
            graph_version_reference,
            color_count,
            source_metadata,
        )
    }

    pub fn missing(
        graph_version_reference: HadwigerArtifactReference,
        color_count: u32,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        colorability_record(
            HadwigerAspectPosture::Missing,
            graph_version_reference,
            color_count,
            source_metadata,
        )
    }

    pub fn deferred(
        graph_version_reference: HadwigerArtifactReference,
        color_count: u32,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        colorability_record(
            HadwigerAspectPosture::Deferred,
            graph_version_reference,
            color_count,
            source_metadata,
        )
    }

    pub fn unsupported(
        graph_version_reference: HadwigerArtifactReference,
        color_count: u32,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        colorability_record(
            HadwigerAspectPosture::Unsupported,
            graph_version_reference,
            color_count,
            source_metadata,
        )
    }

    pub fn satisfies_mathematical_dependency(&self) -> bool {
        self.record
            .aspect_posture()
            .satisfies_mathematical_dependency()
    }

    pub fn aspect_posture(&self) -> HadwigerAspectPosture {
        self.record.aspect_posture()
    }

    pub fn artifact_reference(&self) -> &HadwigerArtifactReference {
        self.record.artifact_reference()
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }

    pub fn stable_token(&self) -> String {
        self.record.stable_token()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryAspectRecord(HadwigerAspectRecord);

impl AdvisoryAspectRecord {
    pub fn advisory(
        advisory_reference: HadwigerArtifactReference,
        source_metadata: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        HadwigerAspectRecord::new(
            HadwigerAspectKind::AIAdvisory,
            HadwigerAspectPosture::Advisory,
            advisory_reference,
            HadwigerArtifactAuthorityOwner::AIAdvisory,
            source_metadata,
        )
        .map(Self)
    }
}

impl From<GraphShapeAspectRecord> for HadwigerAspectRecord {
    fn from(value: GraphShapeAspectRecord) -> Self {
        value.0
    }
}

impl From<UnitDistanceAspectRecord> for HadwigerAspectRecord {
    fn from(value: UnitDistanceAspectRecord) -> Self {
        value.0
    }
}

impl From<ColorabilityAspectRecord> for HadwigerAspectRecord {
    fn from(value: ColorabilityAspectRecord) -> Self {
        value.record
    }
}

impl From<AdvisoryAspectRecord> for HadwigerAspectRecord {
    fn from(value: AdvisoryAspectRecord) -> Self {
        value.0
    }
}

fn math_record(
    aspect_kind: HadwigerAspectKind,
    posture: HadwigerAspectPosture,
    artifact_reference: HadwigerArtifactReference,
    source_metadata: impl Into<String>,
) -> Result<HadwigerAspectRecord, HadwigerAspectAuthorityError> {
    HadwigerAspectRecord::new(
        aspect_kind,
        posture,
        artifact_reference,
        HadwigerArtifactAuthorityOwner::Checker,
        source_metadata,
    )
}

fn colorability_record(
    posture: HadwigerAspectPosture,
    graph_version_reference: HadwigerArtifactReference,
    color_count: u32,
    source_metadata: impl Into<String>,
) -> Result<ColorabilityAspectRecord, HadwigerAspectAuthorityError> {
    if color_count == 0 {
        return Err(HadwigerAspectAuthorityError::EmptyField {
            field: "color_count",
        });
    }
    let record = math_record(
        HadwigerAspectKind::NotKColorable,
        posture,
        graph_version_reference,
        format!("k={color_count}:{}", source_metadata.into()),
    )?;
    Ok(ColorabilityAspectRecord {
        record,
        color_count,
    })
}

fn checker_math_record(
    aspect_kind: HadwigerAspectKind,
    posture: HadwigerAspectPosture,
    artifact_reference: HadwigerArtifactReference,
    source_metadata: impl Into<String>,
) -> Result<HadwigerAspectRecord, HadwigerAspectAuthorityError> {
    let aspect_scope = HadwigerAspectScope::artifact(artifact_reference.stable_token())?;
    Ok(HadwigerAspectRecord {
        aspect_kind,
        aspect_posture: posture,
        artifact_reference,
        authority_owner: HadwigerArtifactAuthorityOwner::Checker,
        aspect_scope,
        source_metadata: require_non_empty(source_metadata, "source_metadata")?,
    })
}

fn colorability_checker_record(
    posture: HadwigerAspectPosture,
    graph_version_reference: HadwigerArtifactReference,
    color_count: u32,
    source_metadata: impl Into<String>,
) -> Result<ColorabilityAspectRecord, HadwigerAspectAuthorityError> {
    if color_count == 0 {
        return Err(HadwigerAspectAuthorityError::EmptyField {
            field: "color_count",
        });
    }
    let record = checker_math_record(
        HadwigerAspectKind::NotKColorable,
        posture,
        graph_version_reference,
        format!("k={color_count}:{}", source_metadata.into()),
    )?;
    Ok(ColorabilityAspectRecord {
        record,
        color_count,
    })
}
