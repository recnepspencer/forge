use crate::{WorthServerOperationFamily, WorthServerOperationRequest};

use super::{WorthServerOperationAuthorityMetadata, WorthServerProductSessionCoordinationTarget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerSharedReadBasisKind {
    QuerySharedReadBasis,
    QueryDerived,
    ProductSessionDerived,
    DurableProductDerived,
    FixtureOnly,
}

impl WorthServerSharedReadBasisKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::QuerySharedReadBasis => "query-shared-read-basis",
            Self::QueryDerived => "query-derived",
            Self::ProductSessionDerived => "product-session-derived",
            Self::DurableProductDerived => "durable-product-derived",
            Self::FixtureOnly => "fixture-only",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductSupportPosture {
    ProductionAdmitted,
    Unsupported,
    Unknown,
    IncompatibleBasis,
}

impl WorthServerProductSupportPosture {
    fn as_str(&self) -> &'static str {
        match self {
            Self::ProductionAdmitted => "production-admitted",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
            Self::IncompatibleBasis => "incompatible-basis",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerOperationAuthorityDeclaration {
    SharedRead {
        basis_kind: WorthServerSharedReadBasisKind,
        product_support_posture: WorthServerProductSupportPosture,
    },
    DeterministicSubmission {
        submission_lane: String,
        journal_posture: String,
        base_digest_posture: String,
        idempotency_posture: String,
    },
    ProductDraftMutation {
        draft_scope: String,
        base_digest_posture: String,
        idempotency_posture: String,
    },
    ProductSessionCoordination {
        coordination_lane: String,
    },
    BinaryStreaming {
        stream_kind: String,
        preflight_posture: String,
        size_posture: String,
        cancellation_posture: String,
        partial_failure_posture: String,
    },
    LeaseCoordination {
        coordination_lane: String,
    },
}

impl WorthServerOperationAuthorityDeclaration {
    pub fn query_shared_read() -> Self {
        Self::SharedRead {
            basis_kind: WorthServerSharedReadBasisKind::QuerySharedReadBasis,
            product_support_posture: WorthServerProductSupportPosture::ProductionAdmitted,
        }
    }

    pub fn product_shared_read(basis_kind: WorthServerSharedReadBasisKind) -> Self {
        Self::product_shared_read_with_support_posture(
            basis_kind,
            WorthServerProductSupportPosture::ProductionAdmitted,
        )
    }

    pub fn product_shared_read_with_support_posture(
        basis_kind: WorthServerSharedReadBasisKind,
        product_support_posture: WorthServerProductSupportPosture,
    ) -> Self {
        Self::SharedRead {
            basis_kind,
            product_support_posture,
        }
    }

    pub fn deterministic_submission(
        submission_lane: impl Into<String>,
        journal_posture: impl Into<String>,
        base_digest_posture: impl Into<String>,
        idempotency_posture: impl Into<String>,
    ) -> Self {
        Self::DeterministicSubmission {
            submission_lane: submission_lane.into(),
            journal_posture: journal_posture.into(),
            base_digest_posture: base_digest_posture.into(),
            idempotency_posture: idempotency_posture.into(),
        }
    }

    pub fn product_draft_mutation(
        draft_scope: impl Into<String>,
        base_digest_posture: impl Into<String>,
        idempotency_posture: impl Into<String>,
    ) -> Self {
        Self::ProductDraftMutation {
            draft_scope: draft_scope.into(),
            base_digest_posture: base_digest_posture.into(),
            idempotency_posture: idempotency_posture.into(),
        }
    }

    pub fn product_session_coordination(coordination_lane: impl Into<String>) -> Self {
        Self::ProductSessionCoordination {
            coordination_lane: coordination_lane.into(),
        }
    }

    pub fn binary_streaming(
        stream_kind: impl Into<String>,
        preflight_posture: impl Into<String>,
        size_posture: impl Into<String>,
        cancellation_posture: impl Into<String>,
        partial_failure_posture: impl Into<String>,
    ) -> Self {
        Self::BinaryStreaming {
            stream_kind: stream_kind.into(),
            preflight_posture: preflight_posture.into(),
            size_posture: size_posture.into(),
            cancellation_posture: cancellation_posture.into(),
            partial_failure_posture: partial_failure_posture.into(),
        }
    }

    pub fn lease_coordination(coordination_lane: impl Into<String>) -> Self {
        Self::LeaseCoordination {
            coordination_lane: coordination_lane.into(),
        }
    }

    pub(crate) fn validate_for_family(
        &self,
        family: WorthServerOperationFamily,
    ) -> Result<(), String> {
        let family_matches = matches!(
            (family, self),
            (
                WorthServerOperationFamily::QueryDirectRead
                    | WorthServerOperationFamily::QueryDirectProjection,
                Self::SharedRead {
                    basis_kind: WorthServerSharedReadBasisKind::QuerySharedReadBasis,
                    product_support_posture: WorthServerProductSupportPosture::ProductionAdmitted,
                }
            ) | (
                WorthServerOperationFamily::ProductApplicationRead,
                Self::SharedRead { .. }
            ) | (
                WorthServerOperationFamily::QueryDirectSubmission,
                Self::DeterministicSubmission { .. }
            ) | (
                WorthServerOperationFamily::ProductApplicationMutation,
                Self::ProductDraftMutation { .. }
            ) | (
                WorthServerOperationFamily::ProductSessionCoordination,
                Self::ProductSessionCoordination { .. }
            ) | (
                WorthServerOperationFamily::BinaryTransfer,
                Self::BinaryStreaming { .. }
            ) | (
                WorthServerOperationFamily::SyncLease,
                Self::LeaseCoordination { .. }
            )
        );
        if !family_matches {
            return Err(format!(
                "operation family `{}` does not match the declared authority template",
                family.as_str()
            ));
        }
        for value in self.required_values() {
            if value.trim().is_empty() {
                return Err(
                    "operation authority declarations may not contain blank required fields"
                        .to_string(),
                );
            }
        }
        Ok(())
    }

    pub(crate) fn lower(
        &self,
        operation_request: &WorthServerOperationRequest,
    ) -> Result<WorthServerOperationAuthorityMetadata, String> {
        match self {
            Self::SharedRead {
                basis_kind,
                product_support_posture,
            } => Ok(
                WorthServerOperationAuthorityMetadata::shared_read_with_support_posture(
                    basis_kind.as_str(),
                    operation_request
                        .identity()
                        .basis_digest()
                        .ok_or_else(|| {
                            "declared shared-read authority requires an admitted operation basis digest"
                                .to_string()
                        })?,
                    operation_request.identity().operation_name(),
                    product_support_posture.as_str(),
                ),
            ),
            Self::DeterministicSubmission {
                submission_lane,
                journal_posture,
                base_digest_posture,
                idempotency_posture,
            } => Ok(WorthServerOperationAuthorityMetadata::deterministic_submission(
                submission_lane,
                journal_posture,
                resolve_base_digest_posture(operation_request, base_digest_posture),
                resolve_idempotency_posture(operation_request, idempotency_posture),
            )),
            Self::ProductDraftMutation {
                draft_scope,
                base_digest_posture,
                idempotency_posture,
            } => Ok(WorthServerOperationAuthorityMetadata::product_draft_mutation(
                operation_request
                    .identity()
                    .product_session_identity()
                    .ok_or_else(|| {
                        "declared product draft authority requires an admitted product session identity"
                            .to_string()
                    })?,
                draft_scope,
                resolve_base_digest_posture(operation_request, base_digest_posture),
                resolve_idempotency_posture(operation_request, idempotency_posture),
            )),
            Self::ProductSessionCoordination { coordination_lane } => {
                Ok(WorthServerOperationAuthorityMetadata::product_session_coordination(
                    match operation_request.identity().product_session_identity() {
                        Some(product_session_identity) => {
                            WorthServerProductSessionCoordinationTarget::ExistingSession {
                                product_session_identity: product_session_identity.to_string(),
                            }
                        }
                        None => WorthServerProductSessionCoordinationTarget::SessionCreation,
                    },
                    coordination_lane,
                ))
            }
            Self::BinaryStreaming {
                stream_kind,
                preflight_posture,
                size_posture,
                cancellation_posture,
                partial_failure_posture,
            } => Ok(WorthServerOperationAuthorityMetadata::binary_streaming(
                stream_kind,
                preflight_posture,
                size_posture,
                cancellation_posture,
                partial_failure_posture,
            )),
            Self::LeaseCoordination { coordination_lane } => {
                Ok(WorthServerOperationAuthorityMetadata::lease_coordination(
                    operation_request.identity().operation_name(),
                    operation_request
                        .identity()
                        .basis_digest()
                        .map(ToString::to_string),
                    coordination_lane,
                ))
            }
        }
    }

    fn required_values(&self) -> Vec<&str> {
        match self {
            Self::SharedRead { .. } => Vec::new(),
            Self::DeterministicSubmission {
                submission_lane,
                journal_posture,
                base_digest_posture,
                idempotency_posture,
            } => vec![
                submission_lane,
                journal_posture,
                base_digest_posture,
                idempotency_posture,
            ],
            Self::ProductDraftMutation {
                draft_scope,
                base_digest_posture,
                idempotency_posture,
            } => vec![draft_scope, base_digest_posture, idempotency_posture],
            Self::ProductSessionCoordination { coordination_lane } => vec![coordination_lane],
            Self::BinaryStreaming {
                stream_kind,
                preflight_posture,
                size_posture,
                cancellation_posture,
                partial_failure_posture,
            } => vec![
                stream_kind,
                preflight_posture,
                size_posture,
                cancellation_posture,
                partial_failure_posture,
            ],
            Self::LeaseCoordination { coordination_lane } => vec![coordination_lane],
        }
    }
}

fn resolve_base_digest_posture(
    operation_request: &WorthServerOperationRequest,
    declared_posture: &str,
) -> String {
    if declared_posture != "derive-from-request" {
        return declared_posture.to_string();
    }
    if operation_request.identity().basis_digest().is_some() {
        "caller-basis-bound".to_string()
    } else {
        "caller-basis-unbound".to_string()
    }
}

fn resolve_idempotency_posture(
    operation_request: &WorthServerOperationRequest,
    declared_posture: &str,
) -> String {
    if declared_posture != "derive-from-request" {
        return declared_posture.to_string();
    }
    if operation_request.identity().idempotency_key().is_some() {
        "idempotent".to_string()
    } else {
        "best-effort".to_string()
    }
}
