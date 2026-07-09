#[test]
fn phase_seven_denies_unsupported_or_incomplete_strategy_claims_before_declaration() {
    use super::tests_support::{admit_phase_five_scope, root_manifest_scope};
    use crate::strategy::S8LayoutStrategyFamily;
    use crate::strategy_registry::S8LayoutAdmissionDenial;
    use crate::strategy_registry::{
        layout_admission_registry, S8LayoutAdmissionRequest, S8LayoutRequestedCapability,
    };
    use crate::{ArtifactFamilyAccessLane, S8StrategyDenial};
    use worth_proof::TransitionOutcome;
    use worth_store_contracts::DurableArtifactFamilyId;
    use worth_store_security::{
        StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
        StoreKeyScope, StoreTenantScope,
    };

    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let (root_lifecycle, root_domain) = root_manifest_scope();
    assert_eq!(
        layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
            page_lifecycle,
            page_domain,
            S8LayoutStrategyFamily::StreamingCursorIndex,
            S8LayoutRequestedCapability::point_lookup(),
            ArtifactFamilyAccessLane::HotPath,
        )),
        TransitionOutcome::Denied(S8LayoutAdmissionDenial::StrategyVocabularyDenied(
            S8StrategyDenial::UnsupportedFamily,
        ))
    );
    assert_eq!(
        layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
            root_lifecycle,
            root_domain,
            S8LayoutStrategyFamily::BaselineBTreeRange,
            S8LayoutRequestedCapability::point_lookup(),
            ArtifactFamilyAccessLane::MaintenancePath,
        )),
        TransitionOutcome::Denied(S8LayoutAdmissionDenial::StrategyVocabularyDenied(
            S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineBTree,
        ))
    );
    assert_eq!(
        layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
            page_lifecycle,
            root_domain,
            S8LayoutStrategyFamily::BaselineBTreeRange,
            S8LayoutRequestedCapability::point_lookup(),
            ArtifactFamilyAccessLane::HotPath,
        )),
        TransitionOutcome::Denied(S8LayoutAdmissionDenial::StrategyVocabularyDenied(
            S8StrategyDenial::FamilyDoesNotMatchKeyDomain,
        ))
    );
    assert_eq!(
        layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
            page_lifecycle,
            page_domain,
            S8LayoutStrategyFamily::ExactScan,
            S8LayoutRequestedCapability::exact_scan(),
            ArtifactFamilyAccessLane::HotPath,
        )),
        TransitionOutcome::Denied(S8LayoutAdmissionDenial::StrategyVocabularyDenied(
            S8StrategyDenial::UnsupportedFamily,
        ))
    );
    assert_eq!(
        layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
            page_lifecycle,
            page_domain,
            S8LayoutStrategyFamily::ManifestTable,
            S8LayoutRequestedCapability::point_lookup(),
            ArtifactFamilyAccessLane::HotPath,
        )),
        TransitionOutcome::Denied(S8LayoutAdmissionDenial::StrategyVocabularyDenied(
            S8StrategyDenial::UnsupportedFamily,
        ))
    );
}

