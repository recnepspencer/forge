use super::super::*;

pub(super) fn workspace_with_selector_obligation(
    name: &str,
    selector: WorthQueryGraphTouchSelector,
    lane: WorthQueryGraphObligationSupportLane,
    world: WorthQueryGraphObligationOperatingWorldSelector,
) -> WorthQueryWorkspace {
    workspace_with_registrations(
        name,
        [read_selector_registration(
            "selector", selector, lane, world,
        )],
    )
}

pub(super) fn workspace_with_selector_catalog(name: &str) -> WorthQueryWorkspace {
    let registrations = [
        read_selector_registration(
            "matching",
            WorthQueryGraphTouchSelector::collection("user").unwrap(),
            WorthQueryGraphObligationSupportLane::ReadFamily,
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        read_selector_registration(
            "unrelated",
            WorthQueryGraphTouchSelector::collection("unrelated").unwrap(),
            WorthQueryGraphObligationSupportLane::ReadFamily,
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        read_selector_registration(
            "wrong-world",
            WorthQueryGraphTouchSelector::collection("user").unwrap(),
            WorthQueryGraphObligationSupportLane::ReadFamily,
            WorthQueryGraphObligationOperatingWorldSelector::branch(),
        ),
        read_selector_registration(
            "wrong-lane",
            WorthQueryGraphTouchSelector::read_verb(
                WorthQueryGraphTouchReadVerb::RetainsLiveSubscription,
            ),
            WorthQueryGraphObligationSupportLane::LiveRead,
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        read_selector_registration(
            "mutation-only",
            WorthQueryGraphTouchSelector::mutation_family(WorthQueryMutationFamily::Assertion),
            WorthQueryGraphObligationSupportLane::ReadFamily,
            WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
    ];
    workspace_with_registrations(name, registrations)
}

fn workspace_with_registrations(
    name: &str,
    registrations: impl IntoIterator<Item = WorthQueryGraphObligationRegistration>,
) -> WorthQueryWorkspace {
    let mut builder = complete_backend_from_parts_builder();
    for registration in registrations {
        builder = builder.graph_obligation(registration);
    }
    let runtime = builder
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with graph obligation catalog");
    WorthQueryWorkspace::new(name, runtime).expect("workspace should build")
}

fn read_selector_registration(
    label: &str,
    selector: WorthQueryGraphTouchSelector,
    lane: WorthQueryGraphObligationSupportLane,
    world: WorthQueryGraphObligationOperatingWorldSelector,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::advisory_obligation(
        WorthQueryGraphObligationRuleIdentity::new("test.read-hardening", label, "v1").unwrap(),
        selector,
        world,
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(lane))
}
