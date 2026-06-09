use forge_proof::TransitionOutcome;
use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeSupportProfile,
};
use forge_server::{
    ForgeServerDirectDeclaration, ForgeServerDirectDeclarationDenialCode,
    ForgeServerDirectDeclarationSourceKind, ForgeServerDirectDeclarationSourceSupportStatus,
    ForgeServerDirectViewShape, ForgeServerQueryWorkspaceBindingTarget,
};

use crate::forge_native_runtime::{
    build_server, build_server_with_capturing_workspace_provider,
    build_server_with_profiled_workspace, forge_native_session_input_builder,
};

#[test]
fn direct_named_read_declaration_preserves_identity_and_support_snapshot() {
    let server = build_server(true);

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let prepared = session
        .declarations()
        .read(
            ForgeServerDirectDeclaration::named_read("users.profile")
                .with_view_shape(ForgeServerDirectViewShape::detail()),
        )
        .expect("named read declaration should prepare");

    assert_eq!(
        prepared.declaration().view_shape(),
        ForgeServerDirectViewShape::Detail
    );
    assert_eq!(
        prepared.support_snapshot().source_kind(),
        ForgeServerDirectDeclarationSourceKind::NamedRead
    );
    assert_eq!(
        prepared.support_snapshot().source_support_status(),
        ForgeServerDirectDeclarationSourceSupportStatus::Supported
    );
    assert_eq!(
        prepared
            .support_snapshot()
            .read_family_row()
            .facade_family(),
        Some(ForgeQueryRuntimeFacadeFamily::Read)
    );
    assert!(prepared.support_snapshot().read_family_contract().is_some());
    assert!(prepared.support_snapshot().is_admitted_now());
    assert!(!prepared.declaration_digest().is_empty());

    let admitted = prepared
        .admit()
        .expect("named read declaration should admit");
    assert_eq!(admitted.workspace_name(), "workspace-42");
    assert_eq!(
        admitted.query_family_contract().family(),
        ForgeQueryRuntimeFacadeFamily::Read
    );
}

#[test]
fn direct_saved_query_declaration_fails_closed_with_support_snapshot() {
    let server = build_server(true);

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let prepared = session
        .declarations()
        .read(
            ForgeServerDirectDeclaration::saved_query("users.profile.saved")
                .with_view_shape(ForgeServerDirectViewShape::table()),
        )
        .expect("saved-query declaration should still prepare a support snapshot");

    assert_eq!(
        prepared.support_snapshot().source_kind(),
        ForgeServerDirectDeclarationSourceKind::SavedQuery
    );
    assert_eq!(
        prepared.support_snapshot().source_support_status(),
        ForgeServerDirectDeclarationSourceSupportStatus::DeferredDebt
    );
    assert!(prepared.support_snapshot().read_family_contract().is_some());

    let denial = prepared
        .admit()
        .expect_err("saved-query declaration intake must fail closed in Phase 2");

    assert_eq!(
        denial.code(),
        ForgeServerDirectDeclarationDenialCode::SourceNotAdmitted
    );
    assert_eq!(
        denial
            .support_snapshot()
            .expect("source denial should preserve support snapshot")
            .source_kind(),
        ForgeServerDirectDeclarationSourceKind::SavedQuery
    );
}

#[test]
fn direct_named_read_declaration_localizes_unsupported_query_family() {
    let server = build_server_with_profiled_workspace(
        ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Read,
                "read is intentionally denied in this hostile test profile",
            ),
        ),
    );

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let prepared = session
        .declarations()
        .read(ForgeServerDirectDeclaration::named_read("users.profile"))
        .expect("direct declaration should still prepare support posture");

    assert!(prepared.support_snapshot().read_family_contract().is_none());
    assert_eq!(
        prepared
            .support_snapshot()
            .read_family_row()
            .status()
            .as_str(),
        "unsupported"
    );

    let denial = prepared
        .admit()
        .expect_err("unsupported read family must deny at declaration admission");

    assert_eq!(
        denial.code(),
        ForgeServerDirectDeclarationDenialCode::QueryFacadeFamilyNotAdmitted
    );
    assert_eq!(
        denial
            .support_snapshot()
            .expect("family denial should preserve support snapshot")
            .read_family_row()
            .facade_family(),
        Some(ForgeQueryRuntimeFacadeFamily::Read)
    );
}

