use worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture;

use crate::graph::UiGraphWorldProfile;

pub(crate) fn display_field_projection_consumption(
    lane_label: &str,
) -> (
    crate::capability::ViewBindingId,
    worth_ui_query_binding::WorthUiSettledSnapshotFact,
) {
    let view_binding_identity = format!("{lane_label}.view").replace('-', "_");
    let fact = settled_fact_for_label(lane_label);
    (
        crate::capability::ViewBindingId::new(view_binding_identity).unwrap(),
        fact,
    )
}

pub(crate) fn display_field_projection_context(
    lane_label: &str,
) -> (
    crate::capability::ViewBindingId,
    worth_ui_query_binding::WorthUiSettledSnapshotFact,
    UiGraphWorldProfile,
) {
    projection_context(lane_label)
}

fn projection_context(
    lane_label: &str,
) -> (
    crate::capability::ViewBindingId,
    worth_ui_query_binding::WorthUiSettledSnapshotFact,
    UiGraphWorldProfile,
) {
    let (view_binding_id, fact) = display_field_projection_consumption(lane_label);
    let world_profile = UiGraphWorldProfile::settled_query_fact(view_binding_id.clone(), &fact);
    (view_binding_id, fact, world_profile)
}

fn settled_fact_for_label(lane_label: &str) -> worth_ui_query_binding::WorthUiSettledSnapshotFact {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    static FACTS: OnceLock<
        Mutex<HashMap<String, worth_ui_query_binding::WorthUiSettledSnapshotFact>>,
    > = OnceLock::new();
    let facts = FACTS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut facts = facts.lock().expect("settled fact fixture cache lock");
    facts
        .entry(lane_label.to_owned())
        .or_insert_with(|| {
            let mut fixture = WorthUiInstalledQueryTestFixture::new(lane_label);
            fixture.clone_retained_fact_for_isolated_test()
        })
        .clone()
}
