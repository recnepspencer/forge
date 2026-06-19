use crate::{
    ForgeServerOperationReadinessDenial, ForgeServerOperationReadinessDenialCode,
    ForgeServerOperationRequestDenial, ForgeServerOperationRequestDenialCode,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffDenialFacts,
};

use super::{request::ForgeServerCompatibilityMutationRequest, ForgeServerMutationPrecondition};

pub(crate) fn canonical_mutation_request_digest(
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    operation_name: &str,
    mutation_request: &ForgeServerCompatibilityMutationRequest,
    precondition: &ForgeServerMutationPrecondition,
) -> String {
    format!(
        "compat-http-mutation-request-digest-v1|request:{}|operation:{}|mutation:{}|precondition:{}",
        prepared_request.request_contract().canonical_digest(),
        operation_name.trim(),
        mutation_request.canonical_digest(),
        precondition.request_identity_digest(),
    )
}

pub(crate) fn map_operation_request_denial(
    denial: ForgeServerOperationRequestDenial,
) -> ForgeServerQueryHandoffDenial {
    let code = match denial.code() {
        ForgeServerOperationRequestDenialCode::InvalidIdempotencyKey => {
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid
        }
        ForgeServerOperationRequestDenialCode::UnknownOperationName => {
            ForgeServerQueryHandoffDenialCode::UnknownOperationName
        }
        ForgeServerOperationRequestDenialCode::InvalidBasisDigest => {
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed
        }
        ForgeServerOperationRequestDenialCode::CompatibilityBindingInvalid
        | ForgeServerOperationRequestDenialCode::InvalidOperationName
        | ForgeServerOperationRequestDenialCode::MissingOperationName => {
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid
        }
        _ => ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
    };
    let rejected_operation_name = match denial.code() {
        ForgeServerOperationRequestDenialCode::UnknownOperationName => {
            denial.detail().split('`').nth(1).map(str::to_string)
        }
        _ => None,
    };
    let denial =
        ForgeServerQueryHandoffDenial::new(code, denial.diagnostics_profile(), denial.detail());
    match rejected_operation_name {
        Some(operation_name) => denial.with_facts(
            ForgeServerQueryHandoffDenialFacts::default()
                .with_rejected_operation_name(operation_name),
        ),
        None => denial,
    }
}

pub(crate) fn map_readiness_denial(
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
    denial: ForgeServerOperationReadinessDenial,
) -> ForgeServerQueryHandoffDenial {
    let code = match denial.code() {
        ForgeServerOperationReadinessDenialCode::InvalidPreconditionInput => {
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid
        }
        ForgeServerOperationReadinessDenialCode::PreconditionFailed => {
            ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed
        }
        ForgeServerOperationReadinessDenialCode::DownstreamDeliveryRequiresReadIntent => {
            ForgeServerQueryHandoffDenialCode::DownstreamDeliveryRequiresReadIntent
        }
        ForgeServerOperationReadinessDenialCode::RuntimeBackedResumeUnsupported => {
            ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeUnsupported
        }
        ForgeServerOperationReadinessDenialCode::DurableResumeDeferred => {
            ForgeServerQueryHandoffDenialCode::DurableResumeDeferred
        }
        ForgeServerOperationReadinessDenialCode::MissingQuerySupport
        | ForgeServerOperationReadinessDenialCode::UnsupportedQuerySupport
        | ForgeServerOperationReadinessDenialCode::UnsupportedProductSupport
        | ForgeServerOperationReadinessDenialCode::UnknownProductSupport
        | ForgeServerOperationReadinessDenialCode::FixtureOnlyProductSupport
        | ForgeServerOperationReadinessDenialCode::IncompatibleSupportBasis => {
            ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
        }
    };
    let mut mapped = ForgeServerQueryHandoffDenial::new(
        code,
        prepared_request
            .admission()
            .request_context()
            .diagnostics_profile(),
        denial.detail(),
    );
    if let Some(facts) = denial.facts() {
        let mut query_facts = ForgeServerQueryHandoffDenialFacts::default();
        if let (Some(expected), Some(observed)) =
            (facts.expected_basis_digest(), facts.observed_basis_digest())
        {
            query_facts = query_facts.with_basis_mismatch(expected, observed);
        }
        if let (Some(expected), Some(observed)) =
            (facts.expected_validator(), facts.observed_validator())
        {
            query_facts = query_facts.with_validator_mismatch(expected, observed);
        }
        mapped = mapped.with_facts(query_facts);
    }
    mapped
}
