use forge_proof::{TransitionOutcome, TransitionReadiness};

use crate::{
    CompatHttpSurfaceRoot, ForgeServerPipelineInput, ForgeServerPipelineIntent,
    ForgeServerRequestContextFacade, ForgeServerRequestContextInput, ForgeServerSurfaceFamily,
    ForgeServerTransportClass,
};

use super::{
    request_contract::{
        canonicalization::canonicalize_request, input::RawForgeServerCompatibilityBranchTarget,
        negotiation::negotiate_request,
    },
    ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityDeferred,
    ForgeServerCompatibilityDenial, ForgeServerCompatibilityDenialCode,
    ForgeServerCompatibilityFacade, ForgeServerCompatibilityFailure,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompatibilityRebindRequired,
    ForgeServerCompatibilityRequestInput, ForgeServerCompatibilityRequestOutcome,
    ForgeServerCompatibilityStale, ForgeServerExternalRequestContract,
};

impl ForgeServerCompatibilityFacade {
    pub fn prepare_request(
        &self,
        input: ForgeServerCompatibilityRequestInput,
    ) -> TransitionOutcome<
        ForgeServerCompatibilityPreparedRequest,
        ForgeServerCompatibilityDenial,
        ForgeServerCompatibilityDeferred,
        ForgeServerCompatibilityStale,
        ForgeServerCompatibilityRebindRequired,
        ForgeServerCompatibilityFailure,
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
            return TransitionOutcome::denied(ForgeServerCompatibilityDenial::new(
                ForgeServerCompatibilityDenialCode::IncompatibleMethodForRouteFamily,
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
                return TransitionOutcome::denied(ForgeServerCompatibilityDenial::new(
                    ForgeServerCompatibilityDenialCode::RequestContextDenied,
                    diagnostics_profile,
                    denial.detail(),
                ));
            }
            TransitionReadiness::Deferred(reason) => {
                return TransitionOutcome::deferred(
                    ForgeServerCompatibilityDeferred::RequestContext(reason),
                );
            }
            TransitionReadiness::Stale(reason) => {
                return TransitionOutcome::stale(ForgeServerCompatibilityStale::RequestContext(
                    reason,
                ));
            }
            TransitionReadiness::RebindRequired(reason) => {
                return TransitionOutcome::rebind_required(
                    ForgeServerCompatibilityRebindRequired::RequestContext(reason),
                );
            }
            TransitionReadiness::Failed(reason) => {
                return TransitionOutcome::failed(ForgeServerCompatibilityFailure::RequestContext(
                    reason,
                ));
            }
        };

        let admission = self.middleware.admit(ForgeServerPipelineInput::new(
            resolved_request_context,
            lower_pipeline_intent(input.route_family()),
        ));
        let request_contract = ForgeServerExternalRequestContract::new(
            input.route_family(),
            canonical_request.method().to_string(),
            canonical_request.normalized_path().to_string(),
            canonical_request.normalized_query_pairs().to_vec(),
            canonical_request.canonical_headers().clone(),
            negotiation.representation(),
            negotiation.version(),
            input.diagnostics_profile(),
            input.body_present(),
            canonical_body_content_type(input.body_content_type()),
        );
        match admission {
            TransitionOutcome::Success(admission) => TransitionOutcome::success(
                ForgeServerCompatibilityPreparedRequest::new(admission, request_contract),
            ),
            TransitionOutcome::Denied(denial) => {
                TransitionOutcome::denied(ForgeServerCompatibilityDenial::new(
                    ForgeServerCompatibilityDenialCode::MiddlewareDenied,
                    diagnostics_profile,
                    denial.detail(),
                ))
            }
            TransitionOutcome::Deferred(reason) => {
                TransitionOutcome::deferred(ForgeServerCompatibilityDeferred::Middleware(reason))
            }
            TransitionOutcome::Stale(reason) => {
                TransitionOutcome::stale(ForgeServerCompatibilityStale::Middleware(reason))
            }
            TransitionOutcome::RebindRequired(reason) => TransitionOutcome::rebind_required(
                ForgeServerCompatibilityRebindRequired::Middleware(reason),
            ),
            TransitionOutcome::Failed(reason) => {
                TransitionOutcome::failed(ForgeServerCompatibilityFailure::Middleware(reason))
            }
        }
    }

    pub fn request(
        &self,
        input: ForgeServerCompatibilityRequestInput,
    ) -> ForgeServerCompatibilityRequestOutcome {
        self.prepare_request(input)
            .map_success(ForgeServerCompatibilityPreparedRequest::into_request)
    }
}

