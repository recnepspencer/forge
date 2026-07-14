use worth_store_buffer_pool::AllocationScope;
use worth_store_physical_certification::{
    lower_physical_simulation_plan, physical_scenario,
    reject_hostile_counter_evidence_for_readmission, CounterContractKind, CounterExpectationKind,
    CounterMismatchEvidence, ForbiddenShortcutSet, HostileCounterEvidenceRow,
    HostileResourceEnvelopeObservation, PhysicalResourceEnvelope, PhysicalScenarioActor,
    PhysicalScenarioExpectation, PhysicalScenarioIntent, PhysicalScenarioSchedule,
    PhysicalSimulationCapabilitySet, PhysicalSimulationPlan, PhysicalSimulationProfile,
    PhysicalSimulationProfileSet, PhysicalSimulationScenarioFamily, SimulationEvidencePolicy,
    SimulationPlanDenial, SimulationPlanningContext, SupportedObserverSet,
    SupportedOracleFamilySet,
};
use worth_store_test_support::{
    admitted_developer_smoke_driver_contracts, NativeStoreAspectFixture,
};

const ALL_PROFILES: [PhysicalSimulationProfile; 5] = [
    PhysicalSimulationProfile::DeveloperSmoke,
    PhysicalSimulationProfile::CiCertification,
    PhysicalSimulationProfile::LocalSoak,
    PhysicalSimulationProfile::ReleaseCertification,
    PhysicalSimulationProfile::HardwareQualification,
];

#[test]
fn resource_envelopes_are_profile_scoped_and_ordered_by_profile() {
    let envelopes = ALL_PROFILES.map(PhysicalResourceEnvelope::for_profile);

    for (profile, envelope) in ALL_PROFILES.into_iter().zip(envelopes) {
        assert_eq!(envelope.profile(), profile);
    }
    for pair in envelopes.windows(2) {
        let [lower, higher] = pair else {
            unreachable!("windows(2) always yields two envelopes");
        };
        assert!(
            lower
                .allocation()
                .budget(AllocationScope::Foreground)
                .as_bytes()
                < higher
                    .allocation()
                    .budget(AllocationScope::Foreground)
                    .as_bytes()
        );
        assert!(lower.resident_bytes().as_bytes() < higher.resident_bytes().as_bytes());
        assert!(lower.max_pinned_pages() < higher.max_pinned_pages());
        assert!(lower.max_dirty_pages() < higher.max_dirty_pages());
        assert!(lower.io_queue().max_queue_depth() < higher.io_queue().max_queue_depth());
        assert!(
            lower.io_queue().max_interference_events()
                < higher.io_queue().max_interference_events()
        );
    }
}

#[test]
fn profile_envelope_participates_in_plan_identity() {
    let plans = ALL_PROFILES.map(lower_for_profile);

    for (profile, plan) in ALL_PROFILES.into_iter().zip(plans.iter()) {
        assert_eq!(plan.resource_envelope().profile(), profile);
        assert_eq!(
            plan.counter_contracts()
                .require(CounterContractKind::ProfileResourceEnvelope)
                .unwrap()
                .expectation()
                .kind(),
            CounterExpectationKind::ProfileScoped
        );
    }
    for left in 0..plans.len() {
        for right in left + 1..plans.len() {
            assert_ne!(plans[left].identity(), plans[right].identity());
        }
    }
}

#[test]
fn every_mismatched_resource_envelope_profile_denies_before_plan_identity() {
    for profile in ALL_PROFILES {
        for mismatched_profile in ALL_PROFILES {
            if profile == mismatched_profile {
                continue;
            }
            let denial = lower_physical_simulation_plan(
                physical_isolation_scenario("store.physical.s45.phase8.profile-mismatch-matrix"),
                complete_context_for_profile(profile).with_resource_envelope(
                    PhysicalResourceEnvelope::for_profile(mismatched_profile),
                ),
            )
            .expect_err("profile envelope from another profile must not satisfy the plan");

            assert_eq!(
                denial,
                SimulationPlanDenial::ResourceEnvelopeProfileMismatch {
                    expected: profile,
                    actual: mismatched_profile,
                }
            );
        }
    }
}

#[test]
fn evidence_profile_mismatch_denies_after_plan_lowering_for_every_profile_pair() {
    for profile in ALL_PROFILES {
        let plan = lower_for_profile(profile);
        for mismatched_profile in ALL_PROFILES {
            if profile == mismatched_profile {
                continue;
            }
            let denial = reject_hostile_counter_evidence_for_readmission(
                &plan,
                hostile_satisfied_rows(&plan),
                hostile_observation_for_profile(&plan, mismatched_profile),
            )
            .unwrap_err();

            assert_eq!(
                denial,
                CounterMismatchEvidence::ProfileMismatch {
                    expected: profile,
                    actual: mismatched_profile,
                }
            );
        }
    }
}

