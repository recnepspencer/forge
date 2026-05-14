use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::identity::hash_parts;
use crate::query_context::QueryContextExecutionArtifact;
use crate::runtime::{ForgeQueryReadReceipt, ForgeQueryWriteReceipt};

use super::facts::ProjectMaterializedFacts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionSourceFamily {
    QueryReadReceipt,
    QueryWriteReceipt,
    QueryContextExecution,
}

impl ProjectionSourceFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryReadReceipt => "query_read_receipt",
            Self::QueryWriteReceipt => "query_write_receipt",
            Self::QueryContextExecution => "query_context_execution",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionBindingContext {
    result_shape_digest: String,
    authorized_projection_identity: String,
    authorized_visible_fields: Vec<String>,
}

impl ProjectionConsumptionBindingContext {
    pub fn from_authorized_projection(
        result_shape: &CanonicalResultShapeArtifact,
        authorized_projection: &AuthorizedProjectionArtifact,
    ) -> Self {
        Self {
            result_shape_digest: result_shape.digest().as_str().to_string(),
            authorized_projection_identity: authorized_projection.identity().as_str().to_string(),
            authorized_visible_fields: authorized_projection.visible_fields().to_vec(),
        }
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }

    pub fn authorized_projection_identity(&self) -> &str {
        &self.authorized_projection_identity
    }

    pub fn authorized_visible_fields(&self) -> &[String] {
        &self.authorized_visible_fields
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        result_shape_digest: impl Into<String>,
        authorized_projection_identity: impl Into<String>,
        authorized_visible_fields: Vec<String>,
    ) -> Self {
        Self {
            result_shape_digest: result_shape_digest.into(),
            authorized_projection_identity: authorized_projection_identity.into(),
            authorized_visible_fields,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionSource {
    family: ProjectionSourceFamily,
    query_digest: Option<String>,
    basis_digest: Option<String>,
    result_digest: Option<String>,
    result_shape_digest: Option<String>,
    source_identity: String,
}

impl ProjectionConsumptionSource {
    pub fn from_read_receipt(
        receipt: &ForgeQueryReadReceipt,
        result_shape: &CanonicalResultShapeArtifact,
    ) -> Self {
        Self {
            family: ProjectionSourceFamily::QueryReadReceipt,
            query_digest: Some(receipt.query_digest().to_string()),
            basis_digest: Some(receipt.basis_digest().to_string()),
            result_digest: Some(receipt.result_digest().to_string()),
            result_shape_digest: Some(result_shape.digest().as_str().to_string()),
            source_identity: receipt.read_graph_digest().to_string(),
        }
    }

    pub fn from_write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        Self {
            family: ProjectionSourceFamily::QueryWriteReceipt,
            query_digest: None,
            basis_digest: Some(receipt.snapshot_token().to_string()),
            result_digest: None,
            result_shape_digest: None,
            source_identity: receipt.commit_identity().to_string(),
        }
    }

    pub fn from_query_context_execution(execution: &QueryContextExecutionArtifact) -> Self {
        Self {
            family: ProjectionSourceFamily::QueryContextExecution,
            query_digest: Some(execution.query_digest().to_string()),
            basis_digest: Some(execution.basis_digest().to_string()),
            result_digest: Some(execution.result_digest().to_string()),
            result_shape_digest: Some(execution.result_shape_digest().to_string()),
            source_identity: execution
                .materialization_path_identity()
                .unwrap_or_else(|| execution.family().as_str())
                .to_string(),
        }
    }

    pub fn family(&self) -> ProjectionSourceFamily {
        self.family
    }

    pub fn query_digest(&self) -> Option<&str> {
        self.query_digest.as_deref()
    }

    pub fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub fn result_digest(&self) -> Option<&str> {
        self.result_digest.as_deref()
    }

    pub fn result_shape_digest(&self) -> Option<&str> {
        self.result_shape_digest.as_deref()
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        family: ProjectionSourceFamily,
        query_digest: Option<&str>,
        basis_digest: Option<&str>,
        result_digest: Option<&str>,
        result_shape_digest: Option<&str>,
        source_identity: &str,
    ) -> Self {
        Self {
            family,
            query_digest: query_digest.map(str::to_string),
            basis_digest: basis_digest.map(str::to_string),
            result_digest: result_digest.map(str::to_string),
            result_shape_digest: result_shape_digest.map(str::to_string),
            source_identity: source_identity.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionDeclaration {
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested: ProjectMaterializedFacts,
    declaration_digest: String,
}

impl ProjectionConsumptionDeclaration {
    pub fn source(&self) -> &ProjectionConsumptionSource {
        &self.source
    }

    pub fn binding(&self) -> &ProjectionConsumptionBindingContext {
        &self.binding
    }

    pub fn requested(&self) -> &ProjectMaterializedFacts {
        &self.requested
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionDeclarationError {
    NoRequestedFacts,
    SourceBindingResultShapeMismatch {
        source_result_shape_digest: String,
        binding_result_shape_digest: String,
    },
}

pub fn declare_projection_consumption(
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested: ProjectMaterializedFacts,
) -> Result<ProjectionConsumptionDeclaration, ProjectionConsumptionDeclarationError> {
    if requested.requested_count() == 0 {
        return Err(ProjectionConsumptionDeclarationError::NoRequestedFacts);
    }
    if let Some(source_result_shape_digest) = source.result_shape_digest() {
        if source_result_shape_digest != binding.result_shape_digest() {
            return Err(
                ProjectionConsumptionDeclarationError::SourceBindingResultShapeMismatch {
                    source_result_shape_digest: source_result_shape_digest.to_string(),
                    binding_result_shape_digest: binding.result_shape_digest().to_string(),
                },
            );
        }
    }
    let mut parts = vec![
        format!("source_family:{}", source.family().as_str()),
        format!("source_identity:{}", source.source_identity()),
        format!("result_shape:{}", binding.result_shape_digest()),
        format!(
            "authorized_projection:{}",
            binding.authorized_projection_identity()
        ),
    ];
    if let Some(query_digest) = source.query_digest() {
        parts.push(format!("query:{query_digest}"));
    }
    if let Some(basis_digest) = source.basis_digest() {
        parts.push(format!("basis:{basis_digest}"));
    }
    if let Some(result_digest) = source.result_digest() {
        parts.push(format!("result:{result_digest}"));
    }
    for request in requested.requested() {
        match request.field_key() {
            Some(field) => parts.push(format!("fact:{}:{field}", request.kind().as_str())),
            None => parts.push(format!("fact:{}", request.kind().as_str())),
        }
    }
    Ok(ProjectionConsumptionDeclaration {
        source,
        binding,
        requested,
        declaration_digest: hash_parts(&parts),
    })
}
