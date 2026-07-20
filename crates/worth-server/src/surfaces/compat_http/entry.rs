use worth_proof::{TransitionOutcome, TransitionReadiness};

use crate::{
    CompatHttpSurfaceRoot, WorthServerPipelineInput, WorthServerPipelineIntent,
    WorthServerRequestContextFacade, WorthServerRequestContextInput, WorthServerSurfaceFamily,
    WorthServerTransportClass,
};

use super::{
    abuse_accounting::{byte_class_for_request, WorthServerAbuseBudgetReceipt},
    request_contract::{
        canonicalization::canonicalize_request, input::RawWorthServerCompatibilityBranchTarget,
        negotiation::negotiate_request,
    },
    WorthServerCompatHttpRouteFamily, WorthServerCompatibilityDeferred,
    WorthServerCompatibilityDenial, WorthServerCompatibilityDenialCode,
    WorthServerCompatibilityFacade, WorthServerCompatibilityFailure,
    WorthServerCompatibilityPreparedRequest, WorthServerCompatibilityRebindRequired,
    WorthServerCompatibilityRequestInput, WorthServerCompatibilityRequestOutcome,
    WorthServerCompatibilityStale, WorthServerExternalRequestContract,
};

impl WorthServerCompatibilityFacade {
    pub fn prepare_request(
        &self,
        input: WorthServerCompatibilityRequestInput,
    ) -> TransitionOutcome<
        WorthServerCompatibilityPreparedRequest,
        WorthServerCompatibilityDenial,
        WorthServerCompatibilityDeferred,
        WorthServerCompatibilityStale,
        WorthServerCompatibilityRebindRequired,
        WorthServerCompatibilityFailure,
    > {
        if let Some(denial) = deny_unavailable_surface(self.root(), &input, &self.request_contexts)
        {
            return denial;
        }
        let diagnostics_profile = default_diagnostics_profile(&input, &self.request_contexts);
        let canonical_request = match canonicalize_request(&input, diagnostics_profile) {
            Ok(canonical_request) => canonical_request,
            Err(denial) => return TransitionOutcome::denied(denial),
        };
        let negotiation = match negotiate_request(&canonical_request, diagnostics_profile) {
            Ok(negotiation) => negotiation,
            Err(denial) => return TransitionOutcome::denied(denial),
        };
        if !method_matches_route_family(canonical_request.method(), input.route_family()) {
            return TransitionOutcome::denied(WorthServerCompatibilityDenial::new(
                WorthServerCompatibilityDenialCode::IncompatibleMethodForRouteFamily,
                diagnostics_profile,
                format!(
                    "HTTP method `{}` is not admitted for compatibility route family `{}`",
                    canonical_request.method(),
                    input.route_family().as_str(),
                ),
            ));
        }

        let request_context_input = lower_request_context_input(&input);
        let resolved_request_context = match self.request_contexts.resolve(request_context_input) {
            TransitionReadiness::Ready(context) => context,
            TransitionReadiness::Denied(denial) => {
                return TransitionOutcome::denied(WorthServerCompatibilityDenial::new(
                    WorthServerCompatibilityDenialCode::RequestContextDenied,
                    diagnostics_profile,
                    denial.detail(),
                ));
            }
            TransitionReadiness::Deferred(reason) => {
                return TransitionOutcome::deferred(
                    WorthServerCompatibilityDeferred::RequestContext(reason),
                );
            }
            TransitionReadiness::Stale(reason) => {
                return TransitionOutcome::stale(WorthServerCompatibilityStale::RequestContext(
                    reason,
                ));
            }
            TransitionReadiness::RebindRequired(reason) => {
                return TransitionOutcome::rebind_required(
                    WorthServerCompatibilityRebindRequired::RequestContext(reason),
                );
            }
            TransitionReadiness::Failed(reason) => {
                return TransitionOutcome::failed(WorthServerCompatibilityFailure::RequestContext(
                    reason,
                ));
            }
        };

        let tenant_id = resolved_request_context
            .request_context()
            .workspace_target()
            .tenant_id()
            .to_string();
        let workspace_digest = resolved_request_context
            .request_context()
            .workspace_target()
            .workspace_digest();
        let branch_digest = resolved_request_context
            .request_context()
            .branch_target()
            .branch_digest();
        let admission = self.middleware.admit(WorthServerPipelineInput::new(
            resolved_request_context,
            lower_pipeline_intent(input.route_family()),
        ));
        let request_contract = WorthServerExternalRequestContract::new(
            super::WorthServerExternalRequestContractParts {
                route_family: input.route_family(),
                method: canonical_request.method().to_string(),
                normalized_path: canonical_request.normalized_path().to_string(),
                normalized_query_pairs: canonical_request.normalized_query_pairs().to_vec(),
                canonical_headers: canonical_request.canonical_headers().clone(),
                representation: negotiation.representation(),
                version: negotiation.version(),
                diagnostics_profile: input.diagnostics_profile(),
                body_present: input.body_present(),
                body_content_type: canonical_body_content_type(input.body_content_type()),
            },
        );
        match admission {
            TransitionOutcome::Success(admission) => TransitionOutcome::success(
                WorthServerCompatibilityPreparedRequest::new(admission, request_contract),
            ),
            TransitionOutcome::Denied(denial) => {
                let compatibility_denial = WorthServerCompatibilityDenial::new(
                    WorthServerCompatibilityDenialCode::MiddlewareDenied,
                    diagnostics_profile,
                    denial.detail(),
                );
                let compatibility_denial = if denial.code()
                    == crate::WorthServerDenialCode::CompatHttpDiagnosticsBudgetExceeded
                {
                    compatibility_denial.with_abuse_budget_receipt(
                        WorthServerAbuseBudgetReceipt::denied(
                            input.route_family(),
                            byte_class_for_request(
                                input.route_family(),
                                canonical_request.method(),
                            ),
                            tenant_id,
                            workspace_digest,
                            branch_digest,
                            denial.detail(),
                        ),
                    )
                } else {
                    compatibility_denial
                };
                TransitionOutcome::denied(compatibility_denial)
            }
            TransitionOutcome::Deferred(reason) => {
                TransitionOutcome::deferred(WorthServerCompatibilityDeferred::Middleware(reason))
            }
            TransitionOutcome::Stale(reason) => {
                TransitionOutcome::stale(WorthServerCompatibilityStale::Middleware(reason))
            }
            TransitionOutcome::RebindRequired(reason) => TransitionOutcome::rebind_required(
                WorthServerCompatibilityRebindRequired::Middleware(reason),
            ),
            TransitionOutcome::Failed(reason) => {
                TransitionOutcome::failed(WorthServerCompatibilityFailure::Middleware(reason))
            }
        }
    }

