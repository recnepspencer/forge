#[path = "support/compat_http/phase_two_runtime.rs"]
mod compat_http_phase_two_runtime;
#[path = "support/direct_context_runtime.rs"]
mod direct_context_runtime;
#[path = "support/forge_native/assertions.rs"]
mod forge_native_assertions;
#[path = "support/forge_native/runtime.rs"]
mod forge_native_runtime;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryInspection, ForgeQueryRuntimeRemaskProjection, ForgeQueryRuntimeRemaskReasonKind,
};
use forge_server::{
    ForgeServerCompatibilityExecutionInput, ForgeServerCompatibilityInspection,
    ForgeServerCompatibilityRead, ForgeServerCompatibilityState, ForgeServerQueryHandoffDenialCode,
};

use compat_http_phase_two_runtime::{
    branch_head_execution_input, build_phase_two_server,
    build_phase_two_server_with_workspace_provider, compat_execution_input,
    forge_native_named_read, prepared_read_request, read_input,
};
use direct_context_runtime::RemaskWorkspaceProvider;
use forge_native_assertions::{
    direct_provenance_digest, family_contract_digest, response_provenance_digest,
};
use query_handoff_runtime::PanicOnReadTestWorkspaceProvider;

#[test]
fn compat_http_read_matches_forge_native_direct_read_on_narrow_canonical_artifacts() {
    let server = build_phase_two_server();
    let (session, declaration) = forge_native_named_read(&server, "users.profile");

    let direct = match session.direct().read(&declaration) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected direct read success, got {other:?}"),
    };
    let compat = compat_read_success(
        server
            .compat_http()
            .read(compat_execution_input(&server, "users.profile")),
    );

    assert_eq!(
        compat.declaration_digest(),
        declaration.declaration_digest()
    );
    assert_eq!(compat.handoff_digest(), direct.handoff_digest());
    assert_eq!(
        compat.direct_context().basis_digest(),
        direct.direct_context().basis_digest()
    );
    assert_eq!(
        family_contract_digest(compat.support_posture()),
        family_contract_digest(direct.support_posture())
    );
    assert_eq!(
        direct_provenance_digest(compat.direct_context().provenance()),
        direct_provenance_digest(direct.direct_context().provenance())
    );
    assert_eq!(
        response_provenance_digest(compat.response_envelope()),
        response_provenance_digest(direct.response_envelope())
    );
    assert_eq!(
        compat.validator().entity_tag(),
        format!("\"{}\"", compat.validator().canonical_digest())
    );
    assert_eq!(compat.basis_request().requested_basis_digest(), None);
    assert_eq!(compat.conditional_read().if_match(), None);
    assert_eq!(compat.conditional_read().if_none_match(), None);
    assert_private_cache_policy(compat.cache_policy());
}

