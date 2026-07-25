use std::sync::Arc;

use crate::WorthServerOperationFamily;

use super::{
    WorthServerProductAdapterCertificationCode, WorthServerProductAdapterCertificationError,
    WorthServerProductOperationErrorMap, WorthServerProductOperationSupportSnapshot,
    WorthServerProductPayloadSchemaValidator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductReadTransport {
    FlatQuery,
    StructuredQuery,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationBasisKind {
    QueryDerived,
    ProductSessionDerived,
    DurableProductDerived,
    FixtureOnly,
}

impl WorthServerProductOperationBasisKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryDerived => "query-derived",
            Self::ProductSessionDerived => "product-session-derived",
            Self::DurableProductDerived => "durable-product-derived",
            Self::FixtureOnly => "fixture-only",
        }
    }
}

#[derive(Clone)]
pub struct WorthServerProductOperationDeclaration {
    operation_name: String,
    operation_family: WorthServerOperationFamily,
    payload_schema_identity: String,
    result_contract: crate::WorthServerProductResultContract,
    basis_kind: WorthServerProductOperationBasisKind,
    support_snapshot: WorthServerProductOperationSupportSnapshot,
    authority_requirement: WorthServerProductOperationAuthorityRequirement,
    read_transport: Option<WorthServerProductReadTransport>,
    payload_validator: Option<Arc<dyn WorthServerProductPayloadSchemaValidator>>,
    error_map: Option<Arc<dyn WorthServerProductOperationErrorMap>>,
}

impl std::fmt::Debug for WorthServerProductOperationDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorthServerProductOperationDeclaration")
            .field("operation_name", &self.operation_name)
            .field("operation_family", &self.operation_family)
            .field("payload_schema_identity", &self.payload_schema_identity)
            .field("result_contract", &self.result_contract)
            .field("basis_kind", &self.basis_kind)
            .field("support_snapshot", &self.support_snapshot)
            .field("authority_requirement", &self.authority_requirement)
            .field("read_transport", &self.read_transport)
            .finish()
    }
}

impl WorthServerProductOperationDeclaration {
    pub fn product_read(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        result_contract: crate::WorthServerProductResultContract,
        basis_kind: WorthServerProductOperationBasisKind,
        support_snapshot: WorthServerProductOperationSupportSnapshot,
    ) -> Self {
        Self::product_read_with_transport(
            operation_name,
            payload_schema_identity,
            result_contract,
            basis_kind,
            support_snapshot,
            WorthServerProductReadTransport::FlatQuery,
        )
    }

    pub fn product_structured_read(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        result_contract: crate::WorthServerProductResultContract,
        basis_kind: WorthServerProductOperationBasisKind,
        support_snapshot: WorthServerProductOperationSupportSnapshot,
    ) -> Self {
        Self::product_read_with_transport(
            operation_name,
            payload_schema_identity,
            result_contract,
            basis_kind,
            support_snapshot,
            WorthServerProductReadTransport::StructuredQuery,
        )
    }

