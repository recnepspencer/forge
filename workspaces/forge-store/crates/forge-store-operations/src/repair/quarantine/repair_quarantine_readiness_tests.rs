use forge_store_offline_verifier::{
    OfflineRepairBlastRadiusObservation, OfflineRepairEvidenceKind,
};
use forge_store_security::{
    repair_blast_radius_authenticity, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRawSecurityScopeDeclaration, StoreRepairPhysicalRegionDeclaration,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionStale, StoreTenantScope,
};

use crate::repair::blast_radius::current_authority;
use crate::{
    RepairBlastRadiusDeclaration, RepairBlastRadiusDenial, RepairBlastRadiusPlan,
    RepairQuarantineScopePreservation,
};

#[test]
fn quarantine_report_preserves_scope_only_after_repair_admission() {
    let authority = current_authority("phase8.quarantine");
    let readiness = admitted_readiness(&authority, "repair-region-a");
    let observation = offline_observation(
        &authority,
        StoreKeyVersionPosture::Current,
        Some(repair_blast_radius_authenticity()),
        Some(StoreCustodyPosture::InternalStoreCustody),
        StoreTenantScope::RepairBlastRadius,
    );
    let preserved = RepairQuarantineScopePreservation::preserve_from_admitted_readiness(
        &authority,
        readiness,
        &observation,
    )
    .expect("matching quarantine report should preserve admitted repair scope");

    assert_eq!(
        preserved.security_scope().tenant_scope(),
        StoreTenantScope::RepairBlastRadius
    );
    assert_eq!(preserved.counters().quarantine_preserved_scope(), 1);
}

#[test]
fn quarantine_preservation_rejects_stale_and_rebind_key_posture() {
    let authority = current_authority("phase8.quarantine-key-posture");
    let stale = offline_observation(
        &authority,
        StoreKeyVersionPosture::Stale,
        Some(repair_blast_radius_authenticity()),
        Some(StoreCustodyPosture::InternalStoreCustody),
        StoreTenantScope::RepairBlastRadius,
    );
    let stale_denial = RepairQuarantineScopePreservation::preserve_from_admitted_readiness(
        &authority,
        admitted_readiness(&authority, "repair-region-a"),
        &stale,
    )
    .expect_err("stale quarantine report must not preserve admitted scope");
    assert_stale_denial(stale_denial);

    let rebind = offline_observation(
        &authority,
        StoreKeyVersionPosture::RebindRequired,
        Some(repair_blast_radius_authenticity()),
        Some(StoreCustodyPosture::InternalStoreCustody),
        StoreTenantScope::RepairBlastRadius,
    );
    let rebind_denial = RepairQuarantineScopePreservation::preserve_from_admitted_readiness(
        &authority,
        admitted_readiness(&authority, "repair-region-a"),
        &rebind,
    )
    .expect_err("rebind-required quarantine report must preserve rebind topology");
    assert_rebind_denial(rebind_denial);
}

#[test]
fn quarantine_preservation_rejects_missing_authenticity_and_custody() {
    let authority = current_authority("phase8.quarantine-missing-metadata");
    let missing_auth = offline_observation(
        &authority,
        StoreKeyVersionPosture::Current,
        None,
        Some(StoreCustodyPosture::InternalStoreCustody),
        StoreTenantScope::RepairBlastRadius,
    );
    let denial = RepairQuarantineScopePreservation::preserve_from_admitted_readiness(
        &authority,
        admitted_readiness(&authority, "repair-region-a"),
        &missing_auth,
    )
    .expect_err("quarantine report without authenticity must deny");
    assert_security_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement,
        1,
        0,
    );

    let missing_custody = offline_observation(
        &authority,
        StoreKeyVersionPosture::Current,
        Some(repair_blast_radius_authenticity()),
        None,
        StoreTenantScope::RepairBlastRadius,
    );
    let denial = RepairQuarantineScopePreservation::preserve_from_admitted_readiness(
        &authority,
        admitted_readiness(&authority, "repair-region-a"),
        &missing_custody,
    )
    .expect_err("quarantine report without custody must deny");
    assert_security_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::MissingCustodyPosture,
        0,
        1,
    );
}

