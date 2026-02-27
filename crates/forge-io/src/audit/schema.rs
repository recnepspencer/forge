//! Versioned JSON schema for audit artifacts and bundle manifests.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Current schema version for serialized audit records.
pub const AUDIT_SCHEMA_VERSION: u32 = 1;

/// Current schema version for audit bundle manifests.
pub const AUDIT_BUNDLE_MANIFEST_VERSION: u32 = 1;

/// Identity scope label for serialized audit fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditIdentityScope {
    Snapshot,
    Persistent,
    Hash,
}

/// Declarative label for an audit field name and its semantic scope.
///
/// Used by tests and schema builders to enforce naming conventions
/// (e.g. `*_snapshot`, `*_persistent`, `*_hash`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditFieldLabel {
    pub field_name: String,
    pub scope: AuditIdentityScope,
}

impl AuditFieldLabel {
    pub fn new(field_name: impl Into<String>, scope: AuditIdentityScope) -> Self {
        Self {
            field_name: field_name.into(),
            scope,
        }
    }

    pub fn validate(&self) -> Result<(), AuditConventionError> {
        let name = self.field_name.as_str();
        match self.scope {
            AuditIdentityScope::Snapshot => {
                if name.ends_with("_snapshot") {
                    Ok(())
                } else {
                    Err(AuditConventionError::FieldNameScopeMismatch {
                        field_name: self.field_name.clone(),
                        expected_scope: self.scope,
                    })
                }
            }
            AuditIdentityScope::Persistent => {
                if name.ends_with("_persistent") {
                    Ok(())
                } else {
                    Err(AuditConventionError::FieldNameScopeMismatch {
                        field_name: self.field_name.clone(),
                        expected_scope: self.scope,
                    })
                }
            }
            AuditIdentityScope::Hash => {
                if name.ends_with("_hash") {
                    Ok(())
                } else {
                    Err(AuditConventionError::FieldNameScopeMismatch {
                        field_name: self.field_name.clone(),
                        expected_scope: self.scope,
                    })
                }
            }
        }
    }
}

/// Convention validation failure for audit schema descriptors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditConventionError {
    InvalidSchemaVersion {
        found: u32,
    },
    InvalidOperationVersion {
        found: u32,
    },
    InvalidOperationType {
        found: String,
    },
    FieldNameScopeMismatch {
        field_name: String,
        expected_scope: AuditIdentityScope,
    },
}

impl fmt::Display for AuditConventionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuditConventionError::InvalidSchemaVersion { found } => {
                write!(f, "invalid schema_version {} (must be > 0)", found)
            }
            AuditConventionError::InvalidOperationVersion { found } => {
                write!(f, "invalid operation_version {} (must be > 0)", found)
            }
            AuditConventionError::InvalidOperationType { found } => {
                write!(
                    f,
                    "invalid operation_type '{}' (use snake_case [a-z0-9_]+)",
                    found
                )
            }
            AuditConventionError::FieldNameScopeMismatch {
                field_name,
                expected_scope,
            } => {
                write!(
                    f,
                    "field '{}' does not satisfy {:?} naming convention",
                    field_name, expected_scope
                )
            }
        }
    }
}

/// Versioned envelope for a serializable operation audit payload.
///
/// `record` is intentionally generic so each operation can define a richer
/// payload while sharing the same storage/versioning contract.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionedAuditRecord<T> {
    /// Schema version for the envelope itself.
    pub schema_version: u32,
    /// Stable operation type identifier (e.g. `region_merge`).
    pub operation_type: String,
    /// Version of the operation-specific payload schema/contract.
    pub operation_version: u32,
    /// Operation-specific audit payload.
    pub record: T,
}

impl<T> VersionedAuditRecord<T> {
    /// Construct a versioned audit envelope with the current audit schema version.
    pub fn new(operation_type: impl Into<String>, operation_version: u32, record: T) -> Self {
        Self {
            schema_version: AUDIT_SCHEMA_VERSION,
            operation_type: operation_type.into(),
            operation_version,
            record,
        }
    }

    /// Validate cross-kernel audit schema conventions for the envelope fields.
    pub fn validate_conventions(&self) -> Result<(), AuditConventionError> {
        if self.schema_version == 0 {
            return Err(AuditConventionError::InvalidSchemaVersion {
                found: self.schema_version,
            });
        }
        if self.operation_version == 0 {
            return Err(AuditConventionError::InvalidOperationVersion {
                found: self.operation_version,
            });
        }
        let op = self.operation_type.as_str();
        let valid = !op.is_empty()
            && op
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !valid {
            return Err(AuditConventionError::InvalidOperationType {
                found: self.operation_type.clone(),
            });
        }
        Ok(())
    }
}

/// File names emitted inside a per-operation audit bundle directory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditBundleFiles {
    /// Versioned operation audit record.
    pub operation_json: String,
    /// Serialized decision trace, if emitted.
    pub trace_json: Option<String>,
}

impl Default for AuditBundleFiles {
    fn default() -> Self {
        Self {
            operation_json: "operation.json".to_string(),
            trace_json: Some("trace.json".to_string()),
        }
    }
}

/// Versioned manifest for a per-operation audit bundle.
///
/// The bundle layout is:
/// - `<root>/<operation_id>/manifest.json`
/// - `<root>/<operation_id>/operation.json`
/// - `<root>/<operation_id>/trace.json` (optional)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditBundleManifest {
    /// Schema version for the manifest.
    pub schema_version: u32,
    /// Operation instance id (directory name).
    pub operation_id: String,
    /// Stable operation type identifier (must match audit envelope).
    pub operation_type: String,
    /// Operation payload version (must match audit envelope).
    pub operation_version: u32,
    /// Bundle creation time in Unix epoch milliseconds.
    pub created_at_unix_millis: u128,
    /// Bundle file names.
    pub files: AuditBundleFiles,
}

impl AuditBundleManifest {
    /// Build a manifest from a versioned audit record and chosen file names.
    pub fn from_record<T>(
        operation_id: impl Into<String>,
        record: &VersionedAuditRecord<T>,
        files: AuditBundleFiles,
        created_at_unix_millis: u128,
    ) -> Self {
        Self {
            schema_version: AUDIT_BUNDLE_MANIFEST_VERSION,
            operation_id: operation_id.into(),
            operation_type: record.operation_type.clone(),
            operation_version: record.operation_version,
            created_at_unix_millis,
            files,
        }
    }
}