#[test]
fn compat_http_basis_localization_denies_repeated_and_preview_basis_requests() {
    let baseline_server = build_phase_two_server();
    let admitted_basis_digest = compat_read_success(
        baseline_server
            .compat_http()
            .read(compat_execution_input(&baseline_server, "users.profile")),
    )
    .direct_context()
    .basis_digest()
    .expect("compat read should expose its retained basis digest")
    .to_string();
    let server = build_phase_two_server_with_workspace_provider(PanicOnReadTestWorkspaceProvider);
    let repeated_basis = ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            &server,
            read_input("users.profile")
                .with_query_pair("basis", "basis-a")
                .with_query_pair("basis", "basis-b")
                .build()
                .expect("repeated basis input should validate structurally"),
        ),
        "users.profile",
    );
    let preview_basis = ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            &server,
            forge_server::ForgeServerCompatibilityRequestInput::builder()
                .with_authenticated_principal_id("principal-7")
                .with_tenant_id("tenant-a")
                .with_workspace_id("workspace-42")
                .with_preview_id("preview-1")
                .with_route_family(forge_server::ForgeServerCompatHttpRouteFamily::Read)
                .with_method("GET")
                .with_path("/compat/reads/users.profile")
                .with_query_pair("basis", "basis-preview")
                .build()
                .expect("preview basis input should validate structurally"),
        ),
        "users.profile",
    );
    let drifted_basis = ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            &server,
            read_input("users.profile")
                .with_query_pair("basis", "basis:drifted")
                .build()
                .expect("drifted basis input should validate structurally"),
        ),
        "users.profile",
    );

    let repeated_denial = compat_denial(server.compat_http().read(repeated_basis));
    let preview_denial = compat_denial(server.compat_http().read(preview_basis));
    let drifted_denial = compat_denial(server.compat_http().read(drifted_basis));

    assert_eq!(
        repeated_denial.code(),
        ForgeServerQueryHandoffDenialCode::CompatibilityBasisRequestInvalid
    );
    assert_eq!(
        repeated_denial.detail(),
        "compatibility read admits at most one canonical basis query parameter"
    );
    assert_eq!(
        preview_denial.code(),
        ForgeServerQueryHandoffDenialCode::CompatibilityBasisRequestUnsupported
    );
    assert_eq!(
        preview_denial.detail(),
        "preview-targeted compatibility reads do not admit an additional explicit basis digest"
    );
    assert_eq!(
        drifted_denial.code(),
        ForgeServerQueryHandoffDenialCode::CompatibilityBasisRequestInvalid
    );
    assert_eq!(
        drifted_denial.detail(),
        format!(
            "compatibility basis request `basis:drifted` drifted from the admitted retained basis `{admitted_basis_digest}`"
        )
    );
}

#[test]
fn compat_http_state_and_inspection_stay_distinct_under_the_same_named_read() {
    let server = build_phase_two_server();
    let state = compat_state_success(
        server
            .compat_http()
            .state(compat_execution_input(&server, "users.profile")),
    );
    let inspection = compat_inspection_success(
        server
            .compat_http()
            .inspect(compat_execution_input(&server, "users.profile")),
    );

    assert_ne!(state.canonical_digest(), inspection.canonical_digest());
    assert_ne!(
        std::mem::discriminant(state.support_posture()),
        std::mem::discriminant(inspection.support_posture())
    );
    assert_eq!(
        state.direct_context().basis_digest(),
        Some(state.runtime_state().basis_digest())
    );
    match inspection.inspection_result().inspection() {
        ForgeQueryInspection::LiveView(live) => {
            assert_eq!(
                inspection.direct_context().basis_digest(),
                Some(live.basis_binding_digest())
            );
        }
        other => panic!("expected live inspection result, got {other:?}"),
    }
}

#[test]
fn compat_http_conditional_reads_enforce_exact_validator_contracts() {
    let server = build_phase_two_server();
    let baseline = compat_read_success(
        server
            .compat_http()
            .read(compat_execution_input(&server, "users.profile")),
    );
    let head = compat_read_success(
        server
            .compat_http()
            .read(branch_head_execution_input(&server, "users.profile")),
    );
    let not_modified = ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            &server,
            read_input("users.profile")
                .with_header("if-none-match", baseline.validator().entity_tag())
                .build()
                .expect("conditional request should validate structurally"),
        ),
        "users.profile",
    );
    let mismatch = ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            &server,
            read_input("users.profile")
                .with_header("if-match", "\"basis:wrong\"")
                .build()
                .expect("precondition request should validate structurally"),
        ),
        "users.profile",
    );
    let simultaneous = ForgeServerCompatibilityExecutionInput::new(
        prepared_read_request(
            &server,
            read_input("users.profile")
                .with_header("if-match", baseline.validator().entity_tag())
                .with_header("if-none-match", "\"stale\"")
                .build()
                .expect("simultaneous conditional request should validate structurally"),
        ),
        "users.profile",
    );

    let not_modified_denial = compat_denial(server.compat_http().read(not_modified));
    let mismatch_denial = compat_denial(server.compat_http().read(mismatch));
    let simultaneous_denial = compat_denial(server.compat_http().read(simultaneous));

    assert_eq!(
        baseline.validator().entity_tag(),
        head.validator().entity_tag()
    );
    assert_eq!(
        baseline.direct_context().basis_digest(),
        head.direct_context().basis_digest()
    );
    assert_eq!(
        response_provenance_digest(baseline.response_envelope()),
        response_provenance_digest(head.response_envelope())
    );
    assert_private_cache_policy(baseline.cache_policy());
    assert_eq!(
        baseline.cache_policy().canonical_digest(),
        head.cache_policy().canonical_digest()
    );
    assert_eq!(
        not_modified_denial.code(),
        ForgeServerQueryHandoffDenialCode::CompatibilityConditionalReadNotModified
    );
    assert_eq!(
        not_modified_denial.detail(),
        "compatibility if-none-match validator already matches the canonical read validator"
    );
    assert_eq!(
        mismatch_denial.code(),
        ForgeServerQueryHandoffDenialCode::CompatibilityConditionalReadPreconditionFailed
    );
    assert_eq!(
        mismatch_denial.detail(),
        "compatibility if-match validator does not match the canonical read validator"
    );
    assert_eq!(
        simultaneous_denial.code(),
        ForgeServerQueryHandoffDenialCode::CompatibilityConditionalRequestInvalid
    );
    assert_eq!(
        simultaneous_denial.detail(),
        "compatibility read does not admit simultaneous if-match and if-none-match validators"
    );
}