#[test]
fn copied_quarantine_region_fields_do_not_preserve_wrong_raw_scope() {
    let authority = current_authority("phase8.quarantine-copy");
    let readiness = admitted_readiness(&authority, "repair-region-a");
    let observation = offline_observation(
        &authority,
        StoreKeyVersionPosture::Current,
        Some(repair_blast_radius_authenticity()),
        Some(StoreCustodyPosture::InternalStoreCustody),
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let denial = RepairQuarantineScopePreservation::preserve_from_admitted_readiness(
        &authority,
        readiness,
        &observation,
    )
    .expect_err("copied region text with wrong raw scope must deny");

    assert!(matches!(
        denial,
        RepairBlastRadiusDenial::SecurityScopeAdmissionDenied {
            source: StoreSecurityScopeAdmissionDenial::WrongTenantScope,
            ..
        }
    ));
}

fn admitted_readiness(
    authority: &forge_store_authority::StoreCurrentAuthorityWitness,
    region_id: &str,
) -> crate::RepairBlastRadiusReadiness {
    RepairBlastRadiusPlan::declare(native_declaration(authority, region_id).unwrap())
        .admit_with_store_blast_radius()
        .expect("repair readiness should admit")
}

fn native_declaration(
    authority: &forge_store_authority::StoreCurrentAuthorityWitness,
    region_id: &str,
) -> Result<RepairBlastRadiusDeclaration, RepairBlastRadiusDenial> {
    RepairBlastRadiusDeclaration::native(
        authority,
        StoreRepairPhysicalRegionDeclaration::raw(region_id),
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn offline_observation(
    authority: &forge_store_authority::StoreCurrentAuthorityWitness,
    key_posture: StoreKeyVersionPosture,
    authenticity: Option<forge_store_security::StoreAuthenticityRequirement>,
    custody: Option<StoreCustodyPosture>,
    tenant: StoreTenantScope,
) -> OfflineRepairBlastRadiusObservation {
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::RepairScopeEnvelope,
        key_posture,
        tenant,
        authenticity,
        custody,
    );
    OfflineRepairBlastRadiusObservation::from_offline_repair_report(
        raw,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        OfflineRepairEvidenceKind::QuarantineReport,
    )
    .expect("offline report should observe raw repair declaration")
}

fn assert_security_denial(
    denial: RepairBlastRadiusDenial,
    expected: StoreSecurityScopeAdmissionDenial,
    missing_authenticity: u64,
    unavailable_custody: u64,
) {
    if let RepairBlastRadiusDenial::SecurityScopeAdmissionDenied { source, counters } = denial {
        assert_eq!(source, expected);
        assert_eq!(counters.repair_reads_prepared(), 0);
        assert_eq!(counters.quarantine_preserved_scope(), 0);
        assert_eq!(
            counters.missing_authenticity_rejections(),
            missing_authenticity
        );
        assert_eq!(
            counters.custody_unavailable_rejections(),
            unavailable_custody
        );
    } else {
        panic!("expected security-scope admission denial");
    }
}

fn assert_stale_denial(denial: RepairBlastRadiusDenial) {
    if let RepairBlastRadiusDenial::SecurityScopeAdmissionStale { source, counters } = denial {
        assert_eq!(
            source,
            StoreSecurityScopeAdmissionStale::StaleKeyVersionPosture(StoreKeyVersionPosture::Stale)
        );
        assert_eq!(counters.stale_key_version_rejections(), 1);
        assert_eq!(counters.quarantine_preserved_scope(), 0);
        assert_eq!(counters.repair_reads_prepared(), 0);
    } else {
        panic!("expected stale security-scope admission denial");
    }
}

fn assert_rebind_denial(denial: RepairBlastRadiusDenial) {
    if let RepairBlastRadiusDenial::SecurityScopeAdmissionRebindRequired { source, counters } =
        denial
    {
        assert_eq!(
            source,
            StoreSecurityScopeAdmissionRebindRequired::KeyVersionRebindRequired(
                StoreKeyVersionPosture::RebindRequired
            )
        );
        assert_eq!(counters.key_rebind_required_rejections(), 1);
        assert_eq!(counters.quarantine_preserved_scope(), 0);
        assert_eq!(counters.repair_reads_prepared(), 0);
    } else {
        panic!("expected rebind-required security-scope admission denial");
    }
}
