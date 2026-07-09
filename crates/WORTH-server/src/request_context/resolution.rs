use worth_foundational::DiagnosticRichnessProfile;
use worth_proof::{PreConstructionGate, TransitionReadiness};

use crate::config::WorthServerRequestContextConfig;
use crate::WorthServerSurfaceFamily;

use super::input::RawWorthServerBranchTarget;
use super::{
    denial::{
        diagnostics_profile_exceeds_maximum_detail, incompatible_surface_transport_binding_detail,
    },
    WorthServerAuthenticatedPrincipal, WorthServerBranchTarget, WorthServerRequestContext,
    WorthServerRequestContextDeferred, WorthServerRequestContextDenial,
    WorthServerRequestContextDenialCode, WorthServerRequestContextFailure,
    WorthServerRequestContextInput, WorthServerRequestContextRebindRequired,
    WorthServerRequestContextStale, WorthServerResolvedRequestContext, WorthServerTransportClass,
    WorthServerWorkspaceTarget,
};

pub(crate) type WorthServerRequestContextResolution = TransitionReadiness<
    WorthServerResolvedRequestContext,
    WorthServerRequestContextDenial,
    WorthServerRequestContextDeferred,
    WorthServerRequestContextStale,
    WorthServerRequestContextRebindRequired,
    WorthServerRequestContextFailure,
>;

pub(crate) fn resolve_request_context(
    config: &WorthServerRequestContextConfig,
    input: WorthServerRequestContextInput,
) -> WorthServerRequestContextResolution {
    let requested_or_default_profile = input
        .diagnostics_profile()
        .unwrap_or(config.default_diagnostics_profile());
    let principal_gate = resolve_authenticated_principal(
        input.authenticated_principal_id(),
        requested_or_default_profile,
    );
    let workspace_gate = resolve_workspace_target(
        input.tenant_id(),
        input.workspace_id(),
        requested_or_default_profile,
    );
    let branch_gate =
        resolve_branch_target(config, input.branch_target(), requested_or_default_profile);
    let diagnostics_gate = resolve_diagnostics_profile(config, input.diagnostics_profile());
    let transport_gate = resolve_transport_class(
        input.surface_family(),
        input.transport_class(),
        requested_or_default_profile,
    );

    let authenticated_principal = match principal_gate {
        PreConstructionGate::Ready(value) => value,
        PreConstructionGate::Denied(denial) => return TransitionReadiness::Denied(denial),
        PreConstructionGate::Deferred(reason) => return TransitionReadiness::Deferred(reason),
    };
    let workspace_target = match workspace_gate {
        PreConstructionGate::Ready(value) => value,
        PreConstructionGate::Denied(denial) => return TransitionReadiness::Denied(denial),
        PreConstructionGate::Deferred(reason) => return TransitionReadiness::Deferred(reason),
    };
    let branch_target = match branch_gate {
        PreConstructionGate::Ready(value) => value,
        PreConstructionGate::Denied(denial) => return TransitionReadiness::Denied(denial),
        PreConstructionGate::Deferred(reason) => return TransitionReadiness::Deferred(reason),
    };
    let diagnostics_profile = match diagnostics_gate {
        PreConstructionGate::Ready(value) => value,
        PreConstructionGate::Denied(denial) => return TransitionReadiness::Denied(denial),
        PreConstructionGate::Deferred(reason) => return TransitionReadiness::Deferred(reason),
    };
    let transport_class = match transport_gate {
        PreConstructionGate::Ready(value) => value,
        PreConstructionGate::Denied(denial) => return TransitionReadiness::Denied(denial),
        PreConstructionGate::Deferred(reason) => return TransitionReadiness::Deferred(reason),
    };

    TransitionReadiness::ready(WorthServerResolvedRequestContext::new(
        WorthServerRequestContext::new(
            authenticated_principal,
            workspace_target,
            branch_target,
            diagnostics_profile,
        ),
        input.surface_family(),
        transport_class,
    ))
}

fn resolve_authenticated_principal(
    principal_id: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> PreConstructionGate<
    WorthServerAuthenticatedPrincipal,
    WorthServerRequestContextDenial,
    WorthServerRequestContextDeferred,
> {
    if principal_id.trim().is_empty() {
        return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
            WorthServerRequestContextDenialCode::InvalidAuthenticatedPrincipal,
            diagnostics_profile,
            "authenticated principal id must not be empty",
        ));
    }

    PreConstructionGate::ready(WorthServerAuthenticatedPrincipal::new(
        principal_id.trim().to_owned(),
    ))
}