#[test]
fn compat_http_cache_policy_stays_private_for_branch_and_remasked_reads() {
    let server = build_phase_two_server_with_workspace_provider(RemaskWorkspaceProvider::new(
        ForgeQueryRuntimeRemaskProjection::remasked(
            ForgeQueryRuntimeRemaskReasonKind::PolicyDrift,
            "policy:test",
            "tenant-truth:test",
            "tenant-schema:test",
            "relationship-proof:test",
            "schema-context:test",
        ),
    ));
    let read = compat_read_success(
        server
            .compat_http()
            .read(compat_execution_input(&server, "users.profile")),
    );
    let state = compat_state_success(
        server
            .compat_http()
            .state(compat_execution_input(&server, "users.profile")),
    );
    let inspection = compat_inspection_success(
        server
            .compat_http()
            .inspect(compat_execution_input(&server, "users.profile")),
    );

    for cache_policy in [
        read.cache_policy(),
        state.cache_policy(),
        inspection.cache_policy(),
    ] {
        assert_private_cache_policy(cache_policy);
    }
    assert_eq!(
        state.direct_context().remask_posture().disposition(),
        forge_server::ForgeServerDirectRemaskDisposition::Remasked
    );
    assert_eq!(
        inspection.direct_context().remask_posture().disposition(),
        forge_server::ForgeServerDirectRemaskDisposition::Remasked
    );
}

fn compat_read_success(
    outcome: forge_server::ForgeServerCompatibilityExecutionOutcome<ForgeServerCompatibilityRead>,
) -> ForgeServerCompatibilityRead {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility read success, got {other:?}"),
    }
}

fn compat_state_success(
    outcome: forge_server::ForgeServerCompatibilityExecutionOutcome<ForgeServerCompatibilityState>,
) -> ForgeServerCompatibilityState {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility state success, got {other:?}"),
    }
}

fn compat_inspection_success(
    outcome: forge_server::ForgeServerCompatibilityExecutionOutcome<
        ForgeServerCompatibilityInspection,
    >,
) -> ForgeServerCompatibilityInspection {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected compatibility inspection success, got {other:?}"),
    }
}

fn compat_denial<T: std::fmt::Debug>(
    outcome: forge_server::ForgeServerCompatibilityExecutionOutcome<T>,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(value) => value,
        other => panic!("expected compatibility denial, got {other:?}"),
    }
}

fn assert_private_cache_policy(cache_policy: &forge_server::ForgeServerCompatibilityCachePolicy) {
    assert_eq!(cache_policy.cache_control(), "private, no-store");
    assert!(!cache_policy.publicly_reusable());
    assert_eq!(
        cache_policy.vary(),
        ["authorization", "x-forge-branch", "x-forge-diagnostics"].map(str::to_string)
    );
}
