use crate::scope::security_scope_test_support::{current_authority, platform_request};
use crate::{
    admit_store_security_scope, deny_missing_store_security_scope, propagate_store_security_scope,
    StoreCustodyPosture, StoreKeyVersionPosture, StoreLegacySecurityPosture, StoreSecurityMetadata,
    StoreSecurityScopePropagationDenialKind, StoreSecurityScopePropagationSite, StoreTenantScope,
};
use worth_proof::TransitionOutcome;

#[test]
fn exact_physical_security_scope_propagates_with_preservation_counter() {
    let metadata = current_metadata(StoreKeyVersionPosture::Current);

    let outcome = propagate_store_security_scope(
        metadata,
        metadata,
        StoreSecurityScopePropagationSite::StableReadProtection,
    );

    match outcome {
        TransitionOutcome::Success(witness) => {
            assert_eq!(witness.metadata(), metadata);
            assert_eq!(witness.counters().preserved(), 1);
            assert_eq!(witness.counters().drifted(), 0);
        }
        other => panic!("matching metadata should propagate: {other:?}"),
    }
}

#[test]
fn tenant_scope_drift_is_physical_security_denial() {
    let expected = current_metadata(StoreKeyVersionPosture::Current);
    let observed = replace_tenant_for_test(expected, StoreTenantScope::StoreInternal);

    let outcome = propagate_store_security_scope(
        expected,
        observed,
        StoreSecurityScopePropagationSite::LogicalDecodeEntry,
    );

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.kind(),
                StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode
            );
            assert_eq!(denial.counters().drifted(), 1);
        }
        other => panic!("drift must deny before logical decode: {other:?}"),
    }
}

#[test]
fn stale_key_version_is_not_preserved_scope() {
    let current = current_metadata(StoreKeyVersionPosture::Current);
    let stale = current_metadata(StoreKeyVersionPosture::Stale);

    let outcome = propagate_store_security_scope(
        current,
        stale,
        StoreSecurityScopePropagationSite::RecoveryAdmission,
    );

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.kind(),
                StoreSecurityScopePropagationDenialKind::StalePropagatedSecurityScope
            );
            assert_eq!(denial.counters().stale(), 1);
        }
        other => panic!("stale key posture must deny: {other:?}"),
    }
}

#[test]
fn missing_scope_denial_has_exact_counter() {
    let denial =
        deny_missing_store_security_scope(StoreSecurityScopePropagationSite::StableReadProtection);

    assert_eq!(
        denial.kind(),
        StoreSecurityScopePropagationDenialKind::MissingPropagatedSecurityScope
    );
    assert_eq!(denial.counters().missing(), 1);
}

fn current_metadata(key_version: StoreKeyVersionPosture) -> StoreSecurityMetadata {
    StoreSecurityMetadata::from_current_security_scope(
        admitted_witnesses("propagation-current").witnesses(),
        key_version,
        StoreLegacySecurityPosture::NativeScoped,
    )
}

fn admitted_witnesses(identity: &str) -> crate::StoreAdmittedSecurityScope {
    let authority = current_authority(identity, "platform-page");
    match admit_store_security_scope(platform_request(
        &authority,
        StoreKeyVersionPosture::Current,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        other => panic!("security scope should admit: {other:?}"),
    }
}

#[test]
fn unsupported_custody_denies_as_unsupported_propagated_scope() {
    let expected = current_metadata(StoreKeyVersionPosture::Current);
    let unsupported = StoreSecurityMetadata::from_current_security_scope(
        admitted_witnesses("unsupported-custody").witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let unsupported =
        replace_custody_for_test(unsupported, StoreCustodyPosture::CustodyUnsupported);

    let outcome = propagate_store_security_scope(
        expected,
        unsupported,
        StoreSecurityScopePropagationSite::StableReadProtection,
    );

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.kind(),
                StoreSecurityScopePropagationDenialKind::UnsupportedPropagatedSecurityScope
            );
            assert_eq!(denial.counters().unsupported(), 1);
        }
        other => panic!("unsupported custody must deny: {other:?}"),
    }
}

fn replace_custody_for_test(
    metadata: StoreSecurityMetadata,
    custody: StoreCustodyPosture,
) -> StoreSecurityMetadata {
    StoreSecurityMetadata::from_scope_parts(
        metadata.key_scope(),
        metadata.tenant_scope(),
        metadata.authenticity_requirement(),
        custody,
        metadata.legacy_posture(),
        metadata.key_version_posture(),
    )
}

fn replace_tenant_for_test(
    metadata: StoreSecurityMetadata,
    tenant: StoreTenantScope,
) -> StoreSecurityMetadata {
    StoreSecurityMetadata::from_scope_parts(
        metadata.key_scope(),
        tenant,
        metadata.authenticity_requirement(),
        metadata.custody_posture(),
        metadata.legacy_posture(),
        metadata.key_version_posture(),
    )
}
