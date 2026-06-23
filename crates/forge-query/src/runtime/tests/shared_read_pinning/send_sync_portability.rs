use super::*;

#[test]
fn shared_read_context_is_send_sync_and_resolves_in_scoped_thread() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<crate::runtime::ForgeQuerySharedReadContext>();
    assert_send_sync::<crate::runtime::ForgeQueryPublishedDerivedArtifactHandle>();

    let (workspace, derived) = shared_read_pinning_workspace("shared-read.phase13.send-sync");
    let read = workspace
        .shared_read_context()
        .expect("shared read context should mint");
    let serial_binding = read
        .published_derived_artifact(&derived)
        .expect("serial artifact should resolve")
        .published_binding()
        .expect("serial artifact should be published")
        .binding_for_reporting()
        .to_string();
    let sibling_read = read.clone();

    let (first_thread_binding, second_thread_binding) = std::thread::scope(|scope| {
        let first_handle = scope.spawn(|| {
            read.published_derived_artifact(&derived)
                .expect("scoped thread should resolve through the real shared-read context")
                .published_binding()
                .expect("thread artifact should be published")
                .binding_for_reporting()
                .to_string()
        });
        let second_handle = scope.spawn(|| {
            sibling_read
                .published_derived_artifact(&derived)
                .expect("second scoped thread should resolve through a cloned real context")
                .published_binding()
                .expect("second thread artifact should be published")
                .binding_for_reporting()
                .to_string()
        });
        let first_thread_binding = first_handle
            .join()
            .expect("first scoped thread should finish");
        assert_eq!(first_thread_binding, serial_binding);
        let second_thread_binding = second_handle
            .join()
            .expect("second scoped thread should finish");
        assert_eq!(second_thread_binding, serial_binding);
        (first_thread_binding, second_thread_binding)
    });
    let portability = ForgeQuerySharedReadPortabilityEvidence::proven(evidence_digest(
        "shared-read-send-sync-scoped-thread",
        [
            ("serial_binding", serial_binding),
            ("first_thread_binding", first_thread_binding),
            ("second_thread_binding", second_thread_binding),
        ],
    ));
    assert!(portability.proven_by_scoped_thread());
}