    pub fn request(
        &self,
        input: WorthServerCompatibilityRequestInput,
    ) -> WorthServerCompatibilityRequestOutcome {
        self.prepare_request(input)
            .map_success(WorthServerCompatibilityPreparedRequest::into_request)
    }
}

fn deny_unavailable_surface(
    root: &CompatHttpSurfaceRoot,
    input: &WorthServerCompatibilityRequestInput,
    request_contexts: &WorthServerRequestContextFacade,
) -> Option<
    TransitionOutcome<
        WorthServerCompatibilityPreparedRequest,
        WorthServerCompatibilityDenial,
        WorthServerCompatibilityDeferred,
        WorthServerCompatibilityStale,
        WorthServerCompatibilityRebindRequired,
        WorthServerCompatibilityFailure,
    >,
> {
    if root.capabilities().is_absent() {
        return Some(TransitionOutcome::denied(
            WorthServerCompatibilityDenial::new(
                WorthServerCompatibilityDenialCode::CompatHttpSurfaceAbsent,
                default_diagnostics_profile(input, request_contexts),
                "compatibility HTTP surface family is not registered on this server",
            ),
        ));
    }
    if root.capabilities().is_disabled() {
        return Some(TransitionOutcome::denied(
            WorthServerCompatibilityDenial::new(
                WorthServerCompatibilityDenialCode::CompatHttpSurfaceDisabled,
                default_diagnostics_profile(input, request_contexts),
                "compatibility HTTP surface family is registered but disabled on this server",
            ),
        ));
    }
    if !root.route_families().contains(input.route_family()) {
        return Some(TransitionOutcome::denied(
            WorthServerCompatibilityDenial::new(
                WorthServerCompatibilityDenialCode::UnsupportedRouteFamily,
                default_diagnostics_profile(input, request_contexts),
                format!(
                    "compatibility route family `{}` is not registered on this server",
                    input.route_family().as_str()
                ),
            ),
        ));
    }
    None
}

