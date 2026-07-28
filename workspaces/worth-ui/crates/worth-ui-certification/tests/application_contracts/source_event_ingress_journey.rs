use worth_ui::facade::app::WorthUi;
use worth_ui::facade::source::{
    WorthUiSourceIngressExt, WorthUiSourceProvider, WorthUiWatcherEvent,
};

#[test]
fn named_source_event_ingress_produces_a_preparable_replacement() {
    let app = WorthUi::app()
        .freeze()
        .expect("empty application preparation should succeed");
    let session = app.launch().expect("empty application should launch");
    let provider =
        WorthUiSourceProvider::in_memory("source-event-journey").with_file("app/main.wui", "");
    let mut ingress = session.source_event_ingress(provider).start();
    let settled = ingress
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-event-journey",
        )])
        .expect("one provider revision should settle");
    assert_eq!(settled.counters().source_revisions_emitted(), 1);
    let submission = settled
        .attempt_candidate_for_certification(session.capabilities())
        .expect("settled source should lower through the DSL handoff");
    assert_eq!(submission.counters().candidate_submissions_emitted(), 1);
    let replacement = session
        .prepare_replacement(submission)
        .expect("the public session should prepare the candidate");
    drop(replacement);
    let _ = session.shutdown();
}
