use forge_store_aspect_native::StoreAspectBoundaryFact;
use forge_store_physical_certification::{
    physical_scenario, CertifiedPhysicalScenario, PhysicalScenarioActor,
    PhysicalScenarioDefinitionDenial, PhysicalScenarioExpectation, PhysicalScenarioFault,
    PhysicalScenarioIntent, PhysicalScenarioNonClaim, PhysicalScenarioSchedule,
    PhysicalSimulationScenarioFamily,
};
use forge_store_test_support::NativeStoreAspectFixture;

#[test]
fn golden_path_authoring_lowers_into_canonical_native_scenario_identity() {
    let first =
        physical_isolation_readiness_shape_scenario_with_actor_order(["reclaimer", "reader"]);
    let second =
        physical_isolation_readiness_shape_scenario_with_actor_order(["reader", "reclaimer"]);

    assert_eq!(first.identity(), second.identity());
    assert_eq!(first.definition(), second.definition());
    assert!(first.identity().canonical_basis_entry_count() > 0);
    assert_ne!(first.identity().digest_bytes(), &[0; 32]);
    assert_eq!(first.definition().fixture_set().fixtures().len(), 1);
    assert_eq!(first.definition().actor_set().actors().len(), 2);
}

#[test]
fn authoring_label_does_not_participate_in_scenario_identity() {
    let first = physical_isolation_readiness_shape_scenario_with_label("display.label.one");
    let second = physical_isolation_readiness_shape_scenario_with_label("display.label.two");

    assert_eq!(first.identity(), second.identity());
    assert_ne!(first.definition().label(), second.definition().label());
}

#[test]
fn scenario_identity_changes_when_native_fixture_identity_changes() {
    let first_fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    let second_fixture = NativeStoreAspectFixture::segment_header("beta", 7);

    let first = physical_isolation_readiness_shape_scenario(first_fixture);
    let second = physical_isolation_readiness_shape_scenario(second_fixture);

    assert_ne!(first.identity(), second.identity());
}

#[test]
fn scenario_identity_changes_for_each_native_meaning_field() {
    let baseline = scenario_from_parts(baseline_scenario_parts());

    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            family: PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood,
            ..baseline_scenario_parts()
        }),
        "family",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            intent: PhysicalScenarioIntent::ForbiddenShortcutRejectionShape,
            ..baseline_scenario_parts()
        }),
        "intent",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            fixture: NativeStoreAspectFixture::segment_header("beta", 7),
            ..baseline_scenario_parts()
        }),
        "fixture",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            actors: [
                PhysicalScenarioActor::maintenance_reclaimer("writer"),
                PhysicalScenarioActor::foreground_reader("reader"),
            ],
            ..baseline_scenario_parts()
        }),
        "actor id",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            actors: [
                PhysicalScenarioActor::foreground_reader("reclaimer"),
                PhysicalScenarioActor::foreground_reader("reader"),
            ],
            ..baseline_scenario_parts()
        }),
        "actor role",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            yieldpoint: "root-publication-after-observe",
            ..baseline_scenario_parts()
        }),
        "yieldpoint",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            fault: PhysicalScenarioFault::future_extension_slot(),
            ..baseline_scenario_parts()
        }),
        "fault",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            expectation: PhysicalScenarioExpectation::shortcut_rejection_dogfood(),
            ..baseline_scenario_parts()
        }),
        "expectation",
    );
    assert_identity_differs(
        &baseline,
        scenario_from_parts(ScenarioParts {
            expectation:
                PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape()
                    .with_future_extension_non_claim(),
            ..baseline_scenario_parts()
        }),
        "non-claim set",
    );
}

#[test]
fn fixture_set_order_does_not_participate_in_scenario_identity() {
    let alpha = NativeStoreAspectFixture::segment_header("alpha", 7);
    let beta = NativeStoreAspectFixture::segment_header("beta", 11);

    let first = physical_isolation_readiness_shape_scenario_from_fixtures([
        alpha.boundary_fact().clone(),
        beta.boundary_fact().clone(),
    ]);
    let second = physical_isolation_readiness_shape_scenario_from_fixtures([
        beta.boundary_fact().clone(),
        alpha.boundary_fact().clone(),
    ]);

    assert_eq!(first.identity(), second.identity());
    assert_eq!(first.definition(), second.definition());
}

#[test]
fn scenario_authoring_denies_duplicate_native_fixture_before_certification() {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    let denial = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(fixture.boundary_fact().clone())
        .fixture(fixture.boundary_fact().clone())
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect_err("duplicate native fixture cannot certify");

    assert_eq!(
        denial,
        PhysicalScenarioDefinitionDenial::DuplicateAspectNativeFixture
    );
}

#[test]
fn scenario_authoring_denies_missing_actor_before_certification() {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    let denial = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(fixture.boundary_fact().clone())
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect_err("scenario without actor cannot certify");

    assert_eq!(denial, PhysicalScenarioDefinitionDenial::MissingActor);
}

#[test]
fn scenario_authoring_denies_unnamed_actor_before_certification() {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    let denial = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(fixture.boundary_fact().clone())
        .actor(PhysicalScenarioActor::foreground_reader("  "))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect_err("scenario with unnamed actor cannot certify");

    assert_eq!(denial, PhysicalScenarioDefinitionDenial::UnnamedActorId);
}