fn canonical_body_content_type(content_type: Option<&str>) -> Option<String> {
    content_type.map(|value| value.trim().to_ascii_lowercase())
}

fn lower_request_context_input(
    input: &WorthServerCompatibilityRequestInput,
) -> WorthServerRequestContextInput {
    let mut builder = WorthServerRequestContextInput::builder()
        .with_surface_family(WorthServerSurfaceFamily::CompatHttp)
        .with_transport_class(WorthServerTransportClass::CompatHttp)
        .with_authenticated_principal_id(input.authenticated_principal_id())
        .with_tenant_id(input.tenant_id())
        .with_workspace_id(input.workspace_id());

    builder = match input.branch_target() {
        RawWorthServerCompatibilityBranchTarget::Main => builder.with_main_branch(),
        RawWorthServerCompatibilityBranchTarget::Branch { branch_id } => {
            builder.with_branch_id(branch_id)
        }
        RawWorthServerCompatibilityBranchTarget::Preview { preview_id } => {
            builder.with_preview_id(preview_id)
        }
    };
    if let Some(profile) = input.diagnostics_profile() {
        builder = builder.with_diagnostics_profile(profile);
    }

    builder.build().expect(
        "compatibility request lowering must provide all fixed server-owned request-context fields",
    )
}

fn lower_pipeline_intent(
    route_family: WorthServerCompatHttpRouteFamily,
) -> WorthServerPipelineIntent {
    match route_family {
        WorthServerCompatHttpRouteFamily::Mutation => {
            WorthServerPipelineIntent::query_mutation("compat_http.request_contract")
        }
        WorthServerCompatHttpRouteFamily::Upload => {
            WorthServerPipelineIntent::query_mutation("compat_http.request_contract")
        }
        _ => WorthServerPipelineIntent::query_read("compat_http.request_contract"),
    }
}

fn method_matches_route_family(
    method: &str,
    route_family: WorthServerCompatHttpRouteFamily,
) -> bool {
    match route_family {
        WorthServerCompatHttpRouteFamily::Read => matches!(method, "GET" | "HEAD"),
        WorthServerCompatHttpRouteFamily::Mutation => {
            matches!(method, "POST" | "PUT" | "PATCH" | "DELETE")
        }
        WorthServerCompatHttpRouteFamily::Streaming => matches!(method, "GET" | "HEAD"),
        WorthServerCompatHttpRouteFamily::Upload => matches!(method, "POST" | "PUT"),
        WorthServerCompatHttpRouteFamily::Download => matches!(method, "GET" | "HEAD"),
        WorthServerCompatHttpRouteFamily::Preflight => method == "OPTIONS",
    }
}

fn default_diagnostics_profile(
    input: &WorthServerCompatibilityRequestInput,
    request_contexts: &WorthServerRequestContextFacade,
) -> crate::request_context::DiagnosticRichnessProfile {
    input
        .diagnostics_profile()
        .unwrap_or(request_contexts.default_diagnostics_profile())
}
