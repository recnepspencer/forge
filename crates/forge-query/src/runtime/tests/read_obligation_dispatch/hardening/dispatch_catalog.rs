use super::super::*;

pub(super) fn workspace_with_selector_obligation(
    name: &str,
    selector: ForgeQueryGraphTouchSelector,
    lane: ForgeQueryGraphObligationSupportLane,
    world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryWorkspace {
    workspace_with_registrations(
        name,
        [read_selector_registration(
            "selector", selector, lane, world,
        )],
    )
}

pub(super) fn workspace_with_selector_catalog(name: &str) -> ForgeQueryWorkspace {
    let registrations = [
        read_selector_registration(
            "matching",
            ForgeQueryGraphTouchSelector::collection("user").unwrap(),
            ForgeQueryGraphObligationSupportLane::ReadFamily,
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        read_selector_registration(
            "unrelated",
            ForgeQueryGraphTouchSelector::collection("unrelated").unwrap(),
            ForgeQueryGraphObligationSupportLane::ReadFamily,
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        read_selector_registration(
            "wrong-world",
            ForgeQueryGraphTouchSelector::collection("user").unwrap(),
            ForgeQueryGraphObligationSupportLane::ReadFamily,
            ForgeQueryGraphObligationOperatingWorldSelector::branch(),
        ),
        read_selector_registration(
            "wrong-lane",
            ForgeQueryGraphTouchSelector::read_verb(
                ForgeQueryGraphTouchReadVerb::RetainsLiveSubscription,
            ),
            ForgeQueryGraphObligationSupportLane::LiveRead,
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
        read_selector_registration(
            "mutation-only",
            ForgeQueryGraphTouchSelector::mutation_family(ForgeQueryMutationFamily::Assertion),
            ForgeQueryGraphObligationSupportLane::ReadFamily,
            ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
        ),
    ];
    workspace_with_registrations(name, registrations)
}

fn workspace_with_registrations(
    name: &str,
    registrations: impl IntoIterator<Item = ForgeQueryGraphObligationRegistration>,
) -> ForgeQueryWorkspace {
    let mut builder = complete_backend_from_parts_builder();
    for registration in registrations {
        builder = builder.graph_obligation(registration);
    }
    let runtime = builder
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with graph obligation catalog");
    ForgeQueryWorkspace::new(name, runtime).expect("workspace should build")
}

fn read_selector_registration(
    label: &str,
    selector: ForgeQueryGraphTouchSelector,
    lane: ForgeQueryGraphObligationSupportLane,
    world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::advisory_obligation(
        ForgeQueryGraphObligationRuleIdentity::new("test.read-hardening", label, "v1").unwrap(),
        selector,
        world,
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(lane))
}
