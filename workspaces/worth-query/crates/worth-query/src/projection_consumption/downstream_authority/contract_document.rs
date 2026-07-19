use serde::{Deserialize, Serialize};
use worth_foundational::facade::{CanonicalFieldPath, FieldKey};

use super::{ProjectionAuthorityContract, ProjectionAuthorityRequirement};
use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionFactFieldPath, ProjectionFactRequest,
};

mod terminal_json_codec;

const SCHEMA: &str = "worth-query.projection-authority-contract.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalProjectionAuthorityContractDocument(String);

impl ExternalProjectionAuthorityContractDocument {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionAuthorityContractDocument(String);

impl ProjectionAuthorityContractDocument {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_external(&self) -> ExternalProjectionAuthorityContractDocument {
        ExternalProjectionAuthorityContractDocument(self.0.clone())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionAuthorityContractDocumentErrorKind {
    InvalidJson,
    SchemaMismatch,
    UnknownRequirement,
    UnknownFact,
    InvalidFieldPath,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionAuthorityContractDocumentError {
    kind: ProjectionAuthorityContractDocumentErrorKind,
    detail: String,
}

impl ProjectionAuthorityContractDocumentError {
    pub fn kind(&self) -> ProjectionAuthorityContractDocumentErrorKind {
        self.kind
    }
}

impl std::fmt::Display for ProjectionAuthorityContractDocumentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", error_name(self.kind), self.detail)
    }
}

impl std::error::Error for ProjectionAuthorityContractDocumentError {}

impl ProjectionAuthorityContract {
    pub fn to_terminal_json_document(
        &self,
    ) -> Result<ProjectionAuthorityContractDocument, ProjectionAuthorityContractDocumentError> {
        let document = ContractDocument {
            schema: SCHEMA.to_string(),
            requirements: self
                .requirements()
                .map(|requirement| requirement.as_str().to_string())
                .collect(),
            facts: self.requested_facts().map(FactDocument::from).collect(),
        };
        terminal_json_codec::encode(&document).map(ProjectionAuthorityContractDocument)
    }
}

pub fn load_projection_authority_contract_document(
    document: &ExternalProjectionAuthorityContractDocument,
) -> Result<ProjectionAuthorityContract, ProjectionAuthorityContractDocumentError> {
    let document = terminal_json_codec::decode(document.as_str())?;
    if document.schema != SCHEMA {
        return Err(document_error(
            ProjectionAuthorityContractDocumentErrorKind::SchemaMismatch,
            document.schema,
        ));
    }
    let requirements = document
        .requirements
        .into_iter()
        .map(parse_requirement)
        .collect::<Result<Vec<_>, _>>()?;
    let mut facts = ProjectMaterializedFacts::declare();
    for fact in document.facts {
        facts = fact.apply(facts)?;
    }
    Ok(ProjectionAuthorityContract::certification(
        facts,
        requirements,
    ))
}

#[derive(Debug, Deserialize, Serialize)]
struct ContractDocument {
    schema: String,
    requirements: Vec<String>,
    facts: Vec<FactDocument>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FactDocument {
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<Vec<String>>,
}

impl From<&ProjectionFactRequest> for FactDocument {
    fn from(request: &ProjectionFactRequest) -> Self {
        Self {
            kind: fact_name(request).to_string(),
            path: request.field_path().map(|path| {
                path.canonical_field_path()
                    .fields()
                    .iter()
                    .map(|field| field.as_str().to_string())
                    .collect()
            }),
        }
    }
}

impl FactDocument {
    fn apply(
        self,
        facts: ProjectMaterializedFacts,
    ) -> Result<ProjectMaterializedFacts, ProjectionAuthorityContractDocumentError> {
        Ok(match self.kind.as_str() {
            "entity_identity" => facts.entity_identities(),
            "view_local_identity" => facts.view_local_identities(),
            "target_identity" => facts.target_identity(),
            "source_reference" => facts.source_references(),
            "effect_continuity" => facts.effect_continuity_facts(),
            "membership" => facts.memberships(),
            "relation_endpoint" => facts.relation_endpoints(),
            "display_field" => facts.display_field_path(parse_path(self.path)?),
            "derived_field" => facts.derived_field_path(parse_path(self.path)?),
            unknown => {
                return Err(document_error(
                    ProjectionAuthorityContractDocumentErrorKind::UnknownFact,
                    unknown,
                ))
            }
        })
    }
}

fn parse_requirement(
    value: String,
) -> Result<ProjectionAuthorityRequirement, ProjectionAuthorityContractDocumentError> {
    match value.as_str() {
        "settled_consumption" => Ok(ProjectionAuthorityRequirement::SettledConsumption),
        "source_authority" => Ok(ProjectionAuthorityRequirement::SourceAuthority),
        "basis_generation" => Ok(ProjectionAuthorityRequirement::BasisGeneration),
        "target_identity" => Ok(ProjectionAuthorityRequirement::TargetIdentity),
        unknown => Err(document_error(
            ProjectionAuthorityContractDocumentErrorKind::UnknownRequirement,
            unknown,
        )),
    }
}

fn parse_path(
    path: Option<Vec<String>>,
) -> Result<ProjectionFactFieldPath, ProjectionAuthorityContractDocumentError> {
    let fields = path
        .filter(|path| !path.is_empty())
        .ok_or_else(|| {
            document_error(
                ProjectionAuthorityContractDocumentErrorKind::InvalidFieldPath,
                "field fact requires a non-empty path",
            )
        })?
        .into_iter()
        .map(|field| {
            FieldKey::new(&field).ok_or_else(|| {
                document_error(
                    ProjectionAuthorityContractDocumentErrorKind::InvalidFieldPath,
                    field,
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let path = CanonicalFieldPath::new(fields).ok_or_else(|| {
        document_error(
            ProjectionAuthorityContractDocumentErrorKind::InvalidFieldPath,
            "canonical field path rejected",
        )
    })?;
    Ok(ProjectionFactFieldPath::from_canonical_field_path(path))
}

fn fact_name(request: &ProjectionFactRequest) -> &'static str {
    match request {
        ProjectionFactRequest::EntityIdentity => "entity_identity",
        ProjectionFactRequest::ViewLocalIdentity => "view_local_identity",
        ProjectionFactRequest::TargetIdentity => "target_identity",
        ProjectionFactRequest::SourceReference => "source_reference",
        ProjectionFactRequest::EffectContinuity => "effect_continuity",
        ProjectionFactRequest::Membership => "membership",
        ProjectionFactRequest::RelationEndpoint => "relation_endpoint",
        ProjectionFactRequest::DisplayField(_) => "display_field",
        ProjectionFactRequest::DerivedField(_) => "derived_field",
    }
}

fn document_error(
    kind: ProjectionAuthorityContractDocumentErrorKind,
    detail: impl Into<String>,
) -> ProjectionAuthorityContractDocumentError {
    ProjectionAuthorityContractDocumentError {
        kind,
        detail: detail.into(),
    }
}

fn error_name(kind: ProjectionAuthorityContractDocumentErrorKind) -> &'static str {
    match kind {
        ProjectionAuthorityContractDocumentErrorKind::InvalidJson => "invalid JSON",
        ProjectionAuthorityContractDocumentErrorKind::SchemaMismatch => "schema mismatch",
        ProjectionAuthorityContractDocumentErrorKind::UnknownRequirement => "unknown requirement",
        ProjectionAuthorityContractDocumentErrorKind::UnknownFact => "unknown fact",
        ProjectionAuthorityContractDocumentErrorKind::InvalidFieldPath => "invalid field path",
    }
}
