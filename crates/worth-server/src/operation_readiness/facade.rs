use worth_query::facade::{WorthQueryRuntimeDownstreamDeliveryContract, WorthQueryWorkspace};

use crate::{
    WorthServerCompatibilityPreparedRequest, WorthServerOperationAdmissionPosture,
    WorthServerOperationConcurrencyClass, WorthServerOperationRegistry,
    WorthServerPreparedQueryHandoffKind, WorthServerQueryHandoffOperation,
};

use super::{
    precondition::WorthServerCompatibilityMutationPrecondition, query_support,
    WorthServerOperationPreconditionPosture, WorthServerOperationReadinessClosure,
    WorthServerOperationReadinessDenial, WorthServerOperationReadinessDenialCode,
    WorthServerOperationSupportCompositionReceipt, WorthServerOperationSupportPosture,
};

#[derive(Clone)]
pub struct WorthServerOperationQuerySupportContext<'a> {
    prepared_kind: WorthServerPreparedQueryHandoffKind,
    operation: &'a WorthServerQueryHandoffOperation,
    workspace: &'a WorthQueryWorkspace,
    downstream_delivery_contract: &'a WorthQueryRuntimeDownstreamDeliveryContract,
}

impl<'a> WorthServerOperationQuerySupportContext<'a> {
    pub fn new(
        prepared_kind: WorthServerPreparedQueryHandoffKind,
        operation: &'a WorthServerQueryHandoffOperation,
        workspace: &'a WorthQueryWorkspace,
        downstream_delivery_contract: &'a WorthQueryRuntimeDownstreamDeliveryContract,
    ) -> Self {
        Self {
            prepared_kind,
            operation,
            workspace,
            downstream_delivery_contract,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerCompatibilityMutationPreconditionContext<'a> {
    prepared_request: &'a WorthServerCompatibilityPreparedRequest,
    operation_name: &'a str,
    mutation_request_digest: &'a str,
    observed_basis_digest: &'a str,
    observed_product_session_identity: Option<&'a str>,
}

impl<'a> WorthServerCompatibilityMutationPreconditionContext<'a> {
    pub fn new(
        prepared_request: &'a WorthServerCompatibilityPreparedRequest,
        operation_name: &'a str,
        mutation_request_digest: &'a str,
        observed_basis_digest: &'a str,
    ) -> Self {
        Self {
            prepared_request,
            operation_name,
            mutation_request_digest,
            observed_basis_digest,
            observed_product_session_identity: None,
        }
    }

    pub fn with_observed_product_session_identity(
        mut self,
        observed_product_session_identity: &'a str,
    ) -> Self {
        self.observed_product_session_identity = Some(observed_product_session_identity);
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct WorthServerOperationReadinessFacade {
    operation_registry: Option<WorthServerOperationRegistry>,
}

impl WorthServerOperationReadinessFacade {
    pub(crate) fn with_operation_registry(
        operation_registry: WorthServerOperationRegistry,
    ) -> Self {
        Self {
            operation_registry: Some(operation_registry),
        }
    }

    pub fn compose_support(
        &self,
        operation_admission: &WorthServerOperationAdmissionPosture,
        query_context: Option<WorthServerOperationQuerySupportContext<'_>>,
    ) -> Result<WorthServerOperationSupportPosture, WorthServerOperationReadinessDenial> {
        let dependency_relation = dependency_relation(operation_admission);
        let requires_query_support = dependency_relation != "product-independent";
        let query_support_posture = if requires_query_support {
            let context = query_context.ok_or_else(|| {
                WorthServerOperationReadinessDenial::new(
                    WorthServerOperationReadinessDenialCode::MissingQuerySupport,
                    "operation requires query support composition before planning",
                )
            })?;
            Some(query_support::derive_query_support_posture(
                context.prepared_kind,
                context.operation,
                context.workspace,
                context.downstream_delivery_contract,
                operation_admission
                    .authorization_proof()
                    .admission()
                    .request_context()
                    .diagnostics_profile(),
            )?)
        } else {
            None
        };
        let authority_metadata = operation_admission.authority_metadata();
        deny_unsupported_product_support(authority_metadata)?;
        let planner_posture = if query_support_posture.is_some() {
            "query-and-product-supported"
        } else {
            "product-only-supported"
        };
        let receipt = WorthServerOperationSupportCompositionReceipt::new(
            query_rows(&query_support_posture),
            product_rows(authority_metadata),
            dependency_relation,
            planner_posture,
        );
        Ok(WorthServerOperationSupportPosture::new(
            query_support_posture,
            authority_metadata,
            receipt,
        ))
    }

    pub fn default_precondition_posture(
        &self,
        operation_admission: &WorthServerOperationAdmissionPosture,
    ) -> WorthServerOperationPreconditionPosture {
        WorthServerOperationPreconditionPosture::not_required(
            operation_admission
                .operation_request()
                .identity()
                .operation_family()
                .as_str(),
        )
    }

    pub fn evaluate_compatibility_mutation_preconditions(
        &self,
        input: WorthServerCompatibilityMutationPreconditionContext<'_>,
    ) -> Result<WorthServerCompatibilityMutationPrecondition, WorthServerOperationReadinessDenial>
    {
        let _ = &self.operation_registry;
        WorthServerCompatibilityMutationPrecondition::evaluate(
            input.prepared_request,
            input.operation_name,
            input.mutation_request_digest,
            input.observed_basis_digest,
            input.observed_product_session_identity,
        )
    }

    pub fn close_readiness(
        &self,
        operation_admission: &WorthServerOperationAdmissionPosture,
        query_context: Option<WorthServerOperationQuerySupportContext<'_>>,
        precondition_posture: Option<WorthServerOperationPreconditionPosture>,
    ) -> Result<WorthServerOperationReadinessClosure, WorthServerOperationReadinessDenial> {
        let support_posture = self.compose_support(operation_admission, query_context)?;
        let precondition_posture = precondition_posture
            .unwrap_or_else(|| self.default_precondition_posture(operation_admission));
        let concurrency_class = self.classify_concurrency(
            operation_admission,
            &support_posture,
            &precondition_posture,
        )?;
        Ok(WorthServerOperationReadinessClosure::new(
            support_posture,
            precondition_posture,
            concurrency_class,
        ))
    }

    pub fn classify_concurrency(
        &self,
        operation_admission: &WorthServerOperationAdmissionPosture,
        support_posture: &WorthServerOperationSupportPosture,
        precondition_posture: &WorthServerOperationPreconditionPosture,
    ) -> Result<WorthServerOperationConcurrencyClass, WorthServerOperationReadinessDenial> {
        let _ = &self.operation_registry;
        let _ = precondition_posture.canonical_digest();
        match operation_admission.authority_footprint().authority_kind() {
            crate::WorthServerOperationAuthorityKind::SharedReadOnly => {
                if !support_posture.shared_read_comparable() {
                    return Err(WorthServerOperationReadinessDenial::new(
                        WorthServerOperationReadinessDenialCode::IncompatibleSupportBasis,
                        "shared-read concurrency requires comparable production-admitted basis semantics",
                    ));
                }
                Ok(WorthServerOperationConcurrencyClass::ConcurrentSharedRead)
            }
            crate::WorthServerOperationAuthorityKind::ProductDraftMutation
            | crate::WorthServerOperationAuthorityKind::DeterministicSubmission
            | crate::WorthServerOperationAuthorityKind::ProductSessionCoordination
            | crate::WorthServerOperationAuthorityKind::BinaryStreaming
            | crate::WorthServerOperationAuthorityKind::DiagnosticsOnly
            | crate::WorthServerOperationAuthorityKind::LeaseCoordination => {
                Ok(WorthServerOperationConcurrencyClass::SerializeDeterministically)
            }
        }
    }
}

fn dependency_relation(operation_admission: &WorthServerOperationAdmissionPosture) -> &'static str {
    match operation_admission.authority_metadata() {
        crate::WorthServerOperationAuthorityMetadata::SharedReadOnly { basis_kind, .. }
            if basis_kind == "query-shared-read-basis" || basis_kind == "query-derived" =>
        {
            "query-dependent"
        }
        crate::WorthServerOperationAuthorityMetadata::SharedReadOnly { .. }
        | crate::WorthServerOperationAuthorityMetadata::ProductDraftMutation { .. }
        | crate::WorthServerOperationAuthorityMetadata::ProductSessionCoordination { .. }
        | crate::WorthServerOperationAuthorityMetadata::BinaryStreaming { .. } => {
            "product-independent"
        }
        crate::WorthServerOperationAuthorityMetadata::DeterministicSubmission { .. }
        | crate::WorthServerOperationAuthorityMetadata::LeaseCoordination { .. } => {
            "query-dependent"
        }
        crate::WorthServerOperationAuthorityMetadata::DiagnosticsOnly { .. } => {
            "product-independent"
        }
    }
}

fn deny_unsupported_product_support(
    authority_metadata: &crate::WorthServerOperationAuthorityMetadata,
) -> Result<(), WorthServerOperationReadinessDenial> {
    match authority_metadata {
        crate::WorthServerOperationAuthorityMetadata::SharedReadOnly {
            basis_kind,
            product_support_posture,
            ..
        } if basis_kind == "fixture-only" => Err(WorthServerOperationReadinessDenial::new(
            WorthServerOperationReadinessDenialCode::FixtureOnlyProductSupport,
            "fixture-only product shared-read support is not production-admitted",
        )),
        crate::WorthServerOperationAuthorityMetadata::SharedReadOnly {
            product_support_posture,
            ..
        } if product_support_posture == "unsupported" => {
            Err(WorthServerOperationReadinessDenial::new(
                WorthServerOperationReadinessDenialCode::UnsupportedProductSupport,
                "product shared-read support posture is unsupported for planning",
            ))
        }
        crate::WorthServerOperationAuthorityMetadata::SharedReadOnly {
            product_support_posture,
            ..
        } if product_support_posture == "unknown" => Err(WorthServerOperationReadinessDenial::new(
            WorthServerOperationReadinessDenialCode::UnknownProductSupport,
            "product shared-read support posture is unknown for planning",
        )),
        crate::WorthServerOperationAuthorityMetadata::SharedReadOnly {
            product_support_posture,
            ..
        } if product_support_posture == "incompatible-basis" => {
            Err(WorthServerOperationReadinessDenial::new(
                WorthServerOperationReadinessDenialCode::IncompatibleSupportBasis,
                "product shared-read support posture is incompatible with the planned basis contract",
            ))
        }
        _ => Ok(()),
    }
}

fn query_rows(
    query_support_posture: &Option<crate::WorthServerQuerySupportPosture>,
) -> Vec<String> {
    query_support_posture
        .as_ref()
        .map(crate::WorthServerQuerySupportPosture::canonical_label)
        .into_iter()
        .collect()
}

fn product_rows(authority_metadata: &crate::WorthServerOperationAuthorityMetadata) -> Vec<String> {
    vec![authority_metadata.canonical_digest()]
}
