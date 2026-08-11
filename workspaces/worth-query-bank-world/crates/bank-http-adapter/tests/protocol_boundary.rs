use bank_http_adapter::{
    AuthentikAuthorizationCallback, AuthentikOidcConfiguration, AuthentikOidcConfigurationError,
    AuthentikOidcFlowError,
};

#[test]
fn csrf_comparison_uses_the_timing_resistant_secret_type_contract() {
    let adapter = include_str!("../src/adapter.rs");
    let workspace = include_str!("../Cargo.toml");
    let bank_workspace = include_str!("../../../Cargo.toml");

    assert!(!adapter.contains("state.secret() !="));
    assert!(adapter.contains("pending.state != callback.state"));
    assert!(workspace.contains("openidconnect.workspace = true"));
    assert!(bank_workspace.contains("timing-resistant-secret-traits"));
}

#[test]
fn malformed_authorization_callbacks_fail_at_the_transport_boundary() {
    assert_callback_denial(
        "",
        "state",
        AuthentikOidcFlowError::InvalidAuthorizationCode,
    );
    assert_callback_denial("code", "", AuthentikOidcFlowError::InvalidState);
    assert_callback_denial(
        "code\ninjection",
        "state",
        AuthentikOidcFlowError::InvalidAuthorizationCode,
    );
    assert_callback_denial(
        "code",
        "state\rinjection",
        AuthentikOidcFlowError::InvalidState,
    );
    assert_callback_denial(
        "x".repeat(2_049),
        "state",
        AuthentikOidcFlowError::InvalidAuthorizationCode,
    );
    assert_callback_denial(
        "code",
        "x".repeat(2_049),
        AuthentikOidcFlowError::InvalidState,
    );
}

#[test]
fn callback_debug_output_discloses_neither_code_nor_state() {
    let callback = AuthentikAuthorizationCallback::new("sensitive-code", "sensitive-state")
        .expect("bounded callback should admit");
    let debug = format!("{callback:?}");
    assert_eq!(debug, "AuthentikAuthorizationCallback { .. }");
    assert!(!debug.contains("sensitive-code"));
    assert!(!debug.contains("sensitive-state"));
}

#[test]
fn named_configuration_builder_rejects_missing_or_invalid_security_fields() {
    let missing_secret = AuthentikOidcConfiguration::builder()
        .issuer("https://issuer.example/application/o/bank/")
        .client_id("bank-client")
        .redirect_url("https://bank.example/callback")
        .introspection_url("https://issuer.example/application/o/introspect/")
        .revocation_url("https://issuer.example/application/o/revoke/")
        .build()
        .expect_err("a confidential client requires its secret");
    assert_eq!(
        missing_secret,
        AuthentikOidcConfigurationError::InvalidClientSecret
    );

    let invalid_introspection = valid_configuration_builder()
        .introspection_url("not a URL")
        .build()
        .expect_err("the online revocation boundary requires a valid endpoint");
    assert_eq!(
        invalid_introspection,
        AuthentikOidcConfigurationError::InvalidIntrospectionUrl
    );

    let insecure_issuer = valid_configuration_builder()
        .issuer("http://issuer.example/application/o/bank/")
        .build()
        .expect_err("the identity authority requires transport security");
    assert_eq!(
        insecure_issuer,
        AuthentikOidcConfigurationError::InvalidIssuer
    );

    let insecure_introspection = valid_configuration_builder()
        .introspection_url("http://issuer.example/application/o/introspect/")
        .build()
        .expect_err("introspection requires transport security");
    assert_eq!(
        insecure_introspection,
        AuthentikOidcConfigurationError::InvalidIntrospectionUrl
    );

    let cross_origin_introspection = valid_configuration_builder()
        .introspection_url("https://attacker.example/application/o/introspect/")
        .build()
        .expect_err("introspection must not disclose credentials across origins");
    assert_eq!(
        cross_origin_introspection,
        AuthentikOidcConfigurationError::InvalidIntrospectionUrl
    );

    let insecure_revocation = valid_configuration_builder()
        .revocation_url("http://issuer.example/application/o/revoke/")
        .build()
        .expect_err("revocation requires transport security");
    assert_eq!(
        insecure_revocation,
        AuthentikOidcConfigurationError::InvalidRevocationUrl
    );

    let cross_origin_revocation = valid_configuration_builder()
        .revocation_url("https://attacker.example/application/o/revoke/")
        .build()
        .expect_err("revocation must not disclose credentials across origins");
    assert_eq!(
        cross_origin_revocation,
        AuthentikOidcConfigurationError::InvalidRevocationUrl
    );
}

#[test]
fn configuration_debug_redacts_the_client_secret() {
    let configuration = valid_configuration_builder()
        .build()
        .expect("complete named configuration should admit");
    let debug = format!("{configuration:?}");
    assert!(!debug.contains("sensitive-client-secret"));
    assert!(debug.contains("<redacted>"));
}

fn valid_configuration_builder() -> bank_http_adapter::AuthentikOidcConfigurationBuilder {
    AuthentikOidcConfiguration::builder()
        .issuer("https://issuer.example/application/o/bank/")
        .client_id("bank-client")
        .client_secret("sensitive-client-secret")
        .redirect_url("https://bank.example/callback")
        .introspection_url("https://issuer.example/application/o/introspect/")
        .revocation_url("https://issuer.example/application/o/revoke/")
}

fn assert_callback_denial(
    code: impl Into<String>,
    state: impl Into<String>,
    expected: AuthentikOidcFlowError,
) {
    let error =
        AuthentikAuthorizationCallback::new(code, state).expect_err("malformed callback must fail");
    assert_eq!(error, expected);
}

#[test]
fn r8_49_http_adapter_deserializes_no_aftermath_authority_and_makes_no_recovery_decision() {
    // Temporary HTTP boundary stays descriptive: no recovery/undo/redo route-local
    // decision vocabulary in the adapter crate.
    let adapter_root = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
    let forbidden = [
        "admit_undo",
        "admit_redo",
        "open_commit_recovery",
        "WorthQueryRecoveryHandle",
        "compensate_recovery",
        "reconcile_recovery",
        "rollback",
        "aftermath",
    ];
    let mut hits = Vec::new();
    for entry in std::fs::read_dir(adapter_root).expect("adapter src") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            let text = std::fs::read_to_string(&path).expect("read");
            for needle in forbidden {
                if text.contains(needle) {
                    hits.push(format!("{}:{needle}", path.display()));
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "HTTP adapter must not carry aftermath/recovery authority: {hits:?}"
    );
}