#[test]
fn mismatched_resource_envelope_profile_denies_before_plan_identity() {
    let denial = lower_physical_simulation_plan(
        physical_isolation_scenario("store.physical.s45.phase8.profile-mismatch"),
        complete_context_for_profile(PhysicalSimulationProfile::DeveloperSmoke)
            .with_resource_envelope(PhysicalResourceEnvelope::for_profile(
                PhysicalSimulationProfile::CiCertification,
            )),
    )
    .expect_err("profile envelope from another profile must not satisfy the plan");

    assert_eq!(
        denial,
        SimulationPlanDenial::ResourceEnvelopeProfileMismatch {
            expected: PhysicalSimulationProfile::DeveloperSmoke,
            actual: PhysicalSimulationProfile::CiCertification,
        }
    );
}

#[test]
fn evidence_profile_mismatch_denies_after_plan_lowering() {
    let plan = lower_for_profile(PhysicalSimulationProfile::DeveloperSmoke);
    let denial = reject_hostile_counter_evidence_for_readmission(
        &plan,
        hostile_satisfied_rows(&plan),
        hostile_observation_for_profile(&plan, PhysicalSimulationProfile::CiCertification),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        CounterMismatchEvidence::ProfileMismatch {
            expected: PhysicalSimulationProfile::DeveloperSmoke,
            actual: PhysicalSimulationProfile::CiCertification,
        }
    );
}

fn lower_for_profile(profile: PhysicalSimulationProfile) -> PhysicalSimulationPlan {
    lower_physical_simulation_plan(
        physical_isolation_scenario("store.physical.s45.phase8.profile-envelope"),
        complete_context_for_profile(profile),
    )
    .unwrap()
}

fn hostile_observation_for_profile(
    plan: &PhysicalSimulationPlan,
    profile: PhysicalSimulationProfile,
) -> HostileResourceEnvelopeObservation {
    let envelope = plan.resource_envelope();
    HostileResourceEnvelopeObservation::new(
        profile,
        envelope
            .allocation()
            .budget(AllocationScope::Foreground)
            .as_bytes(),
        envelope.resident_bytes().as_bytes(),
        u64::from(envelope.max_pinned_pages()),
        u64::from(envelope.max_dirty_pages()),
        u64::from(envelope.io_queue().max_queue_depth()),
        u64::from(envelope.io_queue().max_interference_events()),
    )
}

fn hostile_satisfied_rows(plan: &PhysicalSimulationPlan) -> Vec<HostileCounterEvidenceRow> {
    plan.counter_contracts()
        .iter()
        .map(|contract| {
            let observed_count = match contract.expectation().kind() {
                CounterExpectationKind::Zero => 0,
                CounterExpectationKind::Positive => 1,
                CounterExpectationKind::Exact => contract.expectation().value().unwrap(),
                CounterExpectationKind::Monotonic => 0,
                CounterExpectationKind::Bounded => 1,
                CounterExpectationKind::ProfileScoped => 1,
            };
            HostileCounterEvidenceRow::new(
                contract.kind(),
                contract.expectation().kind(),
                observed_count,
            )
        })
        .collect()
}

fn complete_context_for_profile(profile: PhysicalSimulationProfile) -> SimulationPlanningContext {
    SimulationPlanningContext::for_profile(profile)
        .with_supported_profiles(PhysicalSimulationProfileSet::all())
        .with_capabilities(
            PhysicalSimulationCapabilitySet::physical_isolation_readiness_shape_probe(),
        )
        .with_driver_contracts(admitted_developer_smoke_driver_contracts().unwrap())
        .with_supported_observers(SupportedObserverSet::all_for_developer_smoke())
        .with_supported_oracle_families(SupportedOracleFamilySet::all_for_developer_smoke())
        .with_evidence_policy(SimulationEvidencePolicy::minimal_replayable())
        .with_forbidden_shortcuts(ForbiddenShortcutSet::physical_certification_baseline())
}

fn physical_isolation_scenario(
    name: &str,
) -> worth_store_physical_certification::CertifiedPhysicalScenario {
    physical_scenario(name)
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(
            NativeStoreAspectFixture::segment_header("phase8-profile", 8)
                .boundary_fact()
                .clone(),
        )
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .unwrap()
}
