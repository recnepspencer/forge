use std::sync::Arc;

use crate::{ForgeServerOperationFamily, ForgeServerProductSupportPosture};

use super::{
    ForgeServerProductAdapterCertificationCode, ForgeServerProductAdapterCertificationError,
    ForgeServerProductOperationErrorMap, ForgeServerProductPayloadSchemaValidator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerProductOperationBasisKind {
    QueryDerived,
    ProductSessionDerived,
    DurableProductDerived,
    FixtureOnly,
}

impl ForgeServerProductOperationBasisKind {
    pub(crate) fn as_shared_read_basis_kind(self) -> &'static str {
        match self {
            Self::QueryDerived => "query-derived",
            Self::ProductSessionDerived => "product-session-derived",
            Self::DurableProductDerived => "durable-product-derived",
            Self::FixtureOnly => "fixture-only",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationSupportSnapshot {
    support_row: String,
    posture: ForgeServerProductSupportPosture,
}

impl ForgeServerProductOperationSupportSnapshot {
    pub fn production_admitted(support_row: impl Into<String>) -> Self {
        Self {
            support_row: support_row.into(),
            posture: ForgeServerProductSupportPosture::ProductionAdmitted,
        }
    }

    pub fn unsupported(support_row: impl Into<String>) -> Self {
        Self {
            support_row: support_row.into(),
            posture: ForgeServerProductSupportPosture::Unsupported,
        }
    }

    pub fn unknown(support_row: impl Into<String>) -> Self {
        Self {
            support_row: support_row.into(),
            posture: ForgeServerProductSupportPosture::Unknown,
        }
    }

    pub fn incompatible_basis(support_row: impl Into<String>) -> Self {
        Self {
            support_row: support_row.into(),
            posture: ForgeServerProductSupportPosture::IncompatibleBasis,
        }
    }

    pub fn support_row(&self) -> &str {
        &self.support_row
    }

    pub(crate) fn posture(&self) -> ForgeServerProductSupportPosture {
        self.posture.clone()
    }
}

#[derive(Clone)]
pub struct ForgeServerProductOperationDeclaration {
    operation_name: String,
    operation_family: ForgeServerOperationFamily,
    payload_schema_identity: String,
    basis_kind: ForgeServerProductOperationBasisKind,
    support_snapshot: ForgeServerProductOperationSupportSnapshot,
    authority_requirement: ForgeServerProductOperationAuthorityRequirement,
    payload_validator: Option<Arc<dyn ForgeServerProductPayloadSchemaValidator>>,
    error_map: Option<Arc<dyn ForgeServerProductOperationErrorMap>>,
}

impl std::fmt::Debug for ForgeServerProductOperationDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForgeServerProductOperationDeclaration")
            .field("operation_name", &self.operation_name)
            .field("operation_family", &self.operation_family)
            .field("payload_schema_identity", &self.payload_schema_identity)
            .field("basis_kind", &self.basis_kind)
            .field("support_snapshot", &self.support_snapshot)
            .field("authority_requirement", &self.authority_requirement)
            .finish()
    }
}

impl ForgeServerProductOperationDeclaration {
    pub fn product_read(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        basis_kind: ForgeServerProductOperationBasisKind,
        support_snapshot: ForgeServerProductOperationSupportSnapshot,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_family: ForgeServerOperationFamily::ProductApplicationRead,
            payload_schema_identity: payload_schema_identity.into(),
            basis_kind,
            support_snapshot,
            authority_requirement: ForgeServerProductOperationAuthorityRequirement::SharedRead,
            payload_validator: None,
            error_map: None,
        }
    }

    pub fn product_mutation(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        basis_kind: ForgeServerProductOperationBasisKind,
        support_snapshot: ForgeServerProductOperationSupportSnapshot,
        draft_scope: impl Into<String>,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_family: ForgeServerOperationFamily::ProductApplicationMutation,
            payload_schema_identity: payload_schema_identity.into(),
            basis_kind,
            support_snapshot,
            authority_requirement: ForgeServerProductOperationAuthorityRequirement::DraftMutation {
                draft_scope: draft_scope.into(),
            },
            payload_validator: None,
            error_map: None,
        }
    }

    pub fn product_session_coordination(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        basis_kind: ForgeServerProductOperationBasisKind,
        support_snapshot: ForgeServerProductOperationSupportSnapshot,
        coordination_lane: impl Into<String>,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_family: ForgeServerOperationFamily::ProductSessionCoordination,
            payload_schema_identity: payload_schema_identity.into(),
            basis_kind,
            support_snapshot,
            authority_requirement:
                ForgeServerProductOperationAuthorityRequirement::SessionCoordination {
                    coordination_lane: coordination_lane.into(),
                },
            payload_validator: None,
            error_map: None,
        }
    }

    pub fn with_payload_validator(
        mut self,
        payload_validator: Arc<dyn ForgeServerProductPayloadSchemaValidator>,
    ) -> Self {
        self.payload_validator = Some(payload_validator);
        self
    }

    pub fn with_error_map(
        mut self,
        error_map: Arc<dyn ForgeServerProductOperationErrorMap>,
    ) -> Self {
        self.error_map = Some(error_map);
        self
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn operation_family(&self) -> ForgeServerOperationFamily {
        self.operation_family
    }

    pub fn payload_schema_identity(&self) -> &str {
        &self.payload_schema_identity
    }

    pub fn basis_kind(&self) -> ForgeServerProductOperationBasisKind {
        self.basis_kind
    }

    pub fn support_snapshot(&self) -> &ForgeServerProductOperationSupportSnapshot {
        &self.support_snapshot
    }

    pub fn authority_requirement(&self) -> &ForgeServerProductOperationAuthorityRequirement {
        &self.authority_requirement
    }

    pub(crate) fn payload_validator(
        &self,
    ) -> Option<&Arc<dyn ForgeServerProductPayloadSchemaValidator>> {
        self.payload_validator.as_ref()
    }

    pub(crate) fn error_map(&self) -> &Arc<dyn ForgeServerProductOperationErrorMap> {
        self.error_map
            .as_ref()
            .expect("validated product declarations must retain an explicit error map")
    }

    pub(crate) fn validate(&self) -> Result<(), ForgeServerProductAdapterCertificationError> {
        if self.operation_name.trim().is_empty() {
            return Err(ForgeServerProductAdapterCertificationError::new(
                ForgeServerProductAdapterCertificationCode::BlankOperationName,
                "product operation declarations require a non-blank operation name",
            ));
        }
        if self.payload_schema_identity.trim().is_empty() {
            return Err(ForgeServerProductAdapterCertificationError::new(
                ForgeServerProductAdapterCertificationCode::BlankPayloadSchemaIdentity,
                "product operation declarations require a non-blank payload schema identity",
            ));
        }
        if self.support_snapshot.support_row().trim().is_empty() {
            return Err(ForgeServerProductAdapterCertificationError::new(
                ForgeServerProductAdapterCertificationCode::BlankSupportSnapshotRow,
                "product operation declarations require a non-blank support snapshot row",
            ));
        }
        if self.error_map.is_none() {
            return Err(ForgeServerProductAdapterCertificationError::new(
                ForgeServerProductAdapterCertificationCode::MissingErrorMap,
                "product operation declarations require an explicit denial or failure error map",
            ));
        }
        match &self.authority_requirement {
            ForgeServerProductOperationAuthorityRequirement::SharedRead => {}
            ForgeServerProductOperationAuthorityRequirement::DraftMutation { draft_scope } => {
                if draft_scope.trim().is_empty() {
                    return Err(ForgeServerProductAdapterCertificationError::new(
                        ForgeServerProductAdapterCertificationCode::BlankDraftScope,
                        "product mutation declarations require a non-blank draft scope",
                    ));
                }
            }
            ForgeServerProductOperationAuthorityRequirement::SessionCoordination {
                coordination_lane,
            } => {
                if coordination_lane.trim().is_empty() {
                    return Err(ForgeServerProductAdapterCertificationError::new(
                        ForgeServerProductAdapterCertificationCode::BlankCoordinationLane,
                        "product session declarations require a non-blank coordination lane",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerProductOperationAuthorityRequirement {
    SharedRead,
    DraftMutation { draft_scope: String },
    SessionCoordination { coordination_lane: String },
}