fn deny_unavailable_surface(
    root: &CompatHttpSurfaceRoot,
    input: &ForgeServerCompatibilityRequestInput,
    request_contexts: &ForgeServerRequestContextFacade,
) -> Option<
    TransitionOutcome<
        ForgeServerCompatibilityPreparedRequest,
        ForgeServerCompatibilityDenial,
        ForgeServerCompatibilityDeferred,
        ForgeServerCompatibilityStale,
        ForgeServerCompatibilityRebindRequired,
        ForgeServerCompatibilityFailure,
    >,
> {
    if root.capabilities().is_absent() {
        return Some(TransitionOutcome::denied(
            ForgeServerCompatibilityDenial::new(
                ForgeServerCompatibilityDenialCode::CompatHttpSurfaceAbsent,
                default_diagnostics_profile(input, request_contexts),
                "compatibility HTTP surface family is not registered on this server",
            ),
        ));
    }
    if root.capabilities().is_disabled() {
        return Some(TransitionOutcome::denied(
            ForgeServerCompatibilityDenial::new(
                ForgeServerCompatibilityDenialCode::CompatHttpSurfaceDisabled,
                default_diagnostics_profile(input, request_contexts),
                "compatibility HTTP surface family is registered but disabled on this server",
            ),
        ));
    }
    if !root.route_families().contains(input.route_family()) {
        return Some(TransitionOutcome::denied(
            ForgeServerCompatibilityDenial::new(
                ForgeServerCompatibilityDenialCode::UnsupportedRouteFamily,
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
    input: &ForgeServerCompatibilityRequestInput,
) -> ForgeServerRequestContextInput {
    let mut builder = ForgeServerRequestContextInput::builder()
        .with_surface_family(ForgeServerSurfaceFamily::CompatHttp)
        .with_transport_class(ForgeServerTransportClass::CompatHttp)
        .with_authenticated_principal_id(input.authenticated_principal_id())
        .with_tenant_id(input.tenant_id())
        .with_workspace_id(input.workspace_id());

    builder = match input.branch_target() {
        RawForgeServerCompatibilityBranchTarget::Main => builder.with_main_branch(),
        RawForgeServerCompatibilityBranchTarget::Branch { branch_id } => {
            builder.with_branch_id(branch_id)
        }
        RawForgeServerCompatibilityBranchTarget::Preview { preview_id } => {
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
    route_family: ForgeServerCompatHttpRouteFamily,
) -> ForgeServerPipelineIntent {
    match route_family {
        ForgeServerCompatHttpRouteFamily::Mutation => {
            ForgeServerPipelineIntent::query_mutation("compat_http.request_contract")
        }
        _ => ForgeServerPipelineIntent::query_read("compat_http.request_contract"),
    }
}

fn method_matches_route_family(
    method: &str,
    route_family: ForgeServerCompatHttpRouteFamily,
) -> bool {
    match route_family {
        ForgeServerCompatHttpRouteFamily::Read => matches!(method, "GET" | "HEAD"),
        ForgeServerCompatHttpRouteFamily::Mutation => {
            matches!(method, "POST" | "PUT" | "PATCH" | "DELETE")
        }
        ForgeServerCompatHttpRouteFamily::Streaming => matches!(method, "GET" | "HEAD"),
        ForgeServerCompatHttpRouteFamily::Upload => matches!(method, "POST" | "PUT"),
        ForgeServerCompatHttpRouteFamily::Download => matches!(method, "GET" | "HEAD"),
        ForgeServerCompatHttpRouteFamily::Preflight => method == "OPTIONS",
    }
}

fn default_diagnostics_profile(
    input: &ForgeServerCompatibilityRequestInput,
    request_contexts: &ForgeServerRequestContextFacade,
) -> crate::request_context::DiagnosticRichnessProfile {
    input
        .diagnostics_profile()
        .unwrap_or(request_contexts.default_diagnostics_profile())
}