    fn product_read_with_transport(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        result_contract: crate::WorthServerProductResultContract,
        basis_kind: WorthServerProductOperationBasisKind,
        support_snapshot: WorthServerProductOperationSupportSnapshot,
        read_transport: WorthServerProductReadTransport,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_family: WorthServerOperationFamily::ProductApplicationRead,
            payload_schema_identity: payload_schema_identity.into(),
            result_contract,
            basis_kind,
            support_snapshot,
            authority_requirement: WorthServerProductOperationAuthorityRequirement::SharedRead,
            read_transport: Some(read_transport),
            payload_validator: None,
            error_map: None,
        }
    }

    pub fn product_mutation(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        result_contract: crate::WorthServerProductResultContract,
        basis_kind: WorthServerProductOperationBasisKind,
        support_snapshot: WorthServerProductOperationSupportSnapshot,
        draft_scope: impl Into<String>,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_family: WorthServerOperationFamily::ProductApplicationMutation,
            payload_schema_identity: payload_schema_identity.into(),
            result_contract,
            basis_kind,
            support_snapshot,
            authority_requirement: WorthServerProductOperationAuthorityRequirement::DraftMutation {
                draft_scope: draft_scope.into(),
            },
            read_transport: None,
            payload_validator: None,
            error_map: None,
        }
    }

    pub fn product_session_coordination(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        result_contract: crate::WorthServerProductResultContract,
        basis_kind: WorthServerProductOperationBasisKind,
        support_snapshot: WorthServerProductOperationSupportSnapshot,
        coordination_lane: impl Into<String>,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_family: WorthServerOperationFamily::ProductSessionCoordination,
            payload_schema_identity: payload_schema_identity.into(),
            result_contract,
            basis_kind,
            support_snapshot,
            authority_requirement:
                WorthServerProductOperationAuthorityRequirement::SessionCoordination {
                    coordination_lane: coordination_lane.into(),
                },
            read_transport: None,
            payload_validator: None,
            error_map: None,
        }
    }

    pub fn with_payload_validator(
        mut self,
        payload_validator: Arc<dyn WorthServerProductPayloadSchemaValidator>,
    ) -> Self {
        self.payload_validator = Some(payload_validator);
        self
    }

    pub fn with_error_map(
        mut self,
        error_map: Arc<dyn WorthServerProductOperationErrorMap>,
    ) -> Self {
        self.error_map = Some(error_map);
        self
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn operation_family(&self) -> WorthServerOperationFamily {
        self.operation_family
    }

    pub fn payload_schema_identity(&self) -> &str {
        &self.payload_schema_identity
    }

    pub fn durable_product_mutation(
        operation_name: impl Into<String>,
        payload_schema_identity: impl Into<String>,
        result_contract: crate::WorthServerProductResultContract,
        support_snapshot: WorthServerProductOperationSupportSnapshot,
        durable_contract: crate::WorthServerDurableProductMutationContract,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            operation_family: WorthServerOperationFamily::ProductApplicationMutation,
            payload_schema_identity: payload_schema_identity.into(),
            result_contract,
            basis_kind: WorthServerProductOperationBasisKind::DurableProductDerived,
            support_snapshot,
            authority_requirement:
                WorthServerProductOperationAuthorityRequirement::DurableMutation {
                    contract: durable_contract,
                },
            read_transport: None,
            payload_validator: None,
            error_map: None,
        }
    }

    pub fn result_contract(&self) -> &crate::WorthServerProductResultContract {
        &self.result_contract
    }

    pub fn basis_kind(&self) -> WorthServerProductOperationBasisKind {
        self.basis_kind
    }

    pub fn support_snapshot(&self) -> &WorthServerProductOperationSupportSnapshot {
        &self.support_snapshot
    }

    pub fn authority_requirement(&self) -> &WorthServerProductOperationAuthorityRequirement {
        &self.authority_requirement
    }

    pub fn read_transport(&self) -> Option<WorthServerProductReadTransport> {
        self.read_transport
    }

    pub fn durable_mutation_contract(
        &self,
    ) -> Option<&crate::WorthServerDurableProductMutationContract> {
        match &self.authority_requirement {
            WorthServerProductOperationAuthorityRequirement::DurableMutation { contract } => {
                Some(contract)
            }
            _ => None,
        }
    }

    pub(crate) fn canonical_digest(&self) -> String {
        let authority = match &self.authority_requirement {
            WorthServerProductOperationAuthorityRequirement::SharedRead => {
                "shared-read".to_string()
            }
            WorthServerProductOperationAuthorityRequirement::DraftMutation { draft_scope } => {
                format!("draft:{draft_scope}")
            }
            WorthServerProductOperationAuthorityRequirement::DurableMutation { contract } => {
                format!("durable:{}", contract.canonical_digest())
            }
            WorthServerProductOperationAuthorityRequirement::SessionCoordination {
                coordination_lane,
            } => format!("session-coordination:{coordination_lane}"),
        };
        crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-product-operation-declaration-v3",
        )
        .field("operation", &self.operation_name)
        .field("family", self.operation_family.as_str())
        .field("payload", &self.payload_schema_identity)
        .field("result", self.result_contract.canonical_digest())
        .field("basis", self.basis_kind.as_str())
        .field("support_row", self.support_snapshot.support_row())
        .field("support_posture", self.support_snapshot.canonical_label())
        .field("authority", &authority)
        .field(
            "read_transport",
            match self.read_transport {
                Some(WorthServerProductReadTransport::FlatQuery) => "flat-query",
                Some(WorthServerProductReadTransport::StructuredQuery) => "structured-query",
                None => "not-applicable",
            },
        )
        .finish()
    }

    pub(crate) fn payload_validator(
        &self,
    ) -> Option<&Arc<dyn WorthServerProductPayloadSchemaValidator>> {
        self.payload_validator.as_ref()
    }

    pub(crate) fn error_map(&self) -> &Arc<dyn WorthServerProductOperationErrorMap> {
        self.error_map
            .as_ref()
            .expect("validated product declarations must retain an explicit error map")
    }

    pub(crate) fn validate(&self) -> Result<(), WorthServerProductAdapterCertificationError> {
        if self.operation_name.trim().is_empty() {
            return Err(WorthServerProductAdapterCertificationError::new(
                WorthServerProductAdapterCertificationCode::BlankOperationName,
                "product operation declarations require a non-blank operation name",
            ));
        }
        if self.payload_schema_identity.trim().is_empty() {
            return Err(WorthServerProductAdapterCertificationError::new(
                WorthServerProductAdapterCertificationCode::BlankPayloadSchemaIdentity,
                "product operation declarations require a non-blank payload schema identity",
            ));
        }
        if self.support_snapshot.support_row().trim().is_empty() {
            return Err(WorthServerProductAdapterCertificationError::new(
                WorthServerProductAdapterCertificationCode::BlankSupportSnapshotRow,
                "product operation declarations require a non-blank support snapshot row",
            ));
        }
        if self.error_map.is_none() {
            return Err(WorthServerProductAdapterCertificationError::new(
                WorthServerProductAdapterCertificationCode::MissingErrorMap,
                "product operation declarations require an explicit denial or failure error map",
            ));
        }
        match &self.authority_requirement {
            WorthServerProductOperationAuthorityRequirement::SharedRead => {}
            WorthServerProductOperationAuthorityRequirement::DraftMutation { draft_scope } => {
                if draft_scope.trim().is_empty() {
                    return Err(WorthServerProductAdapterCertificationError::new(
                        WorthServerProductAdapterCertificationCode::BlankDraftScope,
                        "product mutation declarations require a non-blank draft scope",
                    ));
                }
            }
            WorthServerProductOperationAuthorityRequirement::DurableMutation { .. } => {}
            WorthServerProductOperationAuthorityRequirement::SessionCoordination {
                coordination_lane,
            } => {
                if coordination_lane.trim().is_empty() {
                    return Err(WorthServerProductAdapterCertificationError::new(
                        WorthServerProductAdapterCertificationCode::BlankCoordinationLane,
                        "product session declarations require a non-blank coordination lane",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationAuthorityRequirement {
    SharedRead,
    DraftMutation {
        draft_scope: String,
    },
    DurableMutation {
        contract: crate::WorthServerDurableProductMutationContract,
    },
    SessionCoordination {
        coordination_lane: String,
    },
}