fn resolve_workspace_target(
    tenant_id: &str,
    workspace_id: &str,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> PreConstructionGate<
    WorthServerWorkspaceTarget,
    WorthServerRequestContextDenial,
    WorthServerRequestContextDeferred,
> {
    if tenant_id.trim().is_empty() {
        return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
            WorthServerRequestContextDenialCode::InvalidWorkspaceTarget,
            diagnostics_profile,
            "tenant id must not be empty",
        ));
    }
    if workspace_id.trim().is_empty() {
        return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
            WorthServerRequestContextDenialCode::InvalidWorkspaceTarget,
            diagnostics_profile,
            "workspace id must not be empty",
        ));
    }

    PreConstructionGate::ready(WorthServerWorkspaceTarget::new(
        tenant_id.trim().to_owned(),
        workspace_id.trim().to_owned(),
    ))
}

fn resolve_branch_target(
    config: &WorthServerRequestContextConfig,
    branch_target: &RawWorthServerBranchTarget,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> PreConstructionGate<
    WorthServerBranchTarget,
    WorthServerRequestContextDenial,
    WorthServerRequestContextDeferred,
> {
    match branch_target {
        RawWorthServerBranchTarget::Main => {
            PreConstructionGate::ready(WorthServerBranchTarget::Main)
        }
        RawWorthServerBranchTarget::Branch { branch_id } => {
            if !config.branch_targeting_enabled() {
                return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
                    WorthServerRequestContextDenialCode::BranchTargetingDisabled,
                    diagnostics_profile,
                    "branch targeting is disabled by server configuration",
                ));
            }
            if branch_id.trim().is_empty() {
                return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
                    WorthServerRequestContextDenialCode::InvalidBranchTarget,
                    diagnostics_profile,
                    "branch id must not be empty",
                ));
            }

            PreConstructionGate::ready(WorthServerBranchTarget::Branch {
                branch_id: branch_id.trim().to_owned(),
            })
        }
        RawWorthServerBranchTarget::Preview { preview_id } => {
            if !config.preview_targeting_enabled() {
                return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
                    WorthServerRequestContextDenialCode::PreviewTargetingDisabled,
                    diagnostics_profile,
                    "preview targeting is disabled by server configuration",
                ));
            }
            if preview_id.trim().is_empty() {
                return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
                    WorthServerRequestContextDenialCode::InvalidBranchTarget,
                    diagnostics_profile,
                    "preview id must not be empty",
                ));
            }

            PreConstructionGate::ready(WorthServerBranchTarget::Preview {
                preview_id: preview_id.trim().to_owned(),
            })
        }
    }
}

fn resolve_diagnostics_profile(
    config: &WorthServerRequestContextConfig,
    requested_profile: Option<DiagnosticRichnessProfile>,
) -> PreConstructionGate<
    DiagnosticRichnessProfile,
    WorthServerRequestContextDenial,
    WorthServerRequestContextDeferred,
> {
    let profile = requested_profile.unwrap_or(config.default_diagnostics_profile());
    if profile > config.maximum_diagnostics_profile() {
        return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
            WorthServerRequestContextDenialCode::DiagnosticsProfileExceedsMaximum,
            profile,
            diagnostics_profile_exceeds_maximum_detail(
                profile,
                config.maximum_diagnostics_profile(),
            ),
        ));
    }

    PreConstructionGate::ready(profile)
}

fn resolve_transport_class(
    surface_family: WorthServerSurfaceFamily,
    transport_class: WorthServerTransportClass,
    diagnostics_profile: DiagnosticRichnessProfile,
) -> PreConstructionGate<
    WorthServerTransportClass,
    WorthServerRequestContextDenial,
    WorthServerRequestContextDeferred,
> {
    let compatible = matches!(
        (surface_family, transport_class),
        (
            WorthServerSurfaceFamily::WorthNative,
            WorthServerTransportClass::WorthNativeInProcess
        ) | (
            WorthServerSurfaceFamily::CompatHttp,
            WorthServerTransportClass::CompatHttp
        )
    );
    if !compatible {
        return PreConstructionGate::denied(WorthServerRequestContextDenial::new(
            WorthServerRequestContextDenialCode::IncompatibleSurfaceTransportBinding,
            diagnostics_profile,
            incompatible_surface_transport_binding_detail(surface_family, transport_class),
        ));
    }

    PreConstructionGate::ready(transport_class)
}
