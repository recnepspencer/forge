use super::support::{
    apply_text, contact_submit_source, prepared_app_with_live_view_source, submit_interaction,
};

#[test]
fn payload_projection_hot_reload_changes_next_submit_payload_shape() {
    let mut app = prepared_app_with_live_view_source(contact_submit_source("payload_values"));
    apply_text(&mut app, "first_name", "Esther");
    apply_text(&mut app, "contact_mode", "no");
    let proof = app
        .live_view_projection_proof()
        .expect("initial projection admits");
    let first = submit_interaction(&app, proof.interactions().first().expect("submit exists"));
    assert_eq!(first.emitted_payload().shape_token(), "payload");

    let next = app
        .hot_reload_live_view_source(contact_submit_source("data_payload_values"))
        .expect("payload shape hot reload admits");
    assert!(next
        .payloads()
        .iter()
        .any(|payload| payload.shape().token() == "data_payload_values"));
    let second = submit_interaction(&app, next.interactions().first().expect("submit exists"));
    assert_eq!(second.emitted_payload().shape_token(), "data_payload");
    assert_ne!(first.submission_digest(), second.submission_digest());
}

#[test]
fn retained_hidden_control_value_does_not_enter_payload_when_not_participating() {
    let mut app = prepared_app_with_live_view_source(contact_submit_source("payload_values"));
    apply_text(&mut app, "first_name", "Esther");
    apply_text(&mut app, "contact_mode", "yes");
    apply_text(&mut app, "company_name", "Worth");
    apply_text(&mut app, "contact_mode", "no");

    let proof = app
        .live_view_projection_proof()
        .expect("projection admits with retained hidden company value");
    let receipt = submit_interaction(&app, proof.interactions().first().expect("submit exists"));
    let field_names = receipt
        .emitted_payload()
        .fields()
        .iter()
        .map(|field| field.name().to_owned())
        .collect::<Vec<_>>();

    assert_eq!(field_names, vec!["first_name", "contact_mode"]);
}
