use forge_query::facade::{ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryWorkspace};

use crate::{
    ForgeServerCompatibilityPreparedRequest, ForgeServerOperationAdmissionPosture,
    ForgeServerOperationConcurrencyClass, ForgeServerOperationRegistry,
    ForgeServerPreparedQueryHandoffKind, ForgeServerQueryHandoffOperation,
};

use super::{
    precondition::ForgeServerCompatibilityMutationPrecondition, query_support,
    ForgeServerOperationPreconditionPosture, ForgeServerOperationReadinessClosure,
    ForgeServerOperationReadinessDenial, ForgeServerOperationReadinessDenialCode,
    ForgeServerOperationSupportCompositionReceipt, ForgeServerOperationSupportPosture,
};

#[derive(Clone)]
pub struct ForgeServerOperationQuerySupportContext<'a> {
    prepared_kind: ForgeServerPreparedQueryHandoffKind,
    operation: &'a ForgeServerQueryHandoffOperation,
    workspace: &'a ForgeQueryWorkspace,
    downstream_delivery_contract: &'a ForgeQueryRuntimeDownstreamDeliveryContract,
}

impl<'a> ForgeServerOperationQuerySupportContext<'a> {
    pub fn new(
        prepared_kind: ForgeServerPreparedQueryHandoffKind,
        operation: &'a ForgeServerQueryHandoffOperation,
        workspace: &'a ForgeQueryWorkspace,
        downstream_delivery_contract: &'a ForgeQueryRuntimeDownstreamDeliveryContract,
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
pub struct ForgeServerCompatibilityMutationPreconditionContext<'a> {
    prepared_request: &'a ForgeServerCompatibilityPreparedRequest,
    operation_name: &'a str,
    mutation_request_digest: &'a str,
    observed_basis_digest: &'a str,
    observed_product_session_identity: Option<&'a str>,
}

impl<'a> ForgeServerCompatibilityMutationPreconditionContext<'a> {
    pub fn new(
        prepared_request: &'a ForgeServerCompatibilityPreparedRequest,
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
pub struct ForgeServerOperationReadinessFacade {
    operation_registry: Option<ForgeServerOperationRegistry>,
}

impl ForgeServerOperationReadinessFacade {
    pub(crate) fn with_operation_registry(
        operation_registry: ForgeServerOperationRegistry,
    ) -> Self {
        Self {
            operation_registry: Some(operation_registry),
        }
    }

    pub fn compose_support(
        &self,
        operation_admission: &ForgeServerOperationAdmissionPosture,
        query_context: Option<ForgeServerOperationQuerySupportContext<'_>>,
    ) -> Result<ForgeServerOperationSupportPosture, ForgeServerOperationReadinessDenial> {
        let dependency_relation = dependency_relation(operation_admission);
        let requires_query_support = dependency_relation != "product-independent";
        let query_support_posture = if requires_query_support {
            let context = query_context.ok_or_else(|| {
                ForgeServerOperationReadinessDenial::new(
                    ForgeServerOperationReadinessDenialCode::MissingQuerySupport,
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
        let receipt = ForgeServerOperationSupportCompositionReceipt::new(
            query_rows(&query_support_posture),
            product_rows(authority_metadata),
            dependency_relation,
            planner_posture,
        );
        Ok(ForgeServerOperationSupportPosture::new(
            query_support_posture,
            authority_metadata,
            receipt,
        ))
    }

    pub fn default_precondition_posture(
        &self,
        operation_admission: &ForgeServerOperationAdmissionPosture,
    ) -> ForgeServerOperationPreconditionPosture {
        ForgeServerOperationPreconditionPosture::not_required(
            operation_admission
                .operation_request()
                .identity()
                .operation_family()
                .as_str(),
        )
    }

    pub fn evaluate_compatibility_mutation_preconditions(
        &self,
        input: ForgeServerCompatibilityMutationPreconditionContext<'_>,
    ) -> Result<ForgeServerCompatibilityMutationPrecondition, ForgeServerOperationReadinessDenial>
    {
        let _ = &self.operation_registry;
        ForgeServerCompatibilityMutationPrecondition::evaluate(
            input.prepared_request,
            input.operation_name,
            input.mutation_request_digest,
            input.observed_basis_digest,
            input.observed_product_session_identity,
        )
    }

    pub fn close_readiness(
        &self,
        operation_admission: &ForgeServerOperationAdmissionPosture,
        query_context: Option<ForgeServerOperationQuerySupportContext<'_>>,
        precondition_posture: Option<ForgeServerOperationPreconditionPosture>,
    ) -> Result<ForgeServerOperationReadinessClosure, ForgeServerOperationReadinessDenial> {
        let support_posture = self.compose_support(operation_admission, query_context)?;
        let precondition_posture = precondition_posture
            .unwrap_or_else(|| self.default_precondition_posture(operation_admission));
        let concurrency_class = self.classify_concurrency(
            operation_admission,
            &support_posture,
            &precondition_posture,
        )?;
        Ok(ForgeServerOperationReadinessClosure::new(
            support_posture,
            precondition_posture,
            concurrency_class,
        ))
    }

    pub fn classify_concurrency(
        &self,
        operation_admission: &ForgeServerOperationAdmissionPosture,
        support_posture: &ForgeServerOperationSupportPosture,
        precondition_posture: &ForgeServerOperationPreconditionPosture,
    ) -> Result<ForgeServerOperationConcurrencyClass, ForgeServerOperationReadinessDenial> {
        let _ = &self.operation_registry;
        let _ = precondition_posture.canonical_digest();
        match operation_admission.authority_footprint().authority_kind() {
            crate::ForgeServerOperationAuthorityKind::SharedReadOnly => {
                if !support_posture.shared_read_comparable() {
                    return Err(ForgeServerOperationReadinessDenial::new(
                        ForgeServerOperationReadinessDenialCode::IncompatibleSupportBasis,
                        "shared-read concurrency requires comparable production-admitted basis semantics",
                    ));
                }
                Ok(ForgeServerOperationConcurrencyClass::ConcurrentSharedRead)
            }
            crate::ForgeServerOperationAuthorityKind::ProductDraftMutation
            | crate::ForgeServerOperationAuthorityKind::DeterministicSubmission
            | crate::ForgeServerOperationAuthorityKind::ProductSessionCoordination
            | crate::ForgeServerOperationAuthorityKind::BinaryStreaming
            | crate::ForgeServerOperationAuthorityKind::DiagnosticsOnly
            | crate::ForgeServerOperationAuthorityKind::LeaseCoordination => {
                Ok(ForgeServerOperationConcurrencyClass::SerializeDeterministically)
            }
        }
    }
}

fn dependency_relation(operation_admission: &ForgeServerOperationAdmissionPosture) -> &'static str {
    match operation_admission.authority_metadata() {
        crate::ForgeServerOperationAuthorityMetadata::SharedReadOnly { basis_kind, .. }
            if basis_kind == "query-shared-read-basis" || basis_kind == "query-derived" =>
        {
            "query-dependent"
        }
        crate::ForgeServerOperationAuthorityMetadata::SharedReadOnly { .. }
        | crate::ForgeServerOperationAuthorityMetadata::ProductDraftMutation { .. }
        | crate::ForgeServerOperationAuthorityMetadata::ProductSessionCoordination { .. }
        | crate::ForgeServerOperationAuthorityMetadata::BinaryStreaming { .. } => {
            "product-independent"
        }
        crate::ForgeServerOperationAuthorityMetadata::DeterministicSubmission { .. }
        | crate::ForgeServerOperationAuthorityMetadata::LeaseCoordination { .. } => {
            "query-dependent"
        }
        crate::ForgeServerOperationAuthorityMetadata::DiagnosticsOnly { .. } => {
            "product-independent"
        }
    }
}

fn deny_unsupported_product_support(
    authority_metadata: &crate::ForgeServerOperationAuthorityMetadata,
) -> Result<(), ForgeServerOperationReadinessDenial> {
    match authority_metadata {
        crate::ForgeServerOperationAuthorityMetadata::SharedReadOnly {
            basis_kind,
            product_support_posture,
            ..
        } if basis_kind == "fixture-only" => Err(ForgeServerOperationReadinessDenial::new(
            ForgeServerOperationReadinessDenialCode::FixtureOnlyProductSupport,
            "fixture-only product shared-read support is not production-admitted",
        )),
        crate::ForgeServerOperationAuthorityMetadata::SharedReadOnly {
            product_support_posture,
            ..
        } if product_support_posture == "unsupported" => {
            Err(ForgeServerOperationReadinessDenial::new(
                ForgeServerOperationReadinessDenialCode::UnsupportedProductSupport,
                "product shared-read support posture is unsupported for planning",
            ))
        }
        crate::ForgeServerOperationAuthorityMetadata::SharedReadOnly {
            product_support_posture,
            ..
        } if product_support_posture == "unknown" => Err(ForgeServerOperationReadinessDenial::new(
            ForgeServerOperationReadinessDenialCode::UnknownProductSupport,
            "product shared-read support posture is unknown for planning",
        )),
        crate::ForgeServerOperationAuthorityMetadata::SharedReadOnly {
            product_support_posture,
            ..
        } if product_support_posture == "incompatible-basis" => {
            Err(ForgeServerOperationReadinessDenial::new(
                ForgeServerOperationReadinessDenialCode::IncompatibleSupportBasis,
                "product shared-read support posture is incompatible with the planned basis contract",
            ))
        }
        _ => Ok(()),
    }
}

fn query_rows(
    query_support_posture: &Option<crate::ForgeServerQuerySupportPosture>,
) -> Vec<String> {
    query_support_posture
        .as_ref()
        .map(crate::ForgeServerQuerySupportPosture::canonical_label)
        .into_iter()
        .collect()
}

fn product_rows(authority_metadata: &crate::ForgeServerOperationAuthorityMetadata) -> Vec<String> {
    vec![authority_metadata.canonical_digest()]
}