#[test]
fn direct_view_shape_changes_declaration_identity_without_changing_family_support() {
    let server = build_server(true);

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let detail = session
        .declarations()
        .read(
            ForgeServerDirectDeclaration::named_read("users.profile")
                .with_view_shape(ForgeServerDirectViewShape::detail()),
        )
        .expect("detail declaration should prepare");
    let table = session
        .declarations()
        .read(
            ForgeServerDirectDeclaration::named_read("users.profile")
                .with_view_shape(ForgeServerDirectViewShape::table()),
        )
        .expect("table declaration should prepare");
    let grouped = session
        .declarations()
        .read(
            ForgeServerDirectDeclaration::named_read("users.profile")
                .with_view_shape(ForgeServerDirectViewShape::grouped()),
        )
        .expect("grouped declaration should prepare");

    assert_ne!(detail.declaration_digest(), table.declaration_digest());
    assert_ne!(detail.declaration_digest(), grouped.declaration_digest());
    assert_eq!(
        detail
            .support_snapshot()
            .read_family_contract()
            .expect("detail declaration should admit read family")
            .contract_digest(),
        table
            .support_snapshot()
            .read_family_contract()
            .expect("table declaration should admit read family")
            .contract_digest()
    );
    assert_eq!(
        table.support_snapshot().support_matrix_digest(),
        grouped.support_snapshot().support_matrix_digest()
    );
}

#[test]
fn direct_named_read_declaration_trims_padding_into_canonical_identity() {
    let server = build_server(true);

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let trimmed = session
        .declarations()
        .read(ForgeServerDirectDeclaration::named_read("users.profile"))
        .expect("trimmed declaration should prepare");
    let padded = session
        .declarations()
        .read(ForgeServerDirectDeclaration::named_read(
            "  users.profile  ",
        ))
        .expect("padded declaration should normalize before preparation");

    assert_eq!(trimmed.declaration(), padded.declaration());
    assert_eq!(trimmed.declaration_digest(), padded.declaration_digest());
    assert_eq!(trimmed.support_snapshot(), padded.support_snapshot());
}

#[test]
fn direct_blank_named_read_declaration_fails_before_workspace_binding() {
    let server = build_server(true);

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let denial = session
        .declarations()
        .read(ForgeServerDirectDeclaration::named_read("   "))
        .expect_err("blank declaration identity must fail closed before binding");

    assert_eq!(
        denial.code(),
        ForgeServerDirectDeclarationDenialCode::InvalidDeclarationIdentity
    );
    assert!(denial
        .detail()
        .contains("named-read declaration identity cannot be blank"));
    assert!(denial.support_snapshot().is_none());
}

#[test]
fn direct_blank_template_declaration_fails_before_deferred_source_posture() {
    let server = build_server(true);

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    let denial = session
        .declarations()
        .read(ForgeServerDirectDeclaration::template("   "))
        .expect_err("blank template identity must fail before deferred-debt posture");

    assert_eq!(
        denial.code(),
        ForgeServerDirectDeclarationDenialCode::InvalidDeclarationIdentity
    );
    assert!(denial
        .detail()
        .contains("template declaration identity cannot be blank"));
    assert!(denial.support_snapshot().is_none());
}

#[test]
fn direct_declaration_workspace_binding_preserves_source_kind_without_query_read_reinterpretation()
{
    let (server, workspace_provider) = build_server_with_capturing_workspace_provider();

    let session = match server.forge_native().session(
        forge_native_session_input_builder()
            .with_branch_id("branch-9")
            .build()
            .expect("forge-native session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected session, got {other:?}"),
    };

    session
        .declarations()
        .read(ForgeServerDirectDeclaration::named_read("users.profile"))
        .expect("named-read declaration should prepare");
    session
        .declarations()
        .read(ForgeServerDirectDeclaration::saved_query(
            "users.profile.saved",
        ))
        .expect("saved-query declaration should still prepare a support snapshot");
    session
        .declarations()
        .read(ForgeServerDirectDeclaration::template(
            "users.profile.template",
        ))
        .expect("template declaration should still prepare a support snapshot");

    assert_eq!(
        workspace_provider.take_captured_targets(),
        vec![
            ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration {
                source_kind: ForgeServerDirectDeclarationSourceKind::NamedRead,
                binding_label: "users.profile".to_string(),
            },
            ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration {
                source_kind: ForgeServerDirectDeclarationSourceKind::SavedQuery,
                binding_label: "users.profile.saved".to_string(),
            },
            ForgeServerQueryWorkspaceBindingTarget::DirectDeclaration {
                source_kind: ForgeServerDirectDeclarationSourceKind::Template,
                binding_label: "users.profile.template".to_string(),
            },
        ]
    );
}
