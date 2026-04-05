use crate::facade::publication::PatchStreamReadErrorClass;
use crate::tests::support::*;

#[test]
fn patch_stream_resume_batches_commits_without_duplication() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let _second = create_entity_outcome(&mut runtime, "b");
    let _third = create_entity_outcome(&mut runtime, "c");

    let first_batch = runtime
        .publication()
        .read_patch_stream(PatchStreamRequest {
            after_position: None,
            max_commits: 2,
        })
        .unwrap();
    let resumed = runtime
        .publication()
        .read_patch_stream(PatchStreamRequest {
            after_position: first_batch.next_position,
            max_commits: 2,
        })
        .unwrap();

    assert_eq!(first_batch.patches.len(), 2);
    assert_eq!(first_batch.next_position, Some(PatchStreamPosition(2)));
    assert_eq!(first_batch.latest_position, Some(PatchStreamPosition(3)));
    assert_eq!(resumed.patches.len(), 1);
    assert_eq!(resumed.resumed_after, Some(PatchStreamPosition(2)));
    assert_eq!(resumed.patches[0].position, PatchStreamPosition(3));
}

#[test]
fn patch_stream_rejects_unknown_resume_position() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    let error = runtime
        .publication()
        .read_patch_stream(PatchStreamRequest {
            after_position: Some(PatchStreamPosition(99)),
            max_commits: 1,
        })
        .unwrap_err();

    assert_eq!(
        error.class,
        PatchStreamReadErrorClass::UnknownResumePosition
    );
}