#[test]
fn scenario_authoring_denies_duplicate_actor_id_before_certification() {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    let denial = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(fixture.boundary_fact().clone())
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect_err("duplicate actor id cannot certify");

    assert_eq!(denial, PhysicalScenarioDefinitionDenial::DuplicateActorId);
}

#[test]
fn scenario_authoring_denies_unnamed_yieldpoint_before_certification() {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    let denial = physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(fixture.boundary_fact().clone())
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(""))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect_err("scenario with unnamed yieldpoint cannot certify");

    assert_eq!(
        denial,
        PhysicalScenarioDefinitionDenial::UnnamedProductionBoundaryYieldpoint
    );
}

#[test]
fn physical_isolation_readiness_shape_probe_carries_explicit_non_claim_evidence() {
    let scenario = physical_isolation_readiness_shape_scenario(
        NativeStoreAspectFixture::segment_header("alpha", 7),
    );

    assert!(scenario
        .definition()
        .expectation()
        .non_claims()
        .contains(&PhysicalScenarioNonClaim::NoPhysicalIsolationCorrectnessClaim));
}

fn physical_isolation_readiness_shape_scenario_from_fixtures(
    fixtures: [StoreAspectBoundaryFact; 2],
) -> CertifiedPhysicalScenario {
    let [first_fixture, second_fixture] = fixtures;
    physical_scenario("store.physical.s5.readiness")
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(first_fixture)
        .fixture(second_fixture)
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect("native scenario should certify")
}

fn physical_isolation_readiness_shape_scenario_with_actor_order(
    actor_order: [&str; 2],
) -> CertifiedPhysicalScenario {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    actor_order
        .into_iter()
        .fold(
            physical_scenario("store.physical.s5.readiness")
                .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
                .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
                .fixture(fixture.boundary_fact().clone()),
            |builder, actor_id| {
                if actor_id == "reclaimer" {
                    builder.actor(PhysicalScenarioActor::maintenance_reclaimer(actor_id))
                } else {
                    builder.actor(PhysicalScenarioActor::foreground_reader(actor_id))
                }
            },
        )
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .fault(PhysicalScenarioFault::no_fault())
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect("native scenario should certify")
}

fn physical_isolation_readiness_shape_scenario(
    fixture: NativeStoreAspectFixture,
) -> CertifiedPhysicalScenario {
    physical_isolation_readiness_shape_scenario_from_label_and_fixture(
        "store.physical.s5.readiness",
        fixture,
    )
}

fn physical_isolation_readiness_shape_scenario_with_label(
    label: &str,
) -> CertifiedPhysicalScenario {
    let fixture = NativeStoreAspectFixture::segment_header("alpha", 7);
    physical_isolation_readiness_shape_scenario_from_label_and_fixture(label, fixture)
}

fn physical_isolation_readiness_shape_scenario_from_label_and_fixture(
    label: &str,
    fixture: NativeStoreAspectFixture,
) -> CertifiedPhysicalScenario {
    physical_scenario(label)
        .family(PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe)
        .intent(PhysicalScenarioIntent::ProtectBeforeObserveShape)
        .fixture(fixture.boundary_fact().clone())
        .actor(PhysicalScenarioActor::maintenance_reclaimer("reclaimer"))
        .actor(PhysicalScenarioActor::foreground_reader("reader"))
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            "root-publication-before-observe",
        ))
        .expectation(PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape())
        .certify_definition()
        .expect("native scenario should certify")
}

#[derive(Clone)]
struct ScenarioParts {
    label: &'static str,
    family: PhysicalSimulationScenarioFamily,
    intent: PhysicalScenarioIntent,
    fixture: NativeStoreAspectFixture,
    actors: [PhysicalScenarioActor; 2],
    yieldpoint: &'static str,
    fault: PhysicalScenarioFault,
    expectation: PhysicalScenarioExpectation,
}

fn baseline_scenario_parts() -> ScenarioParts {
    ScenarioParts {
        label: "store.physical.s5.readiness",
        family: PhysicalSimulationScenarioFamily::PhysicalIsolationReadinessShapeProbe,
        intent: PhysicalScenarioIntent::ProtectBeforeObserveShape,
        fixture: NativeStoreAspectFixture::segment_header("alpha", 7),
        actors: [
            PhysicalScenarioActor::maintenance_reclaimer("reclaimer"),
            PhysicalScenarioActor::foreground_reader("reader"),
        ],
        yieldpoint: "root-publication-before-observe",
        fault: PhysicalScenarioFault::no_fault(),
        expectation: PhysicalScenarioExpectation::non_claiming_physical_isolation_readiness_shape(),
    }
}

fn scenario_from_parts(parts: ScenarioParts) -> CertifiedPhysicalScenario {
    let [first_actor, second_actor] = parts.actors;
    physical_scenario(parts.label)
        .family(parts.family)
        .intent(parts.intent)
        .fixture(parts.fixture.boundary_fact().clone())
        .actor(first_actor)
        .actor(second_actor)
        .schedule(PhysicalScenarioSchedule::named_boundary_yieldpoint(
            parts.yieldpoint,
        ))
        .fault(parts.fault)
        .expectation(parts.expectation)
        .certify_definition()
        .expect("native scenario should certify")
}

fn assert_identity_differs(
    baseline: &CertifiedPhysicalScenario,
    variant: CertifiedPhysicalScenario,
    field: &str,
) {
    assert_ne!(
        baseline.identity(),
        variant.identity(),
        "{field} must participate in scenario identity"
    );
}
