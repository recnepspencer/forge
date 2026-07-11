use forge_store_offline_verifier::{
    OfflineRepairBlastRadiusObservation, OfflineRepairEvidenceKind,
};
use forge_store_security::{
    repair_blast_radius_authenticity, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreRawSecurityScopeDeclaration, StoreRepairPhysicalRegionDeclaration,
    StoreSecurityScopeAdmissionDenial, StoreSecurityScopeAdmissionRebindRequired,
    StoreSecurityScopeAdmissionStale, StoreTenantScope,
};

use super::{current_authority, current_authority_for_boundary};
use crate::{RepairBlastRadiusDeclaration, RepairBlastRadiusDenial, RepairBlastRadiusPlan};
use forge_store_contracts::ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY;

#[test]
fn admitted_repair_read_plan_consumes_tenant_key_and_region_scope() {
    let authority = current_authority("phase8.repair-admitted");
    let declaration = native_declaration(&authority, "repair-region-a").unwrap();
    let region = declaration.physical_region().clone();
    let readiness = RepairBlastRadiusPlan::declare(declaration)
        .admit_with_store_blast_radius()
        .expect("current repair blast radius should admit");
    let read_plan = readiness
        .prepare_repair_read(region)
        .expect("admitted region should prepare repair read");

    assert_eq!(
        read_plan.security_scope().key_scope(),
        StoreKeyScope::RepairScopeEnvelope
    );
    assert_eq!(
        read_plan.security_scope().tenant_scope(),
        StoreTenantScope::RepairBlastRadius
    );
    assert_eq!(read_plan.counters().repair_admitted(), 1);
    assert_eq!(read_plan.counters().repair_reads_prepared(), 1);
}

#[test]
fn cross_scope_region_denies_before_repair_read_plan_exists() {
    let authority = current_authority("phase8.cross-region");
    let readiness = admitted_readiness(&authority, "repair-region-a");
    let requested_region = native_declaration(&authority, "repair-region-b")
        .unwrap()
        .physical_region()
        .clone();
    let denial = readiness
        .prepare_repair_read(requested_region)
        .expect_err("different region should not prepare repair read");

    if let RepairBlastRadiusDenial::CrossScopePhysicalRegion { counters, .. } = denial {
        assert_eq!(counters.cross_scope_region_rejections(), 1);
        assert_eq!(counters.repair_reads_prepared(), 0);
        assert_eq!(counters.repair_denied(), 1);
    } else {
        panic!("cross-region request should use region denial");
    }
}

#[test]
fn stale_key_denies_before_repair_declaration_exists() {
    let authority = current_authority("phase8.stale-key");
    let denial = RepairBlastRadiusDeclaration::native(
        &authority,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        StoreKeyVersionPosture::Stale,
        StoreCustodyPosture::InternalStoreCustody,
    )
    .expect_err("stale key posture should deny repair declaration");

    if let RepairBlastRadiusDenial::SecurityScopeAdmissionStale { source, counters } = denial {
        assert_eq!(
            source,
            StoreSecurityScopeAdmissionStale::StaleKeyVersionPosture(StoreKeyVersionPosture::Stale)
        );
        assert_eq!(counters.stale_key_version_rejections(), 1);
        assert_eq!(counters.key_rebind_required_rejections(), 0);
        assert_eq!(counters.repair_denied(), 1);
        assert_eq!(counters.repair_reads_prepared(), 0);
    } else {
        panic!("stale key should deny through security-scope admission");
    }
}

#[test]
fn rebind_required_key_denies_without_collapsing_to_denied_key_version() {
    let authority = current_authority("phase8.rebind-key");
    let denial = RepairBlastRadiusDeclaration::native(
        &authority,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        StoreKeyVersionPosture::RebindRequired,
        StoreCustodyPosture::InternalStoreCustody,
    )
    .expect_err("rebind-required key posture should deny repair declaration");

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
        assert_eq!(counters.stale_key_version_rejections(), 0);
        assert_eq!(counters.repair_denied(), 1);
        assert_eq!(counters.repair_reads_prepared(), 0);
    } else {
        panic!("rebind-required key should preserve rebind outcome topology");
    }
}

#[test]
fn missing_authenticity_and_custody_deny_offline_report_before_repair_declaration() {
    let authority = current_authority("phase8.missing-metadata");
    let missing_auth = offline_observation(
        &authority,
        None,
        Some(StoreCustodyPosture::InternalStoreCustody),
        StoreTenantScope::RepairBlastRadius,
        "repair-region-a",
        OfflineRepairEvidenceKind::QuarantineReport,
    );
    let denial = RepairBlastRadiusDeclaration::from_offline_observation(&authority, missing_auth)
        .expect_err("missing authenticity should deny repair readiness");
    assert_security_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::MissingAuthenticityRequirement,
        1,
        0,
    );

    let missing_custody = offline_observation(
        &authority,
        Some(repair_blast_radius_authenticity()),
        None,
        StoreTenantScope::RepairBlastRadius,
        "repair-region-a",
        OfflineRepairEvidenceKind::QuarantineReport,
    );
    let denial =
        RepairBlastRadiusDeclaration::from_offline_observation(&authority, missing_custody)
            .expect_err("missing custody should deny repair readiness");
    assert_security_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::MissingCustodyPosture,
        0,
        1,
    );
}

#[test]
fn unavailable_custody_denies_before_repair_declaration_exists() {
    let authority = current_authority("phase8.custody-unavailable");
    let denial = RepairBlastRadiusDeclaration::native(
        &authority,
        StoreRepairPhysicalRegionDeclaration::raw("repair-region-a"),
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::CustodyUnavailable,
    )
    .expect_err("unavailable custody should deny repair declaration");

    assert_security_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::UnavailableCustodyPosture,
        0,
        1,
    );
}

#[test]
fn offline_report_from_replay_boundary_denies_even_with_copied_region_text() {
    let source =
        current_authority_for_boundary("phase8.replay-source", ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY);
    let target = current_authority("phase8.replay-target");
    let observation = offline_observation(
        &source,
        Some(repair_blast_radius_authenticity()),
        Some(StoreCustodyPosture::InternalStoreCustody),
        StoreTenantScope::RepairBlastRadius,
        "repair-region-a",
        OfflineRepairEvidenceKind::RepairReadCloseout,
    );
    let denial = RepairBlastRadiusDeclaration::from_offline_observation(&target, observation)
        .expect_err("copied region text from another physical boundary must deny");

    assert_security_denial(
        denial,
        StoreSecurityScopeAdmissionDenial::WrongPhysicalSecurityScope,
        0,
        0,
    );
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
    authenticity: Option<forge_store_security::StoreAuthenticityRequirement>,
    custody: Option<StoreCustodyPosture>,
    tenant: StoreTenantScope,
    region_id: &str,
    evidence_kind: OfflineRepairEvidenceKind,
) -> OfflineRepairBlastRadiusObservation {
    let raw = StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        authority.physical_witness(),
        StoreKeyScope::RepairScopeEnvelope,
        StoreKeyVersionPosture::Current,
        tenant,
        authenticity,
        custody,
    );
    OfflineRepairBlastRadiusObservation::from_offline_repair_report(
        raw,
        StoreRepairPhysicalRegionDeclaration::raw(region_id),
        evidence_kind,
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
        assert_eq!(counters.repair_denied(), 1);
        assert_eq!(counters.repair_reads_prepared(), 0);
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