#[test]
fn phase_seven_admission_binds_counter_profiles_and_posture_to_strategy_families() {
    use super::tests_support::{admit_btree_page_strategy, admit_lsm_wal_strategy};
    use crate::{
        ArtifactFamilyAccessLane, S8AccessShapeDetail, S8MaterializationStateClass,
        S8StrategyAmplificationProfile, S8StrategyLocalityProfile, S8StrategyLookupInvariant,
        S8StrategyPublicationInvariant, S8StrategyRebuildSourceRequirement,
    };

    let btree = admit_btree_page_strategy();
    let lsm = admit_lsm_wal_strategy();
    let btree_suite = btree.invariant_suite();
    let lsm_suite = lsm.invariant_suite();

    assert_eq!(
        btree_suite.lookup_invariant(),
        S8StrategyLookupInvariant::SeparatorDirectedLookup
    );
    assert_eq!(
        lsm_suite.publication_invariant(),
        S8StrategyPublicationInvariant::ManifestPublication
    );
    assert!(btree.supports_point_access());
    assert!(btree.supports_range_access());
    assert!(btree.supports_prefix_access());
    assert!(!btree.supports_streaming_access());
    assert!(lsm.supports_point_access());
    assert!(!lsm.supports_range_access());
    assert!(btree.allows_access_lane(ArtifactFamilyAccessLane::HotPath));
    assert!(btree.allows_access_lane(ArtifactFamilyAccessLane::MaintenancePath));
    assert!(!btree.allows_access_lane(ArtifactFamilyAccessLane::VerifierPath));
    assert_eq!(
        btree.locality_profile(),
        S8StrategyLocalityProfile::OrderedPageLocality
    );
    assert_eq!(
        lsm.locality_profile(),
        S8StrategyLocalityProfile::BufferedRunLocality
    );
    assert_eq!(
        btree.amplification_profile(),
        S8StrategyAmplificationProfile::SplitMergeBounded
    );
    assert_eq!(
        lsm.amplification_profile(),
        S8StrategyAmplificationProfile::CompactionWriteAmplified
    );
    assert_eq!(
        btree.rebuild_source_requirement(),
        S8StrategyRebuildSourceRequirement::PhysicalSnapshotReplay
    );
    assert_eq!(
        lsm.rebuild_source_requirement(),
        S8StrategyRebuildSourceRequirement::WalReplay
    );
    assert!(btree.supports_materialization_state(S8MaterializationStateClass::Exact));
    assert!(lsm.supports_materialization_state(S8MaterializationStateClass::Lagged));
    assert!(!lsm.supports_materialization_state(S8MaterializationStateClass::Exact));
    assert!(btree.canonical_key_encoding().is_some());
    assert!(btree.comparator_law().is_some());
    assert!(btree.prefix_law().is_some());
    assert!(btree.range_bound_law().is_some());
    assert!(lsm.canonical_key_encoding().is_some());
    assert!(lsm.comparator_law().is_some());
    assert!(lsm.prefix_law().is_none());
    assert!(lsm.range_bound_law().is_none());
    assert_eq!(
        btree.planned_counter_envelope(),
        None
    );
    assert_eq!(
        lsm.planned_counter_envelope()
            .expect("lsm strategy should declare planned counters")
            .aggregate_profile(),
        lsm.declared_counter_profile()
    );
    assert_eq!(
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::PointLookup)
            .expect("btree point counters should be available")
            .aggregate_profile(),
        crate::S8StrategyCounterProfile::new(1, 0, 0, 1, 1)
    );
    assert_eq!(
        btree.declared_counter_profile(),
        btree_suite.counter_evidence().aggregate_profile()
    );
    assert!(btree
        .planned_counter_envelope_for(S8AccessShapeDetail::RangeLookup(
            crate::S8RangeBasis::CanonicalRangeBounds
        ))
        .is_some());
    assert!(btree
        .planned_counter_envelope_for(S8AccessShapeDetail::PrefixLookup(
            crate::S8PrefixBasis::CanonicalPrefixBounds
        ))
        .is_some());

    let btree_evidence = btree_suite.counter_evidence();
    let lsm_evidence = lsm_suite.counter_evidence();

    assert_eq!(btree_evidence.lookup(), None);
    assert_eq!(
        btree_evidence
            .point_lookup()
            .expect("btree point declarative envelope should exist"),
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::PointLookup)
            .expect("btree point counters should be available")
    );
    assert_eq!(
        btree_evidence
            .range_lookup()
            .expect("btree range declarative envelope should exist"),
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::RangeLookup(
                crate::S8RangeBasis::CanonicalRangeBounds
            ))
            .expect("btree range counters should be available")
    );
    assert_eq!(
        btree_evidence
            .prefix_lookup()
            .expect("btree prefix declarative envelope should exist"),
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::PrefixLookup(
                crate::S8PrefixBasis::CanonicalPrefixBounds
            ))
            .expect("btree prefix counters should be available")
    );
    assert_eq!(
        btree_evidence.publication(),
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::PointLookup)
            .expect("btree point counters should be available")
            .publication()
    );
    assert_eq!(
        btree_evidence.recovery(),
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::PointLookup)
            .expect("btree point counters should be available")
            .recovery()
    );
    assert_eq!(
        lsm_evidence
            .lookup()
            .expect("lsm lookup envelope should remain singular"),
        lsm.planned_counter_envelope()
            .expect("lsm strategy should declare planned counters")
    );
    assert_eq!(
        lsm_evidence.publication(),
        lsm.planned_counter_envelope()
            .expect("lsm strategy should declare planned counters")
            .publication()
    );
    assert_eq!(
        lsm_evidence.recovery(),
        lsm.planned_counter_envelope()
            .expect("lsm strategy should declare planned counters")
            .recovery()
    );
}

#[test]
fn phase_seven_strategy_identity_preserves_family_and_lane_posture() {
    use super::tests_support::admit_phase_five_scope;
    use crate::strategy::S8LayoutStrategyFamily;
    use crate::strategy_registry::{
        layout_admission_registry, S8LayoutAdmissionRequest, S8LayoutRequestedCapability,
    };
    use crate::ArtifactFamilyAccessLane;
    use worth_proof::TransitionOutcome;
    use worth_store_contracts::DurableArtifactFamilyId;
    use worth_store_security::{
        StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
        StoreKeyScope, StoreTenantScope,
    };

    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let (segment_lifecycle, segment_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalSegment,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );

    let page = match layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("page strategy should admit: {outcome:?}"),
    };
    let segment = match layout_admission_registry().admit(S8LayoutAdmissionRequest::new(
        segment_lifecycle,
        segment_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("segment strategy should admit: {outcome:?}"),
    };

    assert_ne!(page.key_domain(), segment.key_domain());
    assert_eq!(page.family(), segment.family());
    assert_eq!(
        page.declared_access_lane(),
        ArtifactFamilyAccessLane::HotPath
    );
}

#[test]
fn phase_seventeen_btree_strategy_counter_surface_requires_shape_specific_lookup_truth() {
    use super::tests_support::admit_btree_page_strategy;
    use crate::{S8AccessShapeDetail, S8PrefixBasis, S8RangeBasis};

    let btree = admit_btree_page_strategy();
    let evidence = btree.invariant_suite().counter_evidence();

    assert_eq!(btree.planned_counter_envelope(), None);
    assert_eq!(evidence.lookup(), None);
    assert_eq!(
        btree.planned_counter_envelope_for(S8AccessShapeDetail::PointLookup),
        evidence.point_lookup()
    );
    assert_eq!(
        evidence
            .range_lookup()
            .expect("range declarative counters should exist"),
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::RangeLookup(
                S8RangeBasis::CanonicalRangeBounds
            ))
            .expect("range counters should be available")
    );
    assert_eq!(
        evidence
            .prefix_lookup()
            .expect("prefix declarative counters should exist"),
        btree
            .planned_counter_envelope_for(S8AccessShapeDetail::PrefixLookup(
                S8PrefixBasis::CanonicalPrefixBounds
            ))
            .expect("prefix counters should be available")
    );
    assert_eq!(
        evidence.aggregate_profile(),
        crate::S8StrategyCounterProfile::new(1, 1, 0, 1, 1)
    );
}
